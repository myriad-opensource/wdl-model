"""Base type nodes for the Python WDL object model."""

from __future__ import annotations

from enum import Enum

from wdl_model.model.base import WdlNode


class ComponentType(Enum):
    """High-level type families used for traversal and validation dispatch."""

    PRIMITIVE = "PRIMITIVE"
    ARRAY = "ARRAY"
    MAP = "MAP"
    PAIR = "PAIR"
    TYPE_REFERENCE = "TYPE_REFERENCE"


class WdlType(WdlNode):
    """Base class for WDL types.

    The WDL specification defines primitive types, arrays, maps, pairs, custom type references,
    and optionals. Concrete subclasses represent those categories while this base class carries the
    shared optional marker.
    """

    def __init__(self, optional: bool = False):
        self.optional = optional

    def isOptional(self) -> bool:
        """Return whether this type is optional, as in ``T?``."""
        return self.optional

    def setOptional(self, optional: bool) -> None:
        """Set whether this type is optional, as in ``T?``."""
        self.optional = optional

    def componentType(self) -> ComponentType:
        """Return the broad type family for the current node."""
        raise NotImplementedError
