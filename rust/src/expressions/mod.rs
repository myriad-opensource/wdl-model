//! WDL expression model.

use crate::version::WdlVersion;

/// High-level expression family for traversal and validation dispatch.
/// Mirrors `WdlExpression.ComponentType` in the Java implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExprComponentType {
    BoolLit,
    FloatLit,
    IntLit,
    ArrayLit,
    MapLit,
    NullLit,
    ObjLit,
    PairLit,
    StrLit,
    StructLit,
    Variable,
    BinaryOp,
    FuncOp,
    IdxOp,
    MemberOp,
    TernaryOp,
    UnaryOp,
}

// ---------------------------------------------------------------------------
// String literal and its components
// ---------------------------------------------------------------------------

/// Which quote style surrounds the string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringDelimiter {
    SingleQuote,
    DoubleQuote,
    Multiline,
}

/// Symbol used to introduce a placeholder (`~{…}` or `${…}`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceholderSymbol {
    Tilde,
    Dollar,
}

impl PlaceholderSymbol {
    pub fn to_wdl_str(self) -> &'static str {
        match self {
            PlaceholderSymbol::Tilde => "~",
            PlaceholderSymbol::Dollar => "$",
        }
    }
}

/// Deprecated placeholder option type (WDL < 1.2).
#[derive(Debug, Clone, PartialEq)]
pub enum WdlStringPlaceholderOption {
    /// `sep="…"` option.
    Sep(WdlStringLiteral),
    /// `default="…"` option.
    Default(WdlStringLiteral),
    /// `true="…" false="…"` option.
    TrueFalse {
        true_value: WdlStringLiteral,
        false_value: WdlStringLiteral,
    },
    /// `false="…" true="…"` option.
    FalseTrue {
        false_value: WdlStringLiteral,
        true_value: WdlStringLiteral,
    },
}

/// A single component within a string literal.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlStringComponent {
    /// Plain text segment.
    Text(String),
    /// Escape sequence (e.g. `\n`, `\t`).
    Escape(String),
    /// A `~{expr}` or `${expr}` interpolation.
    Placeholder {
        symbol: PlaceholderSymbol,
        option: Option<Box<WdlStringPlaceholderOption>>,
        expression: Box<WdlExpression>,
    },
    /// Special token text (dollar sign or tilde outside a placeholder).
    Special(String),
}

/// A WDL string literal with its delimiter and ordered components.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlStringLiteral {
    pub delimiter: StringDelimiter,
    pub components: Vec<WdlStringComponent>,
}

impl WdlStringLiteral {
    pub fn new(delimiter: StringDelimiter) -> Self {
        Self {
            delimiter,
            components: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Literals
// ---------------------------------------------------------------------------

/// `Array[…]` literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlArrayLiteral {
    pub entries: Vec<WdlExpression>,
}

/// A single `key: value` entry in a map literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlMapEntry {
    pub key: WdlExpression,
    pub value: Option<WdlExpression>,
}

/// `Map { key: value, … }` literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlMapLiteral {
    pub entries: Vec<WdlMapEntry>,
}

/// A single `key: value` entry in an object literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlObjectEntry {
    pub key: String,
    pub value: Option<WdlExpression>,
}

/// `object { key: value, … }` literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlObjectLiteral {
    pub entries: Vec<WdlObjectEntry>,
}

/// `(left, right)` pair literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlPairLiteral {
    pub left: Box<WdlExpression>,
    pub right: Box<WdlExpression>,
}

/// A single `key: value` entry in a struct literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlStructEntry {
    pub key: String,
    pub value: Option<WdlExpression>,
}

/// `TypeName { key: value, … }` struct literal.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlStructLiteral {
    pub name: String,
    pub entries: Vec<WdlStructEntry>,
}

// ---------------------------------------------------------------------------
// Operations
// ---------------------------------------------------------------------------

/// Binary operator. Mirrors `WdlBinaryOperation.Operator`.
/// Note: the Java source has LT mapped to `"<="` and LTE to `"<"` — mirrored exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Or,
    And,
    Eq,
    Neq,
    /// Mapped to `"<="` in Java source (preserved exactly).
    Lt,
    /// Mapped to `"<"` in Java source (preserved exactly).
    Lte,
    Gt,
    Gte,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

