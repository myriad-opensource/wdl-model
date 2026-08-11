import { readFileSync } from 'node:fs';
import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import {
  WdlBinaryOperation,
  WdlBinaryOperator,
  WdlBooleanLiteral,
  WdlBoundDeclaration,
  WdlException,
  WdlIntLiteral,
  WdlV1Loader,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);
const fixtureText = (...parts: string[]): string => readFileSync(fixture(...parts), 'utf8');

const firstWorkflowDeclaration = (fixtureName: string): WdlBoundDeclaration => {
  const document = WdlV1Loader.loadFromString(fixtureText('grammar_behavior', fixtureName));
  const declaration = document.workflows()[0]?.getElements()[0];
  expect(declaration).toBeInstanceOf(WdlBoundDeclaration);
  return declaration as WdlBoundDeclaration;
};

describe('Grammar behavior fixtures', () => {
  it('parses additive chain as left-associative', () => {
    const declaration = firstWorkflowDeclaration('associativity_additive_chain.wdl');

    const root = declaration.getExpression();
    expect(root).toBeInstanceOf(WdlBinaryOperation);
    expect((root as WdlBinaryOperation).getOperator()).toBe(WdlBinaryOperator.SUBTRACT);
    expect((root as WdlBinaryOperation).getRight()).toBeInstanceOf(WdlIntLiteral);
    expect(((root as WdlBinaryOperation).getRight() as WdlIntLiteral).getValue()).toBe(3);

    const left = (root as WdlBinaryOperation).getLeft();
    expect(left).toBeInstanceOf(WdlBinaryOperation);
    expect((left as WdlBinaryOperation).getOperator()).toBe(WdlBinaryOperator.SUBTRACT);
    expect((left as WdlBinaryOperation).getLeft()).toBeInstanceOf(WdlIntLiteral);
    expect(((left as WdlBinaryOperation).getLeft() as WdlIntLiteral).getValue()).toBe(1);
    expect((left as WdlBinaryOperation).getRight()).toBeInstanceOf(WdlIntLiteral);
    expect(((left as WdlBinaryOperation).getRight() as WdlIntLiteral).getValue()).toBe(2);
  });

  it('parses multiplicative chain as left-associative', () => {
    const declaration = firstWorkflowDeclaration('associativity_multiplicative_chain.wdl');

    const root = declaration.getExpression();
    expect(root).toBeInstanceOf(WdlBinaryOperation);
    expect((root as WdlBinaryOperation).getOperator()).toBe(WdlBinaryOperator.DIVIDE);
    expect((root as WdlBinaryOperation).getRight()).toBeInstanceOf(WdlIntLiteral);
    expect(((root as WdlBinaryOperation).getRight() as WdlIntLiteral).getValue()).toBe(2);

    const left = (root as WdlBinaryOperation).getLeft();
    expect(left).toBeInstanceOf(WdlBinaryOperation);
    expect((left as WdlBinaryOperation).getOperator()).toBe(WdlBinaryOperator.DIVIDE);
    expect((left as WdlBinaryOperation).getLeft()).toBeInstanceOf(WdlIntLiteral);
    expect(((left as WdlBinaryOperation).getLeft() as WdlIntLiteral).getValue()).toBe(8);
    expect((left as WdlBinaryOperation).getRight()).toBeInstanceOf(WdlIntLiteral);
    expect(((left as WdlBinaryOperation).getRight() as WdlIntLiteral).getValue()).toBe(4);
  });

  it('parses logical-or chain as left-associative', () => {
    const declaration = firstWorkflowDeclaration('associativity_logical_or_chain.wdl');

    const root = declaration.getExpression();
    expect(root).toBeInstanceOf(WdlBinaryOperation);
    expect((root as WdlBinaryOperation).getOperator()).toBe(WdlBinaryOperator.OR);
    expect((root as WdlBinaryOperation).getRight()).toBeInstanceOf(WdlBooleanLiteral);
    expect(((root as WdlBinaryOperation).getRight() as WdlBooleanLiteral).getValue()).toBe(true);

    const left = (root as WdlBinaryOperation).getLeft();
    expect(left).toBeInstanceOf(WdlBinaryOperation);
    expect((left as WdlBinaryOperation).getOperator()).toBe(WdlBinaryOperator.OR);
    expect((left as WdlBinaryOperation).getLeft()).toBeInstanceOf(WdlBooleanLiteral);
    expect(((left as WdlBinaryOperation).getLeft() as WdlBooleanLiteral).getValue()).toBe(true);
    expect((left as WdlBinaryOperation).getRight()).toBeInstanceOf(WdlBooleanLiteral);
    expect(((left as WdlBinaryOperation).getRight() as WdlBooleanLiteral).getValue()).toBe(false);
  });

  for (const fixtureName of [
    'keyword_decl_identifier_task.wdl',
    'keyword_decl_identifier_if.wdl',
    'keyword_task_input_in.wdl',
    'keyword_metadata_key_version.wdl',
  ]) {
    it(`rejects reserved keyword fixture ${fixtureName}`, () => {
      expect(() =>
        WdlV1Loader.loadFromString(fixtureText('grammar_behavior', fixtureName)),
      ).toThrow(WdlException);
    });
  }
});
