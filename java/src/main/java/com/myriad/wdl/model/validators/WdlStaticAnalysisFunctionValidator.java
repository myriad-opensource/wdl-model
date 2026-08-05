package com.myriad.wdl.model.validators;

import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.types.WdlArrayType;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import java.util.function.BiConsumer;
import java.util.function.BiPredicate;
import java.util.function.Function;
import java.util.function.Predicate;

/**
 * Extended static-only function validator with broader signature checks.
 *
 * <p>This class only reports definite errors; unknown or uninferable types are tolerated so that
 * the validator stays deterministic. It is responsible for the stricter function-call rules that
 * go beyond the baseline semantic layer, such as the additional checks demonstrated by the
 * synthetic static-analysis tests for {@code keys}, {@code range}, {@code contains_key},
 * {@code size}, {@code basename}, {@code chunk}, {@code cross}, and {@code join_paths}.
 */
public class WdlStaticAnalysisFunctionValidator extends WdlFunctionValidator {

  private final Function<WdlExpression, WdlType> typeInferer;

  public WdlStaticAnalysisFunctionValidator(
      Function<WdlExpression, Object> evaluator,
      Predicate<Object> isUnknownValue,
      Predicate<WdlExpression> hasNonStringMapKey,
      Function<WdlExpression, WdlType> typeInferer,
      BiPredicate<WdlType, WdlType> isTypeAssignable,
      BiConsumer<WdlSemanticError.Code, String> addError) {
    super(evaluator, isUnknownValue, hasNonStringMapKey, typeInferer, isTypeAssignable, addError);
    this.typeInferer = typeInferer;
  }

