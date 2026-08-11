/** Workflow definition nodes for the TypeScript WDL model. */
import type { WdlNode } from '../base/wdl-node.js';
import type { WdlSourceRange } from '../base/wdl-source-range.js';

/** Marker for nodes that may appear directly in a workflow body. */
export interface WdlWorkflowElement extends WdlNode {}

/** Models a WDL workflow definition and preserves its body in source order. */
export class WdlWorkflow {
  private readonly elementValues: WdlWorkflowElement[] = [];
  private sourceRangeValue: WdlSourceRange | undefined;

  /** Creates a workflow from its optional declared name. */
  public constructor(private nameValue?: string) {}

  /** Returns the declared workflow name. */
  public getName(): string | undefined {
    return this.nameValue;
  }
  /** Sets the declared workflow name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }
  /** Returns the source range of this workflow in the document, if set. */
  public getSourceRange(): WdlSourceRange | undefined {
    return this.sourceRangeValue;
  }
  /** Sets the source range of this workflow. */
  public setSourceRange(
    range: WdlSourceRange | undefined,
  ): void {
    this.sourceRangeValue = range;
  }
  /** Returns the ordered workflow body elements. */
  public getElements(): WdlWorkflowElement[] {
    return this.elementValues;
  }
  /** Alias for {@link getElements}. */
  public elements(): WdlWorkflowElement[] {
    return this.elementValues;
  }
}
