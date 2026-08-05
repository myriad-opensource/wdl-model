package com.myriad.wdl.model.validators;

import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlBooleanLiteral;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFloatLiteral;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlIndexAccessOperation;
import com.myriad.wdl.model.expressions.WdlIntLiteral;
import com.myriad.wdl.model.expressions.WdlLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral.WdlMapEntry;
import com.myriad.wdl.model.expressions.WdlMemberAccessOperation;
import com.myriad.wdl.model.expressions.WdlNullLiteral;
import com.myriad.wdl.model.expressions.WdlObjectLiteral;
import com.myriad.wdl.model.expressions.WdlObjectLiteral.WdlObjectEntry;
import com.myriad.wdl.model.expressions.WdlPairLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringComponent;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringEscape;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringText;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringToken;
import com.myriad.wdl.model.expressions.WdlStructLiteral;
import com.myriad.wdl.model.expressions.WdlStructLiteral.WdlStructEntry;
import com.myriad.wdl.model.expressions.WdlTernaryOperation;
import com.myriad.wdl.model.expressions.WdlUnaryOperation;
import com.myriad.wdl.model.expressions.WdlVariable;
import com.myriad.wdl.model.processors.WdlExpressionProcessorBase;
import com.myriad.wdl.model.types.WdlArrayType;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPairType;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import com.myriad.wdl.model.types.WdlTypeReferenceType;
import java.util.AbstractMap;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Objects;
import java.util.Set;
import java.util.function.BiConsumer;
import java.util.function.BiPredicate;

/**
 * Expression-level semantic checks and lightweight constant evaluation.
 *
 * <p>Traversal is based on {@link WdlExpressionProcessorBase} and function checks are delegated to
 * {@link WdlFunctionValidator}. This validator is what turns examples such as
 * {@code spec_examples/v1_3/select_first_empty_fail.wdl},
 * {@code spec_examples/v1_3/test_zip_fail.wdl}, and
 * {@code spec_examples/v1_3/write_json_fail.wdl} into deterministic semantic diagnostics.
 */
public class WdlExpressionValidator extends WdlExpressionProcessorBase {

  private static final Object UNKNOWN = new Object();

  private final Map<String, WdlType> scopeTypes;
  private final Map<String, Object> scopeValues;
  private final Map<String, Set<String>> callOutputs;
  private final Map<String, Map<String, WdlType>> callOutputTypes;
  private final Map<String, Set<String>> structMembers;
  private final Map<String, Map<String, WdlType>> structMemberTypes;
  private final WdlVersion documentVersion;
  private final BiConsumer<WdlSemanticError.Code, String> addError;
  private final WdlFunctionValidator functionValidator;

  public WdlExpressionValidator(
      Map<String, WdlType> scopeTypes,
      Map<String, Object> scopeValues,
      Map<String, Set<String>> callOutputs,
      Map<String, Map<String, WdlType>> callOutputTypes,
      Map<String, Set<String>> structMembers,
      Map<String, Map<String, WdlType>> structMemberTypes,
      WdlVersion documentVersion,
      BiConsumer<WdlSemanticError.Code, String> addError) {
    this.scopeTypes = scopeTypes;
    this.scopeValues = scopeValues;
    this.callOutputs = callOutputs;
    this.callOutputTypes = callOutputTypes;
    this.structMembers = structMembers;
    this.structMemberTypes = structMemberTypes;
    this.documentVersion = documentVersion;
    this.addError = addError;
    this.functionValidator =
        createFunctionValidator(
            this::evaluate,
            this::isUnknownValue,
            this::containsNonStringMapKey,
            this::inferType,
            this::isTypeAssignable,
            addError);
  }

