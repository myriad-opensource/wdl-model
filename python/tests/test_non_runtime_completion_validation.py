from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import (
    WdlSemanticValidator,
    WdlStaticAnalysisSemanticValidator,
    WdlV1Loader,
)
from wdl_model.model.errors import WdlException

ROOT = Path("wdl_tests") / "non_runtime_completion"


@pytest.mark.parametrize(
    "fixture",
    [
        "baseline_function_args/length_bad.wdl",
        "baseline_function_args/contains_key_bad.wdl",
    ],
)
def test_rejects_invalid_baseline_function_args(fixture: str) -> None:
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(ROOT / fixture, WdlSemanticValidator())


def test_accepts_valid_baseline_function_args() -> None:
    WdlV1Loader.load_from_file(
        ROOT / "baseline_function_args/baseline_function_args_ok.wdl",
        WdlSemanticValidator(),
    )


@pytest.mark.parametrize(
    "fixture",
    [
        "member_index_checks/unknown_struct_field_fail.wdl",
        "member_index_checks/unknown_call_output_fail.wdl",
        "member_index_checks/index_out_of_bounds_fail.wdl",
    ],
)
def test_rejects_invalid_member_and_index_access(fixture: str) -> None:
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(ROOT / fixture, WdlSemanticValidator())


def test_accepts_valid_member_and_index_access() -> None:
    WdlV1Loader.load_from_file(
        ROOT / "member_index_checks/member_index_checks_ok.wdl",
        WdlSemanticValidator(),
    )


def test_validates_nested_imported_type_aliases() -> None:
    WdlV1Loader.load_from_file(
        ROOT / "import_alias_nested/root.wdl", WdlSemanticValidator()
    )


def test_validates_placeholder_interpolation_and_section_syntax() -> None:
    placeholders = WdlV1Loader.load_from_file(ROOT / "placeholder_interpolation_ok.wdl")
    sections = WdlV1Loader.load_from_file(ROOT / "requirements_hints_syntax_ok.wdl")

    WdlStaticAnalysisSemanticValidator().validateDocument(placeholders)
    WdlStaticAnalysisSemanticValidator().validateDocument(sections)


def test_rejects_invalid_json_type_level_static_usage() -> None:
    doc = WdlV1Loader.load_from_file(ROOT / "json_type_level_static_fail.wdl")

    WdlSemanticValidator().validateDocument(doc)
    with pytest.raises(WdlException):
        WdlStaticAnalysisSemanticValidator().validateDocument(doc)
