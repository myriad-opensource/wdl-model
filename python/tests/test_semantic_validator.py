from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlSemanticValidator, WdlV1Loader
from wdl_model.model.errors import WdlException

REPO_ROOT = Path(__file__).resolve().parents[2]
SPEC_EXAMPLES_DIR = REPO_ROOT / "wdl-grammar" / "spec_examples"
FIXTURES_ROOT = Path("wdl_tests") / "validator"

PARSE_OK_FAILS_V13 = [
    "empty_array_fail.wdl",
    "illegal_access_fail.wdl",
    "non_empty_optional_fail.wdl",
    "private_declaration_fail.wdl",
    "select_first_empty_fail.wdl",
    "select_first_only_none_fail.wdl",
    "test_as_map_fail.wdl",
    "test_map_fail.wdl",
    "test_zip_fail.wdl",
    "write_json_fail.wdl",
]


@pytest.mark.parametrize("name", PARSE_OK_FAILS_V13)
def test_semantic_validator_rejects_parse_ok_fail_examples(name: str) -> None:
    source = (SPEC_EXAMPLES_DIR / "v1_3" / name).read_text(encoding="utf-8")
    document = WdlV1Loader.load_from_string(source)

    validator = WdlSemanticValidator()
    with pytest.raises(WdlException):
        validator.validateDocument(document)


def test_semantic_validator_accepts_simple_valid_workflow() -> None:
    document = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "accepts_simple_valid_workflow.wdl"
    )
    validator = WdlSemanticValidator()
    validator.validateDocument(document)


def test_loader_runs_validator_and_raises_on_invalid_document() -> None:
    source = (SPEC_EXAMPLES_DIR / "v1_3" / "select_first_empty_fail.wdl").read_text(
        encoding="utf-8"
    )
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_string(source, validator=WdlSemanticValidator())


def test_loader_runs_validator_and_returns_valid_document() -> None:
    document = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "loader_valid_document.wdl",
        validator=WdlSemanticValidator(),
    )
    assert document is not None
