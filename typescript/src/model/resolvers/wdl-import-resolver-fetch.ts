import { WdlImportException } from '../errors/wdl-import-exception.js';
import { WdlImportResolverBase } from './wdl-import-resolver-base.js';

/** Fetch-backed WDL import resolver for HTTP(S) sources. */
export class WdlImportResolverFetch extends WdlImportResolverBase {
  public constructor(private readonly fetchImpl: typeof fetch = fetch) {
    super();
  }

  protected loadHttp(
    _currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string {
    return this.loadFromHttp(importUri, originalImportLocation);
  }

  protected loadHttps(
    _currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string {
    return this.loadFromHttp(importUri, originalImportLocation);
  }

  protected loadFile(
    _currentDocumentLocation: string | undefined,
    _importUri: string,
    originalImportLocation: string,
  ): string {
    throw new WdlImportException(
      'Fetch resolver does not support file imports',
      originalImportLocation,
    );
  }

  protected loadBarePath(
    _currentDocumentLocation: string | undefined,
    _resolvedPath: string,
    originalImportLocation: string,
  ): string {
    throw new WdlImportException(
      'Fetch resolver does not support bare-path imports',
      originalImportLocation,
    );
  }

  private loadFromHttp(importUri: string, originalImportLocation: string): string {
    throw new WdlImportException(
      'Synchronous HTTP imports are not supported by fetch resolver in this runtime',
      originalImportLocation,
      new Error(`Cannot synchronously fetch ${importUri}`),
    );
  }
}
