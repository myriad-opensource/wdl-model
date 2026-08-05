"""Array type nodes for the Python WDL object model."""

from __future__ import annotations

from .wdl_type import ComponentType, WdlType


class WdlArrayType(WdlType):
    """Array type node, including the WDL non-empty marker when present."""

    def __init__(
        self,
        arrayMemberType: WdlType | None = None,
        nonEmpty: bool = False,
        optional: bool = False,
    ):
        super().__init__(optional)
        self.arrayMemberType = arrayMemberType
        self.nonEmpty = nonEmpty

    def componentType(self) -> ComponentType:
        """Return the broad type family for the current node."""
        return ComponentType.ARRAY

    def memberType(self) -> WdlType | None:
        """Return the array member type."""
        return self.arrayMemberType

    def setMemberType(self, arrayMemberType: WdlType) -> None:
        """Set the array member type."""
        self.arrayMemberType = arrayMemberType

    def isNonEmpty(self) -> bool:
        """Return whether the type carries the WDL non-empty array marker (``+``)."""
        return self.nonEmpty

    def setNonEmpty(self, nonEmpty: bool) -> None:
        """Set whether the type carries the WDL non-empty array marker (``+``)."""
        self.nonEmpty = nonEmpty
