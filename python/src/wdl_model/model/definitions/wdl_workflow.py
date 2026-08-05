"""Workflow definition nodes for the Python WDL object model."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlNode


class WdlWorkflowElement(WdlNode):
    """Marker for any node that can appear directly inside a workflow body."""

    pass


@dataclass
class WdlWorkflow:
    """Workflow definition node.

    Workflows compose calls, declarations, scatters, and conditionals into a larger pipeline.
    """

    name: str | None = None
    _elements: deque[WdlWorkflowElement] = field(default_factory=deque)

    def getElements(self) -> deque[WdlWorkflowElement]:
        """Return the ordered workflow body elements exactly as they appeared in the source."""
        return self._elements

    def elements(self) -> deque[WdlWorkflowElement]:
        """Alias for ``getElements`` for Python-facing callers."""
        return self._elements
