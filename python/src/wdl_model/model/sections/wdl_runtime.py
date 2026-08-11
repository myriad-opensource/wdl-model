"""Legacy runtime section nodes."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlStringKeyValue
from wdl_model.model.definitions import WdlTaskElement
from wdl_model.model.expressions import WdlExpression


class WdlRuntimeEntry(WdlStringKeyValue):
    """Single runtime entry such as ``docker: \"image\"`` in older source documents."""

    def __init__(self, key: str | None = None, value: WdlExpression | None = None):
        super().__init__(key, value)


@dataclass
class WdlRuntime(WdlTaskElement):
    """Legacy ``runtime { ... }`` section retained for compatibility."""

    _elements: deque[WdlRuntimeEntry] = field(default_factory=deque)

    def elements(self) -> deque[WdlRuntimeEntry]:
        """Return the ordered runtime entries contained in the section."""
        return self._elements
