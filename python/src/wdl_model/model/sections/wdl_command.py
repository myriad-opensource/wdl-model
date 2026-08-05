"""Command section node for Python WDL tasks.

The WDL specification describes the command section as the template evaluated after inputs are
available and before outputs are evaluated. The stored string literal preserves placeholders used
for interpolation.
"""

from __future__ import annotations

from dataclasses import dataclass

from wdl_model.model.definitions import WdlTaskElement
from wdl_model.model.expressions import WdlStringLiteral


@dataclass
class WdlCommand(WdlTaskElement):
    """Task command template and its multiline/single-line representation."""

    commandText: WdlStringLiteral | None = None
    multiline: bool = False

    def getCommandText(self) -> WdlStringLiteral | None:
        """Return the command template literal."""
        return self.commandText

    def setCommandText(self, commandText: WdlStringLiteral) -> None:
        """Set the command template literal."""
        self.commandText = commandText

    def isMultiline(self) -> bool:
        """Return whether the source used the multiline command form."""
        return self.multiline

    def setMultiline(self, multiline: bool) -> None:
        """Set whether the source used the multiline command form."""
        self.multiline = multiline
