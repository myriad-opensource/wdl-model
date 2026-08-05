"""Workflow call statements for the Python WDL object model."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlStringKeyValue
from wdl_model.model.definitions import WdlWorkflowElement
from wdl_model.model.expressions import WdlExpression

from .wdl_statement import ComponentType, WdlStatement


class WdlCallInput(WdlStringKeyValue):
    """Single call input binding such as ``x = expr``."""

    def __init__(self, key: str | None = None, value: WdlExpression | None = None):
        super().__init__(key, value)


@dataclass
class WdlCall(WdlStatement, WdlWorkflowElement):
    """Workflow call statement.

    A call invokes a task or subworkflow and may include a dotted target path, an alias, an input
    binding block, and ``after`` dependencies.
    """

    alias: str | None = None
    legacyInputColonUsed: bool = False
    _targetPath: deque[str] = field(default_factory=deque)
    _inputs: deque[WdlCallInput] = field(default_factory=deque)
    _afterDependencies: deque[str] = field(default_factory=deque)

    def targetPath(self) -> deque[str]:
        """Return the dotted target path segments for the invoked task or workflow."""
        return self._targetPath

    def targetPathAsString(self) -> str:
        """Return the dotted target path in source form, such as ``lib.task_name``."""
        return ".".join(self._targetPath)

    def inputs(self) -> deque[WdlCallInput]:
        """Return the ordered input bindings supplied by the call input block."""
        return self._inputs

    def afterDependencies(self) -> deque[str]:
        """Return the ordered set of ``after`` dependencies declared on the call."""
        return self._afterDependencies

    def componentType(self) -> ComponentType:
        """Return the statement category for traversal and validation dispatch."""
        return ComponentType.CALL
