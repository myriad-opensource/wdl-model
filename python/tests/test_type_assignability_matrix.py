from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlSemanticValidator, WdlV1Loader
from wdl_model.model.errors import WdlException

FIXTURES_ROOT = Path("wdl_tests") / "type_assignability_matrix"


@pytest.mark.parametrize(
    "fixture",
    [
        "optional_from_none_ok.wdl",
        "array_nested_ok.wdl",
        "map_value_type_ok.wdl",
        "file_directory_from_string_ok.wdl",
        "struct_to_struct_coercion_ok.wdl",
    ],
)
def test_accepts_compatible_assignments(fixture: str) -> None:
    WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture, WdlSemanticValidator())


@pytest.mark.parametrize(
    "fixture",
    [
        "required_from_none_fail.wdl",
        "array_member_type_fail.wdl",
        "required_string_to_int_fail.wdl",
        "array_string_to_int_fail.wdl",
        "map_value_type_fail.wdl",
        "struct_to_struct_incompatible_fail.wdl",
    ],
)
def test_rejects_incompatible_assignments(fixture: str) -> None:
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture, WdlSemanticValidator())
