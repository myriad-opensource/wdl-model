package com.myriad.wdl.model.expressions;

public final class WdlNullLiteral implements WdlExpression {

  public WdlNullLiteral() {}

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.NULL_LIT;
  }
}
