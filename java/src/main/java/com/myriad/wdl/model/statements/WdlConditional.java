package com.myriad.wdl.model.statements;

import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Conditional statement node.
 *
 * <p>This models `if`, `else if`, and `else` workflow control flow exactly as written in the
 * source. See the WDL specification section "Conditional Statement".
 */
public class WdlConditional implements WdlStatement, WdlWorkflowElement {

  @Getter @Setter private WdlExpression condition;
  private final ArrayDeque<WdlStatement> thenStatements = new ArrayDeque<>();
  private final ArrayDeque<WdlConditionalElseIf> elseIfs = new ArrayDeque<>();
  private final ArrayDeque<WdlStatement> elseStatements = new ArrayDeque<>();

  public WdlConditional() {}

  /** Returns the ordered statements in the `then` branch. */
  public ArrayDeque<WdlStatement> thenStatements() {
    return thenStatements;
  }

  /** Returns the ordered `else if` branches. */
  public ArrayDeque<WdlConditionalElseIf> elseIfs() {
    return elseIfs;
  }

  /** Returns the ordered statements in the `else` branch. */
  public ArrayDeque<WdlStatement> elseStatements() {
    return elseStatements;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.CONDITIONAL;
  }

  /** Single `else if` branch inside a conditional statement. */
  public static class WdlConditionalElseIf implements WdlNode {
    @Getter @Setter private WdlExpression condition;
    private final ArrayDeque<WdlStatement> thenStatements = new ArrayDeque<>();

    /** Returns the ordered statements in this `else if` branch. */
    public ArrayDeque<WdlStatement> thenStatements() {
      return thenStatements;
    }
  }
}
