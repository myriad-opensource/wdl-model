package com.myriad.wdl.model.definitions;

import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.types.WdlType;
import java.util.ArrayDeque;
import java.util.Objects;
import java.util.Optional;
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

  /** Returns whether a member with the supplied name exists. */
  public boolean hasMember(String memberName) {
    return member(memberName).isPresent();
  }

  /** Returns the declared member, if present. */
  public Optional<WdlStructMember> member(String memberName) {
    if (memberName == null || memberName.isBlank()) {
      return Optional.empty();
    }
    return elements().stream()
        .filter(WdlStructMember.class::isInstance)
        .map(WdlStructMember.class::cast)
        .filter(member -> Objects.equals(memberName, member.getName()))
        .findFirst();
  }

  /** Returns the declared member type, if present. */
  public Optional<WdlType> memberType(String memberName) {
    return member(memberName).map(WdlStructMember::getType);
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
