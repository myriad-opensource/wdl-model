package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

public final class WdlPairLiteral implements WdlExpression {
  @Getter @Setter private WdlExpression left;
  @Getter @Setter private WdlExpression right;

  public WdlPairLiteral() {}

  public WdlPairLiteral(WdlExpression leftExpression, WdlExpression rightExpression) {
    setLeft(leftExpression);
    setRight(rightExpression);
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.PAIR_LIT;
  }
}
