from __future__ import annotations

from .wdl_type import ComponentType, WdlType


class WdlMapType(WdlType):
    def __init__(
        self,
        mapKeyType: WdlType | None = None,
        mapValueType: WdlType | None = None,
        optional: bool = False,
    ):
        super().__init__(optional)
        self.mapKeyType = mapKeyType
        self.mapValueType = mapValueType

    def componentType(self) -> ComponentType:
        return ComponentType.MAP

    def keyType(self) -> WdlType | None:
        return self.mapKeyType

    def setKeyType(self, mapKeyType: WdlType) -> None:
        self.mapKeyType = mapKeyType

    def valueType(self) -> WdlType | None:
        return self.mapValueType

    def setValueType(self, mapValueType: WdlType) -> None:
        self.mapValueType = mapValueType
