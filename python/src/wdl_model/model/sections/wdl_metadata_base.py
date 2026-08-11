"""Metadata section nodes for tasks, workflows, and structs."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlNode, WdlStringKeyValue
from wdl_model.model.expressions import WdlExpression


class WdlMetadataEntry(WdlStringKeyValue):
    """Single metadata entry such as ``description: \"text\"``."""

    def __init__(self, key: str | None = None, value: WdlExpression | None = None):
        super().__init__(key, value)


@dataclass
class WdlMetadataBase(WdlNode):
    """Base container for ordered metadata entries."""

    _elements: deque[WdlMetadataEntry] = field(default_factory=deque)

    def elements(self) -> deque[WdlMetadataEntry]:
        """Return the ordered metadata entries contained in the section."""
        return self._elements


class WdlMetadata(WdlMetadataBase):
    """General metadata section."""

    pass


class WdlParameterMetadata(WdlMetadataBase):
    """Parameter metadata section."""

    pass
