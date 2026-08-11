import { readFileSync } from 'node:fs';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import {
  WdlException,
  WdlLintingSemanticValidator,
  WdlSemanticError,
  WdlSemanticSeverity,
  WdlSemanticValidator,
  WdlStaticAnalysisSemanticValidator,
  WdlV1Loader,
  WdlV1NodeLoader,
} from '../src/index.js';

const specExample = (version: string, filename: string): string =>
  readFileSync(join(process.cwd(), 'spec_examples', version, filename), 'utf8');

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

const fixtureText = (...parts: string[]): string => readFileSync(fixture(...parts), 'utf8');

describe('TypeScript WDL validators', () => {
  it('rejects known parse-ok fail examples for baseline validator', () => {
    const parseOkFails = [
      'empty_array_fail.wdl',
      'illegal_access_fail.wdl',
      'non_empty_optional_fail.wdl',
      'private_declaration_fail.wdl',
      'select_first_empty_fail.wdl',
      'select_first_only_none_fail.wdl',
      'test_as_map_fail.wdl',
      'test_map_fail.wdl',
      'test_zip_fail.wdl',
      'write_json_fail.wdl',
    ];
    const validator = new WdlSemanticValidator();
    for (const filename of parseOkFails) {
      const document = WdlV1Loader.loadFromString(specExample('v1_3', filename));
      expect(() => validator.validateDocument(document)).toThrow(WdlException);
    }
  });

  it('accepts a simple valid workflow', () => {
    const source = fixtureText('validator', 'accepts_simple_valid_workflow.wdl');
    const document = WdlV1Loader.loadFromString(source);
    expect(() => new WdlSemanticValidator().validateDocument(document)).not.toThrow();
  });

  it('loader runs validator when provided and throws semantic errors', () => {
    const source = specExample('v1_3', 'select_first_empty_fail.wdl');
    expect(() => WdlV1Loader.loadFromString(source, new WdlSemanticValidator())).toThrow(
      WdlException,
    );
  });

  it('normal validator rejects function not available in document version', () => {
    const source = fixtureText('validator', 'function_version_invalid.wdl');
    const document = WdlV1Loader.loadFromString(source);
    expect(() => new WdlSemanticValidator().validateDocument(document)).toThrow(WdlException);
  });

  it('static validator catches additional static function signature errors', () => {
    const source = fixtureText('validator', 'static_function_signature_bad.wdl');
    const document = WdlV1Loader.loadFromString(source);
    expect(() => new WdlSemanticValidator().validateDocument(document)).not.toThrow();
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(document)).toThrow(
      WdlException,
    );
  });

  it('static validator catches additional workflow structure errors', () => {
    const source = fixtureText('validator', 'static_workflow_structure_bad.wdl');
    const document = WdlV1Loader.loadFromString(source);
    expect(() => new WdlSemanticValidator().validateDocument(document)).not.toThrow();
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(document)).toThrow(
      WdlException,
    );
  });

  it('static validator catches nested workflow structure errors', () => {
    const source = fixtureText('validator', 'nested_workflow_structure_bad.wdl');
    const document = WdlV1Loader.loadFromString(source);
    expect(() => new WdlSemanticValidator().validateDocument(document)).not.toThrow();
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(document)).toThrow(
      WdlException,
    );
  });

  it('linting validator catches unused symbols', () => {
    const source = fixtureText('validator', 'lint_unused_symbols_bad.wdl');
    const document = WdlV1Loader.loadFromString(source);
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(document)).not.toThrow();
    try {
      new WdlLintingSemanticValidator().validateDocument(document);
      throw new Error('Expected linting validator to throw');
    } catch (error) {
      expect(error).toBeInstanceOf(WdlException);
      const semanticErrors = (error as WdlException)
        .getErrors()
        .filter((entry) => entry instanceof WdlSemanticError) as WdlSemanticError[];
      expect(semanticErrors.length).toBeGreaterThan(0);
      expect(semanticErrors[0]?.severity()).toBe(WdlSemanticSeverity.WARNING);
    }
  });

  it('linting validator can skip throw on warnings', () => {
    const source = fixtureText('validator', 'lint_unused_symbols_bad.wdl');
    const document = WdlV1Loader.loadFromString(source);
    expect(() =>
      new WdlLintingSemanticValidator().setThrowOnWarnings(false).validateDocument(document),
    ).not.toThrow();
  });

  it('import validation accepts namespaced and member alias visibility', () => {
    const rootPath = fixture('import_validation', 'standard_alias', 'root.wdl');
    const document = WdlV1NodeLoader.loadFromFile(rootPath);
    expect(() => new WdlSemanticValidator().validateDocument(document)).not.toThrow();
  });

  it('import validation rejects missing selected member imports', () => {
    const rootPath = fixture('import_validation', 'unknown_member', 'root.wdl');
    const document = WdlV1NodeLoader.loadFromFile(rootPath);
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(document)).toThrow(
      WdlException,
    );
  });
});
