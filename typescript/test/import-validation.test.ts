import { existsSync } from 'node:fs';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import { WdlException, WdlSemanticValidator, WdlV1NodeLoader } from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);
const specExample = (version: string, filename: string): string =>
  join(process.cwd(), 'spec_examples', version, filename);

const positiveImportExamples = ['call_example.wdl', 'call_imported.wdl'];
const negativeImportExamples = [
  'call_subworkflow_fail.wdl',
  'incomplete_struct_fail.wdl',
  'illegal_access_fail.wdl',
];

describe('TypeScript import validation', () => {
  for (const version of ['v1_1', 'v1_2', 'v1_3']) {
    for (const filename of positiveImportExamples) {
      const path = specExample(version, filename);
      if (!existsSync(path)) continue;
      it(`validates positive import spec example ${version}/${filename}`, () => {
        const doc = WdlV1NodeLoader.loadFromFile(path);
        expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
      });
    }
  }

  for (const version of ['v1_1', 'v1_2', 'v1_3']) {
    for (const filename of negativeImportExamples) {
      const path = specExample(version, filename);
      if (!existsSync(path)) continue;
      it(`rejects negative import spec example ${version}/${filename}`, () => {
        expect(() => {
          const doc = WdlV1NodeLoader.loadFromFile(path);
          new WdlSemanticValidator().validateDocument(doc);
        }).toThrow(WdlException);
      });
    }
  }

  it('validates star and members import forms', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('import_validation', 'star_members', 'root.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
  });

  it('validates standard import struct aliases', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('import_validation', 'standard_alias', 'root.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
  });

  it('rejects unknown member import', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('import_validation', 'unknown_member', 'root.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
  });

  it('rejects duplicate import namespaces', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('import_validation', 'duplicate_namespace', 'root.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
  });

  it('rejects import alias target that does not exist', () => {
    const doc = WdlV1NodeLoader.loadFromFile(fixture('import_validation', 'bad_alias', 'root.wdl'));
    expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
  });

  it('rejects incompatible imported structs without alias', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('import_validation', 'struct_conflict', 'root.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
  });

  it('rejects import from higher minor version', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('import_validation', 'version_mismatch', 'root.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
  });
});
