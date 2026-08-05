package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

/** Prefix unary operator expression. */
public final class WdlUnaryOperation implements WdlExpression {
  public static enum Operator {
    NOT("!"),
    NEGATIVE("-");

    @Getter private final String wdlString;

    private Operator(String wdlString) {
      this.wdlString = wdlString;
    }
  }

  @Getter @Setter private Operator operator;
  @Getter @Setter private WdlExpression operand;

  public WdlUnaryOperation() {}

  public WdlUnaryOperation(Operator operator, WdlExpression operandExpression) {
    setOperator(operator);
    setOperand(operandExpression);
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.UNARY_OP;
  }
}
