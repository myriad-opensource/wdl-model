package com.myriad.wdl.model.expressions;

import com.myriad.wdl.model.base.WdlKeyValue.WdlExpresionKeyValue;
import java.util.ArrayDeque;

public final class WdlMapLiteral implements WdlExpression {
  private final ArrayDeque<WdlMapEntry> entries = new ArrayDeque<>();

  public WdlMapLiteral() {}

  public ArrayDeque<WdlMapEntry> entries() {
    return entries;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.MAP_LIT;
  }

  public static class WdlMapEntry extends WdlExpresionKeyValue {
    public WdlMapEntry() {
      super();
    }

    public WdlMapEntry(WdlExpression key, WdlExpression value) {
      super(key, value);
    }

    public WdlMapEntry(WdlExpression key) {
      super(key);
    }
  }
}
