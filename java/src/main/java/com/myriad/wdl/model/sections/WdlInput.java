package com.myriad.wdl.model.sections;

import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.statements.WdlDeclaration;
import java.util.ArrayDeque;

/**
 * Input section node.
 *
 * <p>This models the explicit `input { ... }` section that may appear in tasks and workflows.
 */
public final class WdlInput implements WdlTaskElement, WdlWorkflowElement {
  private final ArrayDeque<WdlDeclaration> elements = new ArrayDeque<>();

  /** Returns the ordered declarations contained in the input section. */
  public ArrayDeque<WdlDeclaration> elements() {
    return elements;
  }
}
