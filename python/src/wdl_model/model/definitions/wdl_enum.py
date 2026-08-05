"""Enum definition nodes for the Python WDL object model."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlStringKeyValue
from wdl_model.model.expressions import WdlExpression
from wdl_model.model.types import WdlType


class WdlEnumChoice(WdlStringKeyValue):
    """Single enum choice with a symbolic name and optional explicit value expression."""

    def __init__(self, key: str | None = None, value: WdlExpression | None = None):
        super().__init__(key, value)


@dataclass
class WdlEnum:
    """Enum definition node added by newer WDL 1.x specification revisions."""

    name: str | None = None
    valueType: WdlType | None = None
    _elements: deque[WdlEnumChoice] = field(default_factory=deque)

    def elements(self) -> deque[WdlEnumChoice]:
        """Return the ordered enum choices."""
        return self._elements
