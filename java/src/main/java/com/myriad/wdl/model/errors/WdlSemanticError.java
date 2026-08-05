package com.myriad.wdl.model.errors;

/**
 * Semantic diagnostic emitted by the Java WDL validators.
 *
 * <p>The diagnostic model is intentionally shared across baseline semantic validation,
 * deterministic static analysis, and linting. Callers can inspect {@link #code()} and
 * {@link #severity()} to decide whether a collected diagnostic should fail the current
 * validation pass or be shown as a warning only.
 *
 * <p>Representative diagnostics are exercised by spec fixtures such as
 * {@code spec_examples/v1_3/private_declaration_fail.wdl},
 * {@code spec_examples/v1_3/select_first_empty_fail.wdl}, and
 * {@code spec_examples/v1_3/write_json_fail.wdl}.
 */
public class WdlSemanticError extends WdlError {

  /** Diagnostic severity used by the validator throw policy. */
  public static enum Severity {
    WARNING,
    ERROR,
  }

  /** Stable semantic and lint diagnostic codes emitted by this library. */
  public static enum Code {
    GENERIC_SEMANTIC_ERROR(Severity.ERROR),
    FUNCTION_NOT_AVAILABLE_IN_VERSION(Severity.ERROR),
    DUPLICATE_DEFINITION(Severity.ERROR),
    UNKNOWN_REFERENCE(Severity.ERROR),
    TYPE_MISMATCH(Severity.ERROR),
    INVALID_FUNCTION_ARGUMENTS(Severity.ERROR),
    LINT_DEPRECATED_FEATURE(Severity.WARNING),
    LINT_UNUSED_WORKFLOW_DECLARATION(Severity.WARNING),
    LINT_UNUSED_TASK_DECLARATION(Severity.WARNING),
    LINT_UNUSED_SCATTER_VARIABLE(Severity.WARNING),
    LINT_UNUSED_CALL_OUTPUT(Severity.WARNING);

    private final Severity severity;

    private Code(Severity severity) {
      this.severity = severity;
    }

    public Severity severity() {
      return severity;
    }
  }

  private final Code code;

  /** Creates a generic semantic error when a more specific code is not supplied. */
  public WdlSemanticError(String message, int line, int charPositionInLine) {
    this(Code.GENERIC_SEMANTIC_ERROR, message, line, charPositionInLine);
  }

  /** Creates a coded semantic diagnostic. */
  public WdlSemanticError(Code code, String message, int line, int charPositionInLine) {
    super(message, line, charPositionInLine);
    this.code = code == null ? Code.GENERIC_SEMANTIC_ERROR : code;
  }

  public Code code() {
    return code;
  }

  /** Returns the severity implied by {@link #code()}. */
  public Severity severity() {
    return code.severity();
  }

  @Override
  public String toDebugMessage() {
    return super.toDebugMessage() + ":" + code.name() + ":" + severity().name();
  }
}
