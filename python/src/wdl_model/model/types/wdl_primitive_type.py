"""Primitive WDL types for the Python object model."""

from __future__ import annotations

from enum import Enum

from .wdl_type import ComponentType, WdlType


class Type(Enum):
    """Supported primitive type names from the WDL specification."""

    BOOLEAN = "Boolean"
    INT = "Int"
    FLOAT = "Float"
    STRING = "String"
    FILE = "File"
    DIRECTORY = "Directory"
    OBJECT = "Object"

    def toWdlString(self) -> str:
        """Return the source-level WDL type spelling."""
        return self.value

    def getWdlName(self) -> str:
        # Backward-compatible alias.
        return self.value


class WdlPrimitiveType(WdlType):
    """Primitive WDL type with optional marker."""

    def __init__(self, primitiveValueType: Type | None = None, optional: bool = False):
        super().__init__(optional)
        self.primitiveValueType = primitiveValueType

    def componentType(self) -> ComponentType:
        """Return the broad type family for the current node."""
        return ComponentType.PRIMITIVE

    def primitiveType(self) -> Type | None:
        """Return the specific primitive type name represented by this node."""
        return self.primitiveValueType

    def setPrimitiveType(self, primitiveValueType: Type) -> None:
        """Set the specific primitive type name represented by this node."""
        self.primitiveValueType = primitiveValueType
