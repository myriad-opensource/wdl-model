//! `WdlExpressionProcessor` trait — depth-first expression tree walker.
//!
//! This module mirrors Java's `WdlExpressionProcessor` interface and
//! `WdlExpressionProcessorBase` abstract class, unified into a single Rust trait
//! with provided default methods.
//!
//! ## Usage
//!
//! Implement `WdlExpressionProcessor` and override only the hooks you care about, then call
//! `self.walk_expression(root_expr)` (or the free function `walk_expression(processor, expr)`)
//! to start the traversal.
//!
//! ## Traversal order
//!
//! Depth-first, pre-order: `enter_expression` fires before children are visited,
//! `exit_expression` fires after all children have been visited.

use crate::expressions::{
    ExprComponentType, WdlArrayLiteral, WdlBinaryOperation, WdlExpression,
    WdlFunctionCallOperation, WdlIndexAccessOperation, WdlMapEntry, WdlMapLiteral,
    WdlMemberAccessOperation, WdlObjectEntry, WdlObjectLiteral, WdlPairLiteral, WdlStringComponent,
    WdlStringLiteral, WdlStringPlaceholderOption, WdlStructEntry, WdlStructLiteral,
    WdlTernaryOperation, WdlUnaryOperation,
};

// Re-export component-type enum for callers who want to match on it.
pub use crate::expressions::ExprComponentType as ComponentType;

// ---------------------------------------------------------------------------
// WdlExpressionProcessor trait
// ---------------------------------------------------------------------------

/// Depth-first expression tree walker.
///
/// Mirrors Java's `WdlExpressionProcessor` interface + `WdlExpressionProcessorBase`.
///
/// The provided methods `walk_expression` and `walk_string_component` implement the default
/// traversal.  Override the `enter_*` / `exit_*` hooks and the per-node callbacks to plug in
/// your logic without re-implementing the traversal.
pub trait WdlExpressionProcessor {
    // -----------------------------------------------------------------------
    // Primary entry points
    // -----------------------------------------------------------------------

    /// Walk the expression tree rooted at `expression`.
    ///
    /// Calls `enter_expression`, dispatches to per-node hooks (which recurse into children
    /// by default), then calls `exit_expression`.
    fn walk_expression(&mut self, expression: &WdlExpression) {
        self.enter_expression(expression);

        match expression.component_type() {
            ExprComponentType::BoolLit => {
                if let WdlExpression::BoolLit(v) = expression {
                    self.on_bool_literal(*v);
                }
            }
            ExprComponentType::FloatLit => {
                if let WdlExpression::FloatLit(v) = expression {
                    self.on_float_literal(*v);
                }
            }
            ExprComponentType::IntLit => {
                if let WdlExpression::IntLit(v) = expression {
                    self.on_int_literal(*v);
                }
            }
            ExprComponentType::NullLit => self.on_null_literal(),
            ExprComponentType::Variable => {
                if let WdlExpression::Variable(name) = expression {
                    self.on_variable(name);
                }
            }
            ExprComponentType::StrLit => {
                if let WdlExpression::StrLit(lit) = expression {
                    self.process_string_literal(lit);
                }
            }
            ExprComponentType::ArrayLit => {
                if let WdlExpression::ArrayLit(lit) = expression {
                    self.process_array_literal(lit);
                }
            }
            ExprComponentType::MapLit => {
                if let WdlExpression::MapLit(lit) = expression {
                    self.process_map_literal(lit);
                }
            }
            ExprComponentType::ObjLit => {
                if let WdlExpression::ObjLit(lit) = expression {
                    self.process_object_literal(lit);
                }
            }
            ExprComponentType::PairLit => {
                if let WdlExpression::PairLit(lit) = expression {
                    self.process_pair_literal(lit);
                }
            }
            ExprComponentType::StructLit => {
                if let WdlExpression::StructLit(lit) = expression {
                    self.process_struct_literal(lit);
                }
            }
            ExprComponentType::BinaryOp => {
                if let WdlExpression::BinaryOp(op) = expression {
                    self.process_binary_operation(op);
                }
            }
            ExprComponentType::UnaryOp => {
                if let WdlExpression::UnaryOp(op) = expression {
                    self.process_unary_operation(op);
                }
            }
            ExprComponentType::TernaryOp => {
                if let WdlExpression::TernaryOp(op) = expression {
                    self.process_ternary_operation(op);
                }
            }
            ExprComponentType::FuncOp => {
                if let WdlExpression::FuncOp(op) = expression {
                    self.process_function_call_operation(op);
                }
            }
            ExprComponentType::IdxOp => {
                if let WdlExpression::IdxOp(op) = expression {
                    self.process_index_access_operation(op);
                }
            }
            ExprComponentType::MemberOp => {
                if let WdlExpression::MemberOp(op) = expression {
                    self.process_member_access_operation(op);
                }
            }
        }

        self.exit_expression(expression);
    }

