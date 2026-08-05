package com.myriad.wdl.model.types;

/** User-defined type reference, typically naming a struct or enum. */
public final class WdlTypeReferenceType extends WdlType {
  private String referencedTypeName = "";

  public WdlTypeReferenceType() {
    super();
  }

  public WdlTypeReferenceType(String referencedTypeName, boolean optional) {
    super(optional);
    setReferenceName(referencedTypeName);
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.TYPEREF;
  }

  /** Returns the referenced type name as written in the source. */
  public String referenceName() {
    return referencedTypeName;
  }

  /** Sets the referenced type name as written in the source. */
  public void setReferenceName(String referencedTypeName) {
    this.referencedTypeName = referencedTypeName;
  }
}
