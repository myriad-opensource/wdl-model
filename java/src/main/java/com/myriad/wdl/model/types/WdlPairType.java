package com.myriad.wdl.model.types;

/** Pair[L, R] type node. */
public final class WdlPairType extends WdlType {
  private WdlType pairLeftType;
  private WdlType pairRightType;

  public WdlPairType() {
    super();
  }

  public WdlPairType(WdlType pairLeftType, WdlType pairRightType, boolean optional) {
    super(optional);
    this.pairLeftType = pairLeftType;
    this.pairRightType = pairRightType;
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.PAIR;
  }

  /** Returns the left pair member type. */
  public WdlType leftType() {
    return pairLeftType;
  }

  /** Sets the left pair member type. */
  public void setLeftType(WdlType pairLeftType) {
    this.pairLeftType = pairLeftType;
  }

  /** Returns the right pair member type. */
  public WdlType rightType() {
    return pairRightType;
  }

  /** Sets the right pair member type. */
  public void setRightType(WdlType pairRightType) {
    this.pairRightType = pairRightType;
  }
}
