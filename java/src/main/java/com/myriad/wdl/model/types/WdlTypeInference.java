package com.myriad.wdl.model.types;

import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlBooleanLiteral;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFloatLiteral;
import com.myriad.wdl.model.expressions.WdlIntLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral;
import com.myriad.wdl.model.expressions.WdlNullLiteral;
import com.myriad.wdl.model.expressions.WdlObjectLiteral;
import com.myriad.wdl.model.expressions.WdlPairLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import com.myriad.wdl.model.expressions.WdlStructLiteral;
import com.myriad.wdl.model.expressions.WdlVariable;
import java.util.Objects;
import java.util.Optional;

/**
 * Shared lightweight type inference helpers for model-level and validator-level consumers.
 *
 * <p>This utility intentionally limits itself to self-contained expression forms and enum member
 * value coercion. It does not perform scope-aware or import-aware inference.
 */
public final class WdlTypeInference {

  private WdlTypeInference() {}

  /**
   * Infer an enum's effective value type from its choices.
   *
   * <p>If the enum has an explicit declared value type, that type is returned. Otherwise choice
   * values are merged using WDL-compatible widening (for example Int + Float -> Float). If no
   * explicit values are present, String is assumed.
   */
  public static Optional<WdlType> inferEnumValueType(WdlEnum enumDef) {
    if (enumDef == null) {
      return Optional.empty();
    }
    if (enumDef.getValueType() != null) {
      return Optional.of(enumDef.getValueType());
    }

    WdlType inferred = null;
    for (WdlEnum.WdlEnumChoice choice : enumDef.elements()) {
      WdlExpression value = choice.getValue();
      WdlType choiceType =
          value == null
              ? new WdlPrimitiveType(WdlPrimitiveType.Type.STRING, false)
              : inferLiteralExpressionType(value);
      if (choiceType == null) {
        return Optional.empty();
      }
      inferred = mergeCoercibleTypes(inferred, choiceType);
      if (inferred == null) {
        return Optional.empty();
      }
    }

    if (inferred == null) {
      inferred = new WdlPrimitiveType(WdlPrimitiveType.Type.STRING, false);
    }
    return Optional.of(inferred);
  }

  /**
   * Infer the static literal type for expression forms that are self-typed.
   *
   * <p>This helper intentionally avoids scope-aware inference and is safe for structural model
   * queries in processors.
   */
  public static WdlType inferLiteralExpressionType(WdlExpression expression) {
    if (expression == null) {
      return null;
    }
    if (expression instanceof WdlIntLiteral) {
      return new WdlPrimitiveType(WdlPrimitiveType.Type.INT, false);
    }
    if (expression instanceof WdlFloatLiteral) {
      return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT, false);
    }
    if (expression instanceof WdlBooleanLiteral) {
      return new WdlPrimitiveType(WdlPrimitiveType.Type.BOOLEAN, false);
    }
    if (expression instanceof WdlStringLiteral) {
      return new WdlPrimitiveType(WdlPrimitiveType.Type.STRING, false);
    }
    if (expression instanceof WdlNullLiteral) {
      return null;
    }
    if (expression instanceof WdlVariable && "None".equals(((WdlVariable) expression).getName())) {
      return null;
    }
    if (expression instanceof WdlStructLiteral) {
      String name = ((WdlStructLiteral) expression).getName();
      return name == null || name.isBlank() ? null : new WdlTypeReferenceType(name, false);
    }
    if (expression instanceof WdlObjectLiteral) {
      return new WdlTypeReferenceType("Object", false);
    }
    if (expression instanceof WdlArrayLiteral) {
      WdlType memberType = null;
      for (WdlExpression item : ((WdlArrayLiteral) expression).entries()) {
        memberType = mergeCoercibleTypes(memberType, inferLiteralExpressionType(item));
        if (memberType == null) {
          return null;
        }
      }
      return new WdlArrayType(memberType, false, false);
    }
    if (expression instanceof WdlPairLiteral) {
      WdlType left = inferLiteralExpressionType(((WdlPairLiteral) expression).getLeft());
      WdlType right = inferLiteralExpressionType(((WdlPairLiteral) expression).getRight());
      if (left == null || right == null) {
        return null;
      }
      return new WdlPairType(left, right, false);
    }
    if (expression instanceof WdlMapLiteral) {
      WdlType keyType = null;
      WdlType valueType = null;
      for (WdlMapLiteral.WdlMapEntry entry : ((WdlMapLiteral) expression).entries()) {
        keyType = mergeCoercibleTypes(keyType, inferLiteralExpressionType(entry.getKey()));
        valueType = mergeCoercibleTypes(valueType, inferLiteralExpressionType(entry.getValue()));
        if (keyType == null || valueType == null) {
          return null;
        }
      }
      return new WdlMapType(keyType, valueType, false);
    }
    return null;
  }

  private static WdlType mergeCoercibleTypes(WdlType currentType, WdlType nextType) {
    if (nextType == null) {
      return currentType;
    }
    if (currentType == null) {
      return nextType;
    }

    if (sameTypeShape(currentType, nextType)) {
      return currentType;
    }

    if (isPrimitive(currentType, WdlPrimitiveType.Type.INT)
        && isPrimitive(nextType, WdlPrimitiveType.Type.FLOAT)) {
      return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT, false);
    }
    if (isPrimitive(currentType, WdlPrimitiveType.Type.FLOAT)
        && isPrimitive(nextType, WdlPrimitiveType.Type.INT)) {
      return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT, false);
    }

    return null;
  }

  private static boolean sameTypeShape(WdlType left, WdlType right) {
    if (left == null || right == null || left.componentType() != right.componentType()) {
      return false;
    }
    switch (left.componentType()) {
      case PRIMITIVE:
        return ((WdlPrimitiveType) left).primitiveType()
            == ((WdlPrimitiveType) right).primitiveType();
      case TYPEREF:
        return Objects.equals(
            ((WdlTypeReferenceType) left).referenceName(),
            ((WdlTypeReferenceType) right).referenceName());
      case ARRAY:
        return sameTypeShape(
            ((WdlArrayType) left).memberType(), ((WdlArrayType) right).memberType());
      case MAP:
        return sameTypeShape(((WdlMapType) left).keyType(), ((WdlMapType) right).keyType())
            && sameTypeShape(((WdlMapType) left).valueType(), ((WdlMapType) right).valueType());
      case PAIR:
        return sameTypeShape(((WdlPairType) left).leftType(), ((WdlPairType) right).leftType())
            && sameTypeShape(((WdlPairType) left).rightType(), ((WdlPairType) right).rightType());
      default:
        return false;
    }
  }

  private static boolean isPrimitive(WdlType type, WdlPrimitiveType.Type primitiveType) {
    return type instanceof WdlPrimitiveType
        && ((WdlPrimitiveType) type).primitiveType() == primitiveType;
  }
}