    /// Walk a single string component (called from `process_string_literal`).
    fn walk_string_component(
        &mut self,
        context: &WdlStringLiteral,
        component: &WdlStringComponent,
    ) {
        self.enter_string_component(context, component);

        match component {
            WdlStringComponent::Text(t) => self.on_string_text(context, t),
            WdlStringComponent::Escape(e) => self.on_string_escape(context, e),
            WdlStringComponent::Special(s) => self.on_string_token(context, s),
            WdlStringComponent::Placeholder { symbol: _, option, expression } => {
                self.process_string_placeholder(context, option.as_deref(), expression);
            }
        }

        self.exit_string_component(context, component);
    }

    // -----------------------------------------------------------------------
    // Entry / exit hooks
    // -----------------------------------------------------------------------

    /// Called before any expression node is processed.
    fn enter_expression(&mut self, _expression: &WdlExpression) {}

    /// Called after all children of an expression node have been processed.
    fn exit_expression(&mut self, _expression: &WdlExpression) {}

    /// Called before a string component is processed.
    fn enter_string_component(
        &mut self,
        _context: &WdlStringLiteral,
        _component: &WdlStringComponent,
    ) {
    }

    /// Called after a string component has been processed.
    fn exit_string_component(
        &mut self,
        _context: &WdlStringLiteral,
        _component: &WdlStringComponent,
    ) {
    }

    // -----------------------------------------------------------------------
    // Leaf expression hooks (no children — default is no-op)
    // -----------------------------------------------------------------------

    fn on_bool_literal(&mut self, _value: bool) {}
    fn on_float_literal(&mut self, _value: f64) {}
    fn on_int_literal(&mut self, _value: i64) {}
    fn on_null_literal(&mut self) {}
    fn on_variable(&mut self, _name: &str) {}

    // -----------------------------------------------------------------------
    // Leaf string-component hooks (default is no-op)
    // -----------------------------------------------------------------------

    fn on_string_text(&mut self, _context: &WdlStringLiteral, _text: &str) {}
    fn on_string_escape(&mut self, _context: &WdlStringLiteral, _escape: &str) {}
    fn on_string_token(&mut self, _context: &WdlStringLiteral, _token: &str) {}

    // -----------------------------------------------------------------------
    // Compound expression hooks — default implementations recurse into children
    // -----------------------------------------------------------------------

    /// Default: iterates and walks each entry.
    fn process_array_literal(&mut self, expression: &WdlArrayLiteral) {
        for entry in &expression.entries {
            self.walk_expression(entry);
        }
    }

    /// Default: calls `process_map_entry` for each entry.
    fn process_map_literal(&mut self, expression: &WdlMapLiteral) {
        let entries: Vec<_> = expression.entries.iter().collect();
        for entry in entries {
            self.process_map_entry(expression, entry);
        }
    }

    /// Default: walks key and value.
    fn process_map_entry(&mut self, _context: &WdlMapLiteral, entry: &WdlMapEntry) {
        self.walk_expression(&entry.key);
        if let Some(val) = &entry.value {
            self.walk_expression(val);
        }
    }

