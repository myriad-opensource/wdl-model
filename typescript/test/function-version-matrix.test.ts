import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import { WdlException, WdlSemanticValidator, WdlV1NodeLoader } from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

describe('TypeScript function version matrix', () => {
  for (const name of ['v11_keys_ok.wdl', 'v12_contains_ok.wdl', 'v13_value_ok.wdl']) {
    it(`accepts ${name}`, () => {
      const doc = WdlV1NodeLoader.loadFromFile(fixture('function_version_matrix', name));
      expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
    });
  }

  for (const name of [
    'v11_contains_key_fail.wdl',
    'v11_join_paths_fail.wdl',
    'v12_value_fail.wdl',
  ]) {
    it(`rejects ${name}`, () => {
      const doc = WdlV1NodeLoader.loadFromFile(fixture('function_version_matrix', name));
      expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
    });
  }
});
