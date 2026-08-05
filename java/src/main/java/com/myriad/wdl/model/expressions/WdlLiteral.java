package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

public abstract class WdlLiteral<T> implements WdlValueExpression<T> {
  @Getter @Setter protected T value;

  protected WdlLiteral() {}

  protected WdlLiteral(T literalValue) {
    this.value = literalValue;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }
}
