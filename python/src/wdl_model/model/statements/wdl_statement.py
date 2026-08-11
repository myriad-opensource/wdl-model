"""Base statement nodes for the Python WDL object model."""

from __future__ import annotations

from enum import Enum

from wdl_model.model.base import WdlNode


class ComponentType(Enum):
    """High-level statement families used for traversal and validation dispatch."""

    DECLARATION = "DECLARATION"
    CALL = "CALL"
    CONDITIONAL = "CONDITIONAL"
    SCATTER = "SCATTER"


class WdlStatement(WdlNode):
    """Base interface for workflow and task statements."""

    def componentType(self) -> ComponentType:
        """Return the broad statement family for the current node."""
        raise NotImplementedError
