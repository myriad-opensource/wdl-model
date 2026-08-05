package com.myriad.wdl.model.definitions;

import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.types.WdlType;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Struct definition node.
 *
 * <p>The WDL specification describes structs as user-defined composite types whose members are named
 * and typed. This class models the struct declaration itself, while {@link WdlStructMember}
 * models each declared member.
 */
public final class WdlStruct implements WdlDocumentElement {
  /** Marker for any node that can appear directly inside a struct definition. */
  public interface WdlStructElement extends WdlNode {}

  @Getter @Setter private String name;
  private final ArrayDeque<WdlStructElement> elements = new ArrayDeque<>();

  public WdlStruct() {}

  public WdlStruct(String name) {
    setName(name);
  }

  /** Returns the ordered struct elements, usually member declarations and metadata sections. */
  public ArrayDeque<WdlStructElement> elements() {
    return elements;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  public static final class WdlStructMember implements WdlStructElement {
    @Getter @Setter private WdlType type;
    @Getter @Setter private String name;

    public WdlStructMember() {}

    public WdlStructMember(WdlType type, String name) {
      setType(type);
      setName(name);
    }

    @Override
    public String toString() {
      return getClass().getSimpleName();
    }
  }
}
