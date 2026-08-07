//! Free-function rendering helpers: model node → WDL source text.
//!
//! These functions mirror the rendering methods on `WdlProcessorBase` in the Java
//! implementation, but are plain free functions so any code (processors, validators,
//! tests) can call them without inheriting from a base class.

use crate::expressions::{
    WdlArrayLiteral, WdlBinaryOperation, WdlExpression, WdlFunctionCallOperation,
    WdlIndexAccessOperation, WdlMapLiteral, WdlMemberAccessOperation, WdlObjectLiteral,
    WdlPairLiteral, WdlStringComponent, WdlStringLiteral, WdlStringPlaceholderOption,
    WdlStructLiteral, WdlTernaryOperation, WdlUnaryOperation, StringDelimiter,
};
use crate::statements::{WdlBoundDeclaration, WdlDeclaration};
use crate::types::{TypeComponentType, WdlArrayType, WdlType};

// ---------------------------------------------------------------------------
// Top-level expression renderer
// ---------------------------------------------------------------------------

/// Render any expression node back into WDL source text.
///
/// Mirrors `WdlProcessorBase.expressionToWdl`.
pub fn expression_to_wdl(expr: &WdlExpression) -> String {
    match expr {
        WdlExpression::BoolLit(v) => v.to_string(),
        WdlExpression::IntLit(v) => v.to_string(),
        WdlExpression::FloatLit(v) => {
            // Preserve at least one decimal digit so it reads as a float literal.
            let s = v.to_string();
            if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            }
        }
        WdlExpression::NullLit => "None".to_string(),
        WdlExpression::Variable(name) => name.clone(),
        WdlExpression::StrLit(lit) => string_literal_to_wdl(lit, true),
        WdlExpression::ArrayLit(lit) => array_literal_to_wdl(lit),
        WdlExpression::MapLit(lit) => map_literal_to_wdl(lit),
        WdlExpression::ObjLit(lit) => object_literal_to_wdl(lit),
        WdlExpression::PairLit(lit) => pair_literal_to_wdl(lit),
        WdlExpression::StructLit(lit) => struct_literal_to_wdl(lit),
        WdlExpression::BinaryOp(op) => binary_op_to_wdl(op),
        WdlExpression::UnaryOp(op) => unary_op_to_wdl(op),
        WdlExpression::TernaryOp(op) => ternary_op_to_wdl(op),
        WdlExpression::FuncOp(op) => func_op_to_wdl(op),
        WdlExpression::IdxOp(op) => index_op_to_wdl(op),
        WdlExpression::MemberOp(op) => member_op_to_wdl(op),
    }
}

// ---------------------------------------------------------------------------
// String literals
// ---------------------------------------------------------------------------

/// Render a string literal, optionally wrapping it in its delimiter quotes.
///
/// Pass `quote = false` when rendering the content of a `command` section
/// (the outer delimiters are rendered separately).
///
/// Mirrors `WdlProcessorBase.stringLiteralToWdl`.
pub fn string_literal_to_wdl(lit: &WdlStringLiteral, quote: bool) -> String {
    let (start_q, end_q) = if quote {
        match lit.delimiter {
            StringDelimiter::SingleQuote => ("'", "'"),
            StringDelimiter::DoubleQuote => ("\"", "\""),
            StringDelimiter::Multiline => (">>>", "<<<"),
        }
    } else {
        ("", "")
    };

    let mut out = String::from(start_q);
    for component in &lit.components {
        string_component_to_wdl(lit, component, &mut out);
    }
    out.push_str(end_q);
    out
}

fn string_component_to_wdl(
    ctx: &WdlStringLiteral,
    component: &WdlStringComponent,
    out: &mut String,
) {
    match component {
        WdlStringComponent::Text(t) => out.push_str(t),
        WdlStringComponent::Escape(e) => out.push_str(e),
        WdlStringComponent::Special(s) => out.push_str(s),
        WdlStringComponent::Placeholder { symbol, option, expression } => {
            out.push_str(symbol.to_wdl_str());
            out.push('{');
            if let Some(opt) = option {
                placeholder_option_to_wdl(ctx, opt, out);
            }
            out.push_str(&expression_to_wdl(expression));
            out.push('}');
        }
    }
}

