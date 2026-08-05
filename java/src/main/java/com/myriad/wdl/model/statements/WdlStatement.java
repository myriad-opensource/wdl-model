package com.myriad.wdl.model.statements;

import com.myriad.wdl.model.base.WdlNode;

/**
 * Base interface for workflow and task statements.
 *
 * <p>Statements are the executable and scoping constructs that appear inside workflows and, in the
 * case of declarations, task bodies. The WDL specification includes declarations, calls, scatters,
 * and conditionals in this family.
 */
public interface WdlStatement extends WdlNode {
  /** High-level statement family used for traversal and validation dispatch. */
  public static enum ComponentType {
    CALL,
    CONDITIONAL,
    DECLARATION,
    SCATTER
  }

  /** Returns the broad statement family for the current node. */
  ComponentType componentType();
}
