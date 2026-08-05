package com.myriad.wdl.model.statements;

import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Workflow call statement.
 *
 * <p>A call invokes a task or subworkflow and may include a dotted target path, an alias, an input
 * binding block, and after-dependencies. See the WDL specification section "Call Statement".
 */
public final class WdlCall implements WdlStatement, WdlWorkflowElement {
  private final ArrayDeque<String> targetPath = new ArrayDeque<>();
  @Getter @Setter private String alias;
  private final ArrayDeque<WdlCallInput> inputs = new ArrayDeque<>();
  private final ArrayDeque<String> afterCallDependencies = new ArrayDeque<>();
  @Getter @Setter private boolean legacyInputColonUsed;

  public WdlCall() {}

  /** Returns the dotted target path segments for the invoked task or workflow. */
  public ArrayDeque<String> targetPath() {
    return targetPath;
  }

  /** Returns the dotted target path in source form, such as {@code lib.task_name}. */
  public String targetPathAsString() {
    return String.join(".", targetPath());
  }

  /** Returns the ordered input bindings supplied by the call input block. */
  public ArrayDeque<WdlCallInput> inputs() {
    return inputs;
  }

  /** Returns the ordered set of `after` dependencies declared on the call. */
  public ArrayDeque<String> afterDependencies() {
    return afterCallDependencies;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.CALL;
  }

  public static final class WdlCallInput extends WdlStringKeyValue {
    /** Creates a call input binding such as {@code x = expr}. */
    public WdlCallInput() {}

    public WdlCallInput(String key) {
      super(key);
    }

    public WdlCallInput(String key, WdlExpression value) {
      super(key, value);
    }
  }
}
