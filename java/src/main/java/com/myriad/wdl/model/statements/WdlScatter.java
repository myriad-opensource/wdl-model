package com.myriad.wdl.model.statements;

import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Scatter statement node.
 *
 * <p>A scatter iterates a bound name over a collection expression and evaluates a nested block of
 * workflow statements. See the WDL specification section "Scatter Statement".
 */
public final class WdlScatter implements WdlStatement, WdlWorkflowElement {
  @Getter @Setter private String name;
  @Getter @Setter private WdlExpression collection;
  private final ArrayDeque<WdlStatement> statements = new ArrayDeque<>();

  public WdlScatter() {}

  public WdlScatter(String name) {
    setName(name);
  }

  /** Returns the ordered nested statements inside the scatter body. */
  public ArrayDeque<WdlStatement> statements() {
    return statements;
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.SCATTER;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }
}
