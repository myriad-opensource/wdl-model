/** Syntax diagnostic emitted by the ANTLR-backed WDL loader. */
import { WdlError } from './wdl-error.js';

export class WdlSyntaxError extends WdlError {
  /** Creates a syntax diagnostic and retains the underlying parser cause when present. */
  public constructor(
    message: string,
    line: number,
    charPositionInLine: number,
    public readonly cause?: unknown,
  ) {
    super(message, line, charPositionInLine);
  }
}
