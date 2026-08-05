from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlStringKeyValue

from .wdl_expression import ComponentType, WdlExpression


class WdlObjectEntry(WdlStringKeyValue):
    pass


@dataclass
class WdlObjectLiteral(WdlExpression):
    _entries: deque[WdlObjectEntry] = field(default_factory=deque)

    def entries(self) -> deque[WdlObjectEntry]:
        return self._entries

    def __str__(self) -> str:
        return (
            "object {"
            + ", ".join(f"{e.getKey()}: {e.getValue()}" for e in self._entries)
            + "}"
        )

    def componentType(self) -> ComponentType:
        return ComponentType.OBJECT_LITERAL
