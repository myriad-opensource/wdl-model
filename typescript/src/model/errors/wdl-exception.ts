/** Aggregate exception carrying one or more collected WDL diagnostics. */
import { WdlError } from './wdl-error.js';

export class WdlException extends Error {
  public readonly errors: readonly WdlError[];

  /** Creates an exception from a readonly diagnostic list. */
  public constructor(errors: readonly WdlError[]) {
    super(errors.map((error) => error.toDebugMessage()).join('\n'));
    this.errors = [...errors];
  }

  /** Returns the collected diagnostics carried by this exception. */
  public getErrors(): readonly WdlError[] {
    return this.errors;
  }

  /** Returns a compact debug-oriented rendering of all collected diagnostics. */
  public toDebugMessage(): string {
    return `Errors(${this.errors.map((error) => error.toDebugMessage()).join(',')})`;
  }
}
