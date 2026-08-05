package com.myriad.wdl.model.definitions;

import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.base.WdlNode;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Workflow definition node.
 *
 * <p>In the WDL specification, a workflow composes calls, declarations, scatters, and conditionals
 * into a larger pipeline. This class preserves workflow body order so that processors can model
 * scoping and after-dependencies correctly.
 */
public final class WdlWorkflow implements WdlDocumentElement {

  /** Marker for any node that can appear directly inside a workflow body. */
  public interface WdlWorkflowElement extends WdlNode {}

  @Getter @Setter private String name;
  private final ArrayDeque<WdlWorkflowElement> elements = new ArrayDeque<>();

  public WdlWorkflow() {}

  public WdlWorkflow(String name) {
    this.name = name;
  }

  /** Returns the ordered workflow body elements exactly as they appeared in the source. */
  public ArrayDeque<WdlWorkflowElement> getElements() {
    return elements;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }
}
