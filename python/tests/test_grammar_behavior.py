from __future__ import annotations

from pathlib import Path

import pytest

from wdl_model.model import WdlV1Loader
from wdl_model.model.errors import WdlException
from wdl_model.model.expressions import (
    WdlBinaryOperation,
    WdlBinaryOperator,
    WdlBooleanLiteral,
    WdlIntLiteral,
)
from wdl_model.model.statements import WdlBoundDeclaration

FIXTURES_ROOT = Path("wdl_tests") / "grammar_behavior"


def _first_workflow_declaration(fixture_name: str) -> WdlBoundDeclaration:
    document = WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture_name)
    workflow = document.workflows()[0]
    declaration = workflow.elements()[0]
    assert isinstance(declaration, WdlBoundDeclaration)
    return declaration


def test_parses_additive_chain_as_left_associative() -> None:
    declaration = _first_workflow_declaration("associativity_additive_chain.wdl")

    root = declaration.expression
    assert isinstance(root, WdlBinaryOperation)
    assert root.operator == WdlBinaryOperator.SUBTRACT
    assert isinstance(root.right, WdlIntLiteral)
    assert root.right.getValue() == 3

    left = root.left
    assert isinstance(left, WdlBinaryOperation)
    assert left.operator == WdlBinaryOperator.SUBTRACT
    assert isinstance(left.left, WdlIntLiteral)
    assert left.left.getValue() == 1
    assert isinstance(left.right, WdlIntLiteral)
    assert left.right.getValue() == 2


def test_parses_multiplicative_chain_as_left_associative() -> None:
    declaration = _first_workflow_declaration("associativity_multiplicative_chain.wdl")

    root = declaration.expression
    assert isinstance(root, WdlBinaryOperation)
    assert root.operator == WdlBinaryOperator.DIVIDE
    assert isinstance(root.right, WdlIntLiteral)
    assert root.right.getValue() == 2

    left = root.left
    assert isinstance(left, WdlBinaryOperation)
    assert left.operator == WdlBinaryOperator.DIVIDE
    assert isinstance(left.left, WdlIntLiteral)
    assert left.left.getValue() == 8
    assert isinstance(left.right, WdlIntLiteral)
    assert left.right.getValue() == 4


def test_parses_logical_or_chain_as_left_associative() -> None:
    declaration = _first_workflow_declaration("associativity_logical_or_chain.wdl")

    root = declaration.expression
    assert isinstance(root, WdlBinaryOperation)
    assert root.operator == WdlBinaryOperator.LOGICAL_OR
    assert isinstance(root.right, WdlBooleanLiteral)
    assert root.right.getValue() is True

    left = root.left
    assert isinstance(left, WdlBinaryOperation)
    assert left.operator == WdlBinaryOperator.LOGICAL_OR
    assert isinstance(left.left, WdlBooleanLiteral)
    assert left.left.getValue() is True
    assert isinstance(left.right, WdlBooleanLiteral)
    assert left.right.getValue() is False


@pytest.mark.parametrize(
    "fixture_name",
    [
        "keyword_decl_identifier_task.wdl",
        "keyword_decl_identifier_if.wdl",
        "keyword_task_input_in.wdl",
        "keyword_metadata_key_version.wdl",
    ],
)
def test_rejects_reserved_keyword_identifier_fixtures(fixture_name: str) -> None:
    with pytest.raises(WdlException):
        WdlV1Loader.load_from_file(FIXTURES_ROOT / fixture_name)