impl BinaryOperator {
    pub fn to_wdl_str(self) -> &'static str {
        match self {
            BinaryOperator::Or => "||",
            BinaryOperator::And => "&&",
            BinaryOperator::Eq => "==",
            BinaryOperator::Neq => "!=",
            BinaryOperator::Lt => "<=",
            BinaryOperator::Lte => "<",
            BinaryOperator::Gt => ">",
            BinaryOperator::Gte => ">=",
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Power => "**",
        }
    }
}

/// Unary operator. Mirrors `WdlUnaryOperation.Operator`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Not,
    Negative,
}

impl UnaryOperator {
    pub fn to_wdl_str(self) -> &'static str {
        match self {
            UnaryOperator::Not => "!",
            UnaryOperator::Negative => "-",
        }
    }
}

/// `left op right` binary operation.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlBinaryOperation {
    pub left: Box<WdlExpression>,
    pub operator: BinaryOperator,
    pub right: Box<WdlExpression>,
}

/// `op operand` unary operation.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlUnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<WdlExpression>,
}

/// `if condition then true_value else false_value` ternary.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlTernaryOperation {
    pub condition: Box<WdlExpression>,
    pub true_value: Box<WdlExpression>,
    pub false_value: Box<WdlExpression>,
}

/// `target[index]` index access.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlIndexAccessOperation {
    pub target: Box<WdlExpression>,
    pub index: Box<WdlExpression>,
}

/// `target.member` member access.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlMemberAccessOperation {
    pub target: Box<WdlExpression>,
    pub member: String,
}

// ---------------------------------------------------------------------------
// Standard library function catalog
// ---------------------------------------------------------------------------

/// Broad type hints used for function signatures.
/// Mirrors `WdlFunctionCallOperation.WdlFunction.T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionTypeHint {
    Any,
    AnyOptional,
    Number,
    Boolean,
    Int,
    Float,
    String,
    StringOptional,
    File,
    Directory,
    FileOrDirectory,
    Object,
    ArrayAny,
    ArrayFile,
    ArrayOptionalAny,
    ArrayInt,
    ArrayString,
    ArrayObject,
    ArrayPair,
    ArrayArrayAny,
    ArrayArrayString,
    MapAnyAny,
    MapAnyArray,
    MapStringString,
    PairArray,
}

/// Return/argument type hint for one overload of a function.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub returns: FunctionTypeHint,
    pub args: Vec<FunctionTypeHint>,
}

impl FunctionSignature {
    fn new(returns: FunctionTypeHint, args: &[FunctionTypeHint]) -> Self {
        Self {
            returns,
            args: args.to_vec(),
        }
    }
}

/// Sentinel constant for an unbounded number of arguments.
pub const UNBOUNDED: i32 = -1;

/// WDL standard-library function catalog entry.
/// Mirrors `WdlFunctionCallOperation.WdlFunction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WdlFunction {
    Floor,
    Ceil,
    Round,
    Min,
    Max,
    Sub,
    Stdout,
    Stderr,
    ReadLines,
    ReadTsv,
    ReadMap,
    ReadObject,
    ReadObjects,
    ReadJson,
    ReadInt,
    ReadFloat,
    ReadString,
    ReadBoolean,
    WriteLines,
    WriteTsv,
    WriteMap,
    WriteObject,
    WriteObjects,
    WriteJson,
    Glob,
    Size,
    Basename,
    Prefix,
    Suffix,
    Quote,
    Squote,
    Sep,
    Length,
    Range,
    Chunk,
    Cross,
    Zip,
    Unzip,
    Transpose,
    Flatten,
    SelectFirst,
    SelectAll,
    Contains,
    ContainsKey,
    Keys,
    Values,
    AsPairs,
    AsMap,
    CollectByKey,
    Matches,
    Find,
    Defined,
    JoinPaths,
    Value,
    /// Engine-specific non-standard function.
    Nonstandard,
}

