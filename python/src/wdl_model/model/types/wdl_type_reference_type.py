from __future__ import annotations

from .wdl_type import ComponentType, WdlType


class WdlTypeReferenceType(WdlType):
    def __init__(self, referencedTypeName: str | None = None, optional: bool = False):
        super().__init__(optional)
        self.referencedTypeName = referencedTypeName

    def componentType(self) -> ComponentType:
        return ComponentType.TYPE_REFERENCE

    def referenceName(self) -> str | None:
        return self.referencedTypeName

    def setReferenceName(self, referencedTypeName: str) -> None:
        self.referencedTypeName = referencedTypeName
