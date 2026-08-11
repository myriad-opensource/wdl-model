from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlLintingSemanticValidator, WdlV1Loader
from wdl_model.model.errors import WdlException, WdlSemanticError, WdlSemanticErrorCode

FIXTURES_ROOT = Path("wdl_tests") / "deprecations"


@pytest.mark.parametrize(
    "fixture",
    [
        "runtime_section_deprecated.wdl",
        "object_type_deprecated.wdl",
        "placeholder_options_deprecated.wdl",
        "file_scheme_import_deprecated.wdl",
    ],
)
def test_warns_on_deprecated_features(fixture: str) -> None:
    file_path = FIXTURES_ROOT / fixture
    if fixture == "file_scheme_import_deprecated.wdl":
        doc = WdlV1Loader.load_from_string(file_path.read_text(encoding="utf-8"))
    else:
        doc = WdlV1Loader.load_from_file(file_path)

    with pytest.raises(WdlException) as ex_info:
        WdlLintingSemanticValidator().validateDocument(doc)

    semantic_errors = [
        err for err in ex_info.value.getErrors() if isinstance(err, WdlSemanticError)
    ]
    assert any(
        err.code == WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE
        for err in semantic_errors
    )


@pytest.mark.parametrize("fixture", ["no_deprecations.wdl"])
def test_does_not_report_deprecation_warnings(fixture: str) -> None:
    doc = WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture)
    lint = WdlLintingSemanticValidator().setThrowOnWarnings(False)
    lint.validateDocument(doc)
