import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

import { describe, expect, it } from 'vitest';

import {
  WdlImportResolverFilesystem,
  WdlSemanticValidator,
  WdlV1Loader,
  WdlV1NodeLoader,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

describe('TypeScript loader import resolution', () => {
  it('recursively loads imported documents into map', () => {
    const root = fixture('loader_imports', 'recursive', 'root.wdl');
    const child = fixture('loader_imports', 'recursive', 'child.wdl');
    const grandchild = fixture('loader_imports', 'recursive', 'grandchild.wdl');

    const rootDoc = WdlV1NodeLoader.loadFromFile(root);

    expect(rootDoc.importedDocuments().size).toBe(1);
    const childDoc = [...rootDoc.importedDocuments().values()][0];
    expect(childDoc).toBeDefined();
    expect(new URL(childDoc!.getSourceLocation()!).pathname).toBe(resolve(child));

    expect(childDoc!.importedDocuments().size).toBe(1);
    const grandchildDoc = [...childDoc!.importedDocuments().values()][0];
    expect(grandchildDoc).toBeDefined();
    expect(new URL(grandchildDoc!.getSourceLocation()!).pathname).toBe(resolve(grandchild));

    expect(rootDoc.importStatements()[0]?.getSourceText()).toBeDefined();
    expect(childDoc!.importStatements()[0]?.getSourceText()).toBeDefined();
  });

  it('loads from source code with source location resolver then validator', () => {
    const root = fixture('loader_imports', 'string_input', 'root.wdl');
    const rootSource = readFileSync(root, 'utf8');

    const rootDoc = WdlV1Loader.loadFromString(
      rootSource,
      new WdlSemanticValidator(),
      pathToFileURL(root).toString(),
      new WdlImportResolverFilesystem(),
    );

    expect(rootDoc.importStatements().length).toBe(1);
    expect(rootDoc.importedDocuments().size).toBe(1);
    expect([...rootDoc.importedDocuments().values()][0]).toBeDefined();
  });
});
