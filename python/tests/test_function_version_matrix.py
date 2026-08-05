from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlSemanticValidator, WdlV1Loader
from wdl_model.model.errors import WdlException

FIXTURES_ROOT = Path("wdl_tests") / "function_version_matrix"


@pytest.mark.parametrize(
    "fixture",
    [
        "v11_keys_ok.wdl",
        "v12_contains_ok.wdl",
        "v13_value_ok.wdl",
    ],
)
def test_accepts_version_compatible_functions(fixture: str) -> None:
    WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture, WdlSemanticValidator())


@pytest.mark.parametrize(
    "fixture",
    [
        "v11_contains_key_fail.wdl",
        "v11_join_paths_fail.wdl",
        "v12_value_fail.wdl",
    ],
)
def test_rejects_version_incompatible_functions(fixture: str) -> None:
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture, WdlSemanticValidator())
