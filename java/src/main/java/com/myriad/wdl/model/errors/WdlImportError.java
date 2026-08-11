package com.myriad.wdl.model.errors;

/** Diagnostic describing a failure while resolving or loading an imported WDL source. */
public class WdlImportError extends WdlError {

  private final String importLocation;

  public WdlImportError(String message, String importLocation) {
    super(message, -1, -1);
    this.importLocation = importLocation;
  }

  public String getImportLocation() {
    return importLocation;
  }

  @Override
  public String toDebugMessage() {
    return getClass().getSimpleName() + ":" + importLocation + ":" + message;
  }
}