  @Override
  public void processKeys(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("keys expects exactly 1 argument");
      return;
    }
    WdlType argType = typeInferer.apply(functionCall.arguments().peekFirst());
    if (argType != null && !(argType instanceof WdlMapType)) {
      addErrorMessage("keys expects a map argument");
    }
  }

  @Override
  public void processValues(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("values expects exactly 1 argument");
      return;
    }
    WdlType argType = typeInferer.apply(functionCall.arguments().peekFirst());
    if (argType != null && !(argType instanceof WdlMapType)) {
      addErrorMessage("values expects a map argument");
    }
  }

  @Override
  public void processRange(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("range expects exactly 1 argument");
      return;
    }
    WdlType argType = typeInferer.apply(functionCall.arguments().peekFirst());
    if (argType != null
        && !(argType instanceof WdlPrimitiveType
            && ((WdlPrimitiveType) argType).primitiveType() == WdlPrimitiveType.Type.INT)) {
      addErrorMessage("range expects an Int argument");
    }
  }

  @Override
  public void processSelectAll(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage("select_all expects exactly 1 argument");
      return;
    }
    WdlType argType = typeInferer.apply(functionCall.arguments().peekFirst());
    if (argType != null && !(argType instanceof WdlArrayType)) {
      addErrorMessage("select_all expects an array argument");
    }
  }

  @Override
  public void processContains(WdlFunctionCallOperation functionCall) {
    super.processContains(functionCall);
    if (functionCall.arguments().size() != 2) {
      return;
    }
    WdlType left = typeInferer.apply(functionCall.arguments().peekFirst());
    WdlExpression rightArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);
    WdlType right = typeInferer.apply(rightArg);
    if (left != null
        && isPrimitive(left, WdlPrimitiveType.Type.STRING)
        && right != null
        && !isPrimitive(right, WdlPrimitiveType.Type.STRING)) {
      addErrorMessage("contains second argument must be String when first argument is String");
    }
  }

  @Override
  public void processChunk(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 2) {
      addErrorMessage("chunk expects exactly 2 arguments");
      return;
    }
    WdlType first = typeInferer.apply(functionCall.arguments().peekFirst());
    WdlExpression secondArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);
    WdlType second = typeInferer.apply(secondArg);

    if (first != null && !(first instanceof WdlArrayType)) {
      addErrorMessage("chunk first argument must be Array");
    }
    if (second != null && !isPrimitive(second, WdlPrimitiveType.Type.INT)) {
      addErrorMessage("chunk second argument must be Int");
    }
  }

  @Override
  public void processCross(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 2) {
      addErrorMessage("cross expects exactly 2 arguments");
      return;
    }
    WdlType left = typeInferer.apply(functionCall.arguments().peekFirst());
    WdlExpression rightArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);
    WdlType right = typeInferer.apply(rightArg);

    if ((left != null && !(left instanceof WdlArrayType))
        || (right != null && !(right instanceof WdlArrayType))) {
      addErrorMessage("cross expects two array arguments");
    }
  }

  @Override
  public void processJoinPaths(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() < 2) {
      addErrorMessage("join_paths expects at least 2 arguments");
      return;
    }

    WdlType first = typeInferer.apply(functionCall.arguments().peekFirst());
    if (first != null && !isPathLikeType(first)) {
      addErrorMessage("join_paths first argument must be String/File/Directory");
    }

    int index = 0;
    for (WdlExpression arg : functionCall.arguments()) {
      if (index == 0) {
        index++;
        continue;
      }
      WdlType argType = typeInferer.apply(arg);
      if (argType != null && !isPrimitive(argType, WdlPrimitiveType.Type.STRING)) {
        addErrorMessage("join_paths arguments after the first must be String");
        break;
      }
      index++;
    }
  }

  @Override
  public void processContainsKey(WdlFunctionCallOperation functionCall) {
    super.processContainsKey(functionCall);
    if (functionCall.arguments().size() != 2) {
      return;
    }
    WdlType mapType = typeInferer.apply(functionCall.arguments().peekFirst());
    if (mapType != null && !(mapType instanceof WdlMapType)) {
      addErrorMessage("contains_key first argument must be a Map");
    }
  }

  @Override
  public void processSize(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() < 1 || functionCall.arguments().size() > 2) {
      addErrorMessage("size expects 1 or 2 arguments");
      return;
    }

    WdlType first = typeInferer.apply(functionCall.arguments().peekFirst());
    if (first != null && !isPathLikeType(first)) {
      addErrorMessage("size first argument must be String/File/Directory");
    }

    if (functionCall.arguments().size() == 2) {
      WdlExpression secondArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);
      WdlType second = typeInferer.apply(secondArg);
      if (second != null && !isPrimitive(second, WdlPrimitiveType.Type.STRING)) {
        addErrorMessage("size second argument must be String");
      }
    }
  }

  @Override
  public void processBasename(WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() < 1 || functionCall.arguments().size() > 2) {
      addErrorMessage("basename expects 1 or 2 arguments");
      return;
    }

    WdlType first = typeInferer.apply(functionCall.arguments().peekFirst());
    if (first != null && !isPathLikeType(first)) {
      addErrorMessage("basename first argument must be String/File/Directory");
    }

    if (functionCall.arguments().size() == 2) {
      WdlExpression secondArg = functionCall.arguments().stream().skip(1).findFirst().orElse(null);
      WdlType second = typeInferer.apply(secondArg);
      if (second != null && !isPrimitive(second, WdlPrimitiveType.Type.STRING)) {
        addErrorMessage("basename second argument must be String");
      }
    }
  }

  @Override
  public void processReadInt(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_int", functionCall);
  }

  @Override
  public void processReadFloat(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_float", functionCall);
  }

  @Override
  public void processReadString(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_string", functionCall);
  }

  @Override
  public void processReadBoolean(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_boolean", functionCall);
  }

  @Override
  public void processReadLines(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_lines", functionCall);
  }

  @Override
  public void processReadTsv(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_tsv", functionCall);
  }

  @Override
  public void processReadMap(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_map", functionCall);
  }

  @Override
  public void processReadObject(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_object", functionCall);
  }

  @Override
  public void processReadObjects(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_objects", functionCall);
  }

  @Override
  public void processReadJson(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("read_json", functionCall);
  }

  @Override
  public void processGlob(WdlFunctionCallOperation functionCall) {
    validateSinglePathLikeArg("glob", functionCall);
  }

  private void validateSinglePathLikeArg(String name, WdlFunctionCallOperation functionCall) {
    if (functionCall.arguments().size() != 1) {
      addErrorMessage(name + " expects exactly 1 argument");
      return;
    }

    WdlType argType = typeInferer.apply(functionCall.arguments().peekFirst());
    if (argType != null && !isPathLikeType(argType)) {
      addErrorMessage(name + " expects a String/File/Directory argument");
    }
  }

  private boolean isPathLikeType(WdlType type) {
    if (!(type instanceof WdlPrimitiveType)) {
      return false;
    }
    WdlPrimitiveType.Type primitive = ((WdlPrimitiveType) type).primitiveType();
    return primitive == WdlPrimitiveType.Type.STRING
        || primitive == WdlPrimitiveType.Type.FILE
        || primitive == WdlPrimitiveType.Type.DIRECTORY;
  }

  private boolean isPrimitive(WdlType type, WdlPrimitiveType.Type primitiveType) {
    return type instanceof WdlPrimitiveType
        && ((WdlPrimitiveType) type).primitiveType() == primitiveType;
  }
}
