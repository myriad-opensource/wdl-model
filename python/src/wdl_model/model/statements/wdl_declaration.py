"""Declaration statements for the Python WDL object model."""

from __future__ import annotations

from dataclasses import dataclass

from wdl_model.model.expressions import WdlExpression
from wdl_model.model.types import WdlType

from .wdl_statement import ComponentType, WdlStatement


@dataclass
class WdlDeclaration(WdlStatement):
    """Base declaration statement.

    Declarations introduce a typed name and may appear in tasks, workflows, inputs, outputs, and
    other scoped regions.
    """

    type: WdlType | None = None
    name: str | None = None
    environmentVariable: bool = False

    def componentType(self) -> ComponentType:
        """Return the statement category for traversal and validation dispatch."""
        return ComponentType.DECLARATION


@dataclass
class WdlBoundDeclaration(WdlDeclaration):
    """Declaration with an initializing expression."""

    expression: WdlExpression | None = None
