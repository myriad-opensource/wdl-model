package com.myriad.wdl.model.expressions;

import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import java.util.ArrayDeque;

public final class WdlObjectLiteral implements WdlExpression {
  private final ArrayDeque<WdlObjectEntry> entries = new ArrayDeque<>();

  public WdlObjectLiteral() {}

  public ArrayDeque<WdlObjectEntry> entries() {
    return entries;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.OBJ_LIT;
  }

  public static class WdlObjectEntry extends WdlStringKeyValue {
    public WdlObjectEntry() {
      super();
    }

    public WdlObjectEntry(String key, WdlExpression value) {
      super(key, value);
    }

    public WdlObjectEntry(String key) {
      super(key);
    }
  }
}
