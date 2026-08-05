/** Web-oriented file and response loader wrappers for the core WDL parser. */
import type { WdlDocument } from './wdl-document.js';
import { type WdlValidator, WdlV1Loader } from './wdl-v1-loader.js';
import { WdlImportResolverBase } from './resolvers/wdl-import-resolver-base.js';

export class WdlV1WebLoader {
  /** Reads a browser `Blob` or `File` and parses it with the core loader. */
  public static async loadFromFile(
    file: Blob,
    validator?: WdlValidator,
    importResolver?: WdlImportResolverBase,
  ): Promise<WdlDocument> {
    return WdlV1Loader.loadFromString(await file.text(), validator, undefined, importResolver);
  }

  /** Reads a web `Response` body and parses it with the core loader. */
  public static async loadFromResponse(
    response: Response,
    validator?: WdlValidator,
    importResolver?: WdlImportResolverBase,
  ): Promise<WdlDocument> {
    return WdlV1Loader.loadFromString(
      await response.text(),
      validator,
      response.url || undefined,
      importResolver,
    );
  }
}
