package com.myriad.wdl.model.sections;

import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import java.util.ArrayDeque;

/**
 * Legacy runtime section node.
 *
 * <p>This models the `runtime { ... }` section, which is retained for compatibility but is legacy
 * in modern WDL versions where `requirements` and `hints` carry the main execution metadata.
 */
public final class WdlRuntime implements WdlTaskElement {
  private final ArrayDeque<WdlRuntimeEntry> elements = new ArrayDeque<>();

  /** Returns the ordered runtime entries contained in the section. */
  public ArrayDeque<WdlRuntimeEntry> elements() {
    return elements;
  }

  /** Single runtime entry such as `docker: "image"` in older source documents. */
  public static final class WdlRuntimeEntry extends WdlStringKeyValue {
    public WdlRuntimeEntry() {
      super();
    }

    public WdlRuntimeEntry(String key, WdlExpression value) {
      super(key, value);
    }

    public WdlRuntimeEntry(String key) {
      super(key);
    }
  }
}
