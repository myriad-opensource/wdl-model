import { WdlError } from './wdl-error.js';

export enum WdlSemanticSeverity {
  WARNING = 'WARNING',
  ERROR = 'ERROR',
}

export enum WdlSemanticErrorCode {
  GENERIC_SEMANTIC_ERROR = 'GENERIC_SEMANTIC_ERROR',
  FUNCTION_NOT_AVAILABLE_IN_VERSION = 'FUNCTION_NOT_AVAILABLE_IN_VERSION',
  DUPLICATE_DEFINITION = 'DUPLICATE_DEFINITION',
  UNKNOWN_REFERENCE = 'UNKNOWN_REFERENCE',
  TYPE_MISMATCH = 'TYPE_MISMATCH',
  INVALID_FUNCTION_ARGUMENTS = 'INVALID_FUNCTION_ARGUMENTS',
  LINT_UNUSED_WORKFLOW_DECLARATION = 'LINT_UNUSED_WORKFLOW_DECLARATION',
  LINT_UNUSED_TASK_DECLARATION = 'LINT_UNUSED_TASK_DECLARATION',
  LINT_UNUSED_SCATTER_VARIABLE = 'LINT_UNUSED_SCATTER_VARIABLE',
  LINT_UNUSED_CALL_OUTPUT = 'LINT_UNUSED_CALL_OUTPUT',
  LINT_DEPRECATED_FEATURE = 'LINT_DEPRECATED_FEATURE',
}

export function semanticSeverityForCode(code: WdlSemanticErrorCode): WdlSemanticSeverity {
  switch (code) {
    case WdlSemanticErrorCode.LINT_UNUSED_WORKFLOW_DECLARATION:
    case WdlSemanticErrorCode.LINT_UNUSED_TASK_DECLARATION:
    case WdlSemanticErrorCode.LINT_UNUSED_SCATTER_VARIABLE:
    case WdlSemanticErrorCode.LINT_UNUSED_CALL_OUTPUT:
    case WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE:
      return WdlSemanticSeverity.WARNING;
    default:
      return WdlSemanticSeverity.ERROR;
  }
}

export class WdlSemanticError extends WdlError {
  public constructor(
    public readonly code: WdlSemanticErrorCode,
    message: string,
    line: number,
    charPositionInLine: number,
  ) {
    super(message, line, charPositionInLine);
  }

  public severity(): WdlSemanticSeverity {
    return semanticSeverityForCode(this.code);
  }

  public override toDebugMessage(): string {
    return `${super.toDebugMessage()}:${this.code}:${this.severity()}`;
  }
}