  protected WdlFunctionValidator createFunctionValidator(
      java.util.function.Function<WdlExpression, Object> evaluator,
      java.util.function.Predicate<Object> isUnknownValue,
      java.util.function.Predicate<WdlExpression> hasNonStringMapKey,
      java.util.function.Function<WdlExpression, WdlType> typeInferer,
      BiPredicate<WdlType, WdlType> isTypeAssignable,
      BiConsumer<WdlSemanticError.Code, String> addError) {
    return new WdlFunctionValidator(
        evaluator, isUnknownValue, hasNonStringMapKey, typeInferer, isTypeAssignable, addError);
  }

  public void validate(WdlExpression expression) {
    processExpression(expression);
  }

  /**
   * Performs lightweight constant evaluation used by semantic checks such as index bounds,
   * duplicate key detection, and write-time JSON map validation.
   */
  public Object evaluate(WdlExpression expression) {
    if (expression == null) {
      return UNKNOWN;
    }

    if (expression instanceof WdlNullLiteral) {
      return null;
    }

    if (expression instanceof WdlLiteral<?>) {
      return ((WdlLiteral<?>) expression).getValue();
    }

    if (expression instanceof WdlStringLiteral) {
      StringBuilder out = new StringBuilder();
      for (WdlStringComponent component : ((WdlStringLiteral) expression).components()) {
        if (component instanceof WdlStringText) {
          out.append(Objects.toString(((WdlStringText) component).getText(), ""));
        } else if (component instanceof WdlStringEscape) {
          out.append(Objects.toString(((WdlStringEscape) component).getEscapeText(), ""));
        } else if (component instanceof WdlStringToken) {
          out.append(Objects.toString(((WdlStringToken) component).getTokenText(), ""));
        } else {
          return UNKNOWN;
        }
      }
      return out.toString();
    }

    if (expression instanceof WdlVariable) {
      String name = ((WdlVariable) expression).getName();
      if ("None".equals(name)) {
        return null;
      }
      return scopeValues.getOrDefault(name, UNKNOWN);
    }

    if (expression instanceof WdlArrayLiteral) {
      List<Object> out = new ArrayList<>();
      for (WdlExpression item : ((WdlArrayLiteral) expression).entries()) {
        out.add(evaluate(item));
      }
      return out;
    }

    if (expression instanceof WdlPairLiteral) {
      Object left = evaluate(((WdlPairLiteral) expression).getLeft());
      Object right = evaluate(((WdlPairLiteral) expression).getRight());
      if (left == UNKNOWN || right == UNKNOWN) {
        return UNKNOWN;
      }
      return new AbstractMap.SimpleEntry<>(left, right);
    }

    if (expression instanceof WdlMapLiteral) {
      Map<Object, Object> out = new HashMap<>();
      for (WdlMapEntry entry : ((WdlMapLiteral) expression).entries()) {
        Object key = evaluate(entry.getKey());
        Object value = evaluate(entry.getValue());
        if (key == UNKNOWN) {
          return UNKNOWN;
        }
        out.put(key, value);
      }
      return out;
    }

    return UNKNOWN;
  }

  public boolean isUnknownValue(Object value) {
    return value == UNKNOWN;
  }

  public boolean isAssignableFrom(WdlType expectedType, WdlExpression expression) {
    if (expectedType == null || expression == null) {
      return true;
    }
    if (expression instanceof WdlNullLiteral) {
      return expectedType.isOptional();
    }
    if (isExplicitNoneExpression(expression)) {
      return expectedType.isOptional();
    }
    if (expectedType instanceof WdlArrayType && expression instanceof WdlArrayLiteral) {
      WdlType memberType = ((WdlArrayType) expectedType).memberType();
      for (WdlExpression item : ((WdlArrayLiteral) expression).entries()) {
        if (!isAssignableFrom(memberType, item)) {
          return false;
        }
      }
      return true;
    }
    if (expectedType instanceof WdlMapType && expression instanceof WdlMapLiteral) {
      WdlType keyType = ((WdlMapType) expectedType).keyType();
      WdlType valueType = ((WdlMapType) expectedType).valueType();
      for (WdlMapEntry entry : ((WdlMapLiteral) expression).entries()) {
        if (!isAssignableFrom(keyType, entry.getKey())
            || !isAssignableFrom(valueType, entry.getValue())) {
          return false;
        }
      }
      return true;
    }
    if (expectedType instanceof WdlPairType && expression instanceof WdlPairLiteral) {
      WdlPairType pairType = (WdlPairType) expectedType;
      WdlPairLiteral pairLiteral = (WdlPairLiteral) expression;
      return isAssignableFrom(pairType.leftType(), pairLiteral.getLeft())
          && isAssignableFrom(pairType.rightType(), pairLiteral.getRight());
    }
    WdlType actualType = inferType(expression);
    if (actualType == null) {
      return true;
    }
    return isTypeAssignable(expectedType, actualType);
  }

