import { readFileSync } from 'node:fs';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import {
  WdlException,
  WdlLintingSemanticValidator,
  WdlSemanticError,
  WdlSemanticErrorCode,
  WdlSemanticSeverity,
  WdlV1Loader,
  WdlV1NodeLoader,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);
const fixtureText = (...parts: string[]): string => readFileSync(fixture(...parts), 'utf8');

describe('TypeScript deprecation linting', () => {
  for (const filename of [
    'runtime_section_deprecated.wdl',
    'object_type_deprecated.wdl',
    'placeholder_options_deprecated.wdl',
    'file_scheme_import_deprecated.wdl',
  ]) {
    it(`reports deprecation warning for ${filename}`, () => {
      const doc =
        filename === 'file_scheme_import_deprecated.wdl'
          ? WdlV1Loader.loadFromString(fixtureText('deprecations', filename))
          : WdlV1NodeLoader.loadFromFile(fixture('deprecations', filename));

      expect(() => new WdlLintingSemanticValidator().validateDocument(doc)).toThrow(WdlException);

      try {
        new WdlLintingSemanticValidator().validateDocument(doc);
      } catch (error) {
        const semanticErrors = (error as WdlException)
          .getErrors()
          .filter((entry) => entry instanceof WdlSemanticError) as WdlSemanticError[];
        expect(
          semanticErrors.some(
            (entry) =>
              entry.code === WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE &&
              entry.severity() === WdlSemanticSeverity.WARNING,
          ),
        ).toBe(true);
      }
    });
  }

  it('does not report deprecation warnings for non-deprecated fixture', () => {
    const doc = WdlV1NodeLoader.loadFromFile(fixture('deprecations', 'no_deprecations.wdl'));
    const lint = new WdlLintingSemanticValidator().setThrowOnWarnings(false);
    expect(() => lint.validateDocument(doc)).not.toThrow();
  });
});
