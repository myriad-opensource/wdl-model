from __future__ import annotations

from .wdl_expression import ComponentType
from .wdl_literal import WdlLiteral


class WdlBooleanLiteral(WdlLiteral[bool]):
    def __init__(self, value: bool | None = None):
        super().__init__(value)

    def componentType(self) -> ComponentType:
        return ComponentType.LITERAL
