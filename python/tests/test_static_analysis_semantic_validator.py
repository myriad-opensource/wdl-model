from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model.errors import WdlSemanticError, WdlSemanticSeverity
from wdl_model.model import (
    WdlLintingSemanticValidator,
    WdlStaticAnalysisSemanticValidator,
    WdlSemanticValidator,
    WdlV1Loader,
)
from wdl_model.model.errors import WdlException

FIXTURES_ROOT = Path("wdl_tests") / "validator"


def test_full_validator_catches_additional_static_function_signature_errors() -> None:
    document = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "static_function_signature_bad.wdl"
    )

    base_validator = WdlSemanticValidator()
    full_validator = WdlStaticAnalysisSemanticValidator()

    base_validator.validateDocument(document)
    with pytest.raises(WdlException):
        full_validator.validateDocument(document)


def test_full_validator_catches_additional_static_workflow_structure_errors() -> None:
    document = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "static_workflow_structure_bad.wdl"
    )

    base_validator = WdlSemanticValidator()
    full_validator = WdlStaticAnalysisSemanticValidator()

    base_validator.validateDocument(document)
    with pytest.raises(WdlException):
        full_validator.validateDocument(document)


def test_normal_validator_rejects_function_not_available_in_document_version() -> None:
    document = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "function_version_invalid.wdl"
    )
    validator = WdlSemanticValidator()
    with pytest.raises(WdlException):
        validator.validateDocument(document)


def test_static_analysis_validator_catches_nested_workflow_structure_errors() -> None:
    document = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "nested_workflow_structure_bad.wdl"
    )
    base_validator = WdlSemanticValidator()
    static_validator = WdlStaticAnalysisSemanticValidator()

    base_validator.validateDocument(document)
    with pytest.raises(WdlException):
        static_validator.validateDocument(document)


def test_linting_validator_catches_unused_symbols() -> None:
    document = WdlV1Loader.load_from_file(FIXTURES_ROOT / "lint_unused_symbols_bad.wdl")
    static_validator = WdlStaticAnalysisSemanticValidator()
    linting_validator = WdlLintingSemanticValidator()

    static_validator.validateDocument(document)
    with pytest.raises(WdlException) as ex_info:
        linting_validator.validateDocument(document)

    semantic_errors = [
        err for err in ex_info.value.getErrors() if isinstance(err, WdlSemanticError)
    ]
    assert semantic_errors
    assert semantic_errors[0].severity == WdlSemanticSeverity.WARNING


def test_linting_validator_can_skip_throw_on_warnings() -> None:
    document = WdlV1Loader.load_from_file(FIXTURES_ROOT / "lint_unused_symbols_bad.wdl")
    linting_validator = WdlLintingSemanticValidator()
    linting_validator.setThrowOnWarnings(False)

    linting_validator.validateDocument(document)
