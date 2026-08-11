from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import (
    WdlSemanticValidator,
    WdlStaticAnalysisSemanticValidator,
    WdlV1Loader,
)
from wdl_model.model.errors import WdlException

FIXTURES_ROOT = Path("wdl_tests") / "static_function_signature_matrix"


@pytest.mark.parametrize(
    "fixture",
    [
        "keys_bad.wdl",
        "range_bad.wdl",
        "contains_bad.wdl",
        "chunk_bad.wdl",
        "cross_bad.wdl",
        "join_paths_bad_first.wdl",
        "join_paths_bad_tail.wdl",
        "basename_bad_first.wdl",
        "size_bad_second.wdl",
    ],
)
def test_rejects_invalid_signatures_under_static_validator(fixture: str) -> None:
    doc = WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture)
    WdlSemanticValidator().validateDocument(doc)
    with pytest.raises(WdlException):
        WdlStaticAnalysisSemanticValidator().validateDocument(doc)


def test_accepts_valid_signatures_under_static_validator() -> None:
    doc = WdlV1Loader.load_from_file(FIXTURES_ROOT / "static_signatures_ok.wdl")
    WdlSemanticValidator().validateDocument(doc)
    WdlStaticAnalysisSemanticValidator().validateDocument(doc)
