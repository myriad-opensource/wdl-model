/** Scatter statement nodes. */
import type { WdlWorkflowElement } from '../definitions/wdl-workflow.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';
import type { WdlSourceRange } from '../base/wdl-source-range.js';
import { WdlStatementComponentType, type WdlStatement } from './wdl-statement.js';

/** Models a WDL `scatter (x in xs) { ... }` statement. */
export class WdlScatter implements WdlStatement, WdlWorkflowElement {
  private readonly statementValues: WdlStatement[] = [];
  private sourceRangeValue: WdlSourceRange | undefined;

  /** Creates a scatter from its loop variable name and collection expression. */
  public constructor(
    private nameValue?: string,
    private collectionValue?: WdlExpression,
  ) {}

  /** Returns the loop variable name. */
  public getName(): string | undefined {
    return this.nameValue;
  }
  /** Sets the loop variable name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }
  /** Returns the collection expression being iterated. */
  public getCollection(): WdlExpression | undefined {
    return this.collectionValue;
  }
  /** Sets the collection expression being iterated. */
  public setCollection(collection: WdlExpression | undefined): void {
    this.collectionValue = collection;
  }
  /** Returns the ordered nested statements inside the scatter body. */
  public statements(): WdlStatement[] {
    return this.statementValues;
  }
  /** Returns the source range of this scatter in the document, if set. */
  public getSourceRange(): WdlSourceRange | undefined {
    return this.sourceRangeValue;
  }
  /** Sets the source range of this scatter. */
  public setSourceRange(range: WdlSourceRange | undefined): void {
    this.sourceRangeValue = range;
  }
  /** Returns the broad statement family for this node. */
  public componentType(): WdlStatementComponentType {
    return WdlStatementComponentType.SCATTER;
  }
}
