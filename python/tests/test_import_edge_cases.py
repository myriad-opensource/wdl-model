from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlSemanticValidator, WdlV1Loader
from wdl_model.model.errors import WdlException

FIXTURES_ROOT = Path("wdl_tests") / "import_edge_cases"


@pytest.mark.parametrize(
    "fixture_dir",
    [
        "duplicate_namespace",
        "namespace_conflicts_local",
        "member_alias_conflicts_local",
        "member_alias_duplicate",
    ],
)
def test_rejects_import_edge_case(fixture_dir: str) -> None:
    root = FIXTURES_ROOT / fixture_dir / "root.wdl"
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(root, WdlSemanticValidator())


@pytest.mark.parametrize("fixture_dir", ["mixed_forms_ok"])
def test_accepts_import_edge_case(fixture_dir: str) -> None:
    root = FIXTURES_ROOT / fixture_dir / "root.wdl"
    WdlV1Loader.load_from_file(root, WdlSemanticValidator())
