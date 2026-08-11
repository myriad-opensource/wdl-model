package com.myriad.wdl.model.expressions;

import java.util.ArrayDeque;

public final class WdlArrayLiteral implements WdlExpression {
  private final ArrayDeque<WdlExpression> entries = new ArrayDeque<>();

  public ArrayDeque<WdlExpression> entries() {
    return entries;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.ARRAY_LIT;
  }
}
