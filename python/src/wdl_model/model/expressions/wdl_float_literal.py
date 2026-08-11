from __future__ import annotations

from .wdl_expression import ComponentType
from .wdl_number_literal import WdlNumberLiteral


class WdlFloatLiteral(WdlNumberLiteral[float]):
    def __init__(self, value: float | None = None):
        super().__init__(value)

    def negate(self) -> None:
        if self.value is not None:
            self.value = -self.value

    def componentType(self) -> ComponentType:
        return ComponentType.LITERAL
