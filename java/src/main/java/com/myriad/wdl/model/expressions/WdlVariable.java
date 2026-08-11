package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

public final class WdlVariable implements WdlExpression {
  @Getter @Setter private String name;

  public WdlVariable() {}

  public WdlVariable(String name) {
    setName(name);
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.VARIABLE;
  }
}
