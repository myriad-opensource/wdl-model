"""Shared type-inference helpers for lightweight literal and enum analysis."""

from __future__ import annotations

from wdl_model.model.definitions import WdlEnum
from wdl_model.model.expressions import (
    WdlArrayLiteral,
    WdlBooleanLiteral,
    WdlExpression,
    WdlFloatLiteral,
    WdlIntLiteral,
    WdlMapLiteral,
    WdlNullLiteral,
    WdlObjectLiteral,
    WdlPairLiteral,
    WdlStringLiteral,
    WdlStructLiteral,
    WdlVariable,
)

from .wdl_array_type import WdlArrayType
from .wdl_map_type import WdlMapType
from .wdl_pair_type import WdlPairType
from .wdl_primitive_type import Type as WdlPrimitiveTypeEnum
from .wdl_primitive_type import WdlPrimitiveType
from .wdl_type import WdlType
from .wdl_type_reference_type import WdlTypeReferenceType


def infer_enum_value_type(enum_def: WdlEnum | None) -> WdlType | None:
    """Infer an enum's effective value type from its members."""
    if enum_def is None:
        return None
    if enum_def.valueType is not None:
        return enum_def.valueType

    inferred: WdlType | None = None
    for choice in enum_def.elements():
        value = choice.getValue()
        choice_type = (
            WdlPrimitiveType(WdlPrimitiveTypeEnum.STRING)
            if value is None
            else infer_literal_expression_type(value)
        )
        if choice_type is None:
            return None
        inferred = _merge_coercible_types(inferred, choice_type)
        if inferred is None:
            return None

    if inferred is None:
        return WdlPrimitiveType(WdlPrimitiveTypeEnum.STRING)
    return inferred


def infer_literal_expression_type(expression: WdlExpression | None) -> WdlType | None:
    """Infer type for self-contained literal expressions without scope/import context."""
    if expression is None:
        return None
    if isinstance(expression, WdlIntLiteral):
        return WdlPrimitiveType(WdlPrimitiveTypeEnum.INT)
    if isinstance(expression, WdlFloatLiteral):
        return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
    if isinstance(expression, WdlBooleanLiteral):
        return WdlPrimitiveType(WdlPrimitiveTypeEnum.BOOLEAN)
    if isinstance(expression, WdlStringLiteral):
        return WdlPrimitiveType(WdlPrimitiveTypeEnum.STRING)
    if isinstance(expression, WdlNullLiteral):
        return None
    if isinstance(expression, WdlVariable) and expression.name == "None":
        return None
    if isinstance(expression, WdlStructLiteral):
        name = expression.name
        return None if name is None or not name.strip() else WdlTypeReferenceType(name)
    if isinstance(expression, WdlObjectLiteral):
        return WdlTypeReferenceType("Object")
    if isinstance(expression, WdlArrayLiteral):
        member_type: WdlType | None = None
        for item in expression.entries():
            member_type = _merge_coercible_types(
                member_type, infer_literal_expression_type(item)
            )
            if member_type is None:
                return None
        return WdlArrayType(member_type)
    if isinstance(expression, WdlPairLiteral):
        left = infer_literal_expression_type(expression.left)
        right = infer_literal_expression_type(expression.right)
        if left is None or right is None:
            return None
        return WdlPairType(left, right)
    if isinstance(expression, WdlMapLiteral):
        key_type: WdlType | None = None
        value_type: WdlType | None = None
        for entry in expression.entries():
            key_type = _merge_coercible_types(
                key_type, infer_literal_expression_type(entry.getKey())
            )
            value_type = _merge_coercible_types(
                value_type, infer_literal_expression_type(entry.getValue())
            )
            if key_type is None or value_type is None:
                return None
        return WdlMapType(key_type, value_type)
    return None


def _merge_coercible_types(current_type: WdlType | None, next_type: WdlType | None) -> WdlType | None:
    if next_type is None:
        return current_type
    if current_type is None:
        return next_type
    if _same_type_shape(current_type, next_type):
        return current_type
    if _is_primitive(current_type, WdlPrimitiveTypeEnum.INT) and _is_primitive(
        next_type, WdlPrimitiveTypeEnum.FLOAT
    ):
        return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
    if _is_primitive(current_type, WdlPrimitiveTypeEnum.FLOAT) and _is_primitive(
        next_type, WdlPrimitiveTypeEnum.INT
    ):
        return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
    return None


def _same_type_shape(left: WdlType | None, right: WdlType | None) -> bool:
    if left is None or right is None or left.componentType() != right.componentType():
        return False
    if isinstance(left, WdlPrimitiveType) and isinstance(right, WdlPrimitiveType):
        return left.primitiveType() == right.primitiveType()
    if isinstance(left, WdlTypeReferenceType) and isinstance(right, WdlTypeReferenceType):
        return left.referenceName() == right.referenceName()
    if isinstance(left, WdlArrayType) and isinstance(right, WdlArrayType):
        return _same_type_shape(left.memberType(), right.memberType())
    if isinstance(left, WdlMapType) and isinstance(right, WdlMapType):
        return _same_type_shape(left.keyType(), right.keyType()) and _same_type_shape(
            left.valueType(), right.valueType()
        )
    if isinstance(left, WdlPairType) and isinstance(right, WdlPairType):
        return _same_type_shape(left.leftType(), right.leftType()) and _same_type_shape(
            left.rightType(), right.rightType()
        )
    return False


def _is_primitive(t: WdlType | None, p: WdlPrimitiveTypeEnum) -> bool:
    return isinstance(t, WdlPrimitiveType) and t.primitiveType() == p
