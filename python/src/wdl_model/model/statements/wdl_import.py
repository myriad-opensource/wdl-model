"""Import statement nodes for the Python WDL object model.

WDL supports standard imports, star imports, and member-list imports. See the specification section
"Import Statements".
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.base import WdlSourceRange
from wdl_model.model.expressions import WdlStringLiteral


@dataclass
class WdlImport:
    """Base import statement carrying the imported source literal."""

    source: WdlStringLiteral | None = None
    importIdentifier: str | None = None
    sourceText: str | None = None
    source_range: WdlSourceRange | None = None


@dataclass
class WdlImportMember:
    """Single imported member with an optional local alias."""

    member: str | None = None
    alias: str | None = None


@dataclass
class WdlImportStandard(WdlImport):
    """Standard import form: import source [as alias] with optional type alias clauses."""

    alias: str | None = None
    _members: deque[WdlImportMember] = field(default_factory=deque)

    def members(self) -> deque[WdlImportMember]:
        """Return the ordered imported type alias clauses attached to the import."""
        return self._members


@dataclass
class WdlImportMembers(WdlImport):
    """Member-list import form: import { a, b as c } from source."""

    _members: deque[WdlImportMember] = field(default_factory=deque)

    def members(self) -> deque[WdlImportMember]:
        """Return the ordered imported members selected by the import."""
        return self._members


@dataclass
class WdlImportStar(WdlImport):
    """Star import form: import * from source."""

    pass
