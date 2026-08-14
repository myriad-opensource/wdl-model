/** Web-oriented file and response loader wrappers for the core WDL parser. */
import type { WdlDocument } from './wdl-document.js';
import { type WdlValidator, WdlV1Loader } from './wdl-v1-loader.js';
import { WdlImportResolverBase } from './resolvers/wdl-import-resolver-base.js';
import { WdlImportResolverCache } from './resolvers/wdl-import-resolver-cache.js';

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

  /**
   * Fetches, parses, and validates a WDL document and all of its transitive imports.
   *
   * All imports are resolved asynchronously before the synchronous ANTLR parse begins, so this
   * method works in browser environments where synchronous XHR is unavailable.
   *
   * @param source    WDL source text of the root document.
   * @param baseUri   Absolute URI of the root document (used to resolve relative imports).
   * @param fetchFn   Async function returning WDL source text for a given resolved URI.
   *                  Defaults to the global `fetch`.
   * @param validator Optional validator to run after loading.
   */
  public static async loadAsync(
    source: string,
    baseUri?: string,
    fetchFn: (uri: string) => Promise<string> = (uri) => fetch(uri).then((r) => r.text()),
    validator?: WdlValidator,
  ): Promise<WdlDocument> {
    const cache = await WdlImportResolverCache.preloadAsync(source, baseUri, fetchFn);
    return WdlV1Loader.loadFromString(source, validator, baseUri, cache);
  }
}
