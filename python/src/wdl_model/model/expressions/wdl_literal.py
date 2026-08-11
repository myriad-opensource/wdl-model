"""Generic literal expression nodes."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, Optional, TypeVar

from .wdl_value_expression import WdlValueExpression

T = TypeVar("T")


@dataclass
class WdlLiteral(WdlValueExpression[T], Generic[T]):
    """Literal expression carrying a concrete scalar value."""

    value: Optional[T] = None

    def getValue(self) -> Optional[T]:
        """Return the literal value."""
        return self.value

    def setValue(self, value: Optional[T]) -> None:
        """Set the literal value."""
        self.value = value

    def __str__(self) -> str:
        return str(self.value)
