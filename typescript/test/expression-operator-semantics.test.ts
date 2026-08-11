import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import {
  WdlException,
  WdlSemanticValidator,
  WdlStaticAnalysisSemanticValidator,
  WdlV1NodeLoader,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

describe('TypeScript expression operator semantics', () => {
  for (const name of [
    'logical_operand_type_fail.wdl',
    'numeric_operand_type_fail.wdl',
    'order_comparison_type_fail.wdl',
    'ternary_condition_type_fail.wdl',
  ]) {
    it(`static rejects ${name}`, () => {
      const doc = WdlV1NodeLoader.loadFromFile(fixture('expression_operator_semantics', name));
      expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
      expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(doc)).toThrow(
        WdlException,
      );
    });
  }

  it('accepts valid operator expressions', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('expression_operator_semantics', 'operators_ok.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(doc)).not.toThrow();
  });

  it('accepts operator precedence and compound equality', () => {
    const precedenceDoc = WdlV1NodeLoader.loadFromFile(
      fixture('expression_operator_semantics', 'operator_precedence_ok.wdl'),
    );
    const equalityDoc = WdlV1NodeLoader.loadFromFile(
      fixture('expression_operator_semantics', 'compound_equality_ok.wdl'),
    );
    expect(() =>
      new WdlStaticAnalysisSemanticValidator().validateDocument(precedenceDoc),
    ).not.toThrow();
    expect(() =>
      new WdlStaticAnalysisSemanticValidator().validateDocument(equalityDoc),
    ).not.toThrow();
  });

  it('rejects incompatible compound equality', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('expression_operator_semantics', 'compound_equality_incompatible_fail.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
    expect(() => new WdlStaticAnalysisSemanticValidator().validateDocument(doc)).toThrow(
      WdlException,
    );
  });
});
