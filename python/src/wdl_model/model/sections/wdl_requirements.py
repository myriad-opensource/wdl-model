"""Task requirements section nodes."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlStringKeyValue
from wdl_model.model.definitions import WdlTaskElement
from wdl_model.model.expressions import WdlExpression


class WdlRequirementEntry(WdlStringKeyValue):
    """Single requirement entry such as ``cpu: 2`` or ``container: \"image\"``."""

    def __init__(self, key: str | None = None, value: WdlExpression | None = None):
        super().__init__(key, value)


@dataclass
class WdlRequirements(WdlTaskElement):
    """WDL ``requirements { ... }`` section."""

    _elements: deque[WdlRequirementEntry] = field(default_factory=deque)

    def elements(self) -> deque[WdlRequirementEntry]:
        """Return the ordered requirement entries contained in the section."""
        return self._elements