  private boolean isExplicitNoneExpression(WdlExpression expression) {
    return expression instanceof WdlVariable && "None".equals(((WdlVariable) expression).getName());
  }

  public WdlType inferType(WdlExpression expression) {
    if (expression == null) {
      return null;
    }

    if (expression instanceof WdlIntLiteral) {
      return primitive(WdlPrimitiveType.Type.INT);
    }
    if (expression instanceof WdlFloatLiteral) {
      return primitive(WdlPrimitiveType.Type.FLOAT);
    }
    if (expression instanceof WdlBooleanLiteral) {
      return primitive(WdlPrimitiveType.Type.BOOLEAN);
    }
    if (expression instanceof WdlStringLiteral) {
      return primitive(WdlPrimitiveType.Type.STRING);
    }
    if (expression instanceof WdlNullLiteral) {
      return null;
    }

    if (expression instanceof WdlVariable) {
      String name = ((WdlVariable) expression).getName();
      if ("None".equals(name)) {
        return null;
      }
      return scopeTypes.get(name);
    }

    if (expression instanceof WdlArrayLiteral) {
      WdlType memberType = null;
      for (WdlExpression item : ((WdlArrayLiteral) expression).entries()) {
        WdlType itemType = inferType(item);
        if (itemType == null) {
          continue;
        }
        memberType = mergeTypes(memberType, itemType);
        if (memberType == null) {
          break;
        }
      }
      return new WdlArrayType(memberType, false, false);
    }

    if (expression instanceof WdlPairLiteral) {
      WdlType left = inferType(((WdlPairLiteral) expression).getLeft());
      WdlType right = inferType(((WdlPairLiteral) expression).getRight());
      if (left == null || right == null) {
        return null;
      }
      return new WdlPairType(left, right, false);
    }

    if (expression instanceof WdlMapLiteral) {
      WdlType keyType = null;
      WdlType valueType = null;
      for (WdlMapEntry entry : ((WdlMapLiteral) expression).entries()) {
        keyType = mergeTypes(keyType, inferType(entry.getKey()));
        valueType = mergeTypes(valueType, inferType(entry.getValue()));
      }
      return new WdlMapType(keyType, valueType, false);
    }

    if (expression instanceof WdlIndexAccessOperation) {
      WdlType targetType = inferType(((WdlIndexAccessOperation) expression).getTarget());
      if (targetType instanceof WdlArrayType) {
        return ((WdlArrayType) targetType).memberType();
      }
      if (targetType instanceof WdlMapType) {
        return ((WdlMapType) targetType).valueType();
      }
      return null;
    }

    if (expression instanceof WdlMemberAccessOperation) {
      WdlMemberAccessOperation memberAccess = (WdlMemberAccessOperation) expression;
      String member = memberAccess.getMember();
      WdlExpression targetExpression = memberAccess.getTarget();
      if (targetExpression instanceof WdlVariable) {
        String targetName = ((WdlVariable) targetExpression).getName();
        Map<String, WdlType> outputTypes = callOutputTypes.get(targetName);
        if (outputTypes != null) {
          return outputTypes.get(member);
        }
        WdlType targetType = scopeTypes.get(targetName);
        if (targetType instanceof WdlTypeReferenceType) {
          String structName = ((WdlTypeReferenceType) targetType).referenceName();
          Map<String, WdlType> memberTypes = structMemberTypes.get(structName);
          if (memberTypes != null) {
            return memberTypes.get(member);
          }
        }
      }
      return null;
    }

    if (expression instanceof WdlFunctionCallOperation) {
      return inferFunctionType((WdlFunctionCallOperation) expression);
    }

    if (expression instanceof WdlUnaryOperation) {
      WdlUnaryOperation unary = (WdlUnaryOperation) expression;
      if (unary.getOperator() == WdlUnaryOperation.Operator.NOT) {
        return primitive(WdlPrimitiveType.Type.BOOLEAN);
      }
      if (unary.getOperator() == WdlUnaryOperation.Operator.NEGATIVE) {
        return inferType(unary.getOperand());
      }
    }

    if (expression instanceof WdlBinaryOperation) {
      WdlBinaryOperation binary = (WdlBinaryOperation) expression;
      switch (binary.getOperator()) {
        case OR:
        case AND:
        case EQ:
        case NEQ:
        case LT:
        case LTE:
        case GT:
        case GTE:
          return primitive(WdlPrimitiveType.Type.BOOLEAN);
        case ADD:
        case SUTRACT:
        case MULTIPLY:
        case DIVIDE:
        case MODULO:
        case POWER:
          WdlType left = inferType(binary.getLeft());
          WdlType right = inferType(binary.getRight());
          if (isPrimitive(left, WdlPrimitiveType.Type.FLOAT)
              || isPrimitive(right, WdlPrimitiveType.Type.FLOAT)) {
            return primitive(WdlPrimitiveType.Type.FLOAT);
          }
          if (isPrimitive(left, WdlPrimitiveType.Type.INT)
              && isPrimitive(right, WdlPrimitiveType.Type.INT)) {
            return primitive(WdlPrimitiveType.Type.INT);
          }
          if (binary.getOperator() == WdlBinaryOperation.Operator.ADD
              && (isPrimitive(left, WdlPrimitiveType.Type.STRING)
                  || isPrimitive(right, WdlPrimitiveType.Type.STRING))) {
            return primitive(WdlPrimitiveType.Type.STRING);
          }
          return null;
        default:
          return null;
      }
    }

    if (expression instanceof WdlTernaryOperation) {
      WdlType trueType = inferType(((WdlTernaryOperation) expression).getTrueValue());
      WdlType falseType = inferType(((WdlTernaryOperation) expression).getFalseValue());
      return mergeTypes(trueType, falseType);
    }

    return null;
  }

