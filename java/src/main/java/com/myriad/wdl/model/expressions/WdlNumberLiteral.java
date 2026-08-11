package com.myriad.wdl.model.expressions;

public abstract class WdlNumberLiteral<T extends Number> extends WdlLiteral<T>
    implements WdlValueExpression<T> {

  protected WdlNumberLiteral() {}

  protected WdlNumberLiteral(T value) {
    super(value);
  }

  public abstract void negate();
}
