from __future__ import annotations

from wdl_model.model import WdlVersion
from wdl_model.model.expressions import (
    WdlFunction,
    WdlFunctionCallOperation,
    WdlFunctionTypeHint,
)


def test_function_name_resolves_known_function() -> None:
    op = WdlFunctionCallOperation("zip")

    assert op.getFunctionName() == "zip"
    assert op.getFunction() == WdlFunction.ZIP


def test_function_name_keeps_custom_nonstandard_name() -> None:
    op = WdlFunctionCallOperation("my_custom_fn")

    assert op.getFunctionName() == "my_custom_fn"
    assert op.getFunction() == WdlFunction.NONSTANDARD


def test_setting_standard_function_normalizes_name() -> None:
    op = WdlFunctionCallOperation("my_custom_fn")

    op.setFunction(WdlFunction.JOIN_PATHS)

    assert op.getFunctionName() == "join_paths"
    assert op.getFunction() == WdlFunction.JOIN_PATHS


def test_variadic_arity_support_metadata() -> None:
    fn = WdlFunction.JOIN_PATHS
    assert fn.isVariadic()
    assert fn.supportsArity(2)
    assert fn.supportsArity(10)
    assert not fn.supportsArity(1)


def test_signature_metadata_exposes_type_hints() -> None:
    sig = WdlFunction.READ_INT.getSignatures()[0]
    assert sig.returns == WdlFunctionTypeHint.INT
    assert len(sig.args) == 1
    assert sig.args[0] == WdlFunctionTypeHint.FILE


def test_version_metadata_is_exposed() -> None:
    assert WdlFunction.MIN.getAddedIn() == WdlVersion.V1_1
    assert WdlFunction.JOIN_PATHS.getAddedIn() == WdlVersion.V1_2
    assert WdlFunction.VALUE.getAddedIn() == WdlVersion.V1_3

    assert WdlFunction.MIN.getDeprecatedIn() is None
    assert WdlFunction.MIN.getRemovedIn() is None


def test_nonstandard_is_variadic_and_versionless() -> None:
    fn = WdlFunction.NONSTANDARD
    assert fn.toWdlString() == "nonstandard"
    assert fn.isVariadic()
    assert fn.supportsArity(0)
    assert fn.supportsArity(99)
    assert fn.getAddedIn() is None
    assert fn.getDeprecatedIn() is None
    assert fn.getRemovedIn() is None
