"""Import-resolution diagnostic emitted by resolver failures."""

from dataclasses import dataclass

from .wdl_error import WdlError


@dataclass
class WdlImportError(WdlError):
    """Diagnostic describing a failure while resolving or loading an imported WDL source."""

    importLocation: str = ""

    def __init__(self, message: str, importLocation: str):
        super().__init__(message=message, line=-1, charPositionInLine=-1)
        self.importLocation = importLocation

    def toDebugMessage(self) -> str:
        return f"{self.__class__.__name__}:{self.importLocation}:{self.message}"
