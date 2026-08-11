"""Hints section nodes for tasks and workflows."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from typing import Generic, TypeVar

from wdl_model.model.base import WdlStringKeyValue
from wdl_model.model.definitions import WdlTaskElement, WdlWorkflowElement
from wdl_model.model.expressions import WdlExpression


class WdlHint(WdlStringKeyValue):
    """Base class for a single keyed hint entry."""

    def __init__(self, key: str | None = None, value: WdlExpression | None = None):
        super().__init__(key, value)


class WdlTaskHint(WdlHint):
    """Single task-scoped hint entry."""

    pass


class WdlWorkflowHint(WdlHint):
    """Single workflow-scoped hint entry."""

    pass


V = TypeVar("V", bound=WdlHint)


@dataclass
class WdlHints(Generic[V]):
    """Base container for ordered hint entries."""

    _elements: deque[V] = field(default_factory=deque)

    def elements(self) -> deque[V]:
        """Return the ordered hints contained in the section."""
        return self._elements


class WdlTaskHints(WdlHints[WdlTaskHint], WdlTaskElement):
    """Task hints section node."""

    pass


class WdlWorkflowHints(WdlHints[WdlWorkflowHint], WdlWorkflowElement):
    """Workflow hints section node."""

    pass
