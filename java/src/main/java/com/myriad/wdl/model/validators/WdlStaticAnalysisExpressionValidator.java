package com.myriad.wdl.model.validators;

import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlTernaryOperation;
import com.myriad.wdl.model.expressions.WdlUnaryOperation;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPairType;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import java.util.Map;
import java.util.Set;
import java.util.function.BiConsumer;
import java.util.function.BiPredicate;
import java.util.function.Function;
import java.util.function.Predicate;

/**
 * Expression validator used by {@link WdlStaticAnalysisValidator}.
 *
 * <p>In addition to the baseline expression checks, this class enforces operator compatibility and
 * generic function arity and signature rules. The synthetic static-analysis tests exercise cases
 * such as invalid {@code keys}, {@code range}, {@code contains_key}, {@code join_paths}, and
 * ordering comparisons.
 */
public class WdlStaticAnalysisExpressionValidator extends WdlExpressionValidator {

  public WdlStaticAnalysisExpressionValidator(
      Map<String, WdlType> scopeTypes,
      Map<String, Object> scopeValues,
      Map<String, Set<String>> callOutputs,
      Map<String, Map<String, WdlType>> callOutputTypes,
      Map<String, Set<String>> structMembers,
      Map<String, Map<String, WdlType>> structMemberTypes,
      WdlVersion documentVersion,
      BiConsumer<WdlSemanticError.Code, String> addError) {
    super(
        scopeTypes,
        scopeValues,
        callOutputs,
        callOutputTypes,
        structMembers,
        structMemberTypes,
        documentVersion,
        addError);
  }

  @Override
  protected WdlFunctionValidator createFunctionValidator(
      Function<WdlExpression, Object> evaluator,
      Predicate<Object> isUnknownValue,
      Predicate<WdlExpression> hasNonStringMapKey,
      Function<WdlExpression, WdlType> typeInferer,
      BiPredicate<WdlType, WdlType> isTypeAssignable,
      BiConsumer<WdlSemanticError.Code, String> addError) {
    return new WdlStaticAnalysisFunctionValidator(
        evaluator, isUnknownValue, hasNonStringMapKey, typeInferer, isTypeAssignable, addError);
  }

  @Override
  protected void processFunctionCallOperation(WdlFunctionCallOperation expression) {
    validateGenericFunctionArity(expression);
    validateGenericFunctionSignatures(expression);
    super.processFunctionCallOperation(expression);
  }

  @Override
  protected void processBinaryOperation(WdlBinaryOperation expression) {
    super.processBinaryOperation(expression);

    WdlType left = inferType(expression.getLeft());
    WdlType right = inferType(expression.getRight());

    switch (expression.getOperator()) {
      case OR:
      case AND:
        if ((left != null && !isPrimitive(left, WdlPrimitiveType.Type.BOOLEAN))
            || (right != null && !isPrimitive(right, WdlPrimitiveType.Type.BOOLEAN))) {
          addErrorMessage(
              WdlSemanticError.Code.TYPE_MISMATCH, "Logical operators require Boolean operands");
        }
        break;
      case MULTIPLY:
      case DIVIDE:
      case MODULO:
      case POWER:
      case SUTRACT:
        if ((left != null && !isNumeric(left)) || (right != null && !isNumeric(right))) {
          addErrorMessage(
              WdlSemanticError.Code.TYPE_MISMATCH,
              "Numeric operator requires Int or Float operands");
        }
        break;
      case ADD:
        if (left != null
            && right != null
            && !(isNumeric(left) && isNumeric(right))
            && !isPrimitive(left, WdlPrimitiveType.Type.STRING)
            && !isPrimitive(right, WdlPrimitiveType.Type.STRING)) {
          addErrorMessage(
              WdlSemanticError.Code.TYPE_MISMATCH,
              "'+' requires numeric operands or string concatenation");
        }
        break;
      case EQ:
      case NEQ:
        if (left != null
            && right != null
            && !isTypeAssignable(left, right)
            && !isTypeAssignable(right, left)) {
          addErrorMessage(
              WdlSemanticError.Code.TYPE_MISMATCH, "Equality comparison operands are incompatible");
        }
        break;
      case LT:
      case LTE:
      case GT:
      case GTE:
        if (left != null && right != null && !areOrderComparable(left, right)) {
          addErrorMessage(
              WdlSemanticError.Code.TYPE_MISMATCH, "Ordering comparison operands are incompatible");
        }
        break;
      default:
        break;
    }
  }