  public boolean containsNonStringMapKey(WdlExpression expression) {
    if (expression == null) {
      return false;
    }

    if (expression instanceof WdlMapLiteral) {
      for (WdlMapEntry entry : ((WdlMapLiteral) expression).entries()) {
        Object key = evaluate(entry.getKey());
        if (!(key instanceof String)) {
          return true;
        }
        if (containsNonStringMapKey(entry.getValue())) {
          return true;
        }
      }
      return false;
    }

    if (expression instanceof WdlArrayLiteral) {
      for (WdlExpression item : ((WdlArrayLiteral) expression).entries()) {
        if (containsNonStringMapKey(item)) {
          return true;
        }
      }
      return false;
    }

    if (expression instanceof WdlPairLiteral) {
      return containsNonStringMapKey(((WdlPairLiteral) expression).getLeft())
          || containsNonStringMapKey(((WdlPairLiteral) expression).getRight());
    }

    if (expression instanceof WdlObjectLiteral) {
      for (WdlObjectEntry entry : ((WdlObjectLiteral) expression).entries()) {
        if (containsNonStringMapKey(entry.getValue())) {
          return true;
        }
      }
      return false;
    }

    if (expression instanceof WdlStructLiteral) {
      for (WdlStructEntry entry : ((WdlStructLiteral) expression).entries()) {
        if (containsNonStringMapKey(entry.getValue())) {
          return true;
        }
      }
      return false;
    }

    if (expression instanceof WdlVariable) {
      Object value = scopeValues.get(((WdlVariable) expression).getName());
      return containsNonStringMapKeyInValue(value);
    }

    return false;
  }

