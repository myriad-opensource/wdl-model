"""Input section node for tasks and workflows."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.definitions import WdlTaskElement, WdlWorkflowElement
from wdl_model.model.statements import WdlDeclaration


@dataclass
class WdlInput(WdlTaskElement, WdlWorkflowElement):
    """Explicit ``input { ... }`` section."""

    _elements: deque[WdlDeclaration] = field(default_factory=deque)

    def elements(self) -> deque[WdlDeclaration]:
        """Return the ordered declarations contained in the input section."""
        return self._elements