impl WdlFunction {
    pub fn to_wdl_str(self) -> &'static str {
        match self {
            WdlFunction::Floor => "floor",
            WdlFunction::Ceil => "ceil",
            WdlFunction::Round => "round",
            WdlFunction::Min => "min",
            WdlFunction::Max => "max",
            WdlFunction::Sub => "sub",
            WdlFunction::Stdout => "stdout",
            WdlFunction::Stderr => "stderr",
            WdlFunction::ReadLines => "read_lines",
            WdlFunction::ReadTsv => "read_tsv",
            WdlFunction::ReadMap => "read_map",
            WdlFunction::ReadObject => "read_object",
            WdlFunction::ReadObjects => "read_objects",
            WdlFunction::ReadJson => "read_json",
            WdlFunction::ReadInt => "read_int",
            WdlFunction::ReadFloat => "read_float",
            WdlFunction::ReadString => "read_string",
            WdlFunction::ReadBoolean => "read_boolean",
            WdlFunction::WriteLines => "write_lines",
            WdlFunction::WriteTsv => "write_tsv",
            WdlFunction::WriteMap => "write_map",
            WdlFunction::WriteObject => "write_object",
            WdlFunction::WriteObjects => "write_objects",
            WdlFunction::WriteJson => "write_json",
            WdlFunction::Glob => "glob",
            WdlFunction::Size => "size",
            WdlFunction::Basename => "basename",
            WdlFunction::Prefix => "prefix",
            WdlFunction::Suffix => "suffix",
            WdlFunction::Quote => "quote",
            WdlFunction::Squote => "squote",
            WdlFunction::Sep => "sep",
            WdlFunction::Length => "length",
            WdlFunction::Range => "range",
            WdlFunction::Chunk => "chunk",
            WdlFunction::Cross => "cross",
            WdlFunction::Zip => "zip",
            WdlFunction::Unzip => "unzip",
            WdlFunction::Transpose => "transpose",
            WdlFunction::Flatten => "flatten",
            WdlFunction::SelectFirst => "select_first",
            WdlFunction::SelectAll => "select_all",
            WdlFunction::Contains => "contains",
            WdlFunction::ContainsKey => "contains_key",
            WdlFunction::Keys => "keys",
            WdlFunction::Values => "values",
            WdlFunction::AsPairs => "as_pairs",
            WdlFunction::AsMap => "as_map",
            WdlFunction::CollectByKey => "collect_by_key",
            WdlFunction::Matches => "matches",
            WdlFunction::Find => "find",
            WdlFunction::Defined => "defined",
            WdlFunction::JoinPaths => "join_paths",
            WdlFunction::Value => "value",
            WdlFunction::Nonstandard => "nonstandard",
        }
    }

    pub fn from_wdl_str(s: &str) -> Self {
        match s {
            "floor" => WdlFunction::Floor,
            "ceil" => WdlFunction::Ceil,
            "round" => WdlFunction::Round,
            "min" => WdlFunction::Min,
            "max" => WdlFunction::Max,
            "sub" => WdlFunction::Sub,
            "stdout" => WdlFunction::Stdout,
            "stderr" => WdlFunction::Stderr,
            "read_lines" => WdlFunction::ReadLines,
            "read_tsv" => WdlFunction::ReadTsv,
            "read_map" => WdlFunction::ReadMap,
            "read_object" => WdlFunction::ReadObject,
            "read_objects" => WdlFunction::ReadObjects,
            "read_json" => WdlFunction::ReadJson,
            "read_int" => WdlFunction::ReadInt,
            "read_float" => WdlFunction::ReadFloat,
            "read_string" => WdlFunction::ReadString,
            "read_boolean" => WdlFunction::ReadBoolean,
            "write_lines" => WdlFunction::WriteLines,
            "write_tsv" => WdlFunction::WriteTsv,
            "write_map" => WdlFunction::WriteMap,
            "write_object" => WdlFunction::WriteObject,
            "write_objects" => WdlFunction::WriteObjects,
            "write_json" => WdlFunction::WriteJson,
            "glob" => WdlFunction::Glob,
            "size" => WdlFunction::Size,
            "basename" => WdlFunction::Basename,
            "prefix" => WdlFunction::Prefix,
            "suffix" => WdlFunction::Suffix,
            "quote" => WdlFunction::Quote,
            "squote" => WdlFunction::Squote,
            "sep" => WdlFunction::Sep,
            "length" => WdlFunction::Length,
            "range" => WdlFunction::Range,
            "chunk" => WdlFunction::Chunk,
            "cross" => WdlFunction::Cross,
            "zip" => WdlFunction::Zip,
            "unzip" => WdlFunction::Unzip,
            "transpose" => WdlFunction::Transpose,
            "flatten" => WdlFunction::Flatten,
            "select_first" => WdlFunction::SelectFirst,
            "select_all" => WdlFunction::SelectAll,
            "contains" => WdlFunction::Contains,
            "contains_key" => WdlFunction::ContainsKey,
            "keys" => WdlFunction::Keys,
            "values" => WdlFunction::Values,
            "as_pairs" => WdlFunction::AsPairs,
            "as_map" => WdlFunction::AsMap,
            "collect_by_key" => WdlFunction::CollectByKey,
            "matches" => WdlFunction::Matches,
            "find" => WdlFunction::Find,
            "defined" => WdlFunction::Defined,
            "join_paths" => WdlFunction::JoinPaths,
            "value" => WdlFunction::Value,
            _ => WdlFunction::Nonstandard,
        }
    }

    /// Minimum number of arguments accepted.
    pub fn min_arity(self) -> i32 {
        use WdlFunction::*;
        match self {
            Floor | Ceil | Round => 1,
            Min | Max => 2,
            Sub => 3,
            Stdout | Stderr => 0,
            ReadLines | ReadMap | ReadObject | ReadObjects | ReadJson | ReadInt | ReadFloat
            | ReadString | ReadBoolean => 1,
            ReadTsv => 1,
            WriteLines | WriteTsv | WriteMap | WriteObject | WriteObjects | WriteJson => 1,
            Glob => 1,
            Size => 1,
            Basename => 1,
            Prefix => 2,
            Suffix => 2,
            Quote | Squote => 1,
            Sep => 2,
            Length => 1,
            Range => 1,
            Chunk => 2,
            Cross | Zip => 2,
            Unzip => 1,
            Transpose | Flatten => 1,
            SelectFirst => 1,
            SelectAll => 1,
            Contains | ContainsKey => 2,
            Keys => 1,
            Values => 1,
            AsPairs | AsMap | CollectByKey => 1,
            Matches | Find => 2,
            Defined => 1,
            JoinPaths => 2,
            Value => 1,
            Nonstandard => 0,
        }
    }

    /// Maximum number of arguments accepted. `UNBOUNDED` (-1) means variadic.
    pub fn max_arity(self) -> i32 {
        use WdlFunction::*;
        match self {
            Floor | Ceil | Round => 1,
            Min | Max => 2,
            Sub => 4,
            Stdout | Stderr => 0,
            ReadLines | ReadMap | ReadObject | ReadObjects | ReadJson | ReadInt | ReadFloat
            | ReadString | ReadBoolean => 1,
            ReadTsv => 2,
            WriteLines | WriteTsv | WriteMap | WriteObject | WriteObjects | WriteJson => 1,
            Glob => 1,
            Size => 2,
            Basename => 2,
            Prefix => 2,
            Suffix => 2,
            Quote | Squote => 1,
            Sep => 2,
            Length => 1,
            Range => 1,
            Chunk => 2,
            Cross | Zip => 2,
            Unzip => 1,
            Transpose | Flatten => 1,
            SelectFirst => 2,
            SelectAll => 1,
            Contains | ContainsKey => 2,
            Keys => 1,
            Values => 1,
            AsPairs | AsMap | CollectByKey => 1,
            Matches | Find => 2,
            Defined => 1,
            JoinPaths => UNBOUNDED,
            Value => 1,
            Nonstandard => UNBOUNDED,
        }
    }

    /// The WDL version this function was first available in. `None` = available since 1.0.
    pub fn added_in(self) -> Option<WdlVersion> {
        use WdlFunction::*;
        match self {
            Min | Max | Suffix | Quote | Squote | Sep | Unzip | Keys | AsPairs | AsMap
            | CollectByKey => Some(WdlVersion::V1_1),
            Chunk | Contains | ContainsKey | Values | Matches | Find | JoinPaths => {
                Some(WdlVersion::V1_2)
            }
            Value => Some(WdlVersion::V1_3),
            _ => None,
        }
    }

    /// The WDL version this function was deprecated in, if any.
    pub fn deprecated_in(self) -> Option<WdlVersion> {
        None
    }

    /// The WDL version this function was removed in, if any.
    pub fn removed_in(self) -> Option<WdlVersion> {
        None
    }

    /// Whether this function accepts a variable number of arguments beyond `max_arity`.
    pub fn is_variadic(self) -> bool {
        self.max_arity() < 0
    }

    /// Whether the given arity is supported.
    pub fn supports_arity(self, arity: i32) -> bool {
        if arity < self.min_arity() {
            return false;
        }
        self.is_variadic() || arity <= self.max_arity()
    }

    /// Returns the type-hint signatures for this function.
    pub fn signatures(self) -> Vec<FunctionSignature> {
        use FunctionTypeHint as T;
        use WdlFunction::*;
        let sig = |ret, args: &[FunctionTypeHint]| FunctionSignature::new(ret, args);
        match self {
            Floor | Ceil | Round => vec![sig(T::Int, &[T::Float])],
            Min | Max => vec![sig(T::Number, &[T::Number, T::Number])],
            Sub => vec![sig(T::String, &[T::String, T::String, T::String, T::String])],
            Stdout | Stderr => vec![sig(T::File, &[])],
            ReadLines => vec![sig(T::ArrayString, &[T::File])],
            ReadTsv => vec![sig(T::ArrayArrayString, &[T::File])],
            ReadMap => vec![sig(T::MapStringString, &[T::File])],
            ReadObject => vec![sig(T::Object, &[T::File])],
            ReadObjects => vec![sig(T::ArrayObject, &[T::File])],
            ReadJson => vec![sig(T::Any, &[T::File])],
            ReadInt => vec![sig(T::Int, &[T::File])],
            ReadFloat => vec![sig(T::Float, &[T::File])],
            ReadString => vec![sig(T::String, &[T::File])],
            ReadBoolean => vec![sig(T::Boolean, &[T::File])],
            WriteLines => vec![sig(T::File, &[T::ArrayString])],
            WriteTsv => vec![sig(T::File, &[T::ArrayArrayAny])],
            WriteMap => vec![sig(T::File, &[T::MapStringString])],
            WriteObject => vec![sig(T::File, &[T::Object])],
            WriteObjects => vec![sig(T::File, &[T::ArrayObject])],
            WriteJson => vec![sig(T::File, &[T::Any])],
            Glob => vec![sig(T::ArrayFile, &[T::String])],
            Size => vec![
                sig(T::Float, &[T::FileOrDirectory]),
                sig(T::Float, &[T::Any, T::String]),
            ],
            Basename => vec![
                sig(T::String, &[T::FileOrDirectory]),
                sig(T::String, &[T::String, T::String]),
            ],
            Prefix => vec![sig(T::ArrayString, &[T::String, T::ArrayAny])],
            Suffix => vec![sig(T::ArrayString, &[T::String, T::ArrayAny])],
            Quote => vec![sig(T::ArrayString, &[T::ArrayAny])],
            Squote => vec![sig(T::ArrayString, &[T::ArrayAny])],
            Sep => vec![sig(T::String, &[T::String, T::ArrayAny])],
            Length => vec![sig(T::Int, &[T::Any])],
            Range => vec![sig(T::ArrayInt, &[T::Int])],
            Chunk => vec![sig(T::ArrayArrayAny, &[T::ArrayAny, T::Int])],
            Cross => vec![sig(T::ArrayPair, &[T::ArrayAny, T::ArrayAny])],
            Zip => vec![sig(T::ArrayPair, &[T::ArrayAny, T::ArrayAny])],
            Unzip => vec![sig(T::PairArray, &[T::ArrayPair])],
            Transpose => vec![sig(T::ArrayArrayAny, &[T::ArrayArrayAny])],
            Flatten => vec![sig(T::ArrayAny, &[T::ArrayArrayAny])],
            SelectFirst => vec![
                sig(T::Any, &[T::ArrayOptionalAny]),
                sig(T::Any, &[T::ArrayOptionalAny, T::Any]),
            ],
            SelectAll => vec![sig(T::ArrayAny, &[T::ArrayOptionalAny])],
            Contains => vec![
                sig(T::Boolean, &[T::ArrayAny, T::Any]),
                sig(T::Boolean, &[T::String, T::String]),
            ],
            ContainsKey => vec![sig(T::Boolean, &[T::MapAnyAny, T::Any])],
            Keys => vec![sig(T::ArrayAny, &[T::MapAnyAny])],
            Values => vec![sig(T::ArrayAny, &[T::MapAnyAny])],
            AsPairs => vec![sig(T::ArrayPair, &[T::MapAnyAny])],
            AsMap => vec![sig(T::MapAnyAny, &[T::ArrayPair])],
            CollectByKey => vec![sig(T::MapAnyArray, &[T::ArrayPair])],
            Matches => vec![sig(T::Boolean, &[T::String, T::String])],
            Find => vec![sig(T::StringOptional, &[T::String, T::String])],
            Defined => vec![sig(T::Boolean, &[T::AnyOptional])],
            JoinPaths => vec![sig(T::FileOrDirectory, &[T::FileOrDirectory, T::String])],
            Value => vec![sig(T::Any, &[T::Any])],
            Nonstandard => vec![],
        }
    }
}

