package com.myriad.wdl.model.expressions;

public final class WdlFloatLiteral extends WdlNumberLiteral<Double> {
  public WdlFloatLiteral() {}

  public WdlFloatLiteral(double value) {
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
    return ComponentType.FLOAT_LIT;
  }
}
