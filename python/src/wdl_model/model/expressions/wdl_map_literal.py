from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlExpresionKeyValue

from .wdl_expression import ComponentType, WdlExpression


class WdlMapEntry(WdlExpresionKeyValue):
    pass


@dataclass
class WdlMapLiteral(WdlExpression):
    _entries: deque[WdlMapEntry] = field(default_factory=deque)

    def entries(self) -> deque[WdlMapEntry]:
        return self._entries

    def __str__(self) -> str:
        return (
            "{"
            + ", ".join(f"{e.getKey()}: {e.getValue()}" for e in self._entries)
            + "}"
        )

    def componentType(self) -> ComponentType:
        return ComponentType.MAP_LITERAL
