"""Generic key-value model nodes used across the Python WDL object model."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, Optional, TypeVar

from .wdl_node import WdlNode

K = TypeVar("K")
V = TypeVar("V")


@dataclass
class WdlKeyValue(Generic[K, V], WdlNode):
    """Generic key-value node.

    Examples include metadata entries, runtime and requirements entries, call input bindings, and
    map/object-like literal entries.
    """

    key: Optional[K] = None
    value: Optional[V] = None

    def getKey(self) -> Optional[K]:
        """Return the key portion of the pair."""
        return self.key

    def setKey(self, key: Optional[K]) -> None:
        """Set the key portion of the pair."""
        self.key = key

    def getValue(self) -> Optional[V]:
        """Return the value portion of the pair."""
        return self.value

    def setValue(self, value: Optional[V]) -> None:
        """Set the value portion of the pair."""
        self.value = value


class WdlStringKeyValue(WdlKeyValue[str, "WdlExpression"]):
    """Key-value node whose key is a string and whose value is an expression."""

    pass


class WdlExpresionKeyValue(WdlKeyValue["WdlExpression", "WdlExpression"]):
    """Key-value node whose key and value are both expressions."""

    pass
