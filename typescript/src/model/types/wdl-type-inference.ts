/** Shared type-inference helpers for lightweight literal and enum analysis. */
import { WdlEnum } from '../definitions/wdl-enum.js';
import {
  WdlArrayLiteral,
  WdlBooleanLiteral,
  type WdlExpression,
  WdlFloatLiteral,
  WdlIntLiteral,
  WdlMapLiteral,
  WdlNullLiteral,
  WdlObjectLiteral,
  WdlPairLiteral,
  WdlStringLiteral,
  WdlStructLiteral,
  WdlVariable,
} from '../expressions/index.js';
import { WdlArrayType } from './wdl-array-type.js';
import { WdlMapType } from './wdl-map-type.js';
import { WdlPairType } from './wdl-pair-type.js';
import { WdlPrimitiveType } from './wdl-primitive-type.js';
import type { WdlType } from './wdl-type.js';
import { WdlTypeReferenceType } from './wdl-type-reference-type.js';

/** Infers an enum's effective value type from its members. */
export function inferEnumValueType(enumDef: WdlEnum | undefined): WdlType | undefined {
  if (!enumDef) return undefined;
  if (enumDef.getValueType()) return enumDef.getValueType();

  let inferred: WdlType | undefined;
  for (const choice of enumDef.elements()) {
    const value = choice.getValue();
    const choiceType = value
      ? inferLiteralExpressionType(value)
      : new WdlPrimitiveType(WdlPrimitiveType.Type.STRING);
    if (!choiceType) return undefined;
    inferred = mergeCoercibleTypes(inferred, choiceType);
    if (!inferred) return undefined;
  }

  return inferred ?? new WdlPrimitiveType(WdlPrimitiveType.Type.STRING);
}

/** Infers type for self-contained literal expressions without scope/import context. */
export function inferLiteralExpressionType(expr: WdlExpression | undefined): WdlType | undefined {
  if (!expr) return undefined;
  if (expr instanceof WdlIntLiteral) return new WdlPrimitiveType(WdlPrimitiveType.Type.INT);
  if (expr instanceof WdlFloatLiteral) return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
  if (expr instanceof WdlBooleanLiteral)
    return new WdlPrimitiveType(WdlPrimitiveType.Type.BOOLEAN);
  if (expr instanceof WdlStringLiteral) return new WdlPrimitiveType(WdlPrimitiveType.Type.STRING);
  if (expr instanceof WdlNullLiteral) return undefined;
  if (expr instanceof WdlVariable && expr.getName() === 'None') return undefined;
  if (expr instanceof WdlStructLiteral) {
    const name = expr.getName();
    return name && name.trim() ? new WdlTypeReferenceType(name) : undefined;
  }
  if (expr instanceof WdlObjectLiteral) return new WdlTypeReferenceType('Object');
  if (expr instanceof WdlArrayLiteral) {
    let memberType: WdlType | undefined;
    for (const item of expr.entries()) {
      memberType = mergeCoercibleTypes(memberType, inferLiteralExpressionType(item));
      if (!memberType) return undefined;
    }
    return new WdlArrayType(memberType);
  }
  if (expr instanceof WdlPairLiteral) {
    const left = inferLiteralExpressionType(expr.getLeft());
    const right = inferLiteralExpressionType(expr.getRight());
    return left && right ? new WdlPairType(left, right) : undefined;
  }
  if (expr instanceof WdlMapLiteral) {
    let keyType: WdlType | undefined;
    let valueType: WdlType | undefined;
    for (const entry of expr.entries()) {
      keyType = mergeCoercibleTypes(keyType, inferLiteralExpressionType(entry.getKey()));
      valueType = mergeCoercibleTypes(valueType, inferLiteralExpressionType(entry.getValue()));
      if (!keyType || !valueType) return undefined;
    }
    return new WdlMapType(keyType, valueType);
  }
  return undefined;
}

function mergeCoercibleTypes(
  current: WdlType | undefined,
  next: WdlType | undefined,
): WdlType | undefined {
  if (!next) return current;
  if (!current) return next;
  if (sameTypeShape(current, next)) return current;
  if (
    isPrimitive(current, WdlPrimitiveType.Type.INT) &&
    isPrimitive(next, WdlPrimitiveType.Type.FLOAT)
  ) {
    return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
  }
  if (
    isPrimitive(current, WdlPrimitiveType.Type.FLOAT) &&
    isPrimitive(next, WdlPrimitiveType.Type.INT)
  ) {
    return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
  }
  return undefined;
}

function sameTypeShape(left: WdlType | undefined, right: WdlType | undefined): boolean {
  if (!left || !right || left.componentType() !== right.componentType()) return false;
  if (left instanceof WdlPrimitiveType && right instanceof WdlPrimitiveType)
    return left.primitiveType() === right.primitiveType();
  if (left instanceof WdlTypeReferenceType && right instanceof WdlTypeReferenceType)
    return left.referenceName() === right.referenceName();
  if (left instanceof WdlArrayType && right instanceof WdlArrayType)
    return sameTypeShape(left.memberType(), right.memberType());
  if (left instanceof WdlMapType && right instanceof WdlMapType)
    return (
      sameTypeShape(left.keyType(), right.keyType()) &&
      sameTypeShape(left.valueType(), right.valueType())
    );
  if (left instanceof WdlPairType && right instanceof WdlPairType)
    return (
      sameTypeShape(left.leftType(), right.leftType()) &&
      sameTypeShape(left.rightType(), right.rightType())
    );
  return false;
}

function isPrimitive(type: WdlType | undefined, primitive: WdlPrimitiveType.Type): boolean {
  return type instanceof WdlPrimitiveType && type.primitiveType() === primitive;
}
