package com.myriad.wdl.model.expressions;

public final class WdlBooleanLiteral extends WdlLiteral<Boolean> {
  public WdlBooleanLiteral() {}

  public WdlBooleanLiteral(boolean value) {
    super(value);
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.BOOL_LIT;
  }
}
