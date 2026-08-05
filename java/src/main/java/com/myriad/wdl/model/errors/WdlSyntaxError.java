package com.myriad.wdl.model.errors;

import org.antlr.v4.runtime.RecognitionException;

/** Syntax diagnostic produced while lexing or parsing WDL source. */
public class WdlSyntaxError extends WdlError {
  private final RecognitionException cause;

  /** Creates a syntax diagnostic and preserves the parser exception when available. */
  public WdlSyntaxError(
      String message, int line, int charPositionInLine, RecognitionException cause) {
    super(message, line, charPositionInLine);
    this.cause = cause;
  }

  /** Returns the underlying ANTLR recognition exception, if any. */
  public RecognitionException getCause() {
    return cause;
  }
}
