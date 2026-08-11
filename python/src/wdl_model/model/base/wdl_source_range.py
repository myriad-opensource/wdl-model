"""Source text span attached to AST nodes by the loader from ANTLR token positions."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class WdlSourceRange:
    """Immutable source range for a parsed WDL node.

    All line values are 1-based; all column values are 0-based, matching ANTLR convention.
    ``end_column`` is the exclusive end position of the last token.
    """

    start_line: int
    start_column: int
    end_line: int
    end_column: int

    def __str__(self) -> str:
        return f"{self.start_line}:{self.start_column}-{self.end_line}:{self.end_column}"
