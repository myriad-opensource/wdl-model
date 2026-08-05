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
 * Base depth-first expression walker.
 *
 * <p>Dispatch is done via {@code componentType()} switches rather than {@code instanceof} checks.
 */
public abstract class WdlExpressionProcessorBase implements WdlExpressionProcessor {

  @Override
  public final void processExpression(WdlExpression expression) {
    walkExpression(expression);
  }

  protected final void walkExpression(WdlExpression expression) {
    if (expression == null) {
      return;
    }

    enterExpression(expression);

    switch (expression.componentType()) {
      case BOOL_LIT:
        processBooleanLiteral((WdlBooleanLiteral) expression);
        break;
      case FLOAT_LIT:
        processFloatLiteral((WdlFloatLiteral) expression);
        break;
      case INT_LIT:
        processIntLiteral((WdlIntLiteral) expression);
        break;
      case ARRAY_LIT:
        processArrayLiteral((WdlArrayLiteral) expression);
        break;
      case MAP_LIT:
        processMapLiteral((WdlMapLiteral) expression);
        break;
      case NULL_LIT:
        processNullLiteral((WdlNullLiteral) expression);
        break;
      case OBJ_LIT:
        processObjectLiteral((WdlObjectLiteral) expression);
        break;
      case PAIR_LIT:
        processPairLiteral((WdlPairLiteral) expression);
        break;
      case STR_LIT:
        processStringLiteral((WdlStringLiteral) expression);
        break;
      case STRUCT_LIT:
        processStructLiteral((WdlStructLiteral) expression);
        break;
      case VARIABLE:
        processVariable((WdlVariable) expression);
        break;
      case BINARY_OP:
        processBinaryOperation((WdlBinaryOperation) expression);
        break;
      case FUNC_OP:
        processFunctionCallOperation((WdlFunctionCallOperation) expression);
        break;
      case IDX_OP:
        processIndexAccessOperation((WdlIndexAccessOperation) expression);
        break;
      case MEMBER_OP:
        processMemberAccessOperation((WdlMemberAccessOperation) expression);
        break;
      case TERNARY_OP:
        processTernaryOperation((WdlTernaryOperation) expression);
        break;
      case UNARY_OP:
        processUnaryOperation((WdlUnaryOperation) expression);
        break;
      default:
        throw new IllegalStateException(
            "Unhandled expression component type: " + expression.componentType());
    }

    exitExpression(expression);
  }

  protected final void walkStringComponent(WdlStringLiteral context, WdlStringComponent component) {
    if (component == null) {
      return;
    }

    enterStringComponent(context, component);

    switch (component.componentType()) {
      case TEXT:
        processStringText(context, (WdlStringText) component);
        break;
      case ESC:
        processStringEscape(context, (WdlStringEscape) component);
        break;
      case SPECIAL:
        processStringToken(context, (WdlStringToken) component);
        break;
      case PLACEHOLDER:
        processStringPlaceholder(context, (WdlStringPlaceholder) component);
        break;
      default:
        throw new IllegalStateException(
            "Unhandled string component type: " + component.componentType());
    }

    exitStringComponent(context, component);
  }

  protected void enterExpression(WdlExpression expression) {}

  protected void exitExpression(WdlExpression expression) {}

  protected void enterStringComponent(WdlStringLiteral context, WdlStringComponent component) {}

  protected void exitStringComponent(WdlStringLiteral context, WdlStringComponent component) {}

  protected void processBooleanLiteral(WdlBooleanLiteral expression) {}

  protected void processFloatLiteral(WdlFloatLiteral expression) {}

  protected void processIntLiteral(WdlIntLiteral expression) {}

  protected void processNullLiteral(WdlNullLiteral expression) {}

  protected void processVariable(WdlVariable expression) {}

  protected void processArrayLiteral(WdlArrayLiteral expression) {
    expression.entries().forEach(this::walkExpression);
  }

  protected void processMapLiteral(WdlMapLiteral expression) {
    for (WdlMapEntry entry : expression.entries()) {
      processMapEntry(expression, entry);
    }
  }

  protected void processMapEntry(WdlMapLiteral context, WdlMapEntry entry) {
    walkExpression(entry.getKey());
    walkExpression(entry.getValue());
  }

  protected void processObjectLiteral(WdlObjectLiteral expression) {
    for (WdlObjectEntry entry : expression.entries()) {
      processObjectEntry(expression, entry);
    }
  }

  protected void processObjectEntry(WdlObjectLiteral context, WdlObjectEntry entry) {
    walkExpression(entry.getValue());
  }

  protected void processPairLiteral(WdlPairLiteral expression) {
    walkExpression(expression.getLeft());
    walkExpression(expression.getRight());
  }

  protected void processStringLiteral(WdlStringLiteral expression) {
    for (WdlStringComponent component : expression.components()) {
      walkStringComponent(expression, component);
    }
  }

  protected void processStringText(WdlStringLiteral context, WdlStringText text) {}

  protected void processStringEscape(WdlStringLiteral context, WdlStringEscape escape) {}

  protected void processStringToken(WdlStringLiteral context, WdlStringToken token) {}

  protected void processStringPlaceholder(
      WdlStringLiteral context, WdlStringPlaceholder placeholder) {
    processStringPlaceholderOption(context, placeholder, placeholder.getOption());
    walkExpression(placeholder.getExpression());
  }

  protected void processStringPlaceholderOption(
      WdlStringLiteral context,
      WdlStringPlaceholder placeholder,
      WdlStringPlaceholderOption option) {
    if (option == null) {
      return;
    }

    switch (option.getType()) {
      case SEP:
      case DEFAULT:
        walkExpression(option.getValue());
        break;
      case TRUE_FALSE:
      case FALSE_TRUE:
        walkExpression(option.getTrueValue());
        walkExpression(option.getFalseValue());
        break;
      default:
        throw new IllegalStateException("Unhandled placeholder option type: " + option.getType());
    }
  }

  protected void processStructLiteral(WdlStructLiteral expression) {
    for (WdlStructEntry entry : expression.entries()) {
      processStructEntry(expression, entry);
    }
  }

  protected void processStructEntry(WdlStructLiteral context, WdlStructEntry entry) {
    walkExpression(entry.getValue());
  }

  protected void processBinaryOperation(WdlBinaryOperation expression) {
    walkExpression(expression.getLeft());
    walkExpression(expression.getRight());
  }

  protected void processFunctionCallOperation(WdlFunctionCallOperation expression) {
    expression.arguments().forEach(this::walkExpression);
  }

  protected void processIndexAccessOperation(WdlIndexAccessOperation expression) {
    walkExpression(expression.getTarget());
    walkExpression(expression.getIndex());
  }

  protected void processMemberAccessOperation(WdlMemberAccessOperation expression) {
    walkExpression(expression.getTarget());
  }

  protected void processTernaryOperation(WdlTernaryOperation expression) {
    walkExpression(expression.getCondition());
    walkExpression(expression.getTrueValue());
    walkExpression(expression.getFalseValue());
  }

  protected void processUnaryOperation(WdlUnaryOperation expression) {
    walkExpression(expression.getOperand());
  }
}
