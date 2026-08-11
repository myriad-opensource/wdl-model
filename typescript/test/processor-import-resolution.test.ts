import { join } from 'node:path';

import { describe, expect, it } from 'vitest';

import {
  type ResolvedImport,
  WdlDocument,
  WdlEnum,
  WdlImport,
  WdlProcessorBase,
  WdlStruct,
  WdlTask,
  WdlV1NodeLoader,
  WdlWorkflow,
} from '../src/index.js';

const fixture = (...parts: string[]): string => join(process.cwd(), 'wdl_tests', ...parts);

class ProbeProcessor extends WdlProcessorBase {
  public tasks(doc: WdlDocument, callTarget: string): ResolvedImport<WdlTask>[] {
    return this.resolveImportedTasks(doc, callTarget);
  }

  public workflows(doc: WdlDocument, callTarget: string): ResolvedImport<WdlWorkflow>[] {
    return this.resolveImportedWorkflows(doc, callTarget);
  }

  public structs(doc: WdlDocument, typeName: string): ResolvedImport<WdlStruct>[] {
    return this.resolveImportedStructs(doc, typeName);
  }

  public enums(doc: WdlDocument, typeName: string): ResolvedImport<WdlEnum>[] {
    return this.resolveImportedEnums(doc, typeName);
  }

  public importedDoc(doc: WdlDocument, imp: WdlImport): WdlDocument | undefined {
    return this.resolveImportedDocument(doc, imp);
  }
}

describe('TypeScript processor import resolution helpers', () => {
  it('resolves imported call targets and types across import forms', () => {
    const root = fixture('processor_imports', 'root.wdl');
    const rootDoc = WdlV1NodeLoader.loadFromFile(root);
    const processor = new ProbeProcessor();

    const libTasks = processor.tasks(rootDoc, 'lib.lib_task');
    expect(libTasks).toHaveLength(1);
    expect(libTasks[0]?.importNamespace).toBe('lib');
    expect(libTasks[0]?.importedName).toBe('lib_task');

    const starTasks = processor.tasks(rootDoc, 'star_task');
    expect(starTasks).toHaveLength(1);
    expect(starTasks[0]?.localName).toBe('star_task');

    const memberTasks = processor.tasks(rootDoc, 'local_task');
    expect(memberTasks).toHaveLength(1);
    expect(memberTasks[0]?.importedName).toBe('selected_task');

    const workflows = processor.workflows(rootDoc, 'local_flow');
    expect(workflows).toHaveLength(1);
    expect(workflows[0]?.importedName).toBe('selected_flow');

    const aliasedStructs = processor.structs(rootDoc, 'Patient');
    expect(aliasedStructs).toHaveLength(1);
    expect(aliasedStructs[0]?.importedName).toBe('Person');

    const starStructs = processor.structs(rootDoc, 'StarStruct');
    expect(starStructs).toHaveLength(1);

    const memberStructs = processor.structs(rootDoc, 'LocalStruct');
    expect(memberStructs).toHaveLength(1);
    expect(memberStructs[0]?.importedName).toBe('SelectedStruct');

    const aliasedEnums = processor.enums(rootDoc, 'ImportStatus');
    expect(aliasedEnums).toHaveLength(1);
    expect(aliasedEnums[0]?.importedName).toBe('Status');

    const memberEnums = processor.enums(rootDoc, 'LocalEnum');
    expect(memberEnums).toHaveLength(1);
    expect(memberEnums[0]?.importedName).toBe('SelectedEnum');

    expect(rootDoc.importStatements().length).toBeGreaterThan(0);
    expect(processor.importedDoc(rootDoc, rootDoc.importStatements()[0]!)).toBeDefined();
  });
});
