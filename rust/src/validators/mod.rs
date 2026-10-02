//! WDL validators — three-level hierarchy mirroring the Java implementation.
//!
//! `WdlValidator`              — base: function version + base function checks
//! `WdlStaticAnalysisValidator`— static: + duplicate/unknown-ref/type checks
//! `WdlLintingValidator`       — lint: + unused-symbol/deprecated warnings

#![allow(clippy::too_many_arguments)]

use std::collections::{HashMap, HashSet};
use std::mem;

use indexmap::IndexMap;

use crate::definitions::{
    WdlEnum, WdlStruct, WdlStructElement, WdlTask, WdlTaskElement, WdlWorkflow, WdlWorkflowElement,
};
use crate::document::{WdlDocument, WdlDocumentElement};
use crate::errors::{WdlError, WdlErrorCode, WdlSemanticError};
use crate::expressions::{
    BinaryOperator, FunctionTypeHint, UnaryOperator, WdlExpression, WdlFunction,
    WdlFunctionCallOperation, WdlStringComponent, WdlStringPlaceholderOption,
};
use crate::processors::base::import_namespace;
use crate::processors::render::type_to_wdl;
use crate::sections::InputDeclaration;
use crate::statements::{
    WdlBoundDeclaration, WdlCall, WdlConditional, WdlImport, WdlImportMember, WdlScatter,
    WdlStatement,
};
use crate::types::{
    WdlArrayType, WdlMapType, WdlPairType, WdlPrimitiveKind, WdlPrimitiveType, WdlType,
    WdlTypeRefType,
};
use crate::version::WdlVersion;

// ──────────────────────────────────────────────────────────────────────────────
// Data types
// ──────────────────────────────────────────────────────────────────────────────

/// Lightweight constant-folded value used during expression evaluation.
#[derive(Debug, Clone)]
pub enum EvalValue {
    Unknown,
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<EvalValue>),
    /// Map stored as Vec of pairs because `f64` is not `Hash`.
    Map(Vec<(EvalValue, EvalValue)>),
    Pair(Box<EvalValue>, Box<EvalValue>),
}

/// Callable contract for one task or workflow.
#[derive(Debug, Clone, Default)]
struct CallableContract {
    required_inputs: HashSet<String>,
    input_types: HashMap<String, WdlType>,
    outputs: HashSet<String>,
    output_types: HashMap<String, WdlType>,
    private_declarations: HashSet<String>,
}

/// Structural description of a struct, used for compatibility checks.
#[derive(Debug, Clone)]
struct StructShape {
    /// Member names → WDL type string (for equality check).
    ordered_member_type_wdl: IndexMap<String, String>,
    /// Member names → `WdlType` (for type access).
    ordered_member_types: IndexMap<String, WdlType>,
}

impl StructShape {
    /// Structural equality, mirroring Java `StructShape.isCompatibleWith`
    /// (`WdlValidator.java:1060`). Only the rendered member types participate;
    /// `IndexMap` equality is order-independent, matching Java's `Map.equals`.
    fn is_compatible_with(&self, other: &StructShape) -> bool {
        self.ordered_member_type_wdl == other.ordered_member_type_wdl
    }
}

/// Structural description of an enum.
///
/// The two choice collections serve deliberately different purposes and are
/// rendered differently, mirroring Java's `enumShapes` / `enumChoiceNames` split
/// (`WdlValidator.java:65-67`):
///
/// - `choice_names` answers "is this string a legal choice name?" — required by
///   the `Enum` ← `String` coercion rule in WDL 1.3 `SPEC.md:1965`.
/// - `choice_shapes` answers "are these two enums the same enum?" across imports.
///
/// `effective_value_type` folds in Java's third map, `enumValueTypes`
/// (`WdlValidator.java:66`). Java writes all three maps on consecutive lines
/// under the same key (`L617-619`, `L429-431`), so a field cannot desync where
/// parallel maps could.
#[derive(Debug, Clone)]
struct EnumShape {
    /// Rendered *declared* value type, or `<implicit>` when the enum has no
    /// `enum Foo[Type]` parameter. Compared by `is_compatible_with`.
    value_type_wdl: String,
    /// Bare choice names, e.g. `"Red"`. Drives the membership test in
    /// `is_assignable_from`. Unordered: only `contains` is ever called.
    choice_names: HashSet<String>,
    /// Uniform `NAME=value` renderings, e.g. `"Red=\"#FF0000\""`, with `<none>`
    /// for a valueless choice. Order is significant — compared as a sequence.
    choice_shapes: Vec<String>,
    /// *Effective* value type: the declared type when present, otherwise the
    /// type inferred from the choice values, otherwise `String`. Mirrors Java
    /// `effectiveEnumValueType` (`WdlValidator.java:622-625`). Distinct from
    /// `value_type_wdl`, which is the declared rendering only.
    effective_value_type: WdlType,
}

impl EnumShape {
    /// Mirrors Java `EnumShape.isCompatibleWith` (`WdlValidator.java:1097`).
    ///
    /// Note this compares the *declared* value type rendering, not
    /// `effective_value_type` — matching Java, and the reason the value-type
    /// diagnostic in `index_imported_enum` is unreachable (see `.context/B1_plan.md` §5.4).
    fn is_compatible_with(&self, other: &EnumShape) -> bool {
        self.value_type_wdl == other.value_type_wdl && self.choice_shapes == other.choice_shapes
    }
}

/// Resolves the local name an imported type enters the importing document under.
///
/// Mirrors Java `resolveImportedTypeLocalName` (`WdlValidator.java:454-482`):
/// a `Standard` or `Members` import may rename the type via `alias X as Y`, a
/// `Star` import never renames, and a `Members` import that does not list the
/// type at all returns `None` so the type is skipped entirely.
/// Maps each type name exported by an import's source document to the local
/// name it enters this document under, for every type the import names.
///
/// Mirrors Java `importTypeAliasMap` (`WdlValidator.java:543-569`), including
/// its omission of `Star` imports: a star import never renames, so the map is
/// empty and rewriting becomes a no-op.
fn import_type_alias_map(imp: &WdlImport) -> HashMap<String, String> {
    fn collect(members: &[WdlImportMember], out: &mut HashMap<String, String>) {
        for member in members {
            if member.member.trim().is_empty() {
                continue;
            }
            let local = member
                .alias
                .as_deref()
                .filter(|a| !a.trim().is_empty())
                .unwrap_or(member.member.as_str());
            out.insert(member.member.clone(), local.to_string());
        }
    }

    let mut aliases = HashMap::new();
    match imp {
        WdlImport::Standard(std_imp) => collect(&std_imp.members, &mut aliases),
        WdlImport::Members(mem_imp) => collect(&mem_imp.members, &mut aliases),
        WdlImport::Star(_) => {}
    }
    aliases
}

fn resolve_imported_type_local_name(imp: &WdlImport, imported_type_name: &str) -> Option<String> {
    fn aliased(members: &[WdlImportMember], name: &str) -> Option<Option<String>> {
        members.iter().find(|m| m.member == name).map(|m| {
            m.alias
                .as_deref()
                .filter(|a| !a.trim().is_empty())
                .map(str::to_string)
        })
    }

    match imp {
        WdlImport::Standard(std_imp) => Some(
            aliased(&std_imp.members, imported_type_name)
                .flatten()
                .unwrap_or_else(|| imported_type_name.to_string()),
        ),
        WdlImport::Star(_) => Some(imported_type_name.to_string()),
        WdlImport::Members(mem_imp) => aliased(&mem_imp.members, imported_type_name)
            .map(|alias| alias.unwrap_or_else(|| imported_type_name.to_string())),
    }
}

/// Variable/call-output usage set — used by the lint pass.
#[derive(Debug, Default)]
struct Usage {
    used_variables: HashSet<String>,
    used_call_output_targets: HashSet<String>,
}

impl Usage {
    fn merge(&mut self, other: Usage) {
        self.used_variables.extend(other.used_variables);
        self.used_call_output_targets
            .extend(other.used_call_output_targets);
    }
}

/// Which validation level the runner operates at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidatorMode {
    Base,
    Static,
    Lint,
}

// ──────────────────────────────────────────────────────────────────────────────
// ValidatorRunner
// ──────────────────────────────────────────────────────────────────────────────

struct ValidatorRunner {
    errors: Vec<WdlSemanticError>,
    mode: ValidatorMode,
    throw_on_warnings: bool,

    // Indexed contracts (cleared per validate() call)
    callable_contracts: HashMap<String, CallableContract>,
    struct_members: HashMap<String, HashSet<String>>,
    struct_member_types: HashMap<String, HashMap<String, WdlType>>,
    enum_shapes: HashMap<String, EnumShape>,

    // Per-workflow scope (save/restore with mem::take)
    scope_types: HashMap<String, WdlType>,
    scope_values: HashMap<String, EvalValue>,
    call_outputs: HashMap<String, HashSet<String>>,
    call_output_types: HashMap<String, HashMap<String, WdlType>>,

    // Per-document context
    current_doc_version: Option<WdlVersion>,

    // Static/Lint pre-scan
    known_callable_targets: HashSet<String>,
    known_type_names: HashSet<String>,
}

impl ValidatorRunner {
    fn new(mode: ValidatorMode, throw_on_warnings: bool) -> Self {
        Self {
            errors: Vec::new(),
            mode,
            throw_on_warnings,
            callable_contracts: HashMap::new(),
            struct_members: HashMap::new(),
            struct_member_types: HashMap::new(),
            enum_shapes: HashMap::new(),
            scope_types: HashMap::new(),
            scope_values: HashMap::new(),
            call_outputs: HashMap::new(),
            call_output_types: HashMap::new(),
            current_doc_version: None,
            known_callable_targets: HashSet::new(),
            known_type_names: HashSet::new(),
        }
    }

    fn add_error(&mut self, code: WdlErrorCode, msg: impl Into<String>) {
        self.errors.push(WdlSemanticError::new(code, msg, 0, 0));
    }

