package com.myriad.wdl.model.validators;

import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlPairLiteral;
import com.myriad.wdl.model.processors.WdlFunctionProcessorBase;
import com.myriad.wdl.model.types.WdlArrayType;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPairType;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import java.util.List;
import java.util.Objects;
import java.util.function.BiConsumer;
import java.util.function.Function;
import java.util.function.Predicate;

/**
 * Function-level semantic checks dispatched by {@link WdlFunctionCallOperation.WdlFunction}.
 *
 * <p>This validator hosts the definite function-call failures that should be shared by baseline
 * semantic validation and the stricter static-analysis layer. Examples include invalid
 * {@code select_first}, {@code as_map}, {@code zip}, and {@code write_json} usage, matching the
 * repository fixtures under {@code spec_examples/v1_3/}.
 */
public class WdlFunctionValidator extends WdlFunctionProcessorBase {

  private final Function<WdlExpression, Object> evaluator;
  private final Predicate<Object> isUnknownValue;
  private final Predicate<WdlExpression> hasNonStringMapKey;
  private final Function<WdlExpression, WdlType> typeInferer;
  private final java.util.function.BiPredicate<WdlType, WdlType> isTypeAssignable;
  private final BiConsumer<WdlSemanticError.Code, String> addError;

  public WdlFunctionValidator(
      Function<WdlExpression, Object> evaluator,
      Predicate<Object> isUnknownValue,
      Predicate<WdlExpression> hasNonStringMapKey,
      Function<WdlExpression, WdlType> typeInferer,
      java.util.function.BiPredicate<WdlType, WdlType> isTypeAssignable,
      BiConsumer<WdlSemanticError.Code, String> addError) {
    this.evaluator = evaluator;
    this.isUnknownValue = isUnknownValue;
    this.hasNonStringMapKey = hasNonStringMapKey;
    this.typeInferer = typeInferer;
    this.isTypeAssignable = isTypeAssignable;
    this.addError = addError;
  }

  @Override
  public void processSelectFirst(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("select_first expects exactly 1 argument");
      return;
    }
    if (functionCall.arguments().isEmpty()) {
      return;
    }
    WdlExpression firstArg = functionCall.arguments().peekFirst();

    if (firstArg instanceof WdlArrayLiteral && ((WdlArrayLiteral) firstArg).entries().isEmpty()) {
      addErrorMessage("select_first array is empty");
      return;
    }

    Object value = evaluator.apply(firstArg);
    if (value instanceof List) {
      List<?> list = (List<?>) value;
      if (list.isEmpty()) {
        addErrorMessage("select_first array is empty");
      } else if (list.stream().allMatch(Objects::isNull)) {
        addErrorMessage("select_first array contains only None values");
      }
    }
  }

  @Override
  public void processAsMap(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("as_map expects exactly 1 argument");
      return;
    }
    if (functionCall.arguments().isEmpty()) {
      return;
    }
    WdlExpression firstArg = functionCall.arguments().peekFirst();
    if (!(firstArg instanceof WdlArrayLiteral)) {
      WdlType argType = typeInferer.apply(firstArg);
      if (!(argType instanceof WdlArrayType)
          || !(((WdlArrayType) argType).memberType() instanceof WdlPairType)) {
        addErrorMessage("as_map expects Array[Pair[K,V]]");
      }
      return;
    }

    java.util.Set<Object> seen = new java.util.HashSet<>();
    for (WdlExpression item : ((WdlArrayLiteral) firstArg).entries()) {
      if (!(item instanceof WdlPairLiteral)) {
        continue;
      }
      Object key = evaluator.apply(((WdlPairLiteral) item).getLeft());
      if (isUnknownValue.test(key)) {
        continue;
      }
      if (!seen.add(key)) {
        addErrorMessage("as_map has duplicate key: " + key);
        return;
      }
    }
  }

  @Override
  public void processZip(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 2) {
      addErrorMessage("zip expects exactly 2 arguments");
      return;
    }
    if (functionCall.arguments().size() < 2) {
      return;
    }
    WdlExpression leftArg = functionCall.arguments().peekFirst();
    WdlExpression rightArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);

    Object left = evaluator.apply(leftArg);
    Object right = evaluator.apply(rightArg);

    WdlType leftType = typeInferer.apply(leftArg);
    WdlType rightType = typeInferer.apply(rightArg);
    if ((leftType != null && !(leftType instanceof WdlArrayType))
        || (rightType != null && !(rightType instanceof WdlArrayType))) {
      addErrorMessage("zip expects two array arguments");
    }

    if (left instanceof List
        && right instanceof List
        && ((List<?>) left).size() != ((List<?>) right).size()) {
      addErrorMessage("zip arguments must have the same length");
    }
  }

  @Override
  public void processWriteJson(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("write_json expects exactly 1 argument");
      return;
    }
    if (functionCall.arguments().isEmpty()) {
      return;
    }
    WdlExpression arg = functionCall.arguments().peekFirst();
    if (hasNonStringMapKey.test(arg)) {
      addErrorMessage("write_json argument contains a map with non-string keys");
    }
  }

  @Override
  public void processContains(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 2) {
      addErrorMessage("contains expects exactly 2 arguments");
      return;
    }
    WdlType left = typeInferer.apply(functionCall.arguments().peekFirst());
    WdlExpression rightArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);
    WdlType right = typeInferer.apply(rightArg);
    if (left instanceof WdlArrayType && right != null) {
      WdlType memberType = ((WdlArrayType) left).memberType();
      if (memberType != null && !isTypeAssignable.test(memberType, right)) {
        addErrorMessage("contains argument type is incompatible with array member type");
      }
    }
  }

  @Override
  public void processContainsKey(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 2) {
      addErrorMessage("contains_key expects exactly 2 arguments");
      return;
    }
    WdlType mapType = typeInferer.apply(functionCall.arguments().peekFirst());
    WdlExpression keyArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);
    WdlType keyType = typeInferer.apply(keyArg);
    if (mapType instanceof WdlMapType && keyType != null) {
      WdlType expectedKeyType = ((WdlMapType) mapType).keyType();
      if (expectedKeyType != null && !isTypeAssignable.test(expectedKeyType, keyType)) {
        addErrorMessage("contains_key key type is incompatible with map key type");
      }
    }
  }

  @Override
  public void processLength(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("length expects exactly 1 argument");
      return;
    }
    WdlType argType = typeInferer.apply(functionCall.arguments().peekFirst());
    if (argType != null && !(argType instanceof WdlArrayType) && !(argType instanceof WdlMapType)) {
      if (!(argType instanceof WdlPrimitiveType
          && ((WdlPrimitiveType) argType).primitiveType() == WdlPrimitiveType.Type.STRING)) {
        addErrorMessage("length expects String, Array, or Map");
      }
    }
  }

  protected final void addErrorMessage(String message) {
    addError.accept(WdlSemanticError.Code.INVALID_FUNCTION_ARGUMENTS, message);
  }

  protected final void addErrorMessage(WdlSemanticError.Code code, String message) {
    addError.accept(
        code == null ? WdlSemanticError.Code.INVALID_FUNCTION_ARGUMENTS : code, message);
  }
}
