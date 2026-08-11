import { WdlImportException } from '../errors/wdl-import-exception.js';

/** Transport-agnostic base resolver for WDL imports. */
export abstract class WdlImportResolverBase {
  /** Resolves an import reference and loads its source content. */
  public resolveImport(
    currentDocumentLocation: string | undefined,
    importLocation: string,
  ): string {
    const resolved = this.resolveImportLocation(currentDocumentLocation, importLocation);
    return this.dispatchImport(currentDocumentLocation, resolved, importLocation);
  }

  /** Resolves an import reference into a stable location identifier. */
  public resolveImportLocation(
    currentDocumentLocation: string | undefined,
    importLocation: string,
  ): string {
    return this.resolveImportUri(currentDocumentLocation, importLocation);
  }

  /** Dispatches loading behavior based on URI protocol. */
  protected dispatchImport(
    currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string {
    const parsed = this.tryParseUrl(importUri);
    const scheme = parsed?.protocol?.replace(':', '').toLowerCase() ?? '';

    if (!scheme) {
      return this.loadBarePath(currentDocumentLocation, importUri, originalImportLocation);
    }
    if (scheme === 'http') {
      return this.loadHttp(currentDocumentLocation, importUri, originalImportLocation);
    }
    if (scheme === 'https') {
      return this.loadHttps(currentDocumentLocation, importUri, originalImportLocation);
    }
    if (scheme === 'file') {
      return this.loadFile(currentDocumentLocation, importUri, originalImportLocation);
    }
    throw new WdlImportException(
      `Unsupported import URI protocol: ${scheme}`,
      originalImportLocation,
    );
  }

  protected abstract loadHttp(
    currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string;

  protected abstract loadHttps(
    currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string;

  protected abstract loadFile(
    currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string;

  protected abstract loadBarePath(
    currentDocumentLocation: string | undefined,
    resolvedPath: string,
    originalImportLocation: string,
  ): string;

  protected resolveImportUri(
    currentDocumentLocation: string | undefined,
    importLocation: string,
  ): string {
    const parsedImport = this.tryParseUrl(importLocation);
    if (parsedImport?.protocol) {
      return importLocation;
    }

    if (!currentDocumentLocation) {
      if (importLocation.startsWith('/')) {
        return new URL(`file://${importLocation}`).toString();
      }
      return importLocation;
    }

    const base = this.tryParseUrl(currentDocumentLocation);
    const baseScheme = base?.protocol?.replace(':', '').toLowerCase();

    if (baseScheme === 'http' || baseScheme === 'https') {
      return new URL(importLocation, currentDocumentLocation).toString();
    }

    if (baseScheme === 'file') {
      return new URL(importLocation, currentDocumentLocation).toString();
    }

    const baseDir = this.dirname(currentDocumentLocation);
    return this.normalizePathJoin(baseDir, importLocation);
  }

  protected ioFailure(message: string, importLocation: string, cause: unknown): WdlImportException {
    return new WdlImportException(message, importLocation, cause);
  }

  private tryParseUrl(value: string): URL | undefined {
    try {
      return new URL(value);
    } catch {
      return undefined;
    }
  }

  private dirname(path: string): string {
    const normalized = path.replace(/\\/g, '/');
    const idx = normalized.lastIndexOf('/');
    if (idx < 0) return '';
    return normalized.slice(0, idx);
  }

  private normalizePathJoin(baseDir: string, child: string): string {
    if (!baseDir) return child;
    if (child.startsWith('/')) return child;
    return `${baseDir}/${child}`.replace(/\/+/g, '/');
  }
}
