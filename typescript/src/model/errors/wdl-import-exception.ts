import { WdlException } from './wdl-exception.js';
import { WdlImportError } from './wdl-import-error.js';

/** Exception raised when an import URI cannot be resolved or loaded. */
export class WdlImportException extends WdlException {
  public constructor(message: string, importLocation: string, cause?: unknown) {
    super([new WdlImportError(message, importLocation)]);
    if (cause !== undefined) {
      this.cause = cause;
    }
  }
}