    fn should_throw(&self) -> bool {
        self.errors.iter().any(|e| {
            e.severity() == crate::errors::Severity::Error
                || (self.throw_on_warnings && e.severity() == crate::errors::Severity::Warning)
        })
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Expression evaluation helpers
    // ──────────────────────────────────────────────────────────────────────────

    fn eval_expr(&self, expr: &WdlExpression) -> EvalValue {
        match expr {
            WdlExpression::BoolLit(v) => EvalValue::Bool(*v),
            WdlExpression::IntLit(v) => EvalValue::Int(*v),
            WdlExpression::FloatLit(v) => EvalValue::Float(*v),
            WdlExpression::NullLit => EvalValue::Null,
            WdlExpression::Variable(name) => {
                if name == "None" {
                    return EvalValue::Null;
                }
                self.scope_values
                    .get(name)
                    .cloned()
                    .unwrap_or(EvalValue::Unknown)
            }
            WdlExpression::StrLit(lit) => {
                let mut s = String::new();
                let mut pure = true;
                for c in &lit.components {
                    match c {
                        WdlStringComponent::Text(t) => s.push_str(t),
                        WdlStringComponent::Escape(e) => s.push_str(e),
                        WdlStringComponent::Special(t) => s.push_str(t),
                        _ => {
                            pure = false;
                            break;
                        }
                    }
                }
                if pure {
                    EvalValue::Str(s)
                } else {
                    EvalValue::Unknown
                }
            }
            WdlExpression::ArrayLit(arr) => {
                let items = arr.entries.iter().map(|e| self.eval_expr(e)).collect();
                EvalValue::List(items)
            }
            WdlExpression::MapLit(map) => {
                let pairs = map
                    .entries
                    .iter()
                    .map(|e| {
                        let k = self.eval_expr(&e.key);
                        let v = e
                            .value
                            .as_ref()
                            .map(|v| self.eval_expr(v))
                            .unwrap_or(EvalValue::Unknown);
                        (k, v)
                    })
                    .collect();
                EvalValue::Map(pairs)
            }
            WdlExpression::PairLit(p) => EvalValue::Pair(
                Box::new(self.eval_expr(&p.left)),
                Box::new(self.eval_expr(&p.right)),
            ),
            _ => EvalValue::Unknown,
        }
    }

    fn infer_type(&self, expr: &WdlExpression) -> Option<WdlType> {
        // Enforce the `WdlType::Unknown` invariant at a single choke point.
        // `Unknown` is legal *nested* inside a container, but a bare top-level
        // `Unknown` must present as `None`, which is what Java's nullable
        // `inferType` return means. Routing every recursive call through here
        // keeps the two representations in step no matter which arm produced
        // the value — notably `IdxOp` projecting out of an `Array[Unknown]`, and
        // `Variable` reading a scatter binding whose collection was `[]`.
        Self::demote_unknown(self.infer_type_inner(expr))
    }

    fn infer_type_inner(&self, expr: &WdlExpression) -> Option<WdlType> {
        use WdlPrimitiveKind as PK;
        let prim = |k: PK| WdlType::Primitive(WdlPrimitiveType::new(k));
        match expr {
            WdlExpression::BoolLit(_) => Some(prim(PK::Boolean)),
            WdlExpression::IntLit(_) => Some(prim(PK::Int)),
            WdlExpression::FloatLit(_) => Some(prim(PK::Float)),
            WdlExpression::StrLit(_) => Some(prim(PK::String)),
            WdlExpression::NullLit => None,
            WdlExpression::Variable(name) => {
                if name == "None" {
                    return None;
                }
                self.scope_types.get(name).cloned()
            }
            WdlExpression::FuncOp(op) => self.infer_function_type(op),
            WdlExpression::ArrayLit(arr) => {
                // Mirrors `WdlExpressionValidator.inferType` (Java :294-307):
                // fold every entry, *skipping* un-inferable ones, and stop as
                // soon as two entries are irreconcilable. An empty literal, or
                // one whose entries all fail to infer, yields `Array[Unknown]`
                // rather than a guessed member type.
                let mut member: Option<WdlType> = None;
                for entry in &arr.entries {
                    let item = self.infer_type(entry);
                    if item.is_none() {
                        continue;
                    }
                    member = self.merge_types(member, item);
                    if member.is_none() {
                        break;
                    }
                }
                Some(WdlType::Array(WdlArrayType::new(
                    member.unwrap_or(WdlType::Unknown),
                )))
            }
            WdlExpression::MapLit(map) => {
                // Mirrors Java :318-326. Unlike the array arm there is no early
                // bail: an irreconcilable pair of keys (or values) simply leaves
                // that side unknown and the fold continues.
                let mut kt: Option<WdlType> = None;
                let mut vt: Option<WdlType> = None;
                for entry in &map.entries {
                    kt = self.merge_types(kt, self.infer_type(&entry.key));
                    let value = entry.value.as_ref().and_then(|v| self.infer_type(v));
                    vt = self.merge_types(vt, value);
                }
                Some(WdlType::Map(Box::new(WdlMapType::new(
                    kt.unwrap_or(WdlType::Unknown),
                    vt.unwrap_or(WdlType::Unknown),
                ))))
            }
            WdlExpression::PairLit(p) => {
                // Mirrors Java :309-316: if either side is un-inferable the
                // whole pair is un-inferable.
                let l = self.infer_type(&p.left)?;
                let r = self.infer_type(&p.right)?;
                Some(WdlType::Pair(Box::new(WdlPairType::new(l, r))))
            }
            WdlExpression::StructLit(s) => {
                Some(WdlType::TypeRef(WdlTypeRefType::new(s.name.clone())))
            }
            WdlExpression::BinaryOp(op) => {
                use BinaryOperator::*;
                let lt = self.infer_type(&op.left);
                let rt = self.infer_type(&op.right);
                match op.operator {
                    Or | And | Eq | Neq | Lt | Lte | Gt | Gte => Some(prim(PK::Boolean)),
                    Add | Subtract | Multiply | Divide | Modulo | Power => {
                        let is_float = |t: &Option<WdlType>| matches!(t, Some(WdlType::Primitive(p)) if p.primitive_kind == PK::Float);
                        if is_float(&lt) || is_float(&rt) {
                            Some(prim(PK::Float))
                        } else {
                            lt.or(rt)
                        }
                    }
                }
            }
            WdlExpression::UnaryOp(op) => match op.operator {
                UnaryOperator::Not => Some(prim(PK::Boolean)),
                UnaryOperator::Negative => self.infer_type(&op.operand),
            },
            // Mirrors Java :414-418, which merges the two branches rather than
            // taking whichever happens to infer first.
            WdlExpression::TernaryOp(op) => self.merge_types(
                self.infer_type(&op.true_value),
                self.infer_type(&op.false_value),
            ),
            WdlExpression::MemberOp(op) => {
                if let WdlExpression::Variable(name) = op.target.as_ref() {
                    if let Some(outputs) = self.call_output_types.get(name.as_str()) {
                        return outputs.get(&op.member).cloned();
                    }
                    if let Some(WdlType::TypeRef(tr)) = self.scope_types.get(name.as_str()) {
                        let ref_name = tr.reference_name.clone();
                        if let Some(members) = self.struct_member_types.get(&ref_name) {
                            return members.get(&op.member).cloned();
                        }
                    }
                }
                None
            }
            WdlExpression::IdxOp(op) => {
                let target_ty = self.infer_type(&op.target)?;
                match target_ty {
                    WdlType::Array(arr) => Some(*arr.member_type),
                    WdlType::Map(map) => Some(*map.value_type),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn infer_function_type(&self, op: &WdlFunctionCallOperation) -> Option<WdlType> {
        use WdlFunction::*;
        use WdlPrimitiveKind as PK;
        let prim = |k: PK| WdlType::Primitive(WdlPrimitiveType::new(k));
        let arr = |t: WdlType| WdlType::Array(WdlArrayType::new(t));
        match op.function {
            Floor | Ceil | Round | ReadInt | Length => Some(prim(PK::Int)),
            ReadFloat | Size => Some(prim(PK::Float)),
            Sub | Basename | ReadString | Sep => Some(prim(PK::String)),
            ReadBoolean | Defined | Matches => Some(prim(PK::Boolean)),
            Stdout | Stderr | WriteLines | WriteTsv | WriteMap | WriteObject | WriteObjects
            | WriteJson => Some(prim(PK::File)),
            Find => Some(WdlType::Primitive(WdlPrimitiveType::optional(PK::String))),
            ReadLines | Prefix | Suffix | Quote | Squote => Some(arr(prim(PK::String))),
            Glob => Some(arr(prim(PK::File))),
            Range => Some(arr(prim(PK::Int))),
            _ => None,
        }
    }

    /// Collapses a bare `Unknown` to `None`.
    ///
    /// Java uses a single `null` for both "no type at all" and "unknown
    /// component type". Rust splits those into `None` and [`WdlType::Unknown`]
    /// respectively, so the top-level boundary between them has to be policed
    /// explicitly; see [`Self::infer_type`].
    ///
    /// Without this, `scatter (x in [])` would bind `x` to a bare `Unknown` and
    /// `[x, 1]` would infer `Array[Unknown]`, because the array fold skips
    /// `None` but would happily merge a bare `Unknown`. Java infers `Array[Int]`.
    fn demote_unknown(ty: Option<WdlType>) -> Option<WdlType> {
        match ty {
            Some(WdlType::Unknown) => None,
            other => other,
        }
    }

    /// Merges two inferred types into the most general type that accepts both.
    ///
    /// Mirrors `WdlExpressionValidator.mergeTypes` (Java :673-695), including
    /// its treatment of an un-inferable operand as "no information" rather than
    /// as a failure. Returns `None` when the two are irreconcilable.
    fn merge_types(&self, a: Option<WdlType>, b: Option<WdlType>) -> Option<WdlType> {
        use WdlPrimitiveKind as PK;
        let (a, b) = match (a, b) {
            (current, None) => return current,
            (None, next) => return next,
            (Some(a), Some(b)) => (a, b),
        };
        if self.is_type_assignable(&a, &b) {
            return Some(a);
        }
        if self.is_type_assignable(&b, &a) {
            return Some(b);
        }
        // Int and Float are mutually unassignable in the `actual → expected`
        // direction, but a literal containing both widens to Float.
        let is_prim = |t: &WdlType, k: PK| matches!(t, WdlType::Primitive(p) if p.primitive_kind == k && !p.optional);
        if (is_prim(&a, PK::Int) && is_prim(&b, PK::Float))
            || (is_prim(&a, PK::Float) && is_prim(&b, PK::Int))
        {
            return Some(WdlType::Primitive(WdlPrimitiveType::new(PK::Float)));
        }
        None
    }

    /// Returns `true` if `actual` can be assigned to `expected`.
    fn is_type_assignable(&self, expected: &WdlType, actual: &WdlType) -> bool {
        use WdlPrimitiveKind as PK;
        // An un-inferable type on either side is assumed compatible. This sits
        // above the optionality guard to match Java :697-703, and is what makes
        // `Array[Int] a = []` (member type `Unknown`) diagnostic-free.
        if matches!(expected, WdlType::Unknown) || matches!(actual, WdlType::Unknown) {
            return true;
        }
        if expected == actual {
            return true;
        }
        if !expected.is_optional() && actual.is_optional() {
            return false;
        }
        match (expected, actual) {
            // Int → Float promotion
            (WdlType::Primitive(e), WdlType::Primitive(a))
                if e.primitive_kind == PK::Float && a.primitive_kind == PK::Int =>
            {
                true
            }

            // String → File / Directory coercion (WDL spec: string literals/values
            // are assignable to File and Directory typed declarations).
            (WdlType::Primitive(e), WdlType::Primitive(a))
                if (e.primitive_kind == PK::File || e.primitive_kind == PK::Directory)
                    && a.primitive_kind == PK::String =>
            {
                true
            }

            // File / Directory → String coercion. NOT in the spec's coercion
            // table, and no other implementation has it — but it is required by
            // the normative spec example `placeholder_coercion.wdl`, which does
            // `File x` / `String x_as_str = x`. See `.context/C1_plan.md` §5.1.
            (WdlType::Primitive(e), WdlType::Primitive(a))
                if e.primitive_kind == PK::String
                    && (a.primitive_kind == PK::File || a.primitive_kind == PK::Directory) =>
            {
                true
            }

            // Optional expected: strip optional and re-check
            (e, a) if e.is_optional() && !a.is_optional() => {
                let e2 = e.clone().with_optional(false);
                let a2 = a.clone().with_optional(false);
                self.is_type_assignable(&e2, &a2)
            }

            // Array[T] ← Array[U] if T accepts U
            (WdlType::Array(e), WdlType::Array(a)) => {
                self.is_type_assignable(&e.member_type, &a.member_type)
            }

            // Map[K,V] ← Map[K2,V2] if K accepts K2 and V accepts V2
            (WdlType::Map(e), WdlType::Map(a)) => {
                self.is_type_assignable(&e.key_type, &a.key_type)
                    && self.is_type_assignable(&e.value_type, &a.value_type)
            }

            // Pair[L,R] ← Pair[L2,R2]
            (WdlType::Pair(e), WdlType::Pair(a)) => {
                self.is_type_assignable(&e.left_type, &a.left_type)
                    && self.is_type_assignable(&e.right_type, &a.right_type)
            }

            // TypeRef: same name, or structurally compatible structs (same
            // member names, each with an assignable type — mirrors WDL's
            // structural struct-to-struct coercion).
            (WdlType::TypeRef(e), WdlType::TypeRef(a)) => {
                if e.reference_name == a.reference_name {
                    return true;
                }
                match (
                    self.struct_member_types.get(&e.reference_name),
                    self.struct_member_types.get(&a.reference_name),
                ) {
                    (Some(expected_fields), Some(actual_fields)) => {
                        expected_fields.len() == actual_fields.len()
                            && expected_fields.iter().all(|(name, ty)| {
                                actual_fields
                                    .get(name)
                                    .is_some_and(|at| self.is_type_assignable(ty, at))
                            })
                    }
                    _ => false,
                }
            }

            _ => false,
        }
    }

    fn is_assignable_from(&self, expected: &WdlType, expr: &WdlExpression) -> bool {
        // Null literal is assignable to any optional type
        if matches!(expr, WdlExpression::NullLit)
            || matches!(expr, WdlExpression::Variable(n) if n == "None")
        {
            return expected.is_optional();
        }

        // Struct type-ref ← struct / object / map literal. Java recurses over
        // the *expression* here instead of comparing inferred types, which is
        // what makes `Words w = { "a": 1, "b": 2, "c": 3 }` (map coercion to a
        // struct) legal — the inferred type would be `Map[String, Int]`, which
        // is not assignable to `Words`. Mirrors
        // `WdlExpressionValidator.isAssignableFrom` :210-215, and sits before
        // the enum check to match Java's ordering.
        if let WdlType::TypeRef(tr) = expected {
            if self.struct_member_types.contains_key(&tr.reference_name)
                && matches!(
                    expr,
                    WdlExpression::StructLit(_)
                        | WdlExpression::ObjLit(_)
                        | WdlExpression::MapLit(_)
                )
            {
                return self.is_struct_assignable_from_expr(&tr.reference_name, expr);
            }
        }

        // Enum choice check — if the expected type is a TypeRef pointing at
        // a known enum, and the expression evaluates to a string constant,
        // require the string to be one of the enum's declared choice *names*.
        // WDL 1.3 `SPEC.md:1965`: for `Enum` ← `String`, the "`String` value must
        // exactly match one of the enum's choice names".
        // Matches Java `WdlExpressionValidator.isAssignableFrom` L216-224.
        if let WdlType::TypeRef(tr) = expected {
            if let Some(enum_shape) = self.enum_shapes.get(&tr.reference_name) {
                if let EvalValue::Str(s) = self.eval_expr(expr) {
                    // An empty name set means we have nothing to check against;
                    // fall through rather than reject (Java guards the same way
                    // with `choices != null && !choices.isEmpty()`).
                    if !enum_shape.choice_names.is_empty() {
                        return enum_shape.choice_names.contains(&s);
                    }
                }
                // Unknown evaluation — fall through to type-based check
                // (conservative: assume compatible if we can't evaluate).
            }
        }

        match self.infer_type(expr) {
            Some(actual) => self.is_type_assignable(expected, &actual),
            None => true, // can't infer → assume compatible
        }
    }

    /// Structural assignability of a struct, object, or map literal to a named
    /// struct type.
    ///
    /// Mirrors `WdlExpressionValidator.isStructAssignableFromExpression`
    /// (:776-817), except for the member-presence rule -- see
    /// [`Self::keyed_entries_match_members`].
    fn is_struct_assignable_from_expr(&self, struct_name: &str, expr: &WdlExpression) -> bool {
        let Some(expected_members) = self.struct_member_types.get(struct_name) else {
            return false;
        };

        let keyed: Vec<(String, Option<&WdlExpression>)> = match expr {
            WdlExpression::StructLit(lit) => {
                // A named struct literal must name a compatible struct. Java
                // skips this when the name is absent or blank (:785).
                if !lit.name.trim().is_empty() && lit.name != struct_name {
                    let expected_ref =
                        WdlType::TypeRef(WdlTypeRefType::new(struct_name.to_string()));
                    let actual_ref = WdlType::TypeRef(WdlTypeRefType::new(lit.name.clone()));
                    if !self.is_type_assignable(&expected_ref, &actual_ref) {
                        return false;
                    }
                }
                lit.entries
                    .iter()
                    .map(|e| (e.key.clone(), e.value.as_ref()))
                    .collect()
            }
            WdlExpression::ObjLit(lit) => lit
                .entries
                .iter()
                .map(|e| (e.key.clone(), e.value.as_ref()))
                .collect(),
            WdlExpression::MapLit(lit) => {
                let mut entries = Vec::with_capacity(lit.entries.len());
                for entry in &lit.entries {
                    // Java requires every key to evaluate to a String (:799-802);
                    // anything else makes the whole literal unassignable.
                    match self.eval_expr(&entry.key) {
                        EvalValue::Str(key) => entries.push((key, entry.value.as_ref())),
                        _ => return false,
                    }
                }
                entries
            }
            _ => return false,
        };

        self.keyed_entries_match_members(expected_members, &keyed)
    }

    /// Every key in the literal must name a declared member and carry an
    /// assignable value. Members absent from the literal are **not** rejected.
    ///
    /// This deliberately diverges from Java's
    /// `keyedEntriesMatchExpectedMembers` (:821-836), which compares key *sets*
    /// for equality and so rejects any literal that omits a member, even an
    /// optional one. Java is wrong here, and provably so: the normative spec
    /// example `test_struct.wdl` omits the optional `String? username` and
    /// carries the comment "it's okay to leave out username since it's
    /// optional". Running Java's own validator over that file rejects it.
    /// Java never notices because its spec-example tests only parse valid
    /// examples; the validator runs only on `_fail.wdl` files. Recorded in
    /// `java/TODO.md`.
    ///
    /// Adopting Java's rule here would newly reject `test_struct.wdl` and
    /// `import_structs.wdl`, both of which this validator accepts today.
    ///
    /// Note the spec's actual rule is stronger than what is implemented here:
    /// optional members may be omitted but *required* members may not.
    /// `incomplete_struct_fail.wdl` is meant to pin that from the other side
    /// ("error! missing required account_number"), but it writes its struct
    /// literal with quoted string keys, which the grammar rejects, so it fails
    /// at parse time and never reaches semantic validation -- in this
    /// implementation or in Java. Nothing in the corpus currently exercises the
    /// required-member rule, and enforcement is deliberately left out so that
    /// this change adds no presence checking Rust did not already do; see
    /// `rust/.context/B4_plan.md`.
    fn keyed_entries_match_members(
        &self,
        expected_members: &HashMap<String, WdlType>,
        actual: &[(String, Option<&WdlExpression>)],
    ) -> bool {
        let mut seen: HashMap<&str, Option<&WdlExpression>> = HashMap::new();
        for (key, value) in actual {
            // A key that names no declared member cannot be type-checked at all.
            if !expected_members.contains_key(key.as_str()) {
                return false;
            }
            seen.insert(key.as_str(), *value);
        }
        seen.iter().all(|(member, value)| match value {
            Some(expr) => expected_members
                .get(*member)
                .is_some_and(|ty| self.is_assignable_from(ty, expr)),
            // A keyed entry with no value. Java reaches
            // `isAssignableFrom(type, null)`, which returns true (:201-203).
            None => true,
        })
    }

    fn contains_non_string_map_key(&self, expr: &WdlExpression) -> bool {
        if let WdlExpression::MapLit(m) = expr {
            return m.entries.iter().any(|e| match self.infer_type(&e.key) {
                Some(WdlType::Primitive(p)) => p.primitive_kind != WdlPrimitiveKind::String,
                Some(_) => true,
                None => false,
            });
        }
        false
    }

    // ─── type predicates ───────────────────────────────────────────────────────

    fn is_numeric(&self, ty: &WdlType) -> bool {
        matches!(
            ty,
            WdlType::Primitive(p)
                if p.primitive_kind == WdlPrimitiveKind::Int
                    || p.primitive_kind == WdlPrimitiveKind::Float
        )
    }

    fn is_boolean(&self, ty: &WdlType) -> bool {
        matches!(
            ty,
            WdlType::Primitive(p) if p.primitive_kind == WdlPrimitiveKind::Boolean
        )
    }

    fn is_string(&self, ty: &WdlType) -> bool {
        matches!(
            ty,
            WdlType::Primitive(p) if p.primitive_kind == WdlPrimitiveKind::String
        )
    }

    fn is_path_like(&self, ty: &WdlType) -> bool {
        matches!(
            ty,
            WdlType::Primitive(p)
                if p.primitive_kind == WdlPrimitiveKind::File
                    || p.primitive_kind == WdlPrimitiveKind::Directory
                    || p.primitive_kind == WdlPrimitiveKind::String
        )
    }

    fn is_orderable(&self, ty: &WdlType) -> bool {
        match ty {
            WdlType::Primitive(p) => matches!(
                p.primitive_kind,
                WdlPrimitiveKind::Int
                    | WdlPrimitiveKind::Float
                    | WdlPrimitiveKind::String
                    | WdlPrimitiveKind::Boolean
            ),
            _ => false,
        }
    }

    fn are_order_comparable(&self, l: Option<&WdlType>, r: Option<&WdlType>) -> bool {
        match (l, r) {
            (Some(lt), Some(rt)) => self.is_orderable(lt) && self.is_orderable(rt),
            _ => true, // unknown → assume ok
        }
    }

    fn matches_signature_type(&self, actual: Option<&WdlType>, hint: FunctionTypeHint) -> bool {
        use FunctionTypeHint as T;
        use WdlPrimitiveKind as PK;
        match hint {
            T::Any | T::AnyOptional => true,
            T::Number => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::Int || p.primitive_kind == PK::Float),
            T::Boolean => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::Boolean),
            T::Int => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::Int),
            T::Float => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::Float),
            T::String | T::StringOptional => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::String),
            T::File => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::File),
            T::Directory => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::Directory),
            // FileOrDirectory also accepts String (path-like)
            T::FileOrDirectory => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::File
                    || p.primitive_kind == PK::Directory
                    || p.primitive_kind == PK::String),
            T::Object => matches!(actual, Some(WdlType::Primitive(p))
                if p.primitive_kind == PK::Object),
            T::ArrayAny | T::ArrayOptionalAny => {
                matches!(actual, Some(WdlType::Array(_))) || actual.is_none()
            }
            T::ArrayFile => matches!(actual, Some(WdlType::Array(a))
                if matches!(a.member_type.as_ref(), WdlType::Primitive(p)
                    if p.primitive_kind == PK::File)),
            T::ArrayInt => matches!(actual, Some(WdlType::Array(a))
                if matches!(a.member_type.as_ref(), WdlType::Primitive(p)
                    if p.primitive_kind == PK::Int)),
            T::ArrayString => matches!(actual, Some(WdlType::Array(a))
                if matches!(a.member_type.as_ref(), WdlType::Primitive(p)
                    if p.primitive_kind == PK::String)),
            T::ArrayObject => matches!(actual, Some(WdlType::Array(a))
                if matches!(a.member_type.as_ref(), WdlType::Primitive(p)
                    if p.primitive_kind == PK::Object)),
            T::ArrayPair => matches!(actual, Some(WdlType::Array(a))
                if matches!(a.member_type.as_ref(), WdlType::Pair(_))),
            T::ArrayArrayAny | T::ArrayArrayString => matches!(actual, Some(WdlType::Array(a))
                if matches!(a.member_type.as_ref(), WdlType::Array(_))),
            T::MapAnyAny | T::MapAnyArray => {
                matches!(actual, Some(WdlType::Map(_))) || actual.is_none()
            }
            T::MapStringString => {
                if let Some(WdlType::Map(m)) = actual {
                    let key_ok = matches!(m.key_type.as_ref(), WdlType::Primitive(p)
                        if p.primitive_kind == PK::String);
                    let val_ok = matches!(m.value_type.as_ref(), WdlType::Primitive(p)
                        if p.primitive_kind == PK::String);
                    key_ok && val_ok
                } else {
                    actual.is_none()
                }
            }
            T::PairArray => matches!(actual, Some(WdlType::Pair(_))),
        }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Chunk 2 — Function validation
    // ──────────────────────────────────────────────────────────────────────────

    fn validate_function_version(&mut self, func: WdlFunction, name: &str) {
        let doc_ver = match self.current_doc_version {
            Some(v) => v,
            None => return,
        };
        if let Some(added) = func.added_in() {
            if doc_ver < added {
                self.add_error(
                    WdlErrorCode::FunctionNotAvailableInVersion,
                    format!(
                        "Function '{}' is not available in WDL {}; requires {}",
                        name, doc_ver, added
                    ),
                );
            }
        }
        if let Some(removed) = func.removed_in() {
            if doc_ver >= removed {
                self.add_error(
                    WdlErrorCode::FunctionNotAvailableInVersion,
                    format!("Function '{}' was removed in WDL {}", name, removed),
                );
            }
        }
    }

    fn check_select_first(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        let arg = &op.arguments[0];
        match self.infer_type(arg) {
            Some(WdlType::Array(_)) => {}
            Some(_) => {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "select_first: argument must be an Array",
                );
                return;
            }
            None => return,
        }
        if let EvalValue::List(items) = self.eval_expr(arg) {
            if items.is_empty() {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "select_first: array is empty",
                );
            } else if items.iter().all(|v| matches!(v, EvalValue::Null)) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "select_first: all elements are null",
                );
            }
        }
    }

    fn check_as_map(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let EvalValue::List(pairs) = self.eval_expr(&op.arguments[0]) {
            let mut seen_keys: Vec<String> = Vec::new();
            for pair in &pairs {
                if let EvalValue::Pair(k, _) = pair {
                    let key_str = format!("{:?}", k);
                    if seen_keys.contains(&key_str) {
                        self.add_error(
                            WdlErrorCode::InvalidFunctionArguments,
                            "as_map: duplicate key in array",
                        );
                        break;
                    }
                    seen_keys.push(key_str);
                }
            }
        }
    }

    fn check_zip(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.len() < 2 {
            return;
        }
        if let (EvalValue::List(a), EvalValue::List(b)) = (
            self.eval_expr(&op.arguments[0]),
            self.eval_expr(&op.arguments[1]),
        ) {
            if a.len() != b.len() {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    format!("zip: array lengths differ ({} vs {})", a.len(), b.len()),
                );
            }
        }
    }

    fn check_write_json(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.len() == 1 && self.contains_non_string_map_key(&op.arguments[0]) {
            self.add_error(
                WdlErrorCode::InvalidFunctionArguments,
                "write_json: map keys must be strings",
            );
        }
    }

    fn check_contains(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.len() < 2 {
            return;
        }
        let first_ty = self.infer_type(&op.arguments[0]);
        let second_ty = self.infer_type(&op.arguments[1]);

        // Base: if first is Array[T], second must be assignable to T
        if let Some(WdlType::Array(arr)) = &first_ty {
            let elem_ty = arr.member_type.as_ref();
            if let Some(st) = &second_ty {
                if !self.is_type_assignable(elem_ty, st) {
                    self.add_error(
                        WdlErrorCode::InvalidFunctionArguments,
                        format!(
                            "contains: second argument type '{}' is not assignable to array element type '{}'",
                            type_to_wdl(st),
                            type_to_wdl(elem_ty)
                        ),
                    );
                }
            }
        }

        // Static/Lint: if first is String, second must be String
        if self.mode != ValidatorMode::Base {
            if let Some(ft) = &first_ty {
                if self.is_string(ft) {
                    if let Some(st) = &second_ty {
                        if !self.is_string(st) {
                            self.add_error(
                                WdlErrorCode::InvalidFunctionArguments,
                                "contains: when first argument is String, second must also be String",
                            );
                        }
                    }
                }
            }
        }
    }

    fn check_contains_key(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        let first_ty = self.infer_type(&op.arguments[0]);

        // Base: if first is Map, key type must match
        if let Some(WdlType::Map(m)) = &first_ty {
            if op.arguments.len() >= 2 {
                let key_ty = m.key_type.clone();
                if let Some(second_ty) = self.infer_type(&op.arguments[1]) {
                    if !self.is_type_assignable(&key_ty, &second_ty) {
                        self.add_error(
                            WdlErrorCode::InvalidFunctionArguments,
                            "contains_key: second argument type does not match map key type",
                        );
                    }
                }
            }
        }

        // Static/Lint: first must be Map
        if self.mode != ValidatorMode::Base {
            if let Some(ft) = &first_ty {
                if !matches!(ft, WdlType::Map(_)) {
                    self.add_error(
                        WdlErrorCode::InvalidFunctionArguments,
                        "contains_key: first argument must be a Map",
                    );
                }
            }
        }
    }

    fn check_length(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            let ok = matches!(&ty,
                WdlType::Primitive(p) if p.primitive_kind == WdlPrimitiveKind::String)
                || matches!(&ty, WdlType::Array(_))
                || matches!(&ty, WdlType::Map(_));
            if !ok {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "length: argument must be String, Array, or Map",
                );
            }
        }
    }

    fn check_keys(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !matches!(ty, WdlType::Map(_)) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "keys: argument must be a Map",
                );
            }
        }
    }

    fn check_values(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !matches!(ty, WdlType::Map(_)) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "values: argument must be a Map",
                );
            }
        }
    }

    fn check_range(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !matches!(&ty, WdlType::Primitive(p) if p.primitive_kind == WdlPrimitiveKind::Int) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "range: argument must be Int",
                );
            }
        }
    }

    fn check_select_all(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !matches!(ty, WdlType::Array(_)) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "select_all: argument must be an Array",
                );
            }
        }
    }

    fn check_chunk(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.len() < 2 {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !matches!(ty, WdlType::Array(_)) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "chunk: first argument must be an Array",
                );
            }
        }
        if let Some(ty) = self.infer_type(&op.arguments[1]) {
            if !matches!(&ty, WdlType::Primitive(p) if p.primitive_kind == WdlPrimitiveKind::Int) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "chunk: second argument must be Int",
                );
            }
        }
    }

    fn check_cross(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.len() < 2 {
            return;
        }
        for (i, arg) in op.arguments.iter().enumerate() {
            if let Some(ty) = self.infer_type(arg) {
                if !matches!(ty, WdlType::Array(_)) {
                    self.add_error(
                        WdlErrorCode::InvalidFunctionArguments,
                        format!("cross: argument {} must be an Array", i + 1),
                    );
                }
            }
        }
    }

    fn check_join_paths(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        // First argument must be path-like
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !self.is_path_like(&ty) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "join_paths: first argument must be a path-like type (File, Directory, or String)",
                );
            }
        }
        // Remaining arguments must be String
        for i in 1..op.arguments.len() {
            if let Some(ty) = self.infer_type(&op.arguments[i]) {
                if !self.is_string(&ty) {
                    self.add_error(
                        WdlErrorCode::InvalidFunctionArguments,
                        format!("join_paths: argument {} must be String", i + 1),
                    );
                }
            }
        }
    }

    fn check_size(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !self.is_path_like(&ty) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "size: first argument must be a path-like type",
                );
            }
        }
        if op.arguments.len() >= 2 {
            if let Some(ty) = self.infer_type(&op.arguments[1]) {
                if !self.is_string(&ty) {
                    self.add_error(
                        WdlErrorCode::InvalidFunctionArguments,
                        "size: second argument must be String",
                    );
                }
            }
        }
    }

    fn check_basename(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !self.is_path_like(&ty) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    "basename: first argument must be a path-like type",
                );
            }
        }
        if op.arguments.len() >= 2 {
            if let Some(ty) = self.infer_type(&op.arguments[1]) {
                if !self.is_string(&ty) {
                    self.add_error(
                        WdlErrorCode::InvalidFunctionArguments,
                        "basename: second argument must be String",
                    );
                }
            }
        }
    }

    fn check_single_path_like_arg(&mut self, op: &WdlFunctionCallOperation) {
        if op.arguments.is_empty() {
            return;
        }
        if let Some(ty) = self.infer_type(&op.arguments[0]) {
            if !self.is_path_like(&ty) {
                self.add_error(
                    WdlErrorCode::InvalidFunctionArguments,
                    format!(
                        "{}: argument must be a path-like type (File, Directory, or String)",
                        op.function.to_wdl_str()
                    ),
                );
            }
        }
    }

    fn validate_generic_function_arity(&mut self, op: &WdlFunctionCallOperation) {
        let argc = op.arguments.len() as i32;
        if op.function.supports_arity(argc) {
            return;
        }
        let func = op.function;
        let name = func.to_wdl_str();
        if func.is_variadic() {
            self.add_error(
                WdlErrorCode::InvalidFunctionArguments,
                format!(
                    "{}: expected at least {} arguments, got {}",
                    name,
                    func.min_arity(),
                    argc
                ),
            );
        } else if func.min_arity() == func.max_arity() {
            self.add_error(
                WdlErrorCode::InvalidFunctionArguments,
                format!(
                    "{}: expected exactly {} arguments, got {}",
                    name,
                    func.min_arity(),
                    argc
                ),
            );
        } else {
            self.add_error(
                WdlErrorCode::InvalidFunctionArguments,
                format!(
                    "{}: expected between {} and {} arguments, got {}",
                    name,
                    func.min_arity(),
                    func.max_arity(),
                    argc
                ),
            );
        }
    }

    fn validate_generic_function_signatures(&mut self, op: &WdlFunctionCallOperation) {
        let argc = op.arguments.len() as i32;
        let sigs = op.function.signatures();
        if sigs.is_empty() {
            return;
        }

        let mut any_len_match = false;
        let mut any_compatible = false;

        // Collect inferred types up-front (avoid repeated &mut self calls in loop)
        let arg_types: Vec<Option<WdlType>> =
            op.arguments.iter().map(|a| self.infer_type(a)).collect();

        for sig in &sigs {
            if sig.args.len() as i32 == argc {
                any_len_match = true;
                let all_match = sig
                    .args
                    .iter()
                    .enumerate()
                    .all(|(i, hint)| self.matches_signature_type(arg_types[i].as_ref(), *hint));
                if all_match {
                    any_compatible = true;
                    break;
                }
            }
        }

        if any_len_match && !any_compatible {
            self.add_error(
                WdlErrorCode::InvalidFunctionArguments,
                format!(
                    "Invalid argument types for function '{}'",
                    op.function.to_wdl_str()
                ),
            );
        }
    }

    fn validate_function_call(&mut self, op: &WdlFunctionCallOperation) {
        // Always: version check
        self.validate_function_version(op.function, &op.function_name);

        // Always: base checks
        match op.function {
            WdlFunction::SelectFirst => self.check_select_first(op),
            WdlFunction::AsMap => self.check_as_map(op),
            WdlFunction::Zip => self.check_zip(op),
            WdlFunction::WriteJson => self.check_write_json(op),
            WdlFunction::Contains => self.check_contains(op),
            WdlFunction::ContainsKey => self.check_contains_key(op),
            WdlFunction::Length => self.check_length(op),
            _ => {}
        }

        // Static/Lint only: additional checks
        if self.mode != ValidatorMode::Base {
            match op.function {
                WdlFunction::Keys => self.check_keys(op),
                WdlFunction::Values => self.check_values(op),
                WdlFunction::Range => self.check_range(op),
                WdlFunction::SelectAll => self.check_select_all(op),
                WdlFunction::Chunk => self.check_chunk(op),
                WdlFunction::Cross => self.check_cross(op),
                WdlFunction::JoinPaths => self.check_join_paths(op),
                WdlFunction::Size => self.check_size(op),
                WdlFunction::Basename => self.check_basename(op),
                WdlFunction::ReadLines
                | WdlFunction::ReadTsv
                | WdlFunction::ReadMap
                | WdlFunction::ReadObject
                | WdlFunction::ReadObjects
                | WdlFunction::ReadJson
                | WdlFunction::ReadInt
                | WdlFunction::ReadFloat
                | WdlFunction::ReadString
                | WdlFunction::ReadBoolean
                | WdlFunction::Glob => self.check_single_path_like_arg(op),
                _ => {}
            }
            self.validate_generic_function_arity(op);
            self.validate_generic_function_signatures(op);
        }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Chunk 3 — Expression validation
    // ──────────────────────────────────────────────────────────────────────────

    fn validate_expression(&mut self, expr: &WdlExpression) {
        match expr {
            WdlExpression::BoolLit(_)
            | WdlExpression::IntLit(_)
            | WdlExpression::FloatLit(_)
            | WdlExpression::NullLit
            | WdlExpression::Variable(_) => {}

            WdlExpression::StrLit(lit) => {
                let components = lit.components.clone();
                for comp in &components {
                    if let WdlStringComponent::Placeholder { expression, .. } = comp {
                        self.validate_expression(expression);
                    }
                }
            }

            WdlExpression::ArrayLit(arr) => {
                let entries = arr.entries.clone();
                for e in &entries {
                    self.validate_expression(e);
                }
            }

            WdlExpression::MapLit(map) => {
                let entries = map.entries.clone();
                for e in &entries {
                    self.validate_expression(&e.key);
                    if let Some(v) = &e.value {
                        let v2 = v.clone();
                        self.validate_expression(&v2);
                    }
                }
            }

            WdlExpression::ObjLit(obj) => {
                let entries = obj.entries.clone();
                for e in &entries {
                    if let Some(v) = &e.value {
                        let v2 = v.clone();
                        self.validate_expression(&v2);
                    }
                }
            }

            WdlExpression::PairLit(p) => {
                let left = p.left.clone();
                let right = p.right.clone();
                self.validate_expression(&left);
                self.validate_expression(&right);
            }

            WdlExpression::StructLit(s) => {
                let entries = s.entries.clone();
                for e in &entries {
                    if let Some(v) = &e.value {
                        let v2 = v.clone();
                        self.validate_expression(&v2);
                    }
                }
            }

            WdlExpression::FuncOp(op) => {
                let op_clone = op.clone();
                // Recurse into arguments first
                let args = op_clone.arguments.clone();
                for arg in &args {
                    self.validate_expression(arg);
                }
                self.validate_function_call(&op_clone);
            }

            WdlExpression::IdxOp(op) => {
                let target = op.target.clone();
                let index = op.index.clone();
                self.validate_expression(&target);
                self.validate_expression(&index);
                // Bounds check (all modes)
                if let (EvalValue::List(items), EvalValue::Int(i)) =
                    (self.eval_expr(&target), self.eval_expr(&index))
                {
                    if i < 0 || i >= items.len() as i64 {
                        self.add_error(
                            WdlErrorCode::UnknownReference,
                            format!(
                                "Array index {} is out of bounds (length {})",
                                i,
                                items.len()
                            ),
                        );
                    }
                }
                // Map key check (all modes)
                if let (EvalValue::Map(pairs), idx_val) =
                    (self.eval_expr(&target), self.eval_expr(&index))
                {
                    if !matches!(idx_val, EvalValue::Unknown) {
                        let key_str = format!("{:?}", idx_val);
                        let found = pairs.iter().any(|(k, _)| format!("{:?}", k) == key_str);
                        if !found {
                            self.add_error(WdlErrorCode::UnknownReference, "Map key not found");
                        }
                    }
                }
            }

            WdlExpression::MemberOp(op) => {
                let target = op.target.clone();
                let member = op.member.clone();
                self.validate_expression(&target);
                // Check member exists on call output or struct
                if let WdlExpression::Variable(name) = target.as_ref() {
                    if let Some(outputs) = self.call_outputs.get(name.as_str()).cloned() {
                        if !outputs.contains(&member) {
                            self.add_error(
                                WdlErrorCode::UnknownReference,
                                format!("Unknown member '{}' on call output '{}'", member, name),
                            );
                        }
                    } else if let Some(WdlType::TypeRef(tr)) =
                        self.scope_types.get(name.as_str()).cloned()
                    {
                        if let Some(struct_mems) =
                            self.struct_members.get(&tr.reference_name).cloned()
                        {
                            if !struct_mems.contains(&member) {
                                self.add_error(
                                    WdlErrorCode::UnknownReference,
                                    format!(
                                        "Unknown member '{}' on struct '{}'",
                                        member, tr.reference_name
                                    ),
                                );
                            }
                        }
                    }
                }
            }

            WdlExpression::BinaryOp(op) => {
                let left = op.left.clone();
                let right = op.right.clone();
                let operator = op.operator;
                self.validate_expression(&left);
                self.validate_expression(&right);
                if self.mode != ValidatorMode::Base {
                    let lt = self.infer_type(&left);
                    let rt = self.infer_type(&right);
                    self.validate_binary_op(operator, lt.as_ref(), rt.as_ref());
                }
            }

            WdlExpression::UnaryOp(op) => {
                let operand = op.operand.clone();
                let operator = op.operator;
                self.validate_expression(&operand);
                if self.mode != ValidatorMode::Base {
                    let ot = self.infer_type(&operand);
                    self.validate_unary_op(operator, ot.as_ref());
                }
            }

            WdlExpression::TernaryOp(op) => {
                let cond = op.condition.clone();
                let tv = op.true_value.clone();
                let fv = op.false_value.clone();
                self.validate_expression(&cond);
                self.validate_expression(&tv);
                self.validate_expression(&fv);
                if self.mode != ValidatorMode::Base {
                    let ct = self.infer_type(&cond);
                    let tt = self.infer_type(&tv);
                    let ft = self.infer_type(&fv);
                    self.validate_ternary_op(ct.as_ref(), tt.as_ref(), ft.as_ref());
                }
            }
        }
    }

    fn validate_binary_op(
        &mut self,
        op: BinaryOperator,
        lt: Option<&WdlType>,
        rt: Option<&WdlType>,
    ) {
        match op {
            BinaryOperator::Or | BinaryOperator::And => {
                if let Some(t) = lt {
                    if !self.is_boolean(t) {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            format!("Operator '{}' requires Boolean operands", op.to_wdl_str()),
                        );
                        return;
                    }
                }
                if let Some(t) = rt {
                    if !self.is_boolean(t) {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            format!("Operator '{}' requires Boolean operands", op.to_wdl_str()),
                        );
                    }
                }
            }
            BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulo
            | BinaryOperator::Power
            | BinaryOperator::Subtract => {
                if let Some(t) = lt {
                    if !self.is_numeric(t) {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            format!("Operator '{}' requires numeric operands", op.to_wdl_str()),
                        );
                        return;
                    }
                }
                if let Some(t) = rt {
                    if !self.is_numeric(t) {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            format!("Operator '{}' requires numeric operands", op.to_wdl_str()),
                        );
                    }
                }
            }
            BinaryOperator::Add => {
                let l_num = lt.map(|t| self.is_numeric(t)).unwrap_or(true);
                let r_num = rt.map(|t| self.is_numeric(t)).unwrap_or(true);
                let l_str = lt.map(|t| self.is_string(t)).unwrap_or(false);
                let r_str = rt.map(|t| self.is_string(t)).unwrap_or(false);
                let numeric_pair = l_num && r_num;
                let string_operand = l_str || r_str;
                if !(numeric_pair || string_operand) {
                    self.add_error(
                        WdlErrorCode::TypeMismatch,
                        "Operator '+' requires numeric or string operands",
                    );
                }
            }
            BinaryOperator::Eq | BinaryOperator::Neq => {
                if let (Some(l), Some(r)) = (lt, rt) {
                    if !self.is_type_assignable(l, r) && !self.is_type_assignable(r, l) {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            format!(
                                "Cannot compare types '{}' and '{}' with '{}'",
                                type_to_wdl(l),
                                type_to_wdl(r),
                                op.to_wdl_str()
                            ),
                        );
                    }
                }
            }
            BinaryOperator::Lt | BinaryOperator::Lte | BinaryOperator::Gt | BinaryOperator::Gte => {
                if !self.are_order_comparable(lt, rt) {
                    self.add_error(
                        WdlErrorCode::TypeMismatch,
                        format!(
                            "Operator '{}' requires orderable primitive operands",
                            op.to_wdl_str()
                        ),
                    );
                }
            }
        }
    }

    fn validate_unary_op(&mut self, op: UnaryOperator, operand_ty: Option<&WdlType>) {
        match op {
            UnaryOperator::Not => {
                if let Some(t) = operand_ty {
                    if !self.is_boolean(t) {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            "Operator '!' requires a Boolean operand",
                        );
                    }
                }
            }
            UnaryOperator::Negative => {
                if let Some(t) = operand_ty {
                    if !self.is_numeric(t) {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            "Unary '-' requires a numeric operand",
                        );
                    }
                }
            }
        }
    }

    fn validate_ternary_op(
        &mut self,
        cond_ty: Option<&WdlType>,
        _true_ty: Option<&WdlType>,
        _false_ty: Option<&WdlType>,
    ) {
        if let Some(ct) = cond_ty {
            if !self.is_boolean(ct) {
                self.add_error(
                    WdlErrorCode::TypeMismatch,
                    "Ternary condition must be Boolean",
                );
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Chunk 4 — Indexing + imports
    // ──────────────────────────────────────────────────────────────────────────

    fn to_struct_shape(&self, s: &WdlStruct) -> StructShape {
        let mut ordered_wdl = IndexMap::new();
        let mut ordered_types = IndexMap::new();
        for elem in &s.elements {
            if let WdlStructElement::Member(m) = elem {
                ordered_wdl.insert(m.name.clone(), type_to_wdl(&m.wdl_type));
                ordered_types.insert(m.name.clone(), m.wdl_type.clone());
            }
        }
        StructShape {
            ordered_member_type_wdl: ordered_wdl,
            ordered_member_types: ordered_types,
        }
    }

    /// Rebuilds `ty` with every struct/enum type reference renamed to the local
    /// name it enters the importing document under.
    ///
    /// Mirrors Java `rewriteTypeAliases` (`WdlValidator.java:572-600`): it
    /// recurses through `Array`, `Pair` and `Map` component types, preserves
    /// optionality and non-emptiness, and leaves anything else untouched.
    ///
    /// Without this, a struct imported under an alias keeps its *source*
    /// document's names in its member types. For
    /// `import "lib.wdl" alias Address as Addr alias Person as PersonAlias`,
    /// `PersonAlias.addr` would stay `TypeRef("Address")` while only `Addr` is
    /// indexed, so resolving that member type fails.
    fn rewrite_type_aliases(ty: &WdlType, aliases: &HashMap<String, String>) -> WdlType {
        if aliases.is_empty() {
            return ty.clone();
        }
        match ty {
            WdlType::TypeRef(tr) => {
                let renamed = aliases
                    .get(&tr.reference_name)
                    .cloned()
                    .unwrap_or_else(|| tr.reference_name.clone());
                WdlType::TypeRef(WdlTypeRefType {
                    reference_name: renamed,
                    optional: tr.optional,
                })
            }
            WdlType::Array(arr) => WdlType::Array(WdlArrayType {
                member_type: Box::new(Self::rewrite_type_aliases(&arr.member_type, aliases)),
                non_empty: arr.non_empty,
                optional: arr.optional,
            }),
            WdlType::Pair(pair) => WdlType::Pair(Box::new(WdlPairType {
                left_type: Box::new(Self::rewrite_type_aliases(&pair.left_type, aliases)),
                right_type: Box::new(Self::rewrite_type_aliases(&pair.right_type, aliases)),
                optional: pair.optional,
            })),
            WdlType::Map(map) => WdlType::Map(Box::new(WdlMapType {
                key_type: Box::new(Self::rewrite_type_aliases(&map.key_type, aliases)),
                value_type: Box::new(Self::rewrite_type_aliases(&map.value_type, aliases)),
                optional: map.optional,
            })),
            other => other.clone(),
        }
    }

    /// Struct shape for a struct arriving through an import, with member types
    /// rewritten through the import's alias map. Mirrors Java
    /// `toImportedStructShape` (`WdlValidator.java:528-541`).
    fn to_imported_struct_shape(
        &self,
        s: &WdlStruct,
        aliases: &HashMap<String, String>,
    ) -> StructShape {
        let mut ordered_wdl = IndexMap::new();
        let mut ordered_types = IndexMap::new();
        for elem in &s.elements {
            if let WdlStructElement::Member(m) = elem {
                let rewritten = Self::rewrite_type_aliases(&m.wdl_type, aliases);
                ordered_wdl.insert(m.name.clone(), type_to_wdl(&rewritten));
                ordered_types.insert(m.name.clone(), rewritten);
            }
        }
        StructShape {
            ordered_member_type_wdl: ordered_wdl,
            ordered_member_types: ordered_types,
        }
    }

    fn build_task_contract(&self, task: &WdlTask) -> CallableContract {
        let mut contract = CallableContract::default();
        for elem in &task.elements {
            match elem {
                WdlTaskElement::Input(inp) => {
                    for decl in &inp.elements {
                        match decl {
                            InputDeclaration::Unbound(d) => {
                                contract
                                    .input_types
                                    .insert(d.name.clone(), d.wdl_type.clone());
                                if !d.wdl_type.is_optional() {
                                    contract.required_inputs.insert(d.name.clone());
                                }
                            }
                            InputDeclaration::Bound(d) => {
                                contract
                                    .input_types
                                    .insert(d.name.clone(), d.wdl_type.clone());
                                // has default → not required
                            }
                        }
                    }
                }
                WdlTaskElement::Output(out) => {
                    for decl in &out.elements {
                        contract.outputs.insert(decl.name.clone());
                        contract
                            .output_types
                            .insert(decl.name.clone(), decl.wdl_type.clone());
                    }
                }
                WdlTaskElement::BoundDeclaration(d) => {
                    contract.private_declarations.insert(d.name.clone());
                }
                WdlTaskElement::Declaration(d) => {
                    contract.private_declarations.insert(d.name.clone());
                }
                _ => {}
            }
        }
        contract
    }

    fn build_workflow_contract(&self, wf: &WdlWorkflow) -> CallableContract {
        let mut contract = CallableContract::default();
        for elem in &wf.elements {
            match elem {
                WdlWorkflowElement::Input(inp) => {
                    for decl in &inp.elements {
                        match decl {
                            InputDeclaration::Unbound(d) => {
                                contract
                                    .input_types
                                    .insert(d.name.clone(), d.wdl_type.clone());
                                if !d.wdl_type.is_optional() {
                                    contract.required_inputs.insert(d.name.clone());
                                }
                            }
                            InputDeclaration::Bound(d) => {
                                contract
                                    .input_types
                                    .insert(d.name.clone(), d.wdl_type.clone());
                            }
                        }
                    }
                }
                WdlWorkflowElement::Output(out) => {
                    for decl in &out.elements {
                        contract.outputs.insert(decl.name.clone());
                        contract
                            .output_types
                            .insert(decl.name.clone(), decl.wdl_type.clone());
                    }
                }
                _ => {}
            }
        }
        contract
    }

    fn index_local_task(&mut self, task: &WdlTask) {
        let contract = self.build_task_contract(task);
        if self.callable_contracts.contains_key(&task.name) {
            // duplicate handled by pre_scan_static
        } else {
            self.callable_contracts.insert(task.name.clone(), contract);
        }
    }

    fn index_local_workflow(&mut self, wf: &WdlWorkflow) {
        let contract = self.build_workflow_contract(wf);
        if !self.callable_contracts.contains_key(&wf.name) {
            self.callable_contracts.insert(wf.name.clone(), contract);
        }
    }

    /// Rebuilds a `StructShape` from the indexed maps, mirroring Java
    /// `structShapeFor` (`WdlValidator.java:484-491`).
    fn struct_shape_for(&self, struct_name: &str) -> Option<StructShape> {
        let member_types = self.struct_member_types.get(struct_name)?;
        self.struct_members.get(struct_name)?;
        let mut ordered_member_type_wdl = IndexMap::new();
        let mut ordered_member_types = IndexMap::new();
        for (name, ty) in member_types {
            ordered_member_type_wdl.insert(name.clone(), type_to_wdl(ty));
            ordered_member_types.insert(name.clone(), ty.clone());
        }
        Some(StructShape {
            ordered_member_type_wdl,
            ordered_member_types,
        })
    }

    /// Commits a struct shape to the indexed maps.
    fn store_struct_shape(&mut self, name: &str, shape: &StructShape) {
        let mut members_set = HashSet::new();
        let mut members_types = HashMap::new();
        for (member, ty) in &shape.ordered_member_types {
            members_set.insert(member.clone());
            members_types.insert(member.clone(), ty.clone());
        }
        self.struct_members.insert(name.to_string(), members_set);
        self.struct_member_types
            .insert(name.to_string(), members_types);
    }

    /// Mirrors Java `indexLocalStruct` (`WdlValidator.java:492-510`): on an
    /// incompatible clash with an already-indexed (imported) struct, emit and
    /// bail; otherwise overwrite.
    fn index_local_struct(&mut self, s: &WdlStruct) {
        if s.name.trim().is_empty() {
            return;
        }
        let shape = self.to_struct_shape(s);
        if let Some(existing) = self.struct_shape_for(&s.name) {
            if !existing.is_compatible_with(&shape) {
                self.add_error(
                    WdlErrorCode::TypeMismatch,
                    format!(
                        "Struct '{}' conflicts with imported struct definition; \
                         alias imported structs to disambiguate",
                        s.name
                    ),
                );
                return;
            }
        }
        self.store_struct_shape(&s.name.clone(), &shape);
    }

    /// Mirrors Java `indexImportedStructs` (`WdlValidator.java:392-415`).
    /// `local_name` is the alias under which the struct enters this document;
    /// `aliases` renames type references inside its member types.
    fn index_imported_struct(
        &mut self,
        s: &WdlStruct,
        local_name: &str,
        aliases: &HashMap<String, String>,
    ) {
        if s.name.trim().is_empty() || local_name.trim().is_empty() {
            return;
        }
        let incoming = self.to_imported_struct_shape(s, aliases);
        match self.struct_shape_for(local_name) {
            None => self.store_struct_shape(local_name, &incoming),
            Some(existing) => {
                if !existing.is_compatible_with(&incoming) {
                    self.add_error(
                        WdlErrorCode::TypeMismatch,
                        format!(
                            "Imported struct '{}' has incompatible definitions across imports; \
                             use aliases to disambiguate",
                            local_name
                        ),
                    );
                }
            }
        }
    }

    /// Effective enum value type: declared if present, else inferred from the
    /// choice values, else `String`. Mirrors Java `effectiveEnumValueType`
    /// (`WdlValidator.java:622-625`).
    fn effective_enum_value_type(en: &WdlEnum) -> WdlType {
        en.value_type
            .clone()
            .or_else(|| crate::processors::base::infer_enum_value_type(en))
            .unwrap_or_else(|| WdlType::Primitive(WdlPrimitiveType::new(WdlPrimitiveKind::String)))
    }

    /// Builds the structural description of an enum. Mirrors Java
    /// `EnumShape.from` (`WdlValidator.java:1079-1095`) plus the two companion
    /// maps Java writes alongside it.
    fn to_enum_shape(en: &WdlEnum) -> EnumShape {
        // Sentinels mirror Java `WdlValidator.EnumShape.from` (L1084-1093): an
        // absent value type renders `<implicit>` and a valueless choice renders
        // `<none>`, so the shape format is uniform and cannot collide with a
        // real rendering.
        let value_type_wdl = en
            .value_type
            .as_ref()
            .map(type_to_wdl)
            .unwrap_or_else(|| "<implicit>".to_string());
        let choice_shapes: Vec<String> = en
            .elements
            .iter()
            .map(|c| {
                use crate::processors::render::expression_to_wdl;
                let value = c
                    .value
                    .as_ref()
                    .map(expression_to_wdl)
                    .unwrap_or_else(|| "<none>".to_string());
                format!("{}={}", c.name, value)
            })
            .collect();
        // Bare names only. Blank names are dropped, matching Java
        // `enumChoiceNameSet` / `addIfNonBlank` (L627-631).
        let choice_names: HashSet<String> = en
            .elements
            .iter()
            .filter(|c| !c.name.trim().is_empty())
            .map(|c| c.name.clone())
            .collect();
        EnumShape {
            value_type_wdl,
            choice_names,
            choice_shapes,
            effective_value_type: Self::effective_enum_value_type(en),
        }
    }

    /// Mirrors Java `indexLocalEnum` (`WdlValidator.java:603-620`): on an
    /// incompatible clash with an already-indexed (imported) enum, emit and
    /// bail; otherwise overwrite.
    fn index_local_enum(&mut self, en: &WdlEnum) {
        if en.name.trim().is_empty() {
            return;
        }
        let incoming = Self::to_enum_shape(en);
        if let Some(existing) = self.enum_shapes.get(&en.name) {
            if !existing.is_compatible_with(&incoming) {
                self.add_error(
                    WdlErrorCode::TypeMismatch,
                    format!(
                        "Enum '{}' conflicts with imported enum definition; \
                         alias imported enums to disambiguate",
                        en.name
                    ),
                );
                return;
            }
        }
        self.enum_shapes.insert(en.name.clone(), incoming);
    }

    /// Mirrors Java `indexImportedEnums` (`WdlValidator.java:417-452`).
    /// `local_name` is the alias under which the enum enters this document.
    fn index_imported_enum(&mut self, en: &WdlEnum, local_name: &str) {
        if en.name.trim().is_empty() || local_name.trim().is_empty() {
            return;
        }
        let incoming = Self::to_enum_shape(en);
        let conflict = match self.enum_shapes.get(local_name) {
            None => {
                self.enum_shapes.insert(local_name.to_string(), incoming);
                return;
            }
            Some(existing) => {
                if !existing.is_compatible_with(&incoming) {
                    Some(format!(
                        "Imported enum '{}' has incompatible definitions across imports; \
                         use aliases to disambiguate",
                        local_name
                    ))
                } else if type_to_wdl(&existing.effective_value_type)
                    != type_to_wdl(&incoming.effective_value_type)
                {
                    // Java `WdlValidator.java:438-449`. Implemented for parity,
                    // but unreachable: `is_compatible_with` already pins both the
                    // declared value type and every `NAME=value` rendering, and
                    // the effective type is a pure function of exactly those.
                    // See `.context/B1_plan.md` §5.4.
                    Some(format!(
                        "Imported enum '{}' has incompatible value types across imports; \
                         use aliases to disambiguate",
                        local_name
                    ))
                } else {
                    None
                }
            }
        };
        if let Some(message) = conflict {
            self.add_error(WdlErrorCode::TypeMismatch, message);
        }
    }

    /// Indexes every struct and enum exported by `imported_doc` under the local
    /// name it enters this document with. Mirrors Java's unconditional
    /// `indexImportedStructs` / `indexImportedEnums` pair
    /// (`WdlValidator.java:193-194`), which runs for all three import forms.
    fn index_imported_types(&mut self, imp: &WdlImport, imported_doc: &WdlDocument) {
        let aliases = import_type_alias_map(imp);
        let structs: Vec<WdlStruct> = imported_doc.structs().cloned().collect();
        for s in &structs {
            if let Some(local_name) = resolve_imported_type_local_name(imp, &s.name) {
                self.index_imported_struct(s, &local_name, &aliases);
            }
        }
        let enums: Vec<WdlEnum> = imported_doc.enums().cloned().collect();
        for en in &enums {
            if let Some(local_name) = resolve_imported_type_local_name(imp, &en.name) {
                self.index_imported_enum(en, &local_name);
            }
        }
    }

    fn validate_imports(&mut self, doc: &WdlDocument) {
        // Collect local names to detect namespace collisions
        let local_names: HashSet<String> = doc
            .elements
            .iter()
            .filter_map(|e| match e {
                WdlDocumentElement::Task(t) => Some(t.name.clone()),
                WdlDocumentElement::Workflow(w) => Some(w.name.clone()),
                _ => None,
            })
            .collect();

        // Clone imports so we can call &mut self methods
        let imports: Vec<WdlImport> = doc.import_statements().cloned().collect();

        // ── Pass 1: structural checks (no resolved document needed) ───────────
        // These checks run on every import regardless of whether it was resolved,
        // so they fire even when load_from_path (no resolver) is used.
        let mut namespaces: HashSet<String> = HashSet::new();
        for imp in &imports {
            match imp {
                WdlImport::Standard(std_imp) => {
                    let ns = import_namespace(std_imp);
                    if namespaces.contains(&ns) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate import namespace: {}", ns),
                        );
                    } else if local_names.contains(&ns) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Import namespace '{}' conflicts with local name", ns),
                        );
                    } else {
                        namespaces.insert(ns);
                    }
                }
                WdlImport::Members(mem_imp) => {
                    // Check for duplicate aliases and local-name conflicts within
                    // a single `import { A as X, B as Y } from "..."` statement.
                    let mut seen: HashSet<String> = HashSet::new();
                    for member in &mem_imp.members {
                        let local_name = member.alias.as_deref().unwrap_or(member.member.as_str());
                        if !seen.insert(local_name.to_string()) {
                            self.add_error(
                                WdlErrorCode::DuplicateDefinition,
                                format!("Duplicate import alias: {}", local_name),
                            );
                        } else if local_names.contains(local_name) {
                            self.add_error(
                                WdlErrorCode::DuplicateDefinition,
                                format!("Import alias '{}' conflicts with local name", local_name),
                            );
                        }
                    }
                }
                WdlImport::Star(_) => {}
            }
        }

        // ── Pass 2: content checks (need resolved document) ───────────────────
        for imp in &imports {
            let id = match imp.import_identifier() {
                Some(id) => id.to_string(),
                None => continue,
            };
            let imported_doc = match doc.imported_documents.get(&id).cloned() {
                Some(d) => d,
                None => continue,
            };

            // Version compatibility
            if let (Some(doc_ver), Some(imp_ver)) = (doc.wdl_version, imported_doc.wdl_version) {
                if doc_ver.major() != imp_ver.major() || imp_ver.minor() > doc_ver.minor() {
                    self.add_error(
                        WdlErrorCode::GenericSemanticError,
                        format!(
                            "Import version {} is incompatible with document version {}",
                            imp_ver, doc_ver
                        ),
                    );
                }
            }

            match imp {
                WdlImport::Standard(std_imp) => {
                    let ns = import_namespace(std_imp);
                    // Validate struct/enum alias members exist in the imported doc
                    let alias_members = std_imp.members.clone();
                    for alias_member in &alias_members {
                        let exists = imported_doc
                            .structs()
                            .any(|s| s.name == alias_member.member)
                            || imported_doc.enums().any(|e| e.name == alias_member.member);
                        if !exists {
                            self.add_error(
                                WdlErrorCode::UnknownReference,
                                format!(
                                    "Aliased type '{}' not found in imported document",
                                    alias_member.member
                                ),
                            );
                        }
                    }
                    // Only index callables if this namespace had no structural error
                    if namespaces.contains(&ns) {
                        for t in imported_doc.tasks() {
                            let key = format!("{}.{}", ns, t.name);
                            let contract = self.build_task_contract(t);
                            self.callable_contracts.insert(key, contract);
                        }
                        for w in imported_doc.workflows() {
                            let key = format!("{}.{}", ns, w.name);
                            let contract = self.build_workflow_contract(w);
                            self.callable_contracts.insert(key, contract);
                        }
                    }
                    self.index_imported_types(imp, &imported_doc);
                }
                WdlImport::Star(_) => {
                    for t in imported_doc.tasks() {
                        let contract = self.build_task_contract(t);
                        self.callable_contracts
                            .entry(t.name.clone())
                            .or_insert(contract);
                    }
                    for w in imported_doc.workflows() {
                        let contract = self.build_workflow_contract(w);
                        self.callable_contracts
                            .entry(w.name.clone())
                            .or_insert(contract);
                    }
                    self.index_imported_types(imp, &imported_doc);
                }
                WdlImport::Members(mem_imp) => {
                    let members = mem_imp.members.clone();
                    for member in &members {
                        let local_name = member.alias.as_deref().unwrap_or(&member.member);
                        // Validate the member exists in the imported doc
                        let task_match = imported_doc.tasks().find(|t| t.name == member.member);
                        let wf_match = imported_doc.workflows().find(|w| w.name == member.member);
                        let struct_match = imported_doc.structs().any(|s| s.name == member.member);
                        let enum_match = imported_doc.enums().any(|e| e.name == member.member);
                        if task_match.is_none()
                            && wf_match.is_none()
                            && !struct_match
                            && !enum_match
                        {
                            self.add_error(
                                WdlErrorCode::UnknownReference,
                                format!(
                                    "Import member '{}' not found in imported document",
                                    member.member
                                ),
                            );
                        } else if let Some(t) = task_match {
                            let contract = self.build_task_contract(t);
                            self.callable_contracts
                                .insert(local_name.to_string(), contract);
                        } else if let Some(w) = wf_match {
                            let contract = self.build_workflow_contract(w);
                            self.callable_contracts
                                .insert(local_name.to_string(), contract);
                        }
                    }
                    self.index_imported_types(imp, &imported_doc);
                }
            }
        }
    }

    fn index_top_level_contracts(&mut self, doc: &WdlDocument) {
        self.callable_contracts.clear();
        self.struct_members.clear();
        self.struct_member_types.clear();
        self.enum_shapes.clear();

        // First, process imports (adds imported callables/structs/enums)
        self.validate_imports(doc);

        // Then, index local elements
        let elements = doc.elements.clone();
        for elem in &elements {
            match elem {
                WdlDocumentElement::Task(t) => self.index_local_task(t),
                WdlDocumentElement::Workflow(w) => self.index_local_workflow(w),
                WdlDocumentElement::Struct(s) => self.index_local_struct(s),
                WdlDocumentElement::Enum(en) => self.index_local_enum(en),
                _ => {}
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Chunk 5 — Traversal + entry point
    // ──────────────────────────────────────────────────────────────────────────

    fn validate_known_type_reference(&mut self, ty: &WdlType, location: &str) {
        match ty {
            WdlType::TypeRef(tr) => {
                let name = tr.reference_name.clone();
                if !self.known_type_names.contains(&name) {
                    self.add_error(
                        WdlErrorCode::UnknownReference,
                        format!("Unknown type reference '{}' at '{}'", name, location),
                    );
                }
            }
            WdlType::Array(arr) => {
                let mt = arr.member_type.clone();
                self.validate_known_type_reference(&mt, location);
            }
            WdlType::Map(m) => {
                let kt = m.key_type.clone();
                let vt = m.value_type.clone();
                self.validate_known_type_reference(&kt, location);
                self.validate_known_type_reference(&vt, location);
            }
            WdlType::Pair(p) => {
                let lt = p.left_type.clone();
                let rt = p.right_type.clone();
                self.validate_known_type_reference(&lt, location);
                self.validate_known_type_reference(&rt, location);
            }
            WdlType::Primitive(_) => {}
            // Only ever called on declared types, which come from the parser and
            // so can never be `Unknown`.
            WdlType::Unknown => {}
        }
    }

    fn validate_bound_declaration(&mut self, decl: &WdlBoundDeclaration) {
        // Validate the expression
        let expr = decl.expression.clone();
        self.validate_expression(&expr);

        // Assignability check — runs at all validation levels (matches Java
        // `WdlValidator.validateBoundDeclaration` baseline behavior).
        if let WdlType::Array(arr) = &decl.wdl_type {
            if arr.non_empty {
                if let WdlExpression::ArrayLit(lit) = &decl.expression {
                    if lit.entries.is_empty() {
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            format!(
                                "Array+ type '{}' cannot be assigned an empty array",
                                type_to_wdl(&decl.wdl_type)
                            ),
                        );
                    }
                }
            }
            // Check all array literal elements are assignable to the member type
            if let WdlExpression::ArrayLit(lit) = &decl.expression {
                let elem_ty = arr.member_type.clone();
                let entries = lit.entries.clone();
                for entry in &entries {
                    if !self.is_assignable_from(&elem_ty, entry) {
                        let actual = self.infer_type(entry);
                        self.add_error(
                            WdlErrorCode::TypeMismatch,
                            format!(
                                "Array element type '{}' is not assignable to '{}'",
                                actual
                                    .as_ref()
                                    .map(type_to_wdl)
                                    .unwrap_or_else(|| "null".into()),
                                type_to_wdl(&elem_ty)
                            ),
                        );
                    }
                }
            }
        }
        let expected = decl.wdl_type.clone();
        if !self.is_assignable_from(&expected, &decl.expression) {
            let actual_ty = self.infer_type(&decl.expression);
            self.add_error(
                WdlErrorCode::TypeMismatch,
                format!(
                    "Cannot assign {} to type '{}'",
                    actual_ty
                        .as_ref()
                        .map(type_to_wdl)
                        .unwrap_or_else(|| "null".into()),
                    type_to_wdl(&expected)
                ),
            );
        }

        // Add to scope
        let name = decl.name.clone();
        let ty = decl.wdl_type.clone();
        let val = self.eval_expr(&decl.expression);
        self.scope_types.insert(name.clone(), ty);
        self.scope_values.insert(name, val);
    }

    fn process_workflow_call(&mut self, call: &WdlCall) {
        let call_name = call
            .alias
            .clone()
            .unwrap_or_else(|| call.target_path.last().cloned().unwrap_or_default());

        // Look up contract
        let target = call.target_path_as_string();
        let last_seg = call.target_path.last().cloned().unwrap_or_default();
        let contract: Option<CallableContract> = self
            .callable_contracts
            .get(&target)
            .or_else(|| self.callable_contracts.get(&last_seg))
            .cloned();

        if let Some(ref c) = contract {
            // Check call inputs
            let inputs = call.inputs.clone();
            for input in &inputs {
                // Root key (before first '.')
                let root_key = input
                    .key
                    .split('.')
                    .next()
                    .unwrap_or(&input.key)
                    .to_string();

                if c.private_declarations.contains(&root_key) {
                    self.add_error(
                        WdlErrorCode::UnknownReference,
                        format!("Call input '{}' refers to a private declaration", root_key),
                    );
                    continue;
                }

                // Unknown call input — runs at baseline (matches Java
                // `WdlValidator.processWorkflowCall` L829-836).
                if !c.input_types.contains_key(&root_key) {
                    self.add_error(
                        WdlErrorCode::UnknownReference,
                        format!("Unknown call input '{}' for target '{}'", root_key, target),
                    );
                }

                if let Some(expr) = &input.value {
                    let expr_clone = expr.clone();
                    self.validate_expression(&expr_clone);
                    // Call-input assignability — runs at baseline (matches
                    // Java L841-849).
                    if let Some(expected_ty) = c.input_types.get(&root_key).cloned() {
                        if !self.is_assignable_from(&expected_ty, &expr_clone) {
                            self.add_error(
                                WdlErrorCode::TypeMismatch,
                                format!(
                                    "Call input '{}': expression type does not match expected type '{}'",
                                    root_key,
                                    type_to_wdl(&expected_ty)
                                ),
                            );
                        }
                    }
                }
            }

            // Missing required call input — runs at baseline (matches Java
            // L854-864), emits `UnknownReference`.
            let provided: HashSet<String> = call
                .inputs
                .iter()
                .map(|i| i.key.split('.').next().unwrap_or(&i.key).to_string())
                .collect();
            for req in &c.required_inputs {
                if !provided.contains(req) {
                    self.add_error(
                        WdlErrorCode::UnknownReference,
                        format!(
                            "Required input '{}' not provided for call '{}'",
                            req, call_name
                        ),
                    );
                }
            }

            // Register call outputs
            let outputs = c.outputs.clone();
            let output_types = c.output_types.clone();
            self.call_outputs.insert(call_name.clone(), outputs);
            self.call_output_types
                .insert(call_name.clone(), output_types);
        } else {
            // No contract (unresolved import or unknown callable) — do not
            // insert into call_outputs so member access checks are skipped.
        }

        // Add call name to scope_types as a TypeRef placeholder (for member access)
        self.scope_types.insert(
            call_name,
            WdlType::Primitive(WdlPrimitiveType::new(WdlPrimitiveKind::Object)),
        );
    }

    fn process_workflow_scatter(&mut self, scatter: &WdlScatter) {
        // Infer element type of the collection
        let coll_ty = self.infer_type(&scatter.collection);
        let elem_ty = match coll_ty {
            Some(WdlType::Array(arr)) => *arr.member_type,
            // Collection type not inferable (e.g. `as_pairs`, which has no arm
            // in `infer_function_type`). Bind `Unknown`, not a concrete
            // `Object`: `is_type_assignable` treats `Unknown` as compatible in
            // both directions, so uses of the scatter variable go unchecked
            // rather than being checked against a type we invented.
            //
            // This reproduces Java, which never binds the scatter variable at
            // all (`WdlValidator.processWorkflowScatter`, :876-881), so
            // `scopeTypes.get` is null and `isAssignableFrom` short-circuits to
            // true (`WdlExpressionValidator.java:252-253`). Binding `Object`
            // made Rust reject the valid spec examples
            // `serde_homogeneous_pair.wdl` and `serde_pair.wdl`.
            //
            // Where the collection type *is* known Rust still checks uses of
            // the scatter variable, which Java cannot. That is intentional.
            _ => WdlType::Unknown,
        };

        let coll_expr = scatter.collection.clone();
        self.validate_expression(&coll_expr);

        // Add scatter var to scope
        let scatter_name = scatter.name.clone();
        self.scope_types.insert(scatter_name.clone(), elem_ty);

        // Process body
        let stmts = scatter.statements.clone();
        for stmt in &stmts {
            self.process_workflow_statement(stmt);
        }
    }

    fn process_workflow_conditional(&mut self, cond: &WdlConditional) {
        let cond_expr = cond.condition.clone();
        self.validate_expression(&cond_expr);

        let then_stmts = cond.then_statements.clone();
        for stmt in &then_stmts {
            self.process_workflow_statement(stmt);
        }

        let else_ifs = cond.else_ifs.clone();
        for else_if in &else_ifs {
            let ei_cond = else_if.condition.clone();
            self.validate_expression(&ei_cond);
            let ei_stmts = else_if.then_statements.clone();
            for stmt in &ei_stmts {
                self.process_workflow_statement(stmt);
            }
        }

        let else_stmts = cond.else_statements.clone();
        for stmt in &else_stmts {
            self.process_workflow_statement(stmt);
        }
    }

    fn process_workflow_statement(&mut self, stmt: &WdlStatement) {
        match stmt {
            WdlStatement::BoundDeclaration(d) => {
                let d2 = d.clone();
                self.validate_bound_declaration(&d2);
            }
            WdlStatement::Declaration(d) => {
                // Unbound declaration — add to scope
                let name = d.name.clone();
                let ty = d.wdl_type.clone();
                self.scope_types.insert(name, ty);
            }
            WdlStatement::Call(call) => {
                let call2 = call.clone();
                self.process_workflow_call(&call2);
            }
            WdlStatement::Scatter(scatter) => {
                let s2 = scatter.clone();
                self.process_workflow_scatter(&s2);
            }
            WdlStatement::Conditional(cond) => {
                let c2 = cond.clone();
                self.process_workflow_conditional(&c2);
            }
        }
    }

    fn validate_call_structure(
        &mut self,
        call: &WdlCall,
        names_in_block: &mut HashSet<String>,
        available_calls: &mut HashSet<String>,
    ) {
        let call_name = call
            .alias
            .clone()
            .unwrap_or_else(|| call.target_path.last().cloned().unwrap_or_default());

        // Unknown call target
        let target = call.target_path_as_string();
        let last_seg = call.target_path.last().cloned().unwrap_or_default();
        if !self.callable_contracts.contains_key(&target)
            && !self.callable_contracts.contains_key(&last_seg)
        {
            self.add_error(
                WdlErrorCode::UnknownReference,
                format!("Unknown call target: {}", target),
            );
        }

        // Duplicate call name
        if names_in_block.contains(&call_name) {
            self.add_error(
                WdlErrorCode::DuplicateDefinition,
                format!("Duplicate name '{}' in workflow block", call_name),
            );
        }

        // Duplicate call input keys
        let mut seen_keys: HashSet<String> = HashSet::new();
        for input in &call.inputs {
            if !seen_keys.insert(input.key.clone()) {
                self.add_error(
                    WdlErrorCode::DuplicateDefinition,
                    format!(
                        "Duplicate call input key '{}' in call '{}'",
                        input.key, call_name
                    ),
                );
            }
        }

        // Unknown after-dependencies
        for after in &call.after_dependencies {
            if !available_calls.contains(after) {
                self.add_error(
                    WdlErrorCode::UnknownReference,
                    format!(
                        "After-dependency '{}' of call '{}' is not yet defined in this scope",
                        after, call_name
                    ),
                );
            }
        }

        names_in_block.insert(call_name.clone());
        available_calls.insert(call_name);
    }

    fn validate_nested_statements(
        &mut self,
        stmts: &[WdlStatement],
        names_in_block: &mut HashSet<String>,
    ) {
        let mut local_available_calls: HashSet<String> = HashSet::new();

        for stmt in stmts {
            match stmt {
                WdlStatement::Declaration(d) => {
                    if names_in_block.contains(&d.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate declaration '{}' in block", d.name),
                        );
                    } else {
                        names_in_block.insert(d.name.clone());
                    }
                }
                WdlStatement::BoundDeclaration(d) => {
                    if names_in_block.contains(&d.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate declaration '{}' in block", d.name),
                        );
                    } else {
                        names_in_block.insert(d.name.clone());
                    }
                }
                WdlStatement::Call(call) => {
                    let call2 = call.clone();
                    self.validate_call_structure(
                        &call2,
                        names_in_block,
                        &mut local_available_calls,
                    );
                }
                WdlStatement::Scatter(scatter) => {
                    names_in_block.insert(scatter.name.clone());
                    let sub_stmts = scatter.statements.clone();
                    self.validate_nested_statements(&sub_stmts, names_in_block);
                }
                WdlStatement::Conditional(cond) => {
                    let then_stmts = cond.then_statements.clone();
                    self.validate_nested_statements(&then_stmts, names_in_block);
                    let else_ifs = cond.else_ifs.clone();
                    for else_if in &else_ifs {
                        let ei_stmts = else_if.then_statements.clone();
                        self.validate_nested_statements(&ei_stmts, names_in_block);
                    }
                    let else_stmts = cond.else_statements.clone();
                    self.validate_nested_statements(&else_stmts, names_in_block);
                }
            }
        }
    }

    fn validate_nested_workflow_structure(&mut self, workflow: &WdlWorkflow) {
        let mut names_in_block: HashSet<String> = HashSet::new();
        let mut available_calls: HashSet<String> = HashSet::new();

        let elements = workflow.elements.clone();
        for elem in &elements {
            match elem {
                WdlWorkflowElement::Input(inp) => {
                    for decl in &inp.elements {
                        let name = decl.name().to_string();
                        if names_in_block.contains(&name) {
                            self.add_error(
                                WdlErrorCode::DuplicateDefinition,
                                format!("Duplicate input declaration '{}'", name),
                            );
                        } else {
                            names_in_block.insert(name);
                        }
                    }
                }
                WdlWorkflowElement::BoundDeclaration(d) => {
                    if names_in_block.contains(&d.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate declaration '{}'", d.name),
                        );
                    } else {
                        names_in_block.insert(d.name.clone());
                    }
                }
                WdlWorkflowElement::Declaration(d) => {
                    if names_in_block.contains(&d.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate declaration '{}'", d.name),
                        );
                    } else {
                        names_in_block.insert(d.name.clone());
                    }
                }
                WdlWorkflowElement::Call(call) => {
                    let call2 = call.clone();
                    self.validate_call_structure(&call2, &mut names_in_block, &mut available_calls);
                }
                WdlWorkflowElement::Scatter(scatter) => {
                    names_in_block.insert(scatter.name.clone());
                    let sub_stmts = scatter.statements.clone();
                    self.validate_nested_statements(&sub_stmts, &mut names_in_block);
                }
                WdlWorkflowElement::Conditional(cond) => {
                    let then_stmts = cond.then_statements.clone();
                    self.validate_nested_statements(&then_stmts, &mut names_in_block);
                    let else_ifs = cond.else_ifs.clone();
                    for else_if in &else_ifs {
                        let ei_stmts = else_if.then_statements.clone();
                        self.validate_nested_statements(&ei_stmts, &mut names_in_block);
                    }
                    let else_stmts = cond.else_statements.clone();
                    self.validate_nested_statements(&else_stmts, &mut names_in_block);
                }
                _ => {}
            }
        }
    }

    fn traverse_task(&mut self, _doc: &WdlDocument, task: &WdlTask) {
        if self.mode == ValidatorMode::Base {
            return;
        }

        let mut names: HashSet<String> = HashSet::new();
        let elements = task.elements.clone();

        for elem in &elements {
            match elem {
                WdlTaskElement::Input(inp) => {
                    for decl in &inp.elements {
                        let (name, ty) = match decl {
                            InputDeclaration::Unbound(d) => (d.name.clone(), d.wdl_type.clone()),
                            InputDeclaration::Bound(d) => (d.name.clone(), d.wdl_type.clone()),
                        };
                        self.validate_known_type_reference(&ty, &name);
                        if names.contains(&name) {
                            self.add_error(
                                WdlErrorCode::DuplicateDefinition,
                                format!("Duplicate task declaration '{}'", name),
                            );
                        } else {
                            names.insert(name);
                        }
                    }
                }
                WdlTaskElement::BoundDeclaration(d) => {
                    let ty = d.wdl_type.clone();
                    self.validate_known_type_reference(&ty, &d.name);
                    if names.contains(&d.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate task declaration '{}'", d.name),
                        );
                    } else {
                        names.insert(d.name.clone());
                    }
                }
                WdlTaskElement::Declaration(d) => {
                    let ty = d.wdl_type.clone();
                    self.validate_known_type_reference(&ty, &d.name);
                    if names.contains(&d.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate task declaration '{}'", d.name),
                        );
                    } else {
                        names.insert(d.name.clone());
                    }
                }
                WdlTaskElement::Output(out) => {
                    for decl in &out.elements {
                        let ty = decl.wdl_type.clone();
                        self.validate_known_type_reference(&ty, &decl.name);
                    }
                }
                _ => {}
            }
        }

        if self.mode == ValidatorMode::Lint {
            let task2 = task.clone();
            self.lint_task(&task2);
        }
    }

    fn traverse_workflow(&mut self, _doc: &WdlDocument, workflow: &WdlWorkflow) {
        // Static/Lint: structure checks first
        if self.mode != ValidatorMode::Base {
            // Check type refs in declarations
            let elements_for_type_check = workflow.elements.clone();
            for elem in &elements_for_type_check {
                match elem {
                    WdlWorkflowElement::Input(inp) => {
                        for decl in &inp.elements {
                            let (name, ty) = match decl {
                                InputDeclaration::Unbound(d) => {
                                    (d.name.clone(), d.wdl_type.clone())
                                }
                                InputDeclaration::Bound(d) => (d.name.clone(), d.wdl_type.clone()),
                            };
                            self.validate_known_type_reference(&ty, &name);
                        }
                    }
                    WdlWorkflowElement::BoundDeclaration(d) => {
                        let ty = d.wdl_type.clone();
                        self.validate_known_type_reference(&ty, &d.name);
                    }
                    WdlWorkflowElement::Output(out) => {
                        for decl in &out.elements {
                            let ty = decl.wdl_type.clone();
                            self.validate_known_type_reference(&ty, &decl.name);
                        }
                    }
                    _ => {}
                }
            }
            // Nested structure check (duplicates, call targets, after-deps)
            let wf2 = workflow.clone();
            self.validate_nested_workflow_structure(&wf2);
        }

        // Save scope
        let saved_scope_types = mem::take(&mut self.scope_types);
        let saved_scope_values = mem::take(&mut self.scope_values);
        let saved_call_outputs = mem::take(&mut self.call_outputs);
        let saved_call_output_types = mem::take(&mut self.call_output_types);

        // Scope traversal
        let elements = workflow.elements.clone();
        for elem in &elements {
            match elem {
                WdlWorkflowElement::Input(inp) => {
                    for decl in &inp.elements {
                        match decl {
                            InputDeclaration::Unbound(d) => {
                                self.scope_types.insert(d.name.clone(), d.wdl_type.clone());
                            }
                            InputDeclaration::Bound(d) => {
                                let d2 = d.clone();
                                self.validate_bound_declaration(&d2);
                            }
                        }
                    }
                }
                WdlWorkflowElement::BoundDeclaration(d) => {
                    let d2 = d.clone();
                    self.validate_bound_declaration(&d2);
                }
                WdlWorkflowElement::Declaration(d) => {
                    self.scope_types.insert(d.name.clone(), d.wdl_type.clone());
                }
                WdlWorkflowElement::Output(out) => {
                    for decl in &out.elements {
                        let d2 = decl.clone();
                        self.validate_bound_declaration(&d2);
                    }
                }
                WdlWorkflowElement::Call(call) => {
                    let call2 = call.clone();
                    self.process_workflow_call(&call2);
                }
                WdlWorkflowElement::Scatter(scatter) => {
                    let s2 = scatter.clone();
                    self.process_workflow_scatter(&s2);
                }
                WdlWorkflowElement::Conditional(cond) => {
                    let c2 = cond.clone();
                    self.process_workflow_conditional(&c2);
                }
                WdlWorkflowElement::Hints(_)
                | WdlWorkflowElement::Meta(_)
                | WdlWorkflowElement::ParameterMeta(_) => {}
            }
        }

        // Lint
        if self.mode == ValidatorMode::Lint {
            let wf3 = workflow.clone();
            self.lint_workflow(&wf3);
        }

        // Restore scope
        self.scope_types = saved_scope_types;
        self.scope_values = saved_scope_values;
        self.call_outputs = saved_call_outputs;
        self.call_output_types = saved_call_output_types;
    }

    fn traverse_document(&mut self, doc: &WdlDocument) {
        self.current_doc_version = doc.wdl_version;

        let elements = doc.elements.clone();
        for elem in &elements {
            match elem {
                WdlDocumentElement::Task(t) => {
                    let t2 = t.clone();
                    self.traverse_task(doc, &t2);
                }
                WdlDocumentElement::Workflow(w) => {
                    let w2 = w.clone();
                    self.traverse_workflow(doc, &w2);
                }
                _ => {}
            }
        }

        if self.mode == ValidatorMode::Lint {
            self.lint_deprecated_document_features(doc);
        }
    }

    fn pre_scan_static(&mut self, doc: &WdlDocument) {
        self.known_callable_targets.clear();
        self.known_type_names.clear();

        // Built-in primitive types are always known
        for name in &[
            "Boolean",
            "Int",
            "Float",
            "String",
            "File",
            "Directory",
            "Object",
        ] {
            self.known_type_names.insert(name.to_string());
        }

        let elements = doc.elements.clone();
        for elem in &elements {
            match elem {
                WdlDocumentElement::Task(t) => {
                    if self.known_callable_targets.contains(&t.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate top-level definition: '{}'", t.name),
                        );
                    }
                    self.known_callable_targets.insert(t.name.clone());
                }
                WdlDocumentElement::Workflow(w) => {
                    if self.known_callable_targets.contains(&w.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate top-level definition: '{}'", w.name),
                        );
                    }
                    self.known_callable_targets.insert(w.name.clone());
                }
                WdlDocumentElement::Struct(s) => {
                    if self.known_type_names.contains(&s.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate struct definition: '{}'", s.name),
                        );
                    }
                    self.known_type_names.insert(s.name.clone());
                }
                WdlDocumentElement::Enum(en) => {
                    if self.known_type_names.contains(&en.name) {
                        self.add_error(
                            WdlErrorCode::DuplicateDefinition,
                            format!("Duplicate enum definition: '{}'", en.name),
                        );
                    }
                    self.known_type_names.insert(en.name.clone());
                }
                _ => {}
            }
        }
    }

    pub fn validate(&mut self, doc: &WdlDocument) -> Result<(), WdlError> {
        self.errors.clear();
        self.current_doc_version = doc.wdl_version;

        if self.mode != ValidatorMode::Base {
            self.pre_scan_static(doc);
        }

        self.index_top_level_contracts(doc);
        self.traverse_document(doc);

        if self.should_throw() {
            Err(WdlError::Semantic(self.errors.clone()))
        } else {
            Ok(())
        }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Chunk 6 — Lint helpers
    // ──────────────────────────────────────────────────────────────────────────

    fn collect_expression_usage(&mut self, expr: &WdlExpression, usage: &mut Usage) {
        match expr {
            WdlExpression::Variable(name) => {
                usage.used_variables.insert(name.clone());
            }
            WdlExpression::MemberOp(op) => {
                if let WdlExpression::Variable(name) = op.target.as_ref() {
                    usage.used_call_output_targets.insert(name.clone());
                }
                self.collect_expression_usage(&op.target, usage);
            }
            WdlExpression::FuncOp(op) => {
                for arg in &op.arguments {
                    self.collect_expression_usage(arg, usage);
                }
            }
            WdlExpression::BinaryOp(op) => {
                self.collect_expression_usage(&op.left, usage);
                self.collect_expression_usage(&op.right, usage);
            }
            WdlExpression::UnaryOp(op) => {
                self.collect_expression_usage(&op.operand, usage);
            }
            WdlExpression::TernaryOp(op) => {
                self.collect_expression_usage(&op.condition, usage);
                self.collect_expression_usage(&op.true_value, usage);
                self.collect_expression_usage(&op.false_value, usage);
            }
            WdlExpression::IdxOp(op) => {
                self.collect_expression_usage(&op.target, usage);
                self.collect_expression_usage(&op.index, usage);
            }
            WdlExpression::ArrayLit(arr) => {
                for e in &arr.entries {
                    self.collect_expression_usage(e, usage);
                }
            }
            WdlExpression::MapLit(map) => {
                for e in &map.entries {
                    self.collect_expression_usage(&e.key, usage);
                    if let Some(v) = &e.value {
                        self.collect_expression_usage(v, usage);
                    }
                }
            }
            WdlExpression::PairLit(p) => {
                self.collect_expression_usage(&p.left, usage);
                self.collect_expression_usage(&p.right, usage);
            }
            WdlExpression::ObjLit(obj) => {
                for e in &obj.entries {
                    if let Some(v) = &e.value {
                        self.collect_expression_usage(v, usage);
                    }
                }
            }
            WdlExpression::StructLit(s) => {
                for e in &s.entries {
                    if let Some(v) = &e.value {
                        self.collect_expression_usage(v, usage);
                    }
                }
            }
            WdlExpression::StrLit(lit) => {
                self.collect_string_literal_usage(lit, usage);
            }
            _ => {}
        }
    }

    fn collect_string_literal_usage(
        &mut self,
        lit: &crate::expressions::WdlStringLiteral,
        usage: &mut Usage,
    ) {
        for comp in &lit.components {
            if let WdlStringComponent::Placeholder {
                expression, option, ..
            } = comp
            {
                self.collect_expression_usage(expression, usage);
                if let Some(opt) = option {
                    self.add_error(
                        WdlErrorCode::LintDeprecatedFeature,
                        "String placeholder option syntax (sep=, default=, true=/false=) is deprecated",
                    );
                    match opt.as_ref() {
                        WdlStringPlaceholderOption::Sep(s)
                        | WdlStringPlaceholderOption::Default(s) => {
                            self.collect_string_literal_usage(s, usage);
                        }
                        WdlStringPlaceholderOption::TrueFalse {
                            true_value,
                            false_value,
                        }
                        | WdlStringPlaceholderOption::FalseTrue {
                            true_value,
                            false_value,
                        } => {
                            self.collect_string_literal_usage(true_value, usage);
                            self.collect_string_literal_usage(false_value, usage);
                        }
                    }
                }
            }
        }
    }

    fn collect_call_usage(
        &mut self,
        call: &WdlCall,
        usage: &mut Usage,
        call_names: &mut HashSet<String>,
    ) {
        let call_name = call
            .alias
            .clone()
            .unwrap_or_else(|| call.target_path.last().cloned().unwrap_or_default());
        call_names.insert(call_name);
        for input in &call.inputs {
            if let Some(expr) = &input.value {
                self.collect_expression_usage(expr, usage);
            }
        }
    }

    fn collect_scatter_usage(
        &mut self,
        scatter: &WdlScatter,
        usage: &mut Usage,
        declared_names: &mut HashSet<String>,
        call_names: &mut HashSet<String>,
    ) {
        self.collect_expression_usage(&scatter.collection, usage);

        // Build body usage
        let mut body_usage = Usage::default();
        let mut body_declared = HashSet::new();
        let stmts = scatter.statements.clone();
        self.collect_statements_usage(&stmts, &mut body_usage, &mut body_declared, call_names);

        // Check if scatter var is used
        if !body_usage.used_variables.contains(&scatter.name) {
            self.add_error(
                WdlErrorCode::LintUnusedScatterVariable,
                format!(
                    "Scatter variable '{}' is never used in its body",
                    scatter.name
                ),
            );
        }

        // Merge body into outer
        usage.merge(body_usage);
        declared_names.extend(body_declared);
        // NOTE: scatter var itself is NOT added to declared_names
        // to avoid double-reporting (already handled by LintUnusedScatterVariable)
    }

    fn collect_conditional_usage(
        &mut self,
        cond: &WdlConditional,
        usage: &mut Usage,
        declared_names: &mut HashSet<String>,
        call_names: &mut HashSet<String>,
    ) {
        self.collect_expression_usage(&cond.condition, usage);

        let then_stmts = cond.then_statements.clone();
        self.collect_statements_usage_immut(&then_stmts, usage, declared_names, call_names);

        for else_if in &cond.else_ifs {
            self.collect_expression_usage(&else_if.condition, usage);
            let ei_stmts = else_if.then_statements.clone();
            self.collect_statements_usage_immut(&ei_stmts, usage, declared_names, call_names);
        }

        let else_stmts = cond.else_statements.clone();
        self.collect_statements_usage_immut(&else_stmts, usage, declared_names, call_names);
    }

    /// Immutable version of collect_statements_usage (used from collect_conditional_usage).
    fn collect_statements_usage_immut(
        &mut self,
        stmts: &[WdlStatement],
        usage: &mut Usage,
        declared_names: &mut HashSet<String>,
        call_names: &mut HashSet<String>,
    ) {
        for stmt in stmts {
            match stmt {
                WdlStatement::BoundDeclaration(d) => {
                    declared_names.insert(d.name.clone());
                    self.collect_expression_usage(&d.expression, usage);
                }
                WdlStatement::Declaration(d) => {
                    declared_names.insert(d.name.clone());
                }
                WdlStatement::Call(call) => {
                    self.collect_call_usage(call, usage, call_names);
                }
                WdlStatement::Conditional(cond) => {
                    self.collect_conditional_usage(cond, usage, declared_names, call_names);
                }
                // Scatter inside conditional — simplified (no lint for nested scatter in cond)
                WdlStatement::Scatter(scatter) => {
                    self.collect_expression_usage(&scatter.collection, usage);
                    let sub_stmts = scatter.statements.clone();
                    self.collect_statements_usage_immut(
                        &sub_stmts,
                        usage,
                        declared_names,
                        call_names,
                    );
                }
            }
        }
    }

    fn collect_statements_usage(
        &mut self,
        stmts: &[WdlStatement],
        usage: &mut Usage,
        declared_names: &mut HashSet<String>,
        call_names: &mut HashSet<String>,
    ) {
        let stmts_clone: Vec<WdlStatement> = stmts.to_vec();
        for stmt in &stmts_clone {
            match stmt {
                WdlStatement::BoundDeclaration(d) => {
                    declared_names.insert(d.name.clone());
                    self.collect_expression_usage(&d.expression, usage);
                }
                WdlStatement::Declaration(d) => {
                    declared_names.insert(d.name.clone());
                }
                WdlStatement::Call(call) => {
                    let call2 = call.clone();
                    self.collect_call_usage(&call2, usage, call_names);
                }
                WdlStatement::Scatter(scatter) => {
                    let s2 = scatter.clone();
                    self.collect_scatter_usage(&s2, usage, declared_names, call_names);
                }
                WdlStatement::Conditional(cond) => {
                    let c2 = cond.clone();
                    self.collect_conditional_usage(&c2, usage, declared_names, call_names);
                }
            }
        }
    }

    fn lint_deprecated_document_features(&mut self, doc: &WdlDocument) {
        for imp in doc.import_statements() {
            let src = imp.source_text();
            if src.starts_with("file://") {
                self.add_error(
                    WdlErrorCode::LintDeprecatedFeature,
                    format!("Import source '{}' uses deprecated file:// URI scheme", src),
                );
            }
        }
    }

    fn lint_workflow(&mut self, workflow: &WdlWorkflow) {
        let mut declared_names: HashSet<String> = HashSet::new();
        let mut call_names: HashSet<String> = HashSet::new();
        let mut usage = Usage::default();

        let elements = workflow.elements.clone();
        for elem in &elements {
            match elem {
                WdlWorkflowElement::Input(inp) => {
                    for decl in &inp.elements {
                        match decl {
                            InputDeclaration::Bound(d) => {
                                self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                                declared_names.insert(d.name.clone());
                                self.collect_expression_usage(&d.expression, &mut usage);
                            }
                            InputDeclaration::Unbound(d) => {
                                self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                                declared_names.insert(d.name.clone());
                            }
                        }
                    }
                }
                WdlWorkflowElement::BoundDeclaration(d) => {
                    self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                    declared_names.insert(d.name.clone());
                    self.collect_expression_usage(&d.expression, &mut usage);
                }
                WdlWorkflowElement::Declaration(d) => {
                    self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                    declared_names.insert(d.name.clone());
                }
                WdlWorkflowElement::Output(out) => {
                    for decl in &out.elements {
                        self.lint_deprecated_type_usage(&decl.wdl_type.clone(), &decl.name);
                        self.collect_expression_usage(&decl.expression, &mut usage);
                    }
                }
                WdlWorkflowElement::Call(call) => {
                    let call2 = call.clone();
                    self.collect_call_usage(&call2, &mut usage, &mut call_names);
                }
                WdlWorkflowElement::Scatter(scatter) => {
                    let s2 = scatter.clone();
                    self.collect_scatter_usage(
                        &s2,
                        &mut usage,
                        &mut declared_names,
                        &mut call_names,
                    );
                }
                WdlWorkflowElement::Conditional(cond) => {
                    let c2 = cond.clone();
                    self.collect_conditional_usage(
                        &c2,
                        &mut usage,
                        &mut declared_names,
                        &mut call_names,
                    );
                }
                _ => {}
            }
        }

        // Check unused declarations
        for name in &declared_names {
            if !usage.used_variables.contains(name) {
                self.add_error(
                    WdlErrorCode::LintUnusedWorkflowDeclaration,
                    format!("Workflow declaration '{}' is never used", name),
                );
            }
        }

        // Check unused call outputs
        for call_name in &call_names {
            if !usage.used_call_output_targets.contains(call_name) {
                self.add_error(
                    WdlErrorCode::LintUnusedCallOutput,
                    format!("Outputs of call '{}' are never used", call_name),
                );
            }
        }
    }

    // ─── deprecation helpers ───────────────────────────────────────────────────

    fn lint_deprecated_type_usage(&mut self, ty: &WdlType, name: &str) {
        match ty {
            WdlType::Primitive(p) if p.primitive_kind == WdlPrimitiveKind::Object => {
                self.add_error(
                    WdlErrorCode::LintDeprecatedFeature,
                    format!("Declaration '{}' uses deprecated 'Object' type", name),
                );
            }
            // Parser emits Object? as TypeRef("Object") rather than Primitive(Object)
            WdlType::TypeRef(tr) if tr.reference_name == "Object" => {
                self.add_error(
                    WdlErrorCode::LintDeprecatedFeature,
                    format!("Declaration '{}' uses deprecated 'Object' type", name),
                );
            }
            WdlType::Array(a) => {
                let mt = a.member_type.clone();
                self.lint_deprecated_type_usage(&mt, name);
            }
            WdlType::Map(m) => {
                let kt = m.key_type.clone();
                let vt = m.value_type.clone();
                self.lint_deprecated_type_usage(&kt, name);
                self.lint_deprecated_type_usage(&vt, name);
            }
            WdlType::Pair(p) => {
                let lt = p.left_type.clone();
                let rt = p.right_type.clone();
                self.lint_deprecated_type_usage(&lt, name);
                self.lint_deprecated_type_usage(&rt, name);
            }
            _ => {}
        }
    }

    fn lint_task(&mut self, task: &WdlTask) {
        let mut declared_names: HashSet<String> = HashSet::new();
        let mut usage = Usage::default();

        let elements = task.elements.clone();
        for elem in &elements {
            match elem {
                WdlTaskElement::Input(inp) => {
                    for decl in &inp.elements {
                        match decl {
                            InputDeclaration::Unbound(d) => {
                                self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                                declared_names.insert(d.name.clone());
                            }
                            InputDeclaration::Bound(d) => {
                                self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                                declared_names.insert(d.name.clone());
                                self.collect_expression_usage(&d.expression, &mut usage);
                            }
                        }
                    }
                }
                WdlTaskElement::BoundDeclaration(d) => {
                    self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                    declared_names.insert(d.name.clone());
                    self.collect_expression_usage(&d.expression, &mut usage);
                }
                WdlTaskElement::Declaration(d) => {
                    self.lint_deprecated_type_usage(&d.wdl_type.clone(), &d.name);
                    declared_names.insert(d.name.clone());
                }
                WdlTaskElement::Output(out) => {
                    for decl in &out.elements {
                        self.lint_deprecated_type_usage(&decl.wdl_type.clone(), &decl.name);
                        self.collect_expression_usage(&decl.expression, &mut usage);
                    }
                }
                WdlTaskElement::Command(cmd) => {
                    self.collect_string_literal_usage(&cmd.command_text, &mut usage);
                }
                WdlTaskElement::Runtime(rt) => {
                    self.add_error(
                        WdlErrorCode::LintDeprecatedFeature,
                        format!(
                            "Task '{}' uses deprecated 'runtime' section; use 'requirements' instead",
                            task.name
                        ),
                    );
                    for entry in &rt.elements {
                        if let Some(v) = &entry.value {
                            self.collect_expression_usage(v, &mut usage);
                        }
                    }
                }
                WdlTaskElement::Requirements(req) => {
                    for entry in &req.elements {
                        if let Some(v) = &entry.value {
                            self.collect_expression_usage(v, &mut usage);
                        }
                    }
                }
                WdlTaskElement::Hints(hints) => {
                    for entry in &hints.elements {
                        if let Some(v) = &entry.value {
                            self.collect_expression_usage(v, &mut usage);
                        }
                    }
                }
                _ => {}
            }
        }

        for name in &declared_names {
            if !usage.used_variables.contains(name) {
                self.add_error(
                    WdlErrorCode::LintUnusedTaskDeclaration,
                    format!("Task declaration '{}' is never used", name),
                );
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Public wrapper structs
// ──────────────────────────────────────────────────────────────────────────────

/// Base validator: checks function availability by WDL version and base function
/// argument constraints.
pub struct WdlValidator {
    runner: ValidatorRunner,
}

impl WdlValidator {
    pub fn new() -> Self {
        Self {
            runner: ValidatorRunner::new(ValidatorMode::Base, false),
        }
    }

    pub fn set_throw_on_warnings(&mut self, v: bool) -> &mut Self {
        self.runner.throw_on_warnings = v;
        self
    }

    pub fn validate(&mut self, doc: &WdlDocument) -> Result<(), WdlError> {
        self.runner.validate(doc)
    }

    pub fn errors(&self) -> &[WdlSemanticError] {
        &self.runner.errors
    }
}

impl Default for WdlValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Static-analysis validator: base checks + duplicate/unknown-ref/type checks.
pub struct WdlStaticAnalysisValidator {
    runner: ValidatorRunner,
}

impl WdlStaticAnalysisValidator {
    pub fn new() -> Self {
        Self {
            runner: ValidatorRunner::new(ValidatorMode::Static, false),
        }
    }

    pub fn set_throw_on_warnings(&mut self, v: bool) -> &mut Self {
        self.runner.throw_on_warnings = v;
        self
    }

    pub fn validate(&mut self, doc: &WdlDocument) -> Result<(), WdlError> {
        self.runner.validate(doc)
    }

    pub fn errors(&self) -> &[WdlSemanticError] {
        &self.runner.errors
    }
}

impl Default for WdlStaticAnalysisValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Linting validator: static checks + unused-symbol and deprecated-feature warnings.
pub struct WdlLintingValidator {
    runner: ValidatorRunner,
}

impl WdlLintingValidator {
    pub fn new() -> Self {
        Self {
            runner: ValidatorRunner::new(ValidatorMode::Lint, true),
        }
    }

    pub fn set_throw_on_warnings(&mut self, v: bool) -> &mut Self {
        self.runner.throw_on_warnings = v;
        self
    }

    pub fn validate(&mut self, doc: &WdlDocument) -> Result<(), WdlError> {
        self.runner.validate(doc)
    }

    pub fn errors(&self) -> &[WdlSemanticError] {
        &self.runner.errors
    }
}

impl Default for WdlLintingValidator {
    fn default() -> Self {
        Self::new()
    }
}
