/** Cache-backed synchronous import resolver for use with async pre-loading. */
import { WdlImportException } from '../errors/wdl-import-exception.js';
import { WdlImportResolverBase } from './wdl-import-resolver-base.js';

/**
 * Synchronous import resolver backed by a pre-populated content cache.
 *
 * Pair this with {@link WdlImportResolverCache.preloadAsync} to resolve imports asynchronously
 * before handing control to the synchronous ANTLR-based parser. Typical use in a browser editor:
 *
 * ```ts
 * const cache = await WdlImportResolverCache.preloadAsync(
 *   source,
 *   baseUrl,
 *   (url) => fetch(url).then((r) => r.text()),
 * );
 * const doc = await WdlV1WebLoader.loadFromString(source, baseUrl, undefined, cache);
 * ```
 */
export class WdlImportResolverCache extends WdlImportResolverBase {
  /** Creates a cache resolver from a pre-populated map of resolved URI → WDL source text. */
  public constructor(private readonly cache: ReadonlyMap<string, string>) {
    super();
  }

  /**
   * Recursively pre-fetches all transitive imports reachable from `source` using the supplied
   * async fetch function, then returns a populated `WdlImportResolverCache`.
   *
   * @param source      WDL source text of the root document.
   * @param baseUri     Resolved URI of the root document (used to resolve relative imports).
   * @param fetchFn     Async function that fetches WDL source text from a resolved URI string.
   */
  public static async preloadAsync(
    source: string,
    baseUri: string | undefined,
    fetchFn: (uri: string) => Promise<string>,
  ): Promise<WdlImportResolverCache> {
    const cache = new Map<string, string>();
    const pending = new Set<string>();
    await WdlImportResolverCache.collectImports(source, baseUri, fetchFn, cache, pending);
    return new WdlImportResolverCache(cache);
  }

  protected loadHttp(
    _currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string {
    return this.fromCache(importUri, originalImportLocation);
  }

  protected loadHttps(
    _currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string {
    return this.fromCache(importUri, originalImportLocation);
  }

  protected loadFile(
    _currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string {
    return this.fromCache(importUri, originalImportLocation);
  }

  protected loadBarePath(
    _currentDocumentLocation: string | undefined,
    resolvedPath: string,
    originalImportLocation: string,
  ): string {
    return this.fromCache(resolvedPath, originalImportLocation);
  }

  private fromCache(key: string, originalImportLocation: string): string {
    const content = this.cache.get(key);
    if (content === undefined) {
      throw new WdlImportException(
        `Import not pre-loaded in cache: ${key}`,
        originalImportLocation,
      );
    }
    return content;
  }

  // -------------------------------------------------------------------------
  // Async import discovery
  // -------------------------------------------------------------------------

  private static async collectImports(
    source: string,
    baseUri: string | undefined,
    fetchFn: (uri: string) => Promise<string>,
    cache: Map<string, string>,
    visiting: Set<string>,
  ): Promise<void> {
    const uris = WdlImportResolverCache.scanImportUris(source, baseUri);
    // fetch all URIs in parallel at each level
    await Promise.all(
      uris.map(async (uri) => {
        if (cache.has(uri) || visiting.has(uri)) return;
        visiting.add(uri);
        let content: string;
        try {
          content = await fetchFn(uri);
        } catch (err) {
          throw new WdlImportException(`Failed to fetch import: ${uri}`, uri, err);
        }
        cache.set(uri, content);
        await WdlImportResolverCache.collectImports(content, uri, fetchFn, cache, visiting);
      }),
    );
  }

  /**
   * Extracts raw import URI strings from WDL source text using a lightweight regex scan,
   * then resolves them to absolute URIs relative to `baseUri`.
   */
  static scanImportUris(source: string, baseUri: string | undefined): string[] {
    // Matches: import "uri", import 'uri', import { ... } from "uri", import * from "uri"
    const pattern = /\bimport\b(?:[^"']*from\s+)?['"]([^'"]+)['"]/g;
    const uris: string[] = [];
    let match: RegExpExecArray | null;
    while ((match = pattern.exec(source)) !== null) {
      const rawUri = match[1];
      if (!rawUri) continue;
      const resolved = WdlImportResolverCache.resolveUri(rawUri, baseUri);
      if (resolved) uris.push(resolved);
    }
    return uris;
  }

  private static resolveUri(rawUri: string, baseUri: string | undefined): string | undefined {
    try {
      // absolute URI — use as-is
      if (/^[a-zA-Z][a-zA-Z0-9+\-.]*:/.test(rawUri)) return rawUri;
      if (!baseUri) return rawUri.startsWith('/') ? `file://${rawUri}` : rawUri;
      return new URL(rawUri, baseUri).toString();
    } catch {
      return undefined;
    }
  }
}
