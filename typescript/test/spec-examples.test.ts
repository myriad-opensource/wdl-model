import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import { WdlException, WdlSemanticValidator, WdlV1Loader } from '../src/index.js';

const repoRoot = process.cwd();
const specExamplesDir = join(repoRoot, 'spec_examples');

const failsThatShouldParseOkV11 = new Set([
  'select_first_only_none_fail.wdl',
  'empty_array_fail.wdl',
  'test_as_map_fail.wdl',
  'write_json_fail.wdl',
  'test_map_fail.wdl',
  'select_first_empty_fail.wdl',
  'private_declaration_fail.wdl',
  'non_empty_optional_fail.wdl',
  'test_zip_fail.wdl',
]);

const failsThatShouldParseOkV12 = new Set([
  'select_first_only_none_fail.wdl',
  'empty_array_fail.wdl',
  'test_as_map_fail.wdl',
  'write_json_fail.wdl',
  'test_map_fail.wdl',
  'select_first_empty_fail.wdl',
  'private_declaration_fail.wdl',
  'non_empty_optional_fail.wdl',
  'test_zip_fail.wdl',
  'illegal_access_fail.wdl',
]);

const parseFailuresExpectedWithReservedKeywordsV12 = new Set([
  'test_find_task.wdl',
  'test_meta_values.wdl',
  'test_runtime_info_task.wdl',
]);

const failsThatShouldParseOkV13 = new Set([
  'select_first_only_none_fail.wdl',
  'empty_array_fail.wdl',
  'test_as_map_fail.wdl',
  'write_json_fail.wdl',
  'test_map_fail.wdl',
  'select_first_empty_fail.wdl',
  'private_declaration_fail.wdl',
  'non_empty_optional_fail.wdl',
  'test_zip_fail.wdl',
  'illegal_access_fail.wdl',
]);

const parseFailuresExpectedWithReservedKeywordsV13 = new Set([
  'test_find_task.wdl',
  'test_meta_values.wdl',
  'test_runtime_info_task.wdl',
  'test_task_previous.wdl',
]);

const loadExamples = (version: string): string[] => {
  const versionDir = join(specExamplesDir, version);
  return readdirSync(versionDir)
    .filter((name) => name.endsWith('.wdl'))
    .sort();
};

const loadFailExamples = (version: string): string[] =>
  loadExamples(version).filter((name) => name.endsWith('_fail.wdl'));

const readExample = (version: string, filename: string): string =>
  readFileSync(join(specExamplesDir, version, filename), 'utf8');

const assertParseSpecExample = (
  version: string,
  filename: string,
  failsThatShouldParseOk: Set<string>,
): void => {
  const source = readExample(version, filename);
  try {
    const document = WdlV1Loader.loadFromString(source);
    expect(document).toBeDefined();
    expect(document.elements()).toBeDefined();
    if (filename.endsWith('_fail.wdl') && !failsThatShouldParseOk.has(filename)) {
      throw new Error(`Parsed but failure expected: ${filename}`);
    }
  } catch (error) {
    const reservedKeywordParseFailures =
      version === 'v1_3'
        ? parseFailuresExpectedWithReservedKeywordsV13
        : version === 'v1_2'
          ? parseFailuresExpectedWithReservedKeywordsV12
          : new Set<string>();
    if (!filename.endsWith('_fail.wdl') && !reservedKeywordParseFailures.has(filename)) {
      throw error;
    }
  }
};

const assertParseAndValidateFailSpecExample = (version: string, filename: string): void => {
  const source = readExample(version, filename);
  expect(() => WdlV1Loader.loadFromString(source, new WdlSemanticValidator())).toThrow(
    WdlException,
  );
};

describe('TypeScript WDL spec examples', () => {
  for (const filename of loadExamples('v1_1')) {
    it(`parses v1.1 example ${filename}`, () => {
      assertParseSpecExample('v1_1', filename, failsThatShouldParseOkV11);
    });
  }

  for (const filename of loadExamples('v1_2')) {
    it(`parses v1.2 example ${filename}`, () => {
      assertParseSpecExample('v1_2', filename, failsThatShouldParseOkV12);
    });
  }

  for (const filename of loadExamples('v1_3')) {
    it(`parses v1.3 example ${filename}`, () => {
      assertParseSpecExample('v1_3', filename, failsThatShouldParseOkV13);
    });
  }

  for (const filename of loadFailExamples('v1_1')) {
    it(`parse+validate rejects v1.1 fail example ${filename}`, () => {
      assertParseAndValidateFailSpecExample('v1_1', filename);
    });
  }

  for (const filename of loadFailExamples('v1_2')) {
    it(`parse+validate rejects v1.2 fail example ${filename}`, () => {
      assertParseAndValidateFailSpecExample('v1_2', filename);
    });
  }

  for (const filename of loadFailExamples('v1_3')) {
    it(`parse+validate rejects v1.3 fail example ${filename}`, () => {
      assertParseAndValidateFailSpecExample('v1_3', filename);
    });
  }
});