  @Override
  protected void processFunctionCallOperation(WdlFunctionCallOperation expression) {
    validateFunctionVersionAvailability(expression);
    functionValidator.processFunctionCall(expression);
    super.processFunctionCallOperation(expression);
  }

  private void validateFunctionVersionAvailability(WdlFunctionCallOperation expression) {
    if (expression == null || expression.getFunction() == null || documentVersion == null) {
      return;
    }

    WdlFunctionCallOperation.WdlFunction function = expression.getFunction();
    if (function == WdlFunctionCallOperation.WdlFunction.NONSTANDARD) {
      return;
    }

    WdlVersion addedIn = function.getAddedIn();
    if (addedIn != null && documentVersion.ordinal() < addedIn.ordinal()) {
      addError.accept(
          WdlSemanticError.Code.FUNCTION_NOT_AVAILABLE_IN_VERSION,
          "Function '"
              + function.toWdlString()
              + "' is not available in WDL "
              + documentVersion.getVersionString()
              + " (added in "
              + addedIn.getVersionString()
              + ")");
      return;
    }

    WdlVersion removedIn = function.getRemovedIn();
    if (removedIn != null && documentVersion.ordinal() >= removedIn.ordinal()) {
      addError.accept(
          WdlSemanticError.Code.FUNCTION_NOT_AVAILABLE_IN_VERSION,
          "Function '"
              + function.toWdlString()
              + "' is not available in WDL "
              + documentVersion.getVersionString()
              + " (removed in "
              + removedIn.getVersionString()
              + ")");
    }
  }

  @Override
  protected void processIndexAccessOperation(WdlIndexAccessOperation expression) {
    super.processIndexAccessOperation(expression);

    Object target = evaluate(expression.getTarget());
    Object index = evaluate(expression.getIndex());

    if (target instanceof List && index instanceof Number) {
      int i = ((Number) index).intValue();
      int size = ((List<?>) target).size();
      if (i < 0 || i >= size) {
        addError.accept(WdlSemanticError.Code.UNKNOWN_REFERENCE, "Array index out of bounds");
      }
    } else if (target instanceof Map && index != UNKNOWN) {
      if (!((Map<?, ?>) target).containsKey(index)) {
        addError.accept(
            WdlSemanticError.Code.UNKNOWN_REFERENCE, "Map key does not exist: " + index);
      }
    }
  }

  @Override
  protected void processMemberAccessOperation(WdlMemberAccessOperation expression) {
    super.processMemberAccessOperation(expression);

    if (!(expression.getTarget() instanceof WdlVariable)) {
      return;
    }

    String target = ((WdlVariable) expression.getTarget()).getName();
    String member = expression.getMember() == null ? "" : expression.getMember();

    if (target != null && callOutputs.containsKey(target)) {
      if (!callOutputs.get(target).contains(member)) {
        addError.accept(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "'" + member + "' is not an output field of call '" + target + "'");
      }
      return;
    }

    if (target != null && scopeTypes.get(target) instanceof WdlTypeReferenceType) {
      String structName = ((WdlTypeReferenceType) scopeTypes.get(target)).referenceName();
      Set<String> members = structMembers.get(structName);
      if (members != null && !members.contains(member)) {
        addError.accept(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "Field '" + member + "' does not exist in struct '" + structName + "'");
      }
    }
  }

