from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from .wdl_expression import ComponentType, WdlExpression


@dataclass
class WdlArrayLiteral(WdlExpression):
    _entries: deque[WdlExpression] = field(default_factory=deque)

    def entries(self) -> deque[WdlExpression]:
        return self._entries

    def __str__(self) -> str:
        return f"[{', '.join(str(e) for e in self._entries)}]"

    def componentType(self) -> ComponentType:
        return ComponentType.ARRAY_LITERAL