  @Override
  protected void processUnaryOperation(WdlUnaryOperation expression) {
    super.processUnaryOperation(expression);

    WdlType operand = inferType(expression.getOperand());
    if (operand == null) {
      return;
    }

    switch (expression.getOperator()) {
      case NOT:
        if (!isPrimitive(operand, WdlPrimitiveType.Type.BOOLEAN)) {
          addErrorMessage(WdlSemanticError.Code.TYPE_MISMATCH, "'!' requires a Boolean operand");
        }
        break;
      case NEGATIVE:
        if (!isNumeric(operand)) {
          addErrorMessage(
              WdlSemanticError.Code.TYPE_MISMATCH, "Unary '-' requires an Int or Float operand");
        }
        break;
      default:
        break;
    }
  }

  @Override
  protected void processTernaryOperation(WdlTernaryOperation expression) {
    super.processTernaryOperation(expression);

    WdlType conditionType = inferType(expression.getCondition());
    if (conditionType != null && !isPrimitive(conditionType, WdlPrimitiveType.Type.BOOLEAN)) {
      addErrorMessage(WdlSemanticError.Code.TYPE_MISMATCH, "Ternary condition must be Boolean");
    }

    WdlType trueType = inferType(expression.getTrueValue());
    WdlType falseType = inferType(expression.getFalseValue());
    if (trueType != null
        && falseType != null
        && !isTypeAssignable(trueType, falseType)
        && !isTypeAssignable(falseType, trueType)) {
      addErrorMessage(
          WdlSemanticError.Code.TYPE_MISMATCH, "Ternary branches have incompatible types");
    }
  }

  private boolean isNumeric(WdlType type) {
    return isPrimitive(type, WdlPrimitiveType.Type.INT)
        || isPrimitive(type, WdlPrimitiveType.Type.FLOAT);
  }

  private boolean isOrderablePrimitive(WdlType type) {
    return isNumeric(type) || isPrimitive(type, WdlPrimitiveType.Type.STRING);
  }

  private boolean areOrderComparable(WdlType left, WdlType right) {
    if (isNumeric(left) && isNumeric(right)) {
      return true;
    }
    return isOrderablePrimitive(left)
        && isOrderablePrimitive(right)
        && isPrimitive(left, WdlPrimitiveType.Type.STRING)
        && isPrimitive(right, WdlPrimitiveType.Type.STRING);
  }

  private void validateGenericFunctionArity(WdlFunctionCallOperation functionCall) {
    if (functionCall == null || functionCall.getFunction() == null) {
      return;
    }
    WdlFunctionCallOperation.WdlFunction fn = functionCall.getFunction();
    if (fn == WdlFunctionCallOperation.WdlFunction.NONSTANDARD) {
      return;
    }
    int argc = functionCall.arguments().size();
    if (!fn.supportsArity(argc)) {
      if (fn.isVariadic()) {
        addErrorMessage(
            WdlSemanticError.Code.INVALID_FUNCTION_ARGUMENTS,
            fn.toWdlString() + " expects at least " + fn.getMinArity() + " arguments");
      } else if (fn.getMinArity() == fn.getMaxArity()) {
        addErrorMessage(
            WdlSemanticError.Code.INVALID_FUNCTION_ARGUMENTS,
            fn.toWdlString() + " expects exactly " + fn.getMinArity() + " arguments");
      } else {
        addErrorMessage(
            WdlSemanticError.Code.INVALID_FUNCTION_ARGUMENTS,
            fn.toWdlString()
                + " expects between "
                + fn.getMinArity()
                + " and "
                + fn.getMaxArity()
                + " arguments");
      }
    }
  }

  private void validateGenericFunctionSignatures(WdlFunctionCallOperation functionCall) {
    if (functionCall == null || functionCall.getFunction() == null) {
      return;
    }
    WdlFunctionCallOperation.WdlFunction fn = functionCall.getFunction();
    if (fn == WdlFunctionCallOperation.WdlFunction.NONSTANDARD || fn.getSignatures().isEmpty()) {
      return;
    }

    int argc = functionCall.arguments().size();
    boolean anySignatureLengthMatch = false;
    boolean anySignatureCompatible = false;

    for (WdlFunctionCallOperation.WdlFunction.FunctionSignature sig : fn.getSignatures()) {
      if (sig.getArgs().size() != argc) {
        continue;
      }
      anySignatureLengthMatch = true;

      boolean compatible = true;
      int i = 0;
      for (WdlExpression arg : functionCall.arguments()) {
        WdlType argType = inferType(arg);
        if (argType != null && !matchesSignatureType(argType, sig.getArgs().get(i))) {
          compatible = false;
          break;
        }
        i++;
      }

      if (compatible) {
        anySignatureCompatible = true;
        break;
      }
    }

    if (anySignatureLengthMatch && !anySignatureCompatible) {
      addErrorMessage(
          WdlSemanticError.Code.INVALID_FUNCTION_ARGUMENTS,
          "Argument types are incompatible for function '" + fn.toWdlString() + "'");
    }
  }

