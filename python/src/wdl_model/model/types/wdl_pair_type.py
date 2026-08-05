from __future__ import annotations

from .wdl_type import ComponentType, WdlType


class WdlPairType(WdlType):
    def __init__(
        self,
        pairLeftType: WdlType | None = None,
        pairRightType: WdlType | None = None,
        optional: bool = False,
    ):
        super().__init__(optional)
        self.pairLeftType = pairLeftType
        self.pairRightType = pairRightType

    def componentType(self) -> ComponentType:
        return ComponentType.PAIR

    def leftType(self) -> WdlType | None:
        return self.pairLeftType

    def setLeftType(self, pairLeftType: WdlType) -> None:
        self.pairLeftType = pairLeftType

    def rightType(self) -> WdlType | None:
        return self.pairRightType

    def setRightType(self, pairRightType: WdlType) -> None:
        self.pairRightType = pairRightType
