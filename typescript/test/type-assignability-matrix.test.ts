import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import { WdlException, WdlSemanticValidator, WdlV1NodeLoader } from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

describe('TypeScript type assignability matrix', () => {
  for (const name of [
    'optional_from_none_ok.wdl',
    'array_nested_ok.wdl',
    'map_value_type_ok.wdl',
    'file_directory_from_string_ok.wdl',
    'struct_to_struct_coercion_ok.wdl',
  ]) {
    it(`assignability accepts ${name}`, () => {
      const doc = WdlV1NodeLoader.loadFromFile(fixture('type_assignability_matrix', name));
      expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
    });
  }

  for (const name of [
    'required_from_none_fail.wdl',
    'array_member_type_fail.wdl',
    'required_string_to_int_fail.wdl',
    'array_string_to_int_fail.wdl',
    'map_value_type_fail.wdl',
    'struct_to_struct_incompatible_fail.wdl',
  ]) {
    it(`assignability rejects ${name}`, () => {
      const doc = WdlV1NodeLoader.loadFromFile(fixture('type_assignability_matrix', name));
      expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
    });
  }
});
