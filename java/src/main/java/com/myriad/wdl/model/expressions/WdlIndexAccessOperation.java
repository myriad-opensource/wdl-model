package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

public final class WdlIndexAccessOperation implements WdlExpression {
  @Getter @Setter private WdlExpression target;
  @Getter @Setter private WdlExpression index;

  public WdlIndexAccessOperation() {}

  public WdlIndexAccessOperation(WdlExpression target, WdlExpression index) {
    this.target = target;
    this.index = index;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.IDX_OP;
  }
}