fn placeholder_option_to_wdl(
    _ctx: &WdlStringLiteral,
    opt: &WdlStringPlaceholderOption,
    out: &mut String,
) {
    match opt {
        WdlStringPlaceholderOption::TrueFalse { true_value, false_value } => {
            out.push_str("true=");
            out.push_str(&string_literal_to_wdl(true_value, true));
            out.push_str(" false=");
            out.push_str(&string_literal_to_wdl(false_value, true));
            out.push(' ');
        }
        WdlStringPlaceholderOption::FalseTrue { false_value, true_value } => {
            out.push_str("false=");
            out.push_str(&string_literal_to_wdl(false_value, true));
            out.push_str(" true=");
            out.push_str(&string_literal_to_wdl(true_value, true));
            out.push(' ');
        }
        WdlStringPlaceholderOption::Sep(val) => {
            out.push_str("sep=");
            out.push_str(&string_literal_to_wdl(val, true));
            out.push(' ');
        }
        WdlStringPlaceholderOption::Default(val) => {
            out.push_str("default=");
            out.push_str(&string_literal_to_wdl(val, true));
            out.push(' ');
        }
    }
}

// ---------------------------------------------------------------------------
// Literal helpers
// ---------------------------------------------------------------------------

fn array_literal_to_wdl(lit: &WdlArrayLiteral) -> String {
    let items: Vec<String> = lit.entries.iter().map(expression_to_wdl).collect();
    format!("[{}]", items.join(", "))
}

fn map_literal_to_wdl(lit: &WdlMapLiteral) -> String {
    let items: Vec<String> = lit
        .entries
        .iter()
        .map(|e| {
            let key = expression_to_wdl(&e.key);
            let val = e
                .value
                .as_ref()
                .map(expression_to_wdl)
                .unwrap_or_default();
            format!("{}: {}", key, val)
        })
        .collect();
    format!("{{{}}}", items.join(", "))
}

fn object_literal_to_wdl(lit: &WdlObjectLiteral) -> String {
    let items: Vec<String> = lit
        .entries
        .iter()
        .map(|e| {
            let val = e
                .value
                .as_ref()
                .map(expression_to_wdl)
                .unwrap_or_default();
            format!("{}: {}", e.key, val)
        })
        .collect();
    format!("{{{}}}", items.join(", "))
}

fn pair_literal_to_wdl(lit: &WdlPairLiteral) -> String {
    format!(
        "({}, {})",
        expression_to_wdl(&lit.left),
        expression_to_wdl(&lit.right)
    )
}

fn struct_literal_to_wdl(lit: &WdlStructLiteral) -> String {
    let items: Vec<String> = lit
        .entries
        .iter()
        .map(|e| {
            let val = e
                .value
                .as_ref()
                .map(expression_to_wdl)
                .unwrap_or_default();
            format!("{}: {}", e.key, val)
        })
        .collect();
    format!("{} {{{}}}", lit.name, items.join(", "))
}

// ---------------------------------------------------------------------------
// Operation helpers
// ---------------------------------------------------------------------------

fn binary_op_to_wdl(op: &WdlBinaryOperation) -> String {
    format!(
        "{} {} {}",
        expression_to_wdl(&op.left),
        op.operator.to_wdl_str(),
        expression_to_wdl(&op.right)
    )
}

fn unary_op_to_wdl(op: &WdlUnaryOperation) -> String {
    format!("{}{}", op.operator.to_wdl_str(), expression_to_wdl(&op.operand))
}