  private boolean matchesSignatureType(
      WdlType actualType, WdlFunctionCallOperation.WdlFunction.T sigType) {
    if (actualType == null || sigType == null) {
      return true;
    }

    switch (sigType) {
      case ANY:
      case ANY_OPTIONAL:
        return true;
      case NUMBER:
        return isNumeric(actualType);
      case BOOLEAN:
        return isPrimitive(actualType, WdlPrimitiveType.Type.BOOLEAN);
      case INT:
        return isPrimitive(actualType, WdlPrimitiveType.Type.INT);
      case FLOAT:
        return isPrimitive(actualType, WdlPrimitiveType.Type.FLOAT);
      case STRING:
      case STRING_OPTIONAL:
        return isPrimitive(actualType, WdlPrimitiveType.Type.STRING);
      case FILE:
        return isPrimitive(actualType, WdlPrimitiveType.Type.FILE);
      case DIRECTORY:
        return isPrimitive(actualType, WdlPrimitiveType.Type.DIRECTORY);
      case FILE_OR_DIRECTORY:
        return isPrimitive(actualType, WdlPrimitiveType.Type.FILE)
            || isPrimitive(actualType, WdlPrimitiveType.Type.DIRECTORY)
            || isPrimitive(actualType, WdlPrimitiveType.Type.STRING);
      case OBJECT:
        return isPrimitive(actualType, WdlPrimitiveType.Type.OBJECT);
      case ARRAY_ANY:
        return actualType instanceof com.myriad.wdl.model.types.WdlArrayType;
      case ARRAY_FILE:
        return actualType instanceof com.myriad.wdl.model.types.WdlArrayType
            && isPrimitive(
                ((com.myriad.wdl.model.types.WdlArrayType) actualType).memberType(),
                WdlPrimitiveType.Type.FILE);
      case ARRAY_OPTIONAL_ANY:
      case ARRAY_INT:
      case ARRAY_STRING:
      case ARRAY_OBJECT:
      case ARRAY_PAIR:
      case ARRAY_ARRAY_ANY:
      case ARRAY_ARRAY_STRING:
        return matchArraySignature(actualType, sigType);
      case MAP_ANY_ANY:
      case MAP_ANY_ARRAY:
      case MAP_STRING_STRING:
        return matchMapSignature(actualType, sigType);
      case PAIR_ARRAY:
        return actualType instanceof WdlPairType
            && ((WdlPairType) actualType).leftType()
                instanceof com.myriad.wdl.model.types.WdlArrayType
            && ((WdlPairType) actualType).rightType()
                instanceof com.myriad.wdl.model.types.WdlArrayType;
      default:
        return true;
    }
  }

  private boolean matchArraySignature(
      WdlType actualType, WdlFunctionCallOperation.WdlFunction.T sigType) {
    if (!(actualType instanceof com.myriad.wdl.model.types.WdlArrayType)) {
      return false;
    }
    WdlType member = ((com.myriad.wdl.model.types.WdlArrayType) actualType).memberType();
    switch (sigType) {
      case ARRAY_OPTIONAL_ANY:
      case ARRAY_ANY:
        return true;
      case ARRAY_INT:
        return isPrimitive(member, WdlPrimitiveType.Type.INT);
      case ARRAY_STRING:
        return isPrimitive(member, WdlPrimitiveType.Type.STRING);
      case ARRAY_OBJECT:
        return isPrimitive(member, WdlPrimitiveType.Type.OBJECT);
      case ARRAY_PAIR:
        return member instanceof WdlPairType;
      case ARRAY_ARRAY_ANY:
        return member instanceof com.myriad.wdl.model.types.WdlArrayType;
      case ARRAY_ARRAY_STRING:
        return member instanceof com.myriad.wdl.model.types.WdlArrayType
            && isPrimitive(
                ((com.myriad.wdl.model.types.WdlArrayType) member).memberType(),
                WdlPrimitiveType.Type.STRING);
      default:
        return true;
    }
  }

  private boolean matchMapSignature(
      WdlType actualType, WdlFunctionCallOperation.WdlFunction.T sigType) {
    if (!(actualType instanceof WdlMapType)) {
      return false;
    }
    WdlMapType mapType = (WdlMapType) actualType;
    switch (sigType) {
      case MAP_ANY_ANY:
        return true;
      case MAP_ANY_ARRAY:
        return mapType.valueType() instanceof com.myriad.wdl.model.types.WdlArrayType;
      case MAP_STRING_STRING:
        return isPrimitive(mapType.keyType(), WdlPrimitiveType.Type.STRING)
            && isPrimitive(mapType.valueType(), WdlPrimitiveType.Type.STRING);
      default:
        return true;
    }
  }
}
