from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

from .wdl_expression import ComponentType, WdlExpression


class Operator(Enum):
    NOT = "!"
    PLUS = "+"
    MINUS = "-"

    def getWdlString(self) -> str:
        return self.value


@dataclass
class WdlUnaryOperation(WdlExpression):
    operator: Operator | None = None
    operand: WdlExpression | None = None

    def __str__(self) -> str:
        return f"{self.operator.value if self.operator else ''}{self.operand or ''}"

    def componentType(self) -> ComponentType:
        return ComponentType.UNARY_OPERATION
