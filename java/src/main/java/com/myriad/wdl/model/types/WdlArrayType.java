package com.myriad.wdl.model.types;

/**
 * Array type node.
 *
 * <p>This models WDL array types such as {@code Array[Int]} and {@code Array[String]+}. The
 * {@code nonEmpty} flag represents the `+` suffix used by the specification for non-empty arrays.
 */
public final class WdlArrayType extends WdlType {
  private WdlType arrayMemberType;
  private boolean nonEmpty;

  public WdlArrayType() {
    super();
  }

  public WdlArrayType(WdlType arrayMemberType, boolean nonEmpty, boolean optional) {
    super(optional);
    this.arrayMemberType = arrayMemberType;
    this.nonEmpty = nonEmpty;
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.ARRAY;
  }

  /** Returns the array member type. */
  public WdlType memberType() {
    return arrayMemberType;
  }

  /** Sets the array member type. */
  public void setMemberType(WdlType arrayMemberType) {
    this.arrayMemberType = arrayMemberType;
  }

  /** Returns whether the type carries the WDL non-empty array marker (`+`). */
  public boolean isNonEmpty() {
    return nonEmpty;
  }

  /** Sets whether the type carries the WDL non-empty array marker (`+`). */
  public void setNonEmpty(boolean nonEmpty) {
    this.nonEmpty = nonEmpty;
  }
}
