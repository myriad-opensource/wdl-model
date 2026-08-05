from __future__ import annotations

from dataclasses import dataclass

from .wdl_expression import ComponentType, WdlExpression


@dataclass
class WdlVariable(WdlExpression):
    name: str | None = None

    def getName(self) -> str | None:
        return self.name

    def setName(self, name: str) -> None:
        self.name = name

    def __str__(self) -> str:
        return self.name or ""

    def componentType(self) -> ComponentType:
        return ComponentType.VARIABLE