  protected WdlType inferFunctionType(WdlFunctionCallOperation functionCall) {
    if (functionCall == null) {
      return null;
    }
    switch (functionCall.getFunction()) {
      case DEFINED:
      case CONTAINS:
      case CONTAINS_KEY:
      case MATCHES:
        return primitive(WdlPrimitiveType.Type.BOOLEAN);
      case LENGTH:
        return primitive(WdlPrimitiveType.Type.INT);
      case READ_INT:
        return primitive(WdlPrimitiveType.Type.INT);
      case READ_FLOAT:
        return primitive(WdlPrimitiveType.Type.FLOAT);
      case READ_STRING:
      case STDOUT:
      case STDERR:
      case WRITE_LINES:
      case WRITE_TSV:
      case WRITE_MAP:
      case WRITE_OBJECT:
      case WRITE_OBJECTS:
      case WRITE_JSON:
      case BASENAME:
      case PREFIX:
      case SUFFIX:
      case QUOTE:
      case SQUOTE:
      case SEP:
        return primitive(WdlPrimitiveType.Type.STRING);
      case READ_BOOLEAN:
        return primitive(WdlPrimitiveType.Type.BOOLEAN);
      case READ_LINES:
      case GLOB:
        return new WdlArrayType(primitive(WdlPrimitiveType.Type.STRING), false, false);
      case RANGE:
        return new WdlArrayType(primitive(WdlPrimitiveType.Type.INT), false, false);
      case SELECT_FIRST:
        if (!functionCall.arguments().isEmpty()) {
          WdlType argType = inferType(functionCall.arguments().peekFirst());
          if (argType instanceof WdlArrayType) {
            return ((WdlArrayType) argType).memberType();
          }
        }
        return null;
      case ZIP:
        if (functionCall.arguments().size() >= 2) {
          WdlExpression leftArg = functionCall.arguments().peekFirst();
          WdlExpression rightArg =
              functionCall.arguments().stream().skip(1).findFirst().orElse(null);
          WdlType leftType = inferType(leftArg);
          WdlType rightType = inferType(rightArg);
          if (leftType instanceof WdlArrayType && rightType instanceof WdlArrayType) {
            WdlType leftMember = ((WdlArrayType) leftType).memberType();
            WdlType rightMember = ((WdlArrayType) rightType).memberType();
            if (leftMember != null && rightMember != null) {
              WdlPairType pairType = new WdlPairType(leftMember, rightMember, false);
              return new WdlArrayType(pairType, false, false);
            }
          }
        }
        return null;
      case AS_MAP:
        if (!functionCall.arguments().isEmpty()) {
          WdlType argType = inferType(functionCall.arguments().peekFirst());
          if (argType instanceof WdlArrayType
              && ((WdlArrayType) argType).memberType() instanceof WdlPairType) {
            WdlPairType pair = (WdlPairType) ((WdlArrayType) argType).memberType();
            return new WdlMapType(pair.leftType(), pair.rightType(), false);
          }
        }
        return null;
      case KEYS:
        if (!functionCall.arguments().isEmpty()) {
          WdlType argType = inferType(functionCall.arguments().peekFirst());
          if (argType instanceof WdlMapType) {
            return new WdlArrayType(((WdlMapType) argType).keyType(), false, false);
          }
        }
        return null;
      case VALUES:
        if (!functionCall.arguments().isEmpty()) {
          WdlType argType = inferType(functionCall.arguments().peekFirst());
          if (argType instanceof WdlMapType) {
            return new WdlArrayType(((WdlMapType) argType).valueType(), false, false);
          }
        }
        return null;
      default:
        return null;
    }
  }

  protected WdlType mergeTypes(WdlType currentType, WdlType nextType) {
    if (nextType == null) {
      return currentType;
    }
    if (currentType == null) {
      return nextType;
    }
    if (isTypeAssignable(currentType, nextType)) {
      return currentType;
    }
    if (isTypeAssignable(nextType, currentType)) {
      return nextType;
    }
    if (isPrimitive(currentType, WdlPrimitiveType.Type.INT)
        && isPrimitive(nextType, WdlPrimitiveType.Type.FLOAT)) {
      return primitive(WdlPrimitiveType.Type.FLOAT);
    }
    if (isPrimitive(currentType, WdlPrimitiveType.Type.FLOAT)
        && isPrimitive(nextType, WdlPrimitiveType.Type.INT)) {
      return primitive(WdlPrimitiveType.Type.FLOAT);
    }
    return null;
  }

