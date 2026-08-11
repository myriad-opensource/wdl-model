package com.myriad.wdl.model.errors;

import java.util.List;

/** Exception raised when an import URI cannot be resolved or loaded. */
public class WdlImportException extends WdlException {
  private static final long serialVersionUID = 1L;

  public WdlImportException(String message, String importLocation) {
    super(List.of(new WdlImportError(message, importLocation)));
  }

  public WdlImportException(String message, String importLocation, Throwable cause) {
    this(message, importLocation);
    initCause(cause);
  }
}
