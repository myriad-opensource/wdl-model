package com.myriad.wdl.model.definitions;

import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.base.WdlSourceRange;
import java.util.ArrayDeque;
import lombok.Getter;
import lombok.Setter;

/**
 * Task definition node.
 *
 * <p>In the WDL specification, a task is the reusable unit that declares inputs, command text,
 * outputs, requirements, hints, metadata, and private declarations. This class preserves the task
 * body in source order so processors and validators can reason about it.
 */
public final class WdlTask implements WdlDocumentElement {

  /** Marker for any node that can appear directly inside a task body. */
  public interface WdlTaskElement extends WdlNode {}

  @Getter @Setter private String name;
  @Getter @Setter private WdlSourceRange sourceRange;
  private final ArrayDeque<WdlTaskElement> elements = new ArrayDeque<>();

  public WdlTask() {}

  public WdlTask(String name) {
    this.name = name;
  }

  /** Returns the ordered task body elements exactly as they appeared in the source. */
  public ArrayDeque<WdlTaskElement> elements() {
    return elements;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }
}
