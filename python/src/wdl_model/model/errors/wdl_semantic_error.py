"""Semantic diagnostics emitted by the Python WDL validators.

The same diagnostic model is used by baseline semantic validation, stricter static analysis, and
linting. Representative failures are exercised by fixtures such as
`spec_examples/v1_3/private_declaration_fail.wdl`,
`spec_examples/v1_3/select_first_empty_fail.wdl`, and
`spec_examples/v1_3/write_json_fail.wdl`.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

from .wdl_error import WdlError


class WdlSemanticSeverity(Enum):
    """Diagnostic severity used by the validator throw policy."""

    WARNING = "WARNING"
    ERROR = "ERROR"


class WdlSemanticErrorCode(Enum):
    """Stable semantic and lint diagnostic codes emitted by this library."""

    GENERIC_SEMANTIC_ERROR = ("GENERIC_SEMANTIC_ERROR", WdlSemanticSeverity.ERROR)
    FUNCTION_NOT_AVAILABLE_IN_VERSION = (
        "FUNCTION_NOT_AVAILABLE_IN_VERSION",
        WdlSemanticSeverity.ERROR,
    )
    DUPLICATE_DEFINITION = ("DUPLICATE_DEFINITION", WdlSemanticSeverity.ERROR)
    UNKNOWN_REFERENCE = ("UNKNOWN_REFERENCE", WdlSemanticSeverity.ERROR)
    TYPE_MISMATCH = ("TYPE_MISMATCH", WdlSemanticSeverity.ERROR)
    INVALID_FUNCTION_ARGUMENTS = (
        "INVALID_FUNCTION_ARGUMENTS",
        WdlSemanticSeverity.ERROR,
    )
    LINT_UNUSED_WORKFLOW_DECLARATION = (
        "LINT_UNUSED_WORKFLOW_DECLARATION",
        WdlSemanticSeverity.WARNING,
    )
    LINT_UNUSED_TASK_DECLARATION = (
        "LINT_UNUSED_TASK_DECLARATION",
        WdlSemanticSeverity.WARNING,
    )
    LINT_UNUSED_SCATTER_VARIABLE = (
        "LINT_UNUSED_SCATTER_VARIABLE",
        WdlSemanticSeverity.WARNING,
    )
    LINT_UNUSED_CALL_OUTPUT = ("LINT_UNUSED_CALL_OUTPUT", WdlSemanticSeverity.WARNING)
    LINT_DEPRECATED_FEATURE = ("LINT_DEPRECATED_FEATURE", WdlSemanticSeverity.WARNING)

    def __init__(self, code: str, severity: WdlSemanticSeverity):
        self.code = code
        self.severity = severity


@dataclass
class WdlSemanticError(WdlError):
    """Semantic diagnostic with a stable code and derived severity."""

    code: WdlSemanticErrorCode = WdlSemanticErrorCode.GENERIC_SEMANTIC_ERROR

    @property
    def severity(self) -> WdlSemanticSeverity:
        """Return the severity implied by the diagnostic code."""
        return self.code.severity

    def toDebugMessage(self) -> str:
        """Render the base debug message enriched with code and severity."""
        base = super().toDebugMessage()
        return f"{base} [{self.code.code}:{self.severity.value}]"
