"""Task definition nodes for the Python WDL object model."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlNode


class WdlTaskElement(WdlNode):
    """Marker for any node that can appear directly inside a task body."""

    pass


@dataclass
class WdlTask:
    """Task definition node.

    In the WDL specification, a task is the reusable unit that declares inputs, command text,
    outputs, requirements, hints, metadata, and private declarations.
    """

    name: str | None = None
    _elements: deque[WdlTaskElement] = field(default_factory=deque)

    def elements(self) -> deque[WdlTaskElement]:
        """Return the ordered task body elements exactly as they appeared in the source."""
        return self._elements