/// A function-call expression.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlFunctionCallOperation {
    /// Source-level function name as written.
    pub function_name: String,
    /// Resolved catalog entry (`Nonstandard` for unknown functions).
    pub function: WdlFunction,
    /// Ordered argument expressions.
    pub arguments: Vec<WdlExpression>,
}

impl WdlFunctionCallOperation {
    pub fn new(function_name: impl Into<String>) -> Self {
        let name = function_name.into();
        let function = WdlFunction::from_wdl_str(&name);
        Self {
            function_name: name,
            function,
            arguments: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// WdlExpression — the top-level expression enum
// ---------------------------------------------------------------------------

/// WDL expression node. A single enum over all 17 expression families.
/// Mirrors the `WdlExpression` interface + its concrete classes in Java.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlExpression {
    BoolLit(bool),
    IntLit(i64),
    FloatLit(f64),
    StrLit(WdlStringLiteral),
    ArrayLit(WdlArrayLiteral),
    MapLit(WdlMapLiteral),
    ObjLit(WdlObjectLiteral),
    PairLit(Box<WdlPairLiteral>),
    StructLit(WdlStructLiteral),
    /// `None` / `null` literal.
    NullLit,
    Variable(String),
    BinaryOp(Box<WdlBinaryOperation>),
    UnaryOp(Box<WdlUnaryOperation>),
    TernaryOp(Box<WdlTernaryOperation>),
    FuncOp(WdlFunctionCallOperation),
    IdxOp(Box<WdlIndexAccessOperation>),
    MemberOp(Box<WdlMemberAccessOperation>),
}

impl WdlExpression {
    /// Returns the broad category of this expression.
    pub fn component_type(&self) -> ExprComponentType {
        match self {
            WdlExpression::BoolLit(_) => ExprComponentType::BoolLit,
            WdlExpression::IntLit(_) => ExprComponentType::IntLit,
            WdlExpression::FloatLit(_) => ExprComponentType::FloatLit,
            WdlExpression::StrLit(_) => ExprComponentType::StrLit,
            WdlExpression::ArrayLit(_) => ExprComponentType::ArrayLit,
            WdlExpression::MapLit(_) => ExprComponentType::MapLit,
            WdlExpression::ObjLit(_) => ExprComponentType::ObjLit,
            WdlExpression::PairLit(_) => ExprComponentType::PairLit,
            WdlExpression::StructLit(_) => ExprComponentType::StructLit,
            WdlExpression::NullLit => ExprComponentType::NullLit,
            WdlExpression::Variable(_) => ExprComponentType::Variable,
            WdlExpression::BinaryOp(_) => ExprComponentType::BinaryOp,
            WdlExpression::UnaryOp(_) => ExprComponentType::UnaryOp,
            WdlExpression::TernaryOp(_) => ExprComponentType::TernaryOp,
            WdlExpression::FuncOp(_) => ExprComponentType::FuncOp,
            WdlExpression::IdxOp(_) => ExprComponentType::IdxOp,
            WdlExpression::MemberOp(_) => ExprComponentType::MemberOp,
        }
    }
}

// ---------------------------------------------------------------------------
// Needed to fix missing ArrayArrayAny in FunctionTypeHint
// ---------------------------------------------------------------------------
// (Already defined above as ArrayArrayAny — the WriteTsv sig references it)
