package com.myriad.wdl.model.sections;

import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import java.util.ArrayDeque;

/**
 * Output section node.
 *
 * <p>This models the explicit `output { ... }` section that may appear in tasks and workflows.
 * Output declarations are always bound to expressions.
 */
public final class WdlOutput implements WdlTaskElement, WdlWorkflowElement {
  private final ArrayDeque<WdlBoundDeclaration> elements = new ArrayDeque<>();

  /** Returns the ordered bound declarations contained in the output section. */
  public ArrayDeque<WdlBoundDeclaration> elements() {
    return elements;
  }
}
