from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlStringKeyValue

from .wdl_expression import ComponentType, WdlExpression


class WdlStructEntry(WdlStringKeyValue):
    pass


@dataclass
class WdlStructLiteral(WdlExpression):
    name: str | None = None
    _entries: deque[WdlStructEntry] = field(default_factory=deque)

    def entries(self) -> deque[WdlStructEntry]:
        return self._entries

    def __str__(self) -> str:
        payload = ", ".join(f"{e.getKey()}: {e.getValue()}" for e in self._entries)
        return f"{self.name or ''} {{{payload}}}"

    def componentType(self) -> ComponentType:
        return ComponentType.STRUCT_LITERAL
