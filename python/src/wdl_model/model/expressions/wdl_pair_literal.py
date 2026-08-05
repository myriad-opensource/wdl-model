from __future__ import annotations

from dataclasses import dataclass

from .wdl_expression import ComponentType, WdlExpression


@dataclass
class WdlPairLiteral(WdlExpression):
    left: WdlExpression | None = None
    right: WdlExpression | None = None

    def __str__(self) -> str:
        return f"pair({self.left or ''}, {self.right or ''})"

    def componentType(self) -> ComponentType:
        return ComponentType.PAIR_LITERAL
