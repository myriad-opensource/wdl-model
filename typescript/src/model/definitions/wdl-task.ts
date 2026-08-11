/** Task definition nodes for the TypeScript WDL model. */
import type { WdlNode } from '../base/wdl-node.js';
import type { WdlSourceRange } from '../base/wdl-source-range.js';

/** Marker for nodes that may appear directly in a task body. */
export interface WdlTaskElement extends WdlNode {}

/** Models a WDL task definition and preserves its body in source order. */
export class WdlTask {
  private readonly elementValues: WdlTaskElement[] = [];
  private sourceRangeValue: WdlSourceRange | undefined;

  /** Creates a task from its optional declared name. */
  public constructor(private nameValue?: string) {}

  /** Returns the declared task name. */
  public getName(): string | undefined {
    return this.nameValue;
  }
  /** Sets the declared task name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }
  /** Returns the source range of this task in the document, if set. */
  public getSourceRange(): WdlSourceRange | undefined {
    return this.sourceRangeValue;
  }
  /** Sets the source range of this task. */
  public setSourceRange(range: WdlSourceRange | undefined): void {
    this.sourceRangeValue = range;
  }
  /** Returns the ordered task body elements. */
  public elements(): WdlTaskElement[] {
    return this.elementValues;
  }
}
