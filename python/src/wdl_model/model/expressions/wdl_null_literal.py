from __future__ import annotations

from .wdl_expression import ComponentType
from .wdl_expression import WdlExpression


class WdlNullLiteral(WdlExpression):
    def __str__(self) -> str:
        return "null"

    def componentType(self) -> ComponentType:
        return ComponentType.LITERAL
