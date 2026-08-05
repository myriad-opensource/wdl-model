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
