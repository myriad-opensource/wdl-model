"""Aggregate exception carrying one or more WDL diagnostics."""

from __future__ import annotations

from typing import Iterable, List

from .wdl_error import WdlError


class WdlException(Exception):
    """Exception raised when parsing or validation collected diagnostics."""

    def __init__(self, errors: Iterable[WdlError]):
        self.errors: List[WdlError] = list(errors)
        super().__init__(self.toDebugMessage())

    def getErrors(self) -> List[WdlError]:
        """Return the collected diagnostics carried by the exception."""
        return self.errors

    def toDebugMessage(self) -> str:
        """Return a compact debug-oriented rendering of all collected diagnostics."""
        return "\n".join(error.toDebugMessage() for error in self.errors)
