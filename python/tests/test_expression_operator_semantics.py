from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import (
    WdlSemanticValidator,
    WdlStaticAnalysisSemanticValidator,
    WdlV1Loader,
)
from wdl_model.model.errors import WdlException

FIXTURES_ROOT = Path("wdl_tests") / "expression_operator_semantics"


@pytest.mark.parametrize(
    "fixture",
    [
        "logical_operand_type_fail.wdl",
        "numeric_operand_type_fail.wdl",
        "order_comparison_type_fail.wdl",
        "ternary_condition_type_fail.wdl",
    ],
)
def test_rejects_operator_type_mismatches_under_static_validator(fixture: str) -> None:
    doc = WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture)
    WdlSemanticValidator().validateDocument(doc)
    with pytest.raises(WdlException):
        WdlStaticAnalysisSemanticValidator().validateDocument(doc)


def test_accepts_valid_operator_expressions() -> None:
    doc = WdlV1Loader.load_from_file(FIXTURES_ROOT / "operators_ok.wdl")
    WdlSemanticValidator().validateDocument(doc)
    WdlStaticAnalysisSemanticValidator().validateDocument(doc)


def test_accepts_operator_precedence_and_compound_equality() -> None:
    precedence_doc = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "operator_precedence_ok.wdl"
    )
    equality_doc = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "compound_equality_ok.wdl"
    )

    WdlStaticAnalysisSemanticValidator().validateDocument(precedence_doc)
    WdlStaticAnalysisSemanticValidator().validateDocument(equality_doc)


def test_rejects_incompatible_compound_equality() -> None:
    doc = WdlV1Loader.load_from_file(
        FIXTURES_ROOT / "compound_equality_incompatible_fail.wdl"
    )
    WdlSemanticValidator().validateDocument(doc)
    with pytest.raises(WdlException):
        WdlStaticAnalysisSemanticValidator().validateDocument(doc)
