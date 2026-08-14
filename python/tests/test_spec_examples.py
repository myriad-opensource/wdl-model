from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlSemanticValidator, WdlV1Loader
from wdl_model.model.errors import WdlException

REPO_ROOT = Path(__file__).resolve().parents[2]
_SPEC_EXAMPLE_CANDIDATES = [
    REPO_ROOT / "python" / "spec_examples",
    REPO_ROOT / "wdl-grammar" / "spec_examples",
    REPO_ROOT / "spec_examples",
]
SPEC_EXAMPLES_DIR = next(
    (path for path in _SPEC_EXAMPLE_CANDIDATES if path.exists()),
    REPO_ROOT / "python" / "spec_examples",
)

FAILS_THAT_SHOULD_PARSE_OK_V1_1 = {
    "select_first_only_none_fail.wdl",
    "empty_array_fail.wdl",
    "test_as_map_fail.wdl",
    "write_json_fail.wdl",
    "test_map_fail.wdl",
    "select_first_empty_fail.wdl",
    "private_declaration_fail.wdl",
    "non_empty_optional_fail.wdl",
    "test_zip_fail.wdl",
}

FAILS_THAT_SHOULD_PARSE_OK_V1_2 = {
    "select_first_only_none_fail.wdl",
    "empty_array_fail.wdl",
    "test_as_map_fail.wdl",
    "write_json_fail.wdl",
    "test_map_fail.wdl",
    "select_first_empty_fail.wdl",
    "private_declaration_fail.wdl",
    "non_empty_optional_fail.wdl",
    "test_zip_fail.wdl",
    "illegal_access_fail.wdl",
}

PARSE_FAILURES_EXPECTED_WITH_RESERVED_KEYWORDS_V1_2 = {
    "test_find_task.wdl",
    "test_meta_values.wdl",
    "test_runtime_info_task.wdl",
}

FAILS_THAT_SHOULD_PARSE_OK_V1_3 = {
    "select_first_only_none_fail.wdl",
    "empty_array_fail.wdl",
    "test_as_map_fail.wdl",
    "write_json_fail.wdl",
    "test_map_fail.wdl",
    "select_first_empty_fail.wdl",
    "private_declaration_fail.wdl",
    "non_empty_optional_fail.wdl",
    "test_zip_fail.wdl",
    "illegal_access_fail.wdl",
}

PARSE_FAILURES_EXPECTED_WITH_RESERVED_KEYWORDS_V1_3 = {
    "test_find_task.wdl",
    "test_meta_values.wdl",
    "test_runtime_info_task.wdl",
    "test_task_previous.wdl",
}


def _load_wdl_examples(version: str) -> list[Path]:
    version_dir = SPEC_EXAMPLES_DIR / version
    if not version_dir.exists():
        return []
    return sorted(version_dir.glob("*.wdl"), key=lambda path: path.name)


def _assert_parse_spec_example(
    file_path: Path,
    fails_that_should_parse_ok: set[str],
    parse_failures_expected_with_reserved_keywords: set[str],
) -> None:
    filename = file_path.name
    source = file_path.read_text(encoding="utf-8")

    try:
        document = WdlV1Loader.load_from_string(source)
        assert document is not None
        assert document.elements() is not None
        if (
            filename.endswith("_fail.wdl")
            and filename not in fails_that_should_parse_ok
        ):
            raise AssertionError(f"Parsed but failure expected: {filename}")
    except WdlException as err:
        if (
            not filename.endswith("_fail.wdl")
            and filename not in parse_failures_expected_with_reserved_keywords
        ):
            raise AssertionError(f"Failed to parse {filename}: {err}") from err


def _assert_parse_and_validate_fail_spec_example(file_path: Path) -> None:
    source = file_path.read_text(encoding="utf-8")
    validator = WdlSemanticValidator()
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_string(source, validator)


def _load_wdl_fail_examples(version: str) -> list[Path]:
    return [
        path for path in _load_wdl_examples(version) if path.name.endswith("_fail.wdl")
    ]


@pytest.mark.parametrize(
    "file_path",
    _load_wdl_examples("v1_1"),
    ids=lambda path: path.name,
)
def test_parse_v11_spec_example(file_path: Path) -> None:
    _assert_parse_spec_example(file_path, FAILS_THAT_SHOULD_PARSE_OK_V1_1, set())


@pytest.mark.parametrize(
    "file_path",
    _load_wdl_examples("v1_2"),
    ids=lambda path: path.name,
)
def test_parse_v12_spec_example(file_path: Path) -> None:
    _assert_parse_spec_example(
        file_path,
        FAILS_THAT_SHOULD_PARSE_OK_V1_2,
        PARSE_FAILURES_EXPECTED_WITH_RESERVED_KEYWORDS_V1_2,
    )


@pytest.mark.parametrize(
    "file_path",
    _load_wdl_examples("v1_3"),
    ids=lambda path: path.name,
)
def test_parse_v13_spec_example(file_path: Path) -> None:
    _assert_parse_spec_example(
        file_path,
        FAILS_THAT_SHOULD_PARSE_OK_V1_3,
        PARSE_FAILURES_EXPECTED_WITH_RESERVED_KEYWORDS_V1_3,
    )


@pytest.mark.parametrize(
    "file_path",
    _load_wdl_fail_examples("v1_1"),
    ids=lambda path: path.name,
)
def test_parse_and_validate_v11_fail_spec_example(file_path: Path) -> None:
    _assert_parse_and_validate_fail_spec_example(file_path)


@pytest.mark.parametrize(
    "file_path",
    _load_wdl_fail_examples("v1_2"),
    ids=lambda path: path.name,
)
def test_parse_and_validate_v12_fail_spec_example(file_path: Path) -> None:
    _assert_parse_and_validate_fail_spec_example(file_path)


@pytest.mark.parametrize(
    "file_path",
    _load_wdl_fail_examples("v1_3"),
    ids=lambda path: path.name,
)
def test_parse_and_validate_v13_fail_spec_example(file_path: Path) -> None:
    _assert_parse_and_validate_fail_spec_example(file_path)
