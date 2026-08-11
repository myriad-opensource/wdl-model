from __future__ import annotations

from wdl_model.model.definitions import (
    WdlEnum,
    WdlEnumChoice,
    WdlStruct,
    WdlStructMember,
)
from wdl_model.model.expressions import (
    WdlFloatLiteral,
    WdlFunction,
    WdlFunctionCallOperation,
    WdlIntLiteral,
)
from wdl_model.model.types import Type, WdlPrimitiveType
from wdl_model.model.types.wdl_type_inference import infer_enum_value_type


def test_infer_enum_value_type_defaults_to_string() -> None:
    enum_def = WdlEnum("Letters")
    enum_def.elements().append(WdlEnumChoice("A", None))
    enum_def.elements().append(WdlEnumChoice("B", None))

    inferred = infer_enum_value_type(enum_def)

    assert inferred is not None
    assert isinstance(inferred, WdlPrimitiveType)
    assert inferred.primitiveType() == Type.STRING


def test_infer_enum_value_type_widens_int_float() -> None:
    enum_def = WdlEnum("Numbers")
    enum_def.elements().append(WdlEnumChoice("ONE", WdlIntLiteral(1)))
    enum_def.elements().append(WdlEnumChoice("PI", WdlFloatLiteral(3.14)))

    inferred = infer_enum_value_type(enum_def)

    assert inferred is not None
    assert isinstance(inferred, WdlPrimitiveType)
    assert inferred.primitiveType() == Type.FLOAT


def test_infer_enum_value_type_rejects_dynamic_expression() -> None:
    dynamic = WdlFunctionCallOperation(functionName="foo")
    dynamic.setFunction(WdlFunction.NONSTANDARD)

    enum_def = WdlEnum("Bad")
    enum_def.elements().append(WdlEnumChoice("ONE", WdlIntLiteral(1)))
    enum_def.elements().append(WdlEnumChoice("DYNAMIC", dynamic))

    assert infer_enum_value_type(enum_def) is None


def test_struct_and_enum_local_introspection_helpers() -> None:
    struct_def = WdlStruct("Person")
    struct_def.elements().append(WdlStructMember(WdlPrimitiveType(Type.STRING), "name"))
    struct_def.elements().append(WdlStructMember(WdlPrimitiveType(Type.INT), "age"))

    enum_def = WdlEnum("Status")
    enum_def.elements().append(WdlEnumChoice("NEW", None))
    enum_def.elements().append(WdlEnumChoice("DONE", None))

    assert struct_def.hasMember("name")
    assert not struct_def.hasMember("missing")
    assert isinstance(struct_def.memberType("age"), WdlPrimitiveType)
    assert struct_def.member("name") is not None

    assert enum_def.hasChoice("DONE")
    assert not enum_def.hasChoice("FAILED")
    assert enum_def.choice("NEW") is not None
