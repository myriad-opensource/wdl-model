package com.myriad.wdl.model.processors;

import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlBooleanLiteral;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFloatLiteral;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlIndexAccessOperation;
import com.myriad.wdl.model.expressions.WdlIntLiteral;
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
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholderOption;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringText;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringToken;
import com.myriad.wdl.model.expressions.WdlStructLiteral;
import com.myriad.wdl.model.expressions.WdlStructLiteral.WdlStructEntry;
import com.myriad.wdl.model.expressions.WdlTernaryOperation;
import com.myriad.wdl.model.expressions.WdlUnaryOperation;
import com.myriad.wdl.model.expressions.WdlVariable;

/**
 * Processor contract for walking expression trees.
 *
 * <p>The traversal strategy is intentionally unspecified so implementations can do depth-first,
 * breadth-first, or custom traversals.
 *
 * <p>{@link #dispatchByComponentType(WdlExpression)} and
 * {@link #dispatchStringComponentByType(WdlStringLiteral, WdlStringComponent)} are provided as
 * reusable component-type dispatch helpers to avoid {@code instanceof} chains in implementers.
 */
public interface WdlExpressionProcessor {
  /**
   * Process an expression tree rooted at {@code expression}.
   *
   * <p>Implementers own traversal order and recursion/queue policy.
   */
  void processExpression(WdlExpression expression);

  /** Dispatch helper for expression nodes based on {@code componentType()}. */
  default void dispatchByComponentType(WdlExpression expression) {
    if (expression == null) {
      return;
    }

    switch (expression.componentType()) {
      case BOOL_LIT:
        onBooleanLiteral((WdlBooleanLiteral) expression);
        break;
      case FLOAT_LIT:
        onFloatLiteral((WdlFloatLiteral) expression);
        break;
      case INT_LIT:
        onIntLiteral((WdlIntLiteral) expression);
        break;
      case ARRAY_LIT:
        onArrayLiteral((WdlArrayLiteral) expression);
        break;
      case MAP_LIT:
        onMapLiteral((WdlMapLiteral) expression);
        break;
      case NULL_LIT:
        onNullLiteral((WdlNullLiteral) expression);
        break;
      case OBJ_LIT:
        onObjectLiteral((WdlObjectLiteral) expression);
        break;
      case PAIR_LIT:
        onPairLiteral((WdlPairLiteral) expression);
        break;
      case STR_LIT:
        onStringLiteral((WdlStringLiteral) expression);
        break;
      case STRUCT_LIT:
        onStructLiteral((WdlStructLiteral) expression);
        break;
      case VARIABLE:
        onVariable((WdlVariable) expression);
        break;
      case BINARY_OP:
        onBinaryOperation((WdlBinaryOperation) expression);
        break;
      case FUNC_OP:
        onFunctionCallOperation((WdlFunctionCallOperation) expression);
        break;
      case IDX_OP:
        onIndexAccessOperation((WdlIndexAccessOperation) expression);
        break;
      case MEMBER_OP:
        onMemberAccessOperation((WdlMemberAccessOperation) expression);
        break;
      case TERNARY_OP:
        onTernaryOperation((WdlTernaryOperation) expression);
        break;
      case UNARY_OP:
        onUnaryOperation((WdlUnaryOperation) expression);
        break;
      default:
        throw new IllegalStateException(
            "Unhandled expression component type: " + expression.componentType());
    }
  }

  /** Dispatch helper for string components based on {@code componentType()}. */
  default void dispatchStringComponentByType(
      WdlStringLiteral context, WdlStringComponent component) {
    if (component == null) {
      return;
    }

    switch (component.componentType()) {
      case TEXT:
        onStringText(context, (WdlStringText) component);
        break;
      case ESC:
        onStringEscape(context, (WdlStringEscape) component);
        break;
      case SPECIAL:
        onStringToken(context, (WdlStringToken) component);
        break;
      case PLACEHOLDER:
        onStringPlaceholder(context, (WdlStringPlaceholder) component);
        break;
      default:
        throw new IllegalStateException(
            "Unhandled string component type: " + component.componentType());
    }
  }

  default void onEnterExpression(WdlExpression expression) {}

  default void onExitExpression(WdlExpression expression) {}

  default void onEnterStringComponent(WdlStringLiteral context, WdlStringComponent component) {}

  default void onExitStringComponent(WdlStringLiteral context, WdlStringComponent component) {}

  default void onBooleanLiteral(WdlBooleanLiteral expression) {}

  default void onFloatLiteral(WdlFloatLiteral expression) {}

  default void onIntLiteral(WdlIntLiteral expression) {}

  default void onNullLiteral(WdlNullLiteral expression) {}

  default void onVariable(WdlVariable expression) {}

  default void onArrayLiteral(WdlArrayLiteral expression) {}

  default void onMapLiteral(WdlMapLiteral expression) {}

  default void onMapEntry(WdlMapLiteral context, WdlMapEntry entry) {}

  default void onObjectLiteral(WdlObjectLiteral expression) {}

  default void onObjectEntry(WdlObjectLiteral context, WdlObjectEntry entry) {}

  default void onPairLiteral(WdlPairLiteral expression) {}

  default void onStringLiteral(WdlStringLiteral expression) {}

  default void onStringText(WdlStringLiteral context, WdlStringText text) {}

  default void onStringEscape(WdlStringLiteral context, WdlStringEscape escape) {}

  default void onStringToken(WdlStringLiteral context, WdlStringToken token) {}

  default void onStringPlaceholder(WdlStringLiteral context, WdlStringPlaceholder placeholder) {}

  default void onStringPlaceholderOption(
      WdlStringLiteral context,
      WdlStringPlaceholder placeholder,
      WdlStringPlaceholderOption option) {}

  default void onStructLiteral(WdlStructLiteral expression) {}

  default void onStructEntry(WdlStructLiteral context, WdlStructEntry entry) {}

  default void onBinaryOperation(WdlBinaryOperation expression) {}

  default void onFunctionCallOperation(WdlFunctionCallOperation expression) {}

  default void onIndexAccessOperation(WdlIndexAccessOperation expression) {}

  default void onMemberAccessOperation(WdlMemberAccessOperation expression) {}

  default void onTernaryOperation(WdlTernaryOperation expression) {}

  default void onUnaryOperation(WdlUnaryOperation expression) {}
}
