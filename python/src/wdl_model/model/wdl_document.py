"""Root document node for parsed WDL source.

The WDL specification says that a document contains a version declaration followed by imports,
user-defined types, tasks, and workflows. This module mirrors that top-level structure and offers
typed views over the ordered element list.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from typing import Dict

from wdl_model.model.base import WdlNode
from wdl_model.model.definitions import WdlEnum, WdlStruct, WdlTask, WdlWorkflow
from wdl_model.model.statements import WdlImport

from .wdl_version import WdlVersion


class WdlDocumentElement(WdlNode):
    """Marker for any top-level node that can appear directly in a WDL document."""

    pass


@dataclass
class WdlDocument(WdlNode):
    """Root node for a parsed WDL document."""

    wdlVersion: WdlVersion | None = None
    sourceLocation: str | None = None
    _elements: deque[WdlDocumentElement] = field(default_factory=deque)
    _importedDocuments: Dict[str, "WdlDocument"] = field(default_factory=dict)

    def getWdlVersion(self) -> WdlVersion | None:
        """Return the declared WDL version, if one was parsed."""
        return self.wdlVersion

    def setWdlVersion(self, wdlVersion: WdlVersion) -> None:
        """Set the declared WDL version."""
        self.wdlVersion = wdlVersion

    def getSourceLocation(self) -> str | None:
        """Return the source location identifier used for import resolution."""
        return self.sourceLocation

    def setSourceLocation(self, sourceLocation: str | None) -> None:
        """Set the source location identifier used for import resolution."""
        self.sourceLocation = sourceLocation

    def elements(self) -> deque[WdlDocumentElement]:
        """Return the ordered top-level elements exactly as they appeared in the source."""
        return self._elements

    def importStatements(self) -> list[WdlImport]:
        """Return only top-level import statements."""
        return [x for x in self._elements if isinstance(x, WdlImport)]

    def importedDocuments(self) -> Dict[str, "WdlDocument"]:
        """Return imported documents keyed by import identifier."""
        return self._importedDocuments

    def enums(self) -> list[WdlEnum]:
        """Return only top-level enum definitions."""
        return [x for x in self._elements if isinstance(x, WdlEnum)]

    def structs(self) -> list[WdlStruct]:
        """Return only top-level struct definitions."""
        return [x for x in self._elements if isinstance(x, WdlStruct)]

    def tasks(self) -> list[WdlTask]:
        """Return only top-level task definitions."""
        return [x for x in self._elements if isinstance(x, WdlTask)]

    def workflows(self) -> list[WdlWorkflow]:
        """Return only top-level workflow definitions."""
        return [x for x in self._elements if isinstance(x, WdlWorkflow)]
