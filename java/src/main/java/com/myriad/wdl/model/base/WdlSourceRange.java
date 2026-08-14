package com.myriad.wdl.model.base;

/**
 * Source text span attached to AST nodes by the loader from ANTLR token positions.
 *
 * <p>All values are 1-based line numbers and 0-based column offsets, matching the ANTLR token
 * convention. {@code endColumn} is the exclusive end position of the last token.
 */
public final class WdlSourceRange {

  private final int startLine;
  private final int startColumn;
  private final int endLine;
  private final int endColumn;

  public WdlSourceRange(int startLine, int startColumn, int endLine, int endColumn) {
    this.startLine = startLine;
    this.startColumn = startColumn;
    this.endLine = endLine;
    this.endColumn = endColumn;
  }

  /** 1-based line of the first token in the node. */
  public int getStartLine() {
    return startLine;
  }

  /** 0-based column of the first token in the node. */
  public int getStartColumn() {
    return startColumn;
  }

  /** 1-based line of the last token in the node. */
  public int getEndLine() {
    return endLine;
  }

  /** 0-based exclusive-end column of the last token in the node. */
  public int getEndColumn() {
    return endColumn;
  }

  @Override
  public String toString() {
    return startLine + ":" + startColumn + "-" + endLine + ":" + endColumn;
  }
}
