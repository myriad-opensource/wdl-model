package com.myriad.wdl.model.base;

/**
 * Marker interface for every node in the Java WDL object model.
 *
 * <p>The library models a WDL document as a tree of nodes representing top-level definitions,
 * statements, sections, expressions, and types. See the WDL 1.3 specification sections "WDL
 * Documents", "Task Definition", and "Workflow Definition" for the grammar concepts represented
 * by these nodes.
 */
public interface WdlNode {}
