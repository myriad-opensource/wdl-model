"""Conditional statement nodes for the Python WDL object model."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlNode, WdlSourceRange
from wdl_model.model.definitions import WdlWorkflowElement
from wdl_model.model.expressions import WdlExpression

from .wdl_statement import ComponentType, WdlStatement


@dataclass
class WdlConditionalElseIf(WdlNode):
    """Single ``else if`` branch inside a conditional statement."""

    condition: WdlExpression | None = None
    _thenStatements: deque[WdlStatement] = field(default_factory=deque)

    def thenStatements(self) -> deque[WdlStatement]:
        """Return the ordered statements in this ``else if`` branch."""
        return self._thenStatements


@dataclass
class WdlConditional(WdlStatement, WdlWorkflowElement):
    """Conditional statement node with ``if``, ``else if``, and ``else`` branches."""

    condition: WdlExpression | None = None
    source_range: WdlSourceRange | None = None
    _thenStatements: deque[WdlStatement] = field(default_factory=deque)
    _elseIfs: deque[WdlConditionalElseIf] = field(default_factory=deque)
    _elseStatements: deque[WdlStatement] = field(default_factory=deque)

    def thenStatements(self) -> deque[WdlStatement]:
        """Return the ordered statements in the ``then`` branch."""
        return self._thenStatements

    def elseIfs(self) -> deque[WdlConditionalElseIf]:
        """Return the ordered ``else if`` branches."""
        return self._elseIfs

    def elseStatements(self) -> deque[WdlStatement]:
        """Return the ordered statements in the ``else`` branch."""
        return self._elseStatements

    def componentType(self) -> ComponentType:
        """Return the statement category for traversal and validation dispatch."""
        return ComponentType.CONDITIONAL
