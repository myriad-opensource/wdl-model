"""Struct definition nodes for the Python WDL object model."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlNode
from wdl_model.model.types import WdlType


class WdlStructElement(WdlNode):
    """Marker for any node that can appear directly inside a struct definition."""

    pass


@dataclass
class WdlStructMember(WdlStructElement):
    """Named, typed member of a WDL struct."""

    type: WdlType | None = None
    name: str | None = None


@dataclass
class WdlStruct:
    """Struct definition node.

    Structs are user-defined composite types whose members are named and typed.
    """

    name: str | None = None
    _elements: deque[WdlStructElement] = field(default_factory=deque)

    def elements(self) -> deque[WdlStructElement]:
        """Return the ordered struct elements, usually members and metadata sections."""
        return self._elements

    def hasMember(self, member_name: str | None) -> bool:
        """Return whether a member with the supplied name exists."""
        return self.member(member_name) is not None

    def member(self, member_name: str | None) -> WdlStructMember | None:
        """Return the declared member by name, if present."""
        if member_name is None or not member_name.strip():
            return None
        for element in self._elements:
            if isinstance(element, WdlStructMember) and element.name == member_name:
                return element
        return None

    def memberType(self, member_name: str | None) -> WdlType | None:
        """Return the declared member type by name, if present."""
        member = self.member(member_name)
        return member.type if member is not None else None