    /// Default: calls `process_object_entry` for each entry.
    fn process_object_literal(&mut self, expression: &WdlObjectLiteral) {
        let entries: Vec<_> = expression.entries.iter().collect();
        for entry in entries {
            self.process_object_entry(expression, entry);
        }
    }

    /// Default: walks the value expression.
    fn process_object_entry(&mut self, _context: &WdlObjectLiteral, entry: &WdlObjectEntry) {
        if let Some(val) = &entry.value {
            self.walk_expression(val);
        }
    }

    /// Default: walks left and right.
    fn process_pair_literal(&mut self, expression: &WdlPairLiteral) {
        self.walk_expression(&expression.left);
        self.walk_expression(&expression.right);
    }

    /// Default: walks each string component via `walk_string_component`.
    fn process_string_literal(&mut self, expression: &WdlStringLiteral) {
        let components: Vec<_> = expression.components.iter().collect();
        for component in components {
            self.walk_string_component(expression, component);
        }
    }

    /// Default: processes the placeholder option (recurses into value expressions), then
    /// walks the placeholder's main expression.
    fn process_string_placeholder(
        &mut self,
        context: &WdlStringLiteral,
        option: Option<&WdlStringPlaceholderOption>,
        expression: &WdlExpression,
    ) {
        self.process_string_placeholder_option(context, option);
        self.walk_expression(expression);
    }

    /// Default: walks the option's value expression(s) if present.
    fn process_string_placeholder_option(
        &mut self,
        _context: &WdlStringLiteral,
        option: Option<&WdlStringPlaceholderOption>,
    ) {
        if let Some(opt) = option {
            match opt {
                WdlStringPlaceholderOption::Sep(val)
                | WdlStringPlaceholderOption::Default(val) => {
                    self.walk_expression(&WdlExpression::StrLit(val.clone()));
                }
                WdlStringPlaceholderOption::TrueFalse { true_value, false_value }
                | WdlStringPlaceholderOption::FalseTrue { true_value, false_value } => {
                    self.walk_expression(&WdlExpression::StrLit(true_value.clone()));
                    self.walk_expression(&WdlExpression::StrLit(false_value.clone()));
                }
            }
        }
    }

    /// Default: calls `process_struct_entry` for each entry.
    fn process_struct_literal(&mut self, expression: &WdlStructLiteral) {
        let entries: Vec<_> = expression.entries.iter().collect();
        for entry in entries {
            self.process_struct_entry(expression, entry);
        }
    }

    /// Default: walks the value expression.
    fn process_struct_entry(&mut self, _context: &WdlStructLiteral, entry: &WdlStructEntry) {
        if let Some(val) = &entry.value {
            self.walk_expression(val);
        }
    }

    /// Default: walks left and right sub-expressions.
    fn process_binary_operation(&mut self, expression: &WdlBinaryOperation) {
        self.walk_expression(&expression.left);
        self.walk_expression(&expression.right);
    }

    /// Default: walks the operand.
    fn process_unary_operation(&mut self, expression: &WdlUnaryOperation) {
        self.walk_expression(&expression.operand);
    }

    /// Default: walks condition, true value, false value.
    fn process_ternary_operation(&mut self, expression: &WdlTernaryOperation) {
        self.walk_expression(&expression.condition);
        self.walk_expression(&expression.true_value);
        self.walk_expression(&expression.false_value);
    }

    /// Default: walks each argument.
    fn process_function_call_operation(&mut self, expression: &WdlFunctionCallOperation) {
        for arg in &expression.arguments {
            self.walk_expression(arg);
        }
    }

    /// Default: walks the target and index expressions.
    fn process_index_access_operation(&mut self, expression: &WdlIndexAccessOperation) {
        self.walk_expression(&expression.target);
        self.walk_expression(&expression.index);
    }

    /// Default: walks the target expression.
    fn process_member_access_operation(&mut self, expression: &WdlMemberAccessOperation) {
        self.walk_expression(&expression.target);
    }
}
