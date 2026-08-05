"""Base diagnostic type for parsing and validation failures."""

from dataclasses import dataclass


@dataclass
class WdlError:
    """Base diagnostic carrying a message and source location."""

    message: str
    line: int
    charPositionInLine: int

    def toDebugMessage(self) -> str:
        """Return a compact debug-oriented rendering of the diagnostic."""
        return f"{self.message} ({self.line}:{self.charPositionInLine})"
