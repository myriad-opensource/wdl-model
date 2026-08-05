"""Scatter statement nodes for the Python WDL object model."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.definitions import WdlWorkflowElement
from wdl_model.model.expressions import WdlExpression

from .wdl_statement import ComponentType, WdlStatement


@dataclass
class WdlScatter(WdlStatement, WdlWorkflowElement):
    """Scatter statement.

    A scatter iterates a bound name over a collection expression and evaluates a nested block of
    workflow statements.
    """

    name: str | None = None
    collection: WdlExpression | None = None
    _statements: deque[WdlStatement] = field(default_factory=deque)

    def statements(self) -> deque[WdlStatement]:
        """Return the ordered nested statements inside the scatter body."""
        return self._statements

    def componentType(self) -> ComponentType:
        """Return the statement category for traversal and validation dispatch."""
        return ComponentType.SCATTER
