"""Visitor-style processor contract for WDL expression trees."""

from __future__ import annotations

from abc import ABC, abstractmethod

from wdl_model.model.expressions import (
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


class WdlExpressionProcessor(ABC):
    """Processor contract for walking expression trees."""

    @abstractmethod
    def processExpression(self, expression: WdlExpression | None) -> None: ...

    def dispatchByComponentType(self, expression: WdlExpression | None) -> None:
        """Dispatch helper for expression nodes based on concrete type."""
        if expression is None:
            return
        if isinstance(expression, WdlBooleanLiteral):
            self.onBooleanLiteral(expression)
        elif isinstance(expression, WdlFloatLiteral):
            self.onFloatLiteral(expression)
        elif isinstance(expression, WdlIntLiteral):
            self.onIntLiteral(expression)
        elif isinstance(expression, WdlArrayLiteral):
            self.onArrayLiteral(expression)
        elif isinstance(expression, WdlMapLiteral):
            self.onMapLiteral(expression)
        elif isinstance(expression, WdlNullLiteral):
            self.onNullLiteral(expression)
        elif isinstance(expression, WdlObjectLiteral):
            self.onObjectLiteral(expression)
        elif isinstance(expression, WdlPairLiteral):
            self.onPairLiteral(expression)
        elif isinstance(expression, WdlStringLiteral):
            self.onStringLiteral(expression)
        elif isinstance(expression, WdlStructLiteral):
            self.onStructLiteral(expression)
        elif isinstance(expression, WdlVariable):
            self.onVariable(expression)
        elif isinstance(expression, WdlBinaryOperation):
            self.onBinaryOperation(expression)
        elif isinstance(expression, WdlFunctionCallOperation):
            self.onFunctionCallOperation(expression)
        elif isinstance(expression, WdlIndexAccessOperation):
            self.onIndexAccessOperation(expression)
        elif isinstance(expression, WdlMemberAccessOperation):
            self.onMemberAccessOperation(expression)
        elif isinstance(expression, WdlTernaryOperation):
            self.onTernaryOperation(expression)
        elif isinstance(expression, WdlUnaryOperation):
            self.onUnaryOperation(expression)
        else:
            raise RuntimeError(
                f"Unhandled expression type: {expression.__class__.__name__}"
            )

    def dispatchStringComponentByType(
        self,
        context: WdlStringLiteral,
        component: WdlStringComponent | None,
    ) -> None:
        """Dispatch helper for string literal components by concrete type."""
        if component is None:
            return
        if isinstance(component, WdlStringText):
            self.onStringText(context, component)
        elif isinstance(component, WdlStringEscape):
            self.onStringEscape(context, component)
        elif isinstance(component, WdlStringToken):
            self.onStringToken(context, component)
        elif isinstance(component, WdlStringPlaceholder):
            self.onStringPlaceholder(context, component)
        else:
            raise RuntimeError(
                f"Unhandled string component type: {component.__class__.__name__}"
            )

    def onEnterExpression(self, expression: WdlExpression) -> None:
        pass

    def onExitExpression(self, expression: WdlExpression) -> None:
        pass

    def onEnterStringComponent(
        self, context: WdlStringLiteral, component: WdlStringComponent
    ) -> None:
        pass

    def onExitStringComponent(
        self, context: WdlStringLiteral, component: WdlStringComponent
    ) -> None:
        pass

    def onBooleanLiteral(self, expression: WdlBooleanLiteral) -> None:
        pass

    def onFloatLiteral(self, expression: WdlFloatLiteral) -> None:
        pass

    def onIntLiteral(self, expression: WdlIntLiteral) -> None:
        pass

    def onNullLiteral(self, expression: WdlNullLiteral) -> None:
        pass

    def onVariable(self, expression: WdlVariable) -> None:
        pass

    def onArrayLiteral(self, expression: WdlArrayLiteral) -> None:
        pass

    def onMapLiteral(self, expression: WdlMapLiteral) -> None:
        pass

    def onMapEntry(self, context: WdlMapLiteral, entry: WdlMapEntry) -> None:
        pass

    def onObjectLiteral(self, expression: WdlObjectLiteral) -> None:
        pass

    def onObjectEntry(self, context: WdlObjectLiteral, entry: WdlObjectEntry) -> None:
        pass

    def onPairLiteral(self, expression: WdlPairLiteral) -> None:
        pass

    def onStringLiteral(self, expression: WdlStringLiteral) -> None:
        pass

    def onStringText(self, context: WdlStringLiteral, text: WdlStringText) -> None:
        pass

    def onStringEscape(
        self, context: WdlStringLiteral, escape: WdlStringEscape
    ) -> None:
        pass

    def onStringToken(self, context: WdlStringLiteral, token: WdlStringToken) -> None:
        pass

    def onStringPlaceholder(
        self, context: WdlStringLiteral, placeholder: WdlStringPlaceholder
    ) -> None:
        pass

    def onStringPlaceholderOption(
        self,
        context: WdlStringLiteral,
        placeholder: WdlStringPlaceholder,
        option: WdlStringPlaceholderOption,
    ) -> None:
        pass

    def onStructLiteral(self, expression: WdlStructLiteral) -> None:
        pass

    def onStructEntry(self, context: WdlStructLiteral, entry: WdlStructEntry) -> None:
        pass

    def onBinaryOperation(self, expression: WdlBinaryOperation) -> None:
        pass

    def onFunctionCallOperation(self, expression: WdlFunctionCallOperation) -> None:
        pass

    def onIndexAccessOperation(self, expression: WdlIndexAccessOperation) -> None:
        pass

    def onMemberAccessOperation(self, expression: WdlMemberAccessOperation) -> None:
        pass

    def onTernaryOperation(self, expression: WdlTernaryOperation) -> None:
        pass

    def onUnaryOperation(self, expression: WdlUnaryOperation) -> None:
        pass
