"""Exception raised when an import URI cannot be resolved or loaded."""

from __future__ import annotations

from .wdl_exception import WdlException
from .wdl_import_error import WdlImportError


class WdlImportException(WdlException):
    """Exception raised when an import URI cannot be resolved or loaded."""

    def __init__(
        self, message: str, import_location: str, cause: Exception | None = None
    ):
        super().__init__([WdlImportError(message, import_location)])
        if cause is not None:
            self.__cause__ = cause
