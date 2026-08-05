from __future__ import annotations

from dataclasses import dataclass

from .wdl_expression import ComponentType, WdlExpression


@dataclass
class WdlTernaryOperation(WdlExpression):
    condition: WdlExpression | None = None
    trueValue: WdlExpression | None = None
    falseValue: WdlExpression | None = None

    def __str__(self) -> str:
        return f"if {self.condition or ''} then {self.trueValue or ''} else {self.falseValue or ''}"

    def componentType(self) -> ComponentType:
        return ComponentType.TERNARY_OPERATION
