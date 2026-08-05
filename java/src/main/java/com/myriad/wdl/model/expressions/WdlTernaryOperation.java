package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

public class WdlTernaryOperation implements WdlExpression {

  @Getter @Setter private WdlExpression condition;
  @Getter @Setter private WdlExpression trueValue;
  @Getter @Setter private WdlExpression falseValue;

  public WdlTernaryOperation() {}

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.TERNARY_OP;
  }
}
