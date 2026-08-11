import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

import { describe, expect, it } from 'vitest';

import { WdlImportException, WdlImportResolverFilesystem } from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

describe('TypeScript filesystem import resolver', () => {
  it('resolves relative path against current document location', () => {
    const root = fixture('resolver_filesystem', 'root.wdl');
    const resolver = new WdlImportResolverFilesystem();
    const text = resolver.resolveImport(pathToFileURL(root).toString(), 'sub/imported.wdl');
    expect(text).toBe('version 1.3\n');
  });

  it('resolves file scheme import', () => {
    const imported = fixture('resolver_filesystem', 'sub', 'imported.wdl');
    const resolver = new WdlImportResolverFilesystem();
    const text = resolver.resolveImport(undefined, pathToFileURL(imported).toString());
    expect(text).toBe('version 1.3\n');
  });

  it('rejects http imports', () => {
    const resolver = new WdlImportResolverFilesystem();
    expect(() =>
      resolver.resolveImport('file:///tmp/root.wdl', 'http://example.com/a.wdl'),
    ).toThrow(WdlImportException);
  });

  it('rejects https imports', () => {
    const resolver = new WdlImportResolverFilesystem();
    expect(() =>
      resolver.resolveImport('file:///tmp/root.wdl', 'https://example.com/a.wdl'),
    ).toThrow(WdlImportException);
  });
});