fn ternary_op_to_wdl(op: &WdlTernaryOperation) -> String {
    format!(
        "if ({}) {} else {}",
        expression_to_wdl(&op.condition),
        expression_to_wdl(&op.true_value),
        expression_to_wdl(&op.false_value)
    )
}

fn func_op_to_wdl(op: &WdlFunctionCallOperation) -> String {
    let args: Vec<String> = op.arguments.iter().map(expression_to_wdl).collect();
    format!("{}({})", op.function_name, args.join(", "))
}

fn index_op_to_wdl(op: &WdlIndexAccessOperation) -> String {
    format!(
        "{}[{}]",
        expression_to_wdl(&op.target),
        expression_to_wdl(&op.index)
    )
}

fn member_op_to_wdl(op: &WdlMemberAccessOperation) -> String {
    format!("{}.{}", expression_to_wdl(&op.target), op.member)
}

// ---------------------------------------------------------------------------
// Type renderer
// ---------------------------------------------------------------------------

/// Render a WDL type node back into source text.
///
/// Mirrors `WdlProcessorBase.typeToWdl`.
pub fn type_to_wdl(ty: &WdlType) -> String {
    let mut out = match ty.component_type() {
        TypeComponentType::Primitive => {
            if let WdlType::Primitive(p) = ty {
                p.primitive_kind.to_wdl_str().to_string()
            } else {
                unreachable!()
            }
        }
        TypeComponentType::Array => array_type_to_wdl(ty),
        TypeComponentType::Map => map_type_to_wdl(ty),
        TypeComponentType::Pair => pair_type_to_wdl(ty),
        TypeComponentType::TypeRef => {
            if let WdlType::TypeRef(r) = ty {
                r.reference_name.clone()
            } else {
                unreachable!()
            }
        }
    };
    if ty.is_optional() {
        out.push('?');
    }
    out
}

fn array_type_to_wdl(ty: &WdlType) -> String {
    if let WdlType::Array(WdlArrayType { member_type, non_empty, .. }) = ty {
        let inner = type_to_wdl(member_type);
        if *non_empty {
            format!("Array[{}]+", inner)
        } else {
            format!("Array[{}]", inner)
        }
    } else {
        unreachable!()
    }
}

fn map_type_to_wdl(ty: &WdlType) -> String {
    if let WdlType::Map(m) = ty {
        format!("Map[{}, {}]", type_to_wdl(&m.key_type), type_to_wdl(&m.value_type))
    } else {
        unreachable!()
    }
}

fn pair_type_to_wdl(ty: &WdlType) -> String {
    if let WdlType::Pair(p) = ty {
        format!("Pair[{},{}]", type_to_wdl(&p.left_type), type_to_wdl(&p.right_type))
    } else {
        unreachable!()
    }
}

// ---------------------------------------------------------------------------
// Declaration renderers
// ---------------------------------------------------------------------------

/// Render a bound declaration (`Type name = expr`) into WDL source text.
pub fn bound_declaration_to_wdl(decl: &WdlBoundDeclaration) -> String {
    let env_prefix = if decl.environment_variable { "env " } else { "" };
    format!(
        "{}{} {} = {}",
        env_prefix,
        type_to_wdl(&decl.wdl_type),
        decl.name,
        expression_to_wdl(&decl.expression)
    )
}

/// Render an unbound declaration (`Type name`) into WDL source text.
pub fn unbound_declaration_to_wdl(decl: &WdlDeclaration) -> String {
    let env_prefix = if decl.environment_variable { "env " } else { "" };
    format!("{}{} {}", env_prefix, type_to_wdl(&decl.wdl_type), decl.name)
}

/// Render an `InputDeclaration` (which may be bound or unbound) into WDL source text.
pub fn input_declaration_to_wdl(decl: &crate::sections::InputDeclaration) -> String {
    match decl {
        crate::sections::InputDeclaration::Unbound(d) => unbound_declaration_to_wdl(d),
        crate::sections::InputDeclaration::Bound(d) => bound_declaration_to_wdl(d),
    }
}
