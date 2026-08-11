package com.myriad.wdl.model.types;

import java.util.Objects;

/**
 * Primitive WDL type with optional marker.
 *
 * <p>This models primitive type names such as {@code Int}, {@code String}, {@code File}, and
 * {@code Directory} from the specification's "Primitive Types" section.
 */
public final class WdlPrimitiveType extends WdlType {
  /** Supported primitive type names. */
  public enum Type {
    BOOLEAN("Boolean"),
    INT("Int"),
    FLOAT("Float"),
    STRING("String"),
    FILE("File"),
    DIRECTORY("Directory"),
    OBJECT("Object");

    private final String wdlName;

    private Type(String wdlName) {
      this.wdlName = wdlName;
    }

    public String toWdlString() {
      return wdlName;
    }
  }

  private Type primitiveValueType = Type.STRING;

  public WdlPrimitiveType() {
    super();
  }

  public WdlPrimitiveType(Type primitiveValueType, boolean optional) {
    super(optional);
    this.primitiveValueType = Objects.requireNonNull(primitiveValueType, "primitiveValueType");
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.PRIMITIVE;
  }

  /** Returns the specific primitive type name represented by this node. */
  public Type primitiveType() {
    return primitiveValueType;
  }

  /** Sets the specific primitive type name represented by this node. */
  public void setPrimitiveType(Type primitiveValueType) {
    this.primitiveValueType = Objects.requireNonNull(primitiveValueType, "primitiveValueType");
  }
}
