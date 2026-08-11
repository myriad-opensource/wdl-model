package com.myriad.wdl.model.types;

/**
 * Map type node.
 *
 * <p>This models WDL map types such as {@code Map[String, Int]}.
 */
public final class WdlMapType extends WdlType {
  private WdlType mapKeyType;
  private WdlType mapValueType;

  public WdlMapType() {
    super();
  }

  public WdlMapType(WdlType mapKeyType, WdlType mapValueType, boolean optional) {
    super(optional);
    this.mapKeyType = mapKeyType;
    this.mapValueType = mapValueType;
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.MAP;
  }

  /** Returns the map key type. */
  public WdlType keyType() {
    return mapKeyType;
  }

  /** Sets the map key type. */
  public void setKeyType(WdlType mapKeyType) {
    this.mapKeyType = mapKeyType;
  }

  /** Returns the map value type. */
  public WdlType valueType() {
    return mapValueType;
  }

  /** Sets the map value type. */
  public void setValueType(WdlType mapValueType) {
    this.mapValueType = mapValueType;
  }
}
