"""Output section node for tasks and workflows."""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field

from wdl_model.model.definitions import WdlTaskElement, WdlWorkflowElement
from wdl_model.model.statements import WdlBoundDeclaration


@dataclass
class WdlOutput(WdlTaskElement, WdlWorkflowElement):
    """Explicit ``output { ... }`` section whose declarations are expression-bound."""

    _elements: deque[WdlBoundDeclaration] = field(default_factory=deque)

    def elements(self) -> deque[WdlBoundDeclaration]:
        """Return the ordered bound declarations contained in the output section."""
        return self._elements