  protected boolean isTypeAssignable(WdlType expected, WdlType actual) {
    if (expected == null || actual == null) {
      return true;
    }
    if (!expected.isOptional() && actual.isOptional()) {
      return false;
    }
    if (expected.componentType() != actual.componentType()) {
      if (isPrimitive(expected, WdlPrimitiveType.Type.FLOAT)
          && isPrimitive(actual, WdlPrimitiveType.Type.INT)) {
        return true;
      }
      return false;
    }

    switch (expected.componentType()) {
      case PRIMITIVE:
        return ((WdlPrimitiveType) expected).primitiveType()
            == ((WdlPrimitiveType) actual).primitiveType();
      case ARRAY:
        return isTypeAssignable(
            ((WdlArrayType) expected).memberType(), ((WdlArrayType) actual).memberType());
      case MAP:
        return isTypeAssignable(((WdlMapType) expected).keyType(), ((WdlMapType) actual).keyType())
            && isTypeAssignable(
                ((WdlMapType) expected).valueType(), ((WdlMapType) actual).valueType());
      case PAIR:
        return isTypeAssignable(
                ((WdlPairType) expected).leftType(), ((WdlPairType) actual).leftType())
            && isTypeAssignable(
                ((WdlPairType) expected).rightType(), ((WdlPairType) actual).rightType());
      case TYPEREF:
        return Objects.equals(
            ((WdlTypeReferenceType) expected).referenceName(),
            ((WdlTypeReferenceType) actual).referenceName());
      default:
        return true;
    }
  }

  protected boolean isPrimitive(WdlType type, WdlPrimitiveType.Type primitiveType) {
    return type instanceof WdlPrimitiveType
        && ((WdlPrimitiveType) type).primitiveType() == primitiveType;
  }

  protected WdlPrimitiveType primitive(WdlPrimitiveType.Type primitiveType) {
    return new WdlPrimitiveType(primitiveType, false);
  }

  protected final void addErrorMessage(String message) {
    addError.accept(WdlSemanticError.Code.GENERIC_SEMANTIC_ERROR, message);
  }

  protected final void addErrorMessage(WdlSemanticError.Code code, String message) {
    addError.accept(code == null ? WdlSemanticError.Code.GENERIC_SEMANTIC_ERROR : code, message);
  }

  @Override
  protected void processBinaryOperation(WdlBinaryOperation expression) {
    super.processBinaryOperation(expression);
  }

  @Override
  protected void processUnaryOperation(WdlUnaryOperation expression) {
    super.processUnaryOperation(expression);
  }

  @Override
  protected void processTernaryOperation(WdlTernaryOperation expression) {
    super.processTernaryOperation(expression);
  }

  private boolean containsNonStringMapKeyInValue(Object value) {
    if (value == null || value == UNKNOWN) {
      return false;
    }

    if (value instanceof Map<?, ?>) {
      for (Entry<?, ?> entry : ((Map<?, ?>) value).entrySet()) {
        if (!(entry.getKey() instanceof String)) {
          return true;
        }
        if (containsNonStringMapKeyInValue(entry.getValue())) {
          return true;
        }
      }
      return false;
    }

    if (value instanceof List<?>) {
      for (Object item : (List<?>) value) {
        if (containsNonStringMapKeyInValue(item)) {
          return true;
        }
      }
      return false;
    }

    if (value instanceof Entry<?, ?>) {
      Entry<?, ?> pair = (Entry<?, ?>) value;
      return containsNonStringMapKeyInValue(pair.getKey())
          || containsNonStringMapKeyInValue(pair.getValue());
    }

    return false;
  }
}
