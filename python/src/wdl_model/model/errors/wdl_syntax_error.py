"""Syntax diagnostic produced while lexing or parsing WDL source."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Optional

from .wdl_error import WdlError


@dataclass
class WdlSyntaxError(WdlError):
    """Syntax diagnostic that preserves the underlying parser exception when available."""

    cause: Optional[Any] = None

    def getCause(self) -> Optional[Any]:
        """Return the underlying parser exception, if any."""
        return self.cause
