from __future__ import annotations

from dataclasses import dataclass

from .wdl_expression import ComponentType, WdlExpression


@dataclass
class WdlIndexAccessOperation(WdlExpression):
    target: WdlExpression | None = None
    index: WdlExpression | None = None

    def __str__(self) -> str:
        return f"{self.target or ''}[{self.index or ''}]"

    def componentType(self) -> ComponentType:
        return ComponentType.INDEX_ACCESS
