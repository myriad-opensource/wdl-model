from __future__ import annotations

from dataclasses import dataclass

from .wdl_expression import ComponentType, WdlExpression


@dataclass
class WdlMemberAccessOperation(WdlExpression):
    target: WdlExpression | None = None
    member: str | None = None

    def __str__(self) -> str:
        return f"{self.target or ''}.{self.member or ''}"

    def componentType(self) -> ComponentType:
        return ComponentType.MEMBER_ACCESS
