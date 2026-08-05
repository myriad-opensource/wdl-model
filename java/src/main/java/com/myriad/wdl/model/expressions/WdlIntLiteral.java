package com.myriad.wdl.model.expressions;

public final class WdlIntLiteral extends WdlNumberLiteral<Long> {
  public WdlIntLiteral() {}

  public WdlIntLiteral(long value) {
    super(value);
  }

  @Override
  public void negate() {
    if (this.value != null) {
      this.value = -this.value;
    }
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.INT_LIT;
  }
}
