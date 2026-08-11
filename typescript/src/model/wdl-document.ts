/** Root WDL document model types. */
import type { WdlNode } from './base/wdl-node.js';
import { WdlEnum } from './definitions/wdl-enum.js';
import { WdlStruct } from './definitions/wdl-struct.js';
import { WdlTask } from './definitions/wdl-task.js';
import { WdlWorkflow } from './definitions/wdl-workflow.js';
import {
  WdlImport,
  WdlImportMembers,
  WdlImportStandard,
  WdlImportStar,
} from './statements/wdl-import.js';
import { WdlVersion } from './wdl-version.js';

/** Union of all node types that may appear directly in a WDL document. */
export type WdlDocumentElement =
  | WdlImport
  | WdlImportStandard
  | WdlImportMembers
  | WdlImportStar
  | WdlEnum
  | WdlStruct
  | WdlTask
  | WdlWorkflow;

export class WdlDocument implements WdlNode {
  private readonly elementValues: WdlDocumentElement[] = [];
  private sourceLocationValue: string | undefined;
  private readonly importedDocumentValues = new Map<string, WdlDocument>();

  /** Creates a document with an optional declared WDL version. */
  public constructor(private wdlVersionValue?: WdlVersion) {}

  /** Returns the declared WDL version. */
  public getWdlVersion(): WdlVersion | undefined {
    return this.wdlVersionValue;
  }
  /** Sets the declared WDL version. */
  public setWdlVersion(version: WdlVersion | undefined): void {
    this.wdlVersionValue = version;
  }
  /** Returns the ordered top-level document elements. */
  public elements(): WdlDocumentElement[] {
    return this.elementValues;
  }

  /** Returns the source location identifier for this document, when known. */
  public getSourceLocation(): string | undefined {
    return this.sourceLocationValue;
  }

  /** Sets the source location identifier for this document. */
  public setSourceLocation(sourceLocation: string | undefined): void {
    this.sourceLocationValue = sourceLocation;
  }

  /** Returns imported documents keyed by resolved import identifier. */
  public importedDocuments(): Map<string, WdlDocument> {
    return this.importedDocumentValues;
  }
  /** Returns only top-level import statements. */
  public importStatements(): WdlImport[] {
    return this.elementValues.filter((element) => element instanceof WdlImport) as WdlImport[];
  }
  /** Returns only top-level enum definitions. */
  public enums(): WdlEnum[] {
    return this.elementValues.filter((element) => element instanceof WdlEnum) as WdlEnum[];
  }
  /** Returns only top-level struct definitions. */
  public structs(): WdlStruct[] {
    return this.elementValues.filter((element) => element instanceof WdlStruct) as WdlStruct[];
  }
  /** Returns only top-level task definitions. */
  public tasks(): WdlTask[] {
    return this.elementValues.filter((element) => element instanceof WdlTask) as WdlTask[];
  }
  /** Returns only top-level workflow definitions. */
  public workflows(): WdlWorkflow[] {
    return this.elementValues.filter((element) => element instanceof WdlWorkflow) as WdlWorkflow[];
  }
}
