from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

from .wdl_expression import ComponentType, WdlExpression


class Operator(Enum):
    LOGICAL_OR = "||"
    LOGICAL_AND = "&&"
    EQUAL = "=="
    NOT_EQUAL = "!="
    LESS = "<"
    LESS_EQUAL = "<="
    GREATER = ">"
    GREATER_EQUAL = ">="
    ADD = "+"
    SUBTRACT = "-"
    MULTIPLY = "*"
    DIVIDE = "/"
    MODULUS = "%"
    POWER = "**"

    def getWdlString(self) -> str:
        return self.value


@dataclass
class WdlBinaryOperation(WdlExpression):
    left: WdlExpression | None = None
    operator: Operator | None = None
    right: WdlExpression | None = None

    def __str__(self) -> str:
        return f"({self.left or ''} {self.operator.value if self.operator else ''} {self.right or ''})"

    def componentType(self) -> ComponentType:
        return ComponentType.BINARY_OPERATION
