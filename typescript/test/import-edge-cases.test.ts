import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import { WdlException, WdlSemanticValidator, WdlV1NodeLoader } from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

describe('TypeScript import edge cases', () => {
  for (const fixtureDir of [
    'duplicate_namespace',
    'namespace_conflicts_local',
    'member_alias_conflicts_local',
    'member_alias_duplicate',
  ]) {
    it(`import rejects ${fixtureDir}`, () => {
      const doc = WdlV1NodeLoader.loadFromFile(
        fixture('import_edge_cases', fixtureDir, 'root.wdl'),
      );
      expect(() => new WdlSemanticValidator().validateDocument(doc)).toThrow(WdlException);
    });
  }

  it('import accepts mixed_forms_ok', () => {
    const doc = WdlV1NodeLoader.loadFromFile(
      fixture('import_edge_cases', 'mixed_forms_ok', 'root.wdl'),
    );
    expect(() => new WdlSemanticValidator().validateDocument(doc)).not.toThrow();
  });
});
