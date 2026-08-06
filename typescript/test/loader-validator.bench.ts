import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { bench, describe } from 'vitest';

import {
  WdlImportResolverFilesystem,
  WdlLintingSemanticValidator,
  WdlSemanticValidator,
  WdlV1Loader,
  WdlV1NodeLoader,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);
const fixtureText = (...parts: string[]): string => readFileSync(fixture(...parts), 'utf8');

const simpleSource = fixtureText('validator', 'loader_valid_document.wdl');
const importRootPath = fixture('loader_imports', 'recursive', 'root.wdl');

const parsedForValidation = WdlV1Loader.loadFromString(simpleSource);
const semanticValidator = new WdlSemanticValidator(false);
const lintValidator = new WdlLintingSemanticValidator(false);

describe('loader', () => {
  bench('load simple source string', () => {
    WdlV1Loader.loadFromString(simpleSource);
  });

  bench('load recursive imports from filesystem', () => {
    WdlV1NodeLoader.loadFromFile(importRootPath, undefined, new WdlImportResolverFilesystem());
  });
});

describe('validator', () => {
  bench('semantic validation on parsed document', () => {
    semanticValidator.validateDocument(parsedForValidation);
  });

  bench('lint validation on parsed document', () => {
    lintValidator.validateDocument(parsedForValidation);
  });
});
