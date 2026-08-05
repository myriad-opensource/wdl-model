package com.myriad.wdl.model.errors;

/**
 * Base diagnostic type for parsing and validation failures.
 *
 * <p>All diagnostics carry a message and a source location pair. Concrete subclasses distinguish
 * syntax errors from semantic or lint diagnostics.
 */
public abstract class WdlError {
  protected final String message;
  protected final int line;
  protected final int charPositionInLine;

  public WdlError(String message, int line, int charPositionInLine) {
    this.message = message;
    this.line = line;
    this.charPositionInLine = charPositionInLine;
  }

  /** Returns a compact debug-oriented rendering of the diagnostic. */
  public String toDebugMessage() {
    return getClass().getSimpleName() + ":" + line + ":" + charPositionInLine + ":" + message;
  }
}
