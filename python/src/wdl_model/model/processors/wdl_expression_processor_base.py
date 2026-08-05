"""Depth-first expression walker for the Python WDL object model."""

from __future__ import annotations

from wdl_model.model.expressions import (
    WdlStringPlaceholderOptionType,
    WdlArrayLiteral,
    WdlBinaryOperation,
    WdlBooleanLiteral,
    WdlExpression,
    WdlFloatLiteral,
    WdlFunctionCallOperation,
    WdlIndexAccessOperation,
    WdlIntLiteral,
    WdlMapEntry,
    WdlMapLiteral,
    WdlMemberAccessOperation,
    WdlNullLiteral,
    WdlObjectEntry,
    WdlObjectLiteral,
    WdlPairLiteral,
    WdlStringComponent,
    WdlStringEscape,
    WdlStringLiteral,
    WdlStringPlaceholder,
    WdlStringPlaceholderOption,
    WdlStringText,
    WdlStringToken,
    WdlStructEntry,
    WdlStructLiteral,
    WdlTernaryOperation,
    WdlUnaryOperation,
    WdlVariable,
)

from .wdl_expression_processor import WdlExpressionProcessor


class WdlExpressionProcessorBase(WdlExpressionProcessor):
    """Base depth-first expression walker."""

    def processExpression(self, expression: WdlExpression | None) -> None:
        self.walkExpression(expression)

    def walkExpression(self, expression: WdlExpression | None) -> None:
        if expression is None:
            return

        self.enterExpression(expression)

        if isinstance(expression, WdlBooleanLiteral):
            self.processBooleanLiteral(expression)
        elif isinstance(expression, WdlFloatLiteral):
            self.processFloatLiteral(expression)
        elif isinstance(expression, WdlIntLiteral):
            self.processIntLiteral(expression)
        elif isinstance(expression, WdlArrayLiteral):
            self.processArrayLiteral(expression)
        elif isinstance(expression, WdlMapLiteral):
            self.processMapLiteral(expression)
        elif isinstance(expression, WdlNullLiteral):
            self.processNullLiteral(expression)
        elif isinstance(expression, WdlObjectLiteral):
            self.processObjectLiteral(expression)
        elif isinstance(expression, WdlPairLiteral):
            self.processPairLiteral(expression)
        elif isinstance(expression, WdlStringLiteral):
            self.processStringLiteral(expression)
        elif isinstance(expression, WdlStructLiteral):
            self.processStructLiteral(expression)
        elif isinstance(expression, WdlVariable):
            self.processVariable(expression)
        elif isinstance(expression, WdlBinaryOperation):
            self.processBinaryOperation(expression)
        elif isinstance(expression, WdlFunctionCallOperation):
            self.processFunctionCallOperation(expression)
        elif isinstance(expression, WdlIndexAccessOperation):
            self.processIndexAccessOperation(expression)
        elif isinstance(expression, WdlMemberAccessOperation):
            self.processMemberAccessOperation(expression)
        elif isinstance(expression, WdlTernaryOperation):
            self.processTernaryOperation(expression)
        elif isinstance(expression, WdlUnaryOperation):
            self.processUnaryOperation(expression)
        else:
            raise RuntimeError(
                f"Unhandled expression type: {expression.__class__.__name__}"
            )

        self.exitExpression(expression)

    def walkStringComponent(
        self,
        context: WdlStringLiteral,
        component: WdlStringComponent | None,
    ) -> None:
        if component is None:
            return

        self.enterStringComponent(context, component)
        if isinstance(component, WdlStringText):
            self.processStringText(context, component)
        elif isinstance(component, WdlStringEscape):
            self.processStringEscape(context, component)
        elif isinstance(component, WdlStringToken):
            self.processStringToken(context, component)
        elif isinstance(component, WdlStringPlaceholder):
            self.processStringPlaceholder(context, component)
        else:
            raise RuntimeError(
                f"Unhandled string component type: {component.__class__.__name__}"
            )
        self.exitStringComponent(context, component)

    def enterExpression(self, expression: WdlExpression) -> None:
        pass

    def exitExpression(self, expression: WdlExpression) -> None:
        pass

    def enterStringComponent(
        self, context: WdlStringLiteral, component: WdlStringComponent
    ) -> None:
        pass

    def exitStringComponent(
        self, context: WdlStringLiteral, component: WdlStringComponent
    ) -> None:
        pass

    def processBooleanLiteral(self, expression: WdlBooleanLiteral) -> None:
        pass

    def processFloatLiteral(self, expression: WdlFloatLiteral) -> None:
        pass

    def processIntLiteral(self, expression: WdlIntLiteral) -> None:
        pass

    def processNullLiteral(self, expression: WdlNullLiteral) -> None:
        pass

    def processVariable(self, expression: WdlVariable) -> None:
        pass

    def processArrayLiteral(self, expression: WdlArrayLiteral) -> None:
        for e in expression.entries():
            self.walkExpression(e)

    def processMapLiteral(self, expression: WdlMapLiteral) -> None:
        for entry in expression.entries():
            self.processMapEntry(expression, entry)

    def processMapEntry(self, context: WdlMapLiteral, entry: WdlMapEntry) -> None:
        self.walkExpression(entry.getKey())
        self.walkExpression(entry.getValue())

    def processObjectLiteral(self, expression: WdlObjectLiteral) -> None:
        for entry in expression.entries():
            self.processObjectEntry(expression, entry)

    def processObjectEntry(
        self, context: WdlObjectLiteral, entry: WdlObjectEntry
    ) -> None:
        self.walkExpression(entry.getValue())

    def processPairLiteral(self, expression: WdlPairLiteral) -> None:
        self.walkExpression(expression.left)
        self.walkExpression(expression.right)

    def processStringLiteral(self, expression: WdlStringLiteral) -> None:
        for component in expression.components():
            self.walkStringComponent(expression, component)

    def processStringText(self, context: WdlStringLiteral, text: WdlStringText) -> None:
        pass

    def processStringEscape(
        self, context: WdlStringLiteral, escape: WdlStringEscape
    ) -> None:
        pass

    def processStringToken(
        self, context: WdlStringLiteral, token: WdlStringToken
    ) -> None:
        pass

    def processStringPlaceholder(
        self, context: WdlStringLiteral, placeholder: WdlStringPlaceholder
    ) -> None:
        self.processStringPlaceholderOption(context, placeholder, placeholder.option)
        self.walkExpression(placeholder.expression)

    def processStringPlaceholderOption(
        self,
        context: WdlStringLiteral,
        placeholder: WdlStringPlaceholder,
        option: WdlStringPlaceholderOption | None,
    ) -> None:
        if option is None:
            return
        if option.type == WdlStringPlaceholderOptionType.DEFAULT:
            self.walkExpression(option.value)
        elif option.type == WdlStringPlaceholderOptionType.TRUE_FALSE:
            self.walkExpression(option.trueValue)
            self.walkExpression(option.falseValue)
        else:
            raise RuntimeError(f"Unhandled placeholder option type: {option.type}")

    def processStructLiteral(self, expression: WdlStructLiteral) -> None:
        for entry in expression.entries():
            self.processStructEntry(expression, entry)

    def processStructEntry(
        self, context: WdlStructLiteral, entry: WdlStructEntry
    ) -> None:
        self.walkExpression(entry.getValue())

    def processBinaryOperation(self, expression: WdlBinaryOperation) -> None:
        self.walkExpression(expression.left)
        self.walkExpression(expression.right)

    def processFunctionCallOperation(
        self, expression: WdlFunctionCallOperation
    ) -> None:
        for arg in expression.arguments():
            self.walkExpression(arg)

    def processIndexAccessOperation(self, expression: WdlIndexAccessOperation) -> None:
        self.walkExpression(expression.target)
        self.walkExpression(expression.index)

    def processMemberAccessOperation(
        self, expression: WdlMemberAccessOperation
    ) -> None:
        self.walkExpression(expression.target)

    def processTernaryOperation(self, expression: WdlTernaryOperation) -> None:
        self.walkExpression(expression.condition)
        self.walkExpression(expression.trueValue)
        self.walkExpression(expression.falseValue)

    def processUnaryOperation(self, expression: WdlUnaryOperation) -> None:
        self.walkExpression(expression.operand)
