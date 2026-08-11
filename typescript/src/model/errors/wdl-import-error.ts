import { WdlError } from './wdl-error.js';

/** Diagnostic describing a failure while resolving or loading an imported WDL source. */
export class WdlImportError extends WdlError {
  public constructor(
    message: string,
    public readonly importLocation: string,
  ) {
    super(message, -1, -1);
  }

  public override toDebugMessage(): string {
    return `${this.constructor.name}:${this.importLocation}:${this.message}`;
  }
}
