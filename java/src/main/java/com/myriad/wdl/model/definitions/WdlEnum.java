package com.myriad.wdl.model.definitions;

import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.types.WdlType;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Enum definition node.
 *
 * <p>WDL 1.3 adds enumerations as named sets of allowed values. The enum can either use the
 * implicit symbol value type or an explicit scalar or custom type for each choice value.
 */
public final class WdlEnum implements WdlDocumentElement {
  @Getter @Setter private String name;
  @Getter @Setter private WdlType valueType;
  private final ArrayDeque<WdlEnumChoice> elements = new ArrayDeque<>();

  public WdlEnum() {}

  public WdlEnum(String name, WdlType valueType) {
    setName(name);
    setValueType(valueType);
  }

  public WdlEnum(String name) {
    this(name, null);
  }

  public WdlEnum(WdlType valueType) {
    this(null, valueType);
  }

  /** Returns the ordered enum choices. */
  public ArrayDeque<WdlEnumChoice> elements() {
    return elements;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  public static final class WdlEnumChoice extends WdlStringKeyValue {
    /** Creates an enum choice with a symbolic name and optional explicit value expression. */
    public WdlEnumChoice() {}

    public WdlEnumChoice(String key, WdlExpression value) {
      super(key, value);
    }
  }
}
