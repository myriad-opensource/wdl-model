from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlSemanticValidator, WdlV1Loader
from wdl_model.model.errors import WdlException

WDL_TESTS_ROOT = Path("wdl_tests")
SPEC_EXAMPLES_ROOT = Path("../wdl-grammar/spec_examples")

POSITIVE_IMPORT_EXAMPLES = ["call_example.wdl", "call_imported.wdl"]
NEGATIVE_IMPORT_EXAMPLES = [
    "call_subworkflow_fail.wdl",
    "incomplete_struct_fail.wdl",
    "illegal_access_fail.wdl",
]


def _spec_example_paths(names: list[str]) -> list[Path]:
    paths: list[Path] = []
    for version in ["v1_1", "v1_2", "v1_3"]:
        for name in names:
            path = SPEC_EXAMPLES_ROOT / version / name
            if path.exists():
                paths.append(path)
    return paths


@pytest.mark.parametrize("file_path", _spec_example_paths(POSITIVE_IMPORT_EXAMPLES))
def test_validates_positive_import_spec_examples(file_path: Path) -> None:
    WdlV1Loader.load_from_file(file_path, WdlSemanticValidator())


@pytest.mark.parametrize("file_path", _spec_example_paths(NEGATIVE_IMPORT_EXAMPLES))
def test_rejects_negative_import_spec_examples(file_path: Path) -> None:
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(file_path, WdlSemanticValidator())


def test_validates_star_and_members_import_forms() -> None:
    root = WDL_TESTS_ROOT / "import_validation" / "star_members" / "root.wdl"
    WdlV1Loader.load_from_file(root, WdlSemanticValidator())


def test_validates_standard_import_struct_aliases() -> None:
    root = WDL_TESTS_ROOT / "import_validation" / "standard_alias" / "root.wdl"
    WdlV1Loader.load_from_file(root, WdlSemanticValidator())


def test_rejects_unknown_member_import() -> None:
    root = WDL_TESTS_ROOT / "import_validation" / "unknown_member" / "root.wdl"
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(root, WdlSemanticValidator())


def test_rejects_duplicate_import_namespaces() -> None:
    root = WDL_TESTS_ROOT / "import_validation" / "duplicate_namespace" / "root.wdl"
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(root, WdlSemanticValidator())


def test_rejects_import_alias_target_that_does_not_exist() -> None:
    root = WDL_TESTS_ROOT / "import_validation" / "bad_alias" / "root.wdl"
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(root, WdlSemanticValidator())


def test_rejects_incompatible_imported_structs_without_alias() -> None:
    root = WDL_TESTS_ROOT / "import_validation" / "struct_conflict" / "root.wdl"
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(root, WdlSemanticValidator())


def test_rejects_import_from_higher_minor_version() -> None:
    root = WDL_TESTS_ROOT / "import_validation" / "version_mismatch" / "root.wdl"
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(root, WdlSemanticValidator())
