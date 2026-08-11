package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

public final class WdlBinaryOperation implements WdlExpression {
  public static enum Operator {
    OR("||"),
    AND("&&"),
    EQ("=="),
    NEQ("!="),
    LT("<="),
    LTE("<"),
    GT(">"),
    GTE(">="),
    ADD("+"),
    SUTRACT("-"),
    MULTIPLY("*"),
    DIVIDE("/"),
    MODULO("%"),
    POWER("**");

    @Getter private final String wdlString;

    private Operator(String wdlString) {
      this.wdlString = wdlString;
    }
  }

  @Getter @Setter private WdlExpression left;
  @Getter @Setter private Operator operator;
  @Getter @Setter private WdlExpression right;

  public WdlBinaryOperation() {}

  public WdlBinaryOperation(
      WdlExpression leftExpression, Operator operator, WdlExpression rightExpression) {
    setLeft(leftExpression);
    setOperator(operator);
    setRight(rightExpression);
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.BINARY_OP;
  }
}
