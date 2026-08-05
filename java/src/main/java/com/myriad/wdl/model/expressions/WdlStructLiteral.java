package com.myriad.wdl.model.expressions;

import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

public final class WdlStructLiteral implements WdlExpression {
  @Getter @Setter private String name;
  private final ArrayDeque<WdlStructEntry> entries = new ArrayDeque<>();

  public WdlStructLiteral() {}

  public WdlStructLiteral(String name) {
    setName(name);
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.STRUCT_LIT;
  }

  public ArrayDeque<WdlStructEntry> entries() {
    return entries;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  public static class WdlStructEntry extends WdlStringKeyValue {
    public WdlStructEntry() {
      super();
    }

    public WdlStructEntry(String key, WdlExpression value) {
      super(key, value);
    }

    public WdlStructEntry(String key) {
      super(key);
    }
  }
}
