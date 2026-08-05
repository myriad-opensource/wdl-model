package com.myriad.wdl.model.sections;

import com.myriad.wdl.model.base.WdlKeyValue;
import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import java.util.ArrayDeque;

/**
 * Task requirements section node.
 *
 * <p>This models the WDL `requirements { ... }` section, which describes execution requirements
 * such as container, cpu, memory, disks, retries, and return codes.
 */
public final class WdlRequirements implements WdlTaskElement {
  private final ArrayDeque<WdlKeyValue<String, WdlExpression>> elements = new ArrayDeque<>();

  /** Returns the ordered requirement entries contained in the section. */
  public ArrayDeque<WdlKeyValue<String, WdlExpression>> elements() {
    return elements;
  }

  /** Single requirement entry such as `cpu: 2` or `container: "ubuntu:latest"`. */
  public static final class WdlRequirementEntry extends WdlStringKeyValue {
    public WdlRequirementEntry() {
      super();
    }

    public WdlRequirementEntry(String key, WdlExpression value) {
      super(key, value);
    }

    public WdlRequirementEntry(String key) {
      super(key);
    }
  }
}
