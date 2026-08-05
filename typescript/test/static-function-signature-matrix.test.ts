import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import {
  WdlException,
  WdlSemanticValidator,
  WdlStaticAnalysisSemanticValidator,
  WdlV1NodeLoader,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

describe('TypeScript static function signature matrix', () => {
  for (const name of [
    'keys_bad.wdl',
    'range_bad.wdl',
    'contains_bad.wdl',
    'chunk_bad.wdl',
    'cross_bad.wdl',
    'join_paths_bad_first.wdl',
    'join_paths_bad_tail.wdl',
    'basename_bad_first.wdl',
    'size_bad_second.wdl',
  ]) {
    it(`static rejects ${name}`, () => {
      const doc = WdlV1NodeLoader.loadFromFile(fixture('static_function_signature_matrix', name));
      expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
      expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(doc)).toThrow(
        WdlException,
      );
    });
  }

  it('accepts valid signatures under static validator', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('static_function_signature_matrix', 'static_signatures_ok.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(doc)).not.toThrow();
  });
});
