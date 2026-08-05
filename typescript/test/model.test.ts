import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

import {
  WdlException,
  WdlDocument,
  WdlImportResolverFilesystem,
  WdlV1NodeLoader,
  WdlPrimitiveType,
  WdlStaticAnalysisSemanticValidator,
  WdlTask,
  WdlV1Loader,
  WdlV1WebLoader,
  WdlVersion,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

const fixtureText = (...parts: string[]): string => readFileSync(fixture(...parts), 'utf8');

describe('TypeScript WDL model scaffold', () => {
  it('creates a document with tasks', () => {
    const document = new WdlDocument(WdlVersion.V1_3);
    const task = new WdlTask('hello');
    document.elements().push(task);

    expect(document.getWdlVersion()).toBe(WdlVersion.V1_3);
    expect(document.tasks()).toHaveLength(1);
    expect(new WdlPrimitiveType(WdlPrimitiveType.Type.STRING).primitiveType()).toBe(
      WdlPrimitiveType.Type.STRING,
    );
  });

  it('parses a simple task and workflow document', () => {
    const document = WdlV1Loader.loadFromString(
      fixtureText('validator', 'loader_valid_document.wdl'),
    );
    expect(document.getWdlVersion()).toBe(WdlVersion.V1_3);
    expect(document.tasks()).toHaveLength(1);
    expect(document.workflows()).toHaveLength(1);
    expect(document.tasks()[0]?.getName()).toBe('t');
    expect(document.workflows()[0]?.getName()).toBe('ok');
  });

  it('loads from a Node file helper', () => {
    const path = fixture('validator', 'loader_valid_document.wdl');
    const document = WdlV1NodeLoader.loadFromFile(path);
    expect(document.getWdlVersion()).toBe(WdlVersion.V1_3);
    expect(document.workflows()).toHaveLength(1);
  });

  it('loads recursive imports with filesystem resolver in Node loader', () => {
    const path = fixture('loader_imports', 'recursive', 'root.wdl');
    const document = WdlV1NodeLoader.loadFromFile(
      path,
      undefined,
      new WdlImportResolverFilesystem(),
    );
    expect(document.importedDocuments().size).toBe(1);

    const childKey = pathToFileURL(
      resolve(fixture('loader_imports', 'recursive', 'child.wdl')),
    ).toString();
    const child = document.importedDocuments().get(childKey);
    expect(child).toBeDefined();
    expect(child?.importedDocuments().size).toBe(1);
  });

  it('loads from a web file helper', async () => {
    const file = new File([fixtureText('validator', 'loader_valid_document.wdl')], 'example.wdl', {
      type: 'text/plain',
    });
    const document = await WdlV1WebLoader.loadFromFile(file);
    expect(document.getWdlVersion()).toBe(WdlVersion.V1_3);
    expect(document.workflows()).toHaveLength(1);
  });

  it('web loader does not resolve imports unless an explicit resolver is supplied', async () => {
    const file = new File([fixtureText('loader_imports', 'string_input', 'root.wdl')], 'root.wdl', {
      type: 'text/plain',
    });
    await expect(
      WdlV1WebLoader.loadFromFile(file, new WdlStaticAnalysisSemanticValidator()),
    ).rejects.toBeInstanceOf(WdlException);
  });
});
