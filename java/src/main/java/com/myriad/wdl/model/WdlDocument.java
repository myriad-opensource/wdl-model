package com.myriad.wdl.model;

import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.statements.WdlImport;
import java.net.URI;
import java.util.ArrayDeque;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import lombok.Getter;
import lombok.Setter;

/**
 * Root node for a parsed WDL document.
 *
 * <p>The WDL specification says that a document contains a version declaration followed by imports,
 * user-defined types, tasks, and workflows. This class mirrors that shape and offers convenient
 * typed views over the underlying ordered element list.
 */
public final class WdlDocument implements WdlNode {

  /** Marker for any top-level element that can appear directly in a WDL document. */
  public interface WdlDocumentElement extends WdlNode {}

  @Getter @Setter private WdlVersion wdlVersion;
  @Getter @Setter private URI sourceLocation;
  private final ArrayDeque<WdlDocumentElement> elements = new ArrayDeque<>();
  private final Map<String, WdlDocument> importedDocuments = new LinkedHashMap<>();

  public WdlDocument() {}

  public WdlDocument(WdlVersion wdlVersion) {
    this.wdlVersion = wdlVersion;
  }

  /** Returns the ordered top-level elements exactly as they appeared in the source document. */
  public ArrayDeque<WdlDocumentElement> elements() {
    return elements;
  }

  /** Returns only top-level import statements. */
  public List<WdlImport> importStatements() {
    return elements.stream()
        .filter(WdlImport.class::isInstance)
        .map(WdlImport.class::cast)
        .collect(Collectors.toList());
  }

  /** Returns imported documents keyed by import identifier/location. */
  public Map<String, WdlDocument> importedDocuments() {
    return importedDocuments;
  }

  /** Returns only top-level enum definitions. */
  public List<WdlEnum> enums() {
    return elements.stream()
        .filter(WdlEnum.class::isInstance)
        .map(WdlEnum.class::cast)
        .collect(Collectors.toList());
  }

  /** Returns only top-level struct definitions. */
  public List<WdlStruct> structs() {
    return elements.stream()
        .filter(WdlStruct.class::isInstance)
        .map(WdlStruct.class::cast)
        .collect(Collectors.toList());
  }

  /** Returns only top-level task definitions. */
  public List<WdlTask> tasks() {
    return elements.stream()
        .filter(WdlTask.class::isInstance)
        .map(WdlTask.class::cast)
        .collect(Collectors.toList());
  }

  /** Returns only top-level workflow definitions. */
  public List<WdlWorkflow> workflows() {
    return elements.stream()
        .filter(WdlWorkflow.class::isInstance)
        .map(WdlWorkflow.class::cast)
        .collect(Collectors.toList());
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }
}
