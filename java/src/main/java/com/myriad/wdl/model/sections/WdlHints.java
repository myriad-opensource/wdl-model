package com.myriad.wdl.model.sections;

import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import java.util.ArrayDeque;

/**
 * Base class for WDL hints sections.
 *
 * <p>Hints provide advisory execution metadata for tasks and workflows. The spec distinguishes
 * between task hints and workflow hints while using the same general keyed-expression structure.
 */
public abstract class WdlHints<V extends WdlHints.WdlHint>
    implements WdlTaskElement, WdlWorkflowElement {

  protected final ArrayDeque<V> elements = new ArrayDeque<>();

  /** Returns the ordered hints contained in the section. */
  public ArrayDeque<V> elements() {
    return elements;
  }

  /** Base class for a single keyed hint entry. */
  public abstract static class WdlHint extends WdlStringKeyValue {
    public WdlHint() {}

    public WdlHint(String key) {
      this();
      setKey(key);
    }

    public WdlHint(String key, WdlExpression value) {
      this(key);
      setValue(value);
    }
  }

  /** Single task-scoped hint entry. */
  public static final class WdlTaskHint extends WdlHint {

    public WdlTaskHint() {}

    public WdlTaskHint(String key, WdlExpression value) {
      super(key, value);
    }

    public WdlTaskHint(String key) {
      super(key);
    }
  }

  /** Task hints section node. */
  public static final class WdlTaskHints extends WdlHints<WdlTaskHint> implements WdlTaskElement {}

  /** Single workflow-scoped hint entry. */
  public static final class WdlWorkflowHint extends WdlHint implements WdlWorkflowElement {
    public WdlWorkflowHint() {}

    public WdlWorkflowHint(String key, WdlExpression value) {
      super(key, value);
    }

    public WdlWorkflowHint(String key) {
      super(key);
    }
  }

  /** Workflow hints section node. */
  public static final class WdlWorkflowHints extends WdlHints<WdlWorkflowHint>
      implements WdlWorkflowElement {}
}
