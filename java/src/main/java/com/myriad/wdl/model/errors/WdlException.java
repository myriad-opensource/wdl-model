package com.myriad.wdl.model.errors;

import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;
import lombok.Getter;

/**
 * Aggregate exception carrying one or more WDL diagnostics.
 *
 * <p>The loader and validators collect syntax, semantic, static-analysis, or lint diagnostics and
 * then surface them together through this exception type.
 */
public class WdlException extends Exception {
  private static final long serialVersionUID = 1L;

  @Getter private final List<WdlError> errors;

  /** Creates an exception from an immutable snapshot of collected diagnostics. */
  public WdlException(List<? extends WdlError> errors) {
    this.errors = Collections.unmodifiableList(errors);
  }

  /** Returns a compact debug-oriented rendering of all collected diagnostics. */
  public String toDebugMessage() {
    StringBuilder str =
        new StringBuilder("Errors(")
            .append(
                String.join(
                    ",", errors.stream().map(e -> e.toDebugMessage()).collect(Collectors.toList())))
            .append(")");
    return str.toString();
  }
}
