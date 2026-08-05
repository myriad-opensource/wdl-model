package com.myriad.wdl.model.sections;

import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.definitions.WdlStruct.WdlStructElement;
import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlObjectLiteral.WdlObjectEntry;
import java.util.ArrayDeque;

public class WdlMetadataBase implements WdlNode {
  protected final ArrayDeque<WdlMetadataEntry> elements = new ArrayDeque<>();

  protected WdlMetadataBase() {}

  public ArrayDeque<WdlMetadataEntry> elements() {
    return elements;
  }

  public static final class WdlMetadata extends WdlMetadataBase
      implements WdlTaskElement, WdlWorkflowElement, WdlStructElement {}

  public static final class WdlParameterMetadata extends WdlMetadataBase
      implements WdlTaskElement, WdlWorkflowElement, WdlStructElement {}

  public static final class WdlMetadataEntry extends WdlObjectEntry {

    public WdlMetadataEntry() {
      super();
    }

    public WdlMetadataEntry(String key, WdlExpression value) {
      super(key, value);
    }

    public WdlMetadataEntry(String key) {
      super(key);
    }
  }
}
