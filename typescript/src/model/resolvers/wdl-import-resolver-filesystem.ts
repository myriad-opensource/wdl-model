import { readFileSync } from 'node:fs';

import { WdlImportException } from '../errors/wdl-import-exception.js';
import { WdlImportResolverBase } from './wdl-import-resolver-base.js';

/** Filesystem-only import resolver for local paths and file:// URIs. */
export class WdlImportResolverFilesystem extends WdlImportResolverBase {
  protected loadHttp(
    _currentDocumentLocation: string | undefined,
    _importUri: string,
    originalImportLocation: string,
  ): string {
    throw new WdlImportException(
      'Filesystem resolver does not support http imports',
      originalImportLocation,
    );
  }

  protected loadHttps(
    _currentDocumentLocation: string | undefined,
    _importUri: string,
    originalImportLocation: string,
  ): string {
    throw new WdlImportException(
      'Filesystem resolver does not support https imports',
      originalImportLocation,
    );
  }

  protected loadFile(
    _currentDocumentLocation: string | undefined,
    importUri: string,
    originalImportLocation: string,
  ): string {
    try {
      const path = new URL(importUri).pathname;
      return readFileSync(path, 'utf8');
    } catch (error) {
      throw this.ioFailure('Unable to read file import', originalImportLocation, error);
    }
  }

  protected loadBarePath(
    _currentDocumentLocation: string | undefined,
    resolvedPath: string,
    originalImportLocation: string,
  ): string {
    if (!resolvedPath.trim()) {
      throw new WdlImportException('Invalid filesystem import path', originalImportLocation);
    }
    try {
      return readFileSync(resolvedPath, 'utf8');
    } catch (error) {
      throw this.ioFailure('Unable to read filesystem import', originalImportLocation, error);
    }
  }
}
