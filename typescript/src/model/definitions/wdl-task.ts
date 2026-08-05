/** Task definition nodes for the TypeScript WDL model. */
import type { WdlNode } from '../base/wdl-node.js';

/** Marker for nodes that may appear directly in a task body. */
export interface WdlTaskElement extends WdlNode {}

/** Models a WDL task definition and preserves its body in source order. */
export class WdlTask {
  private readonly elementValues: WdlTaskElement[] = [];

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
  /** Returns the ordered task body elements. */
  public elements(): WdlTaskElement[] {
    return this.elementValues;
  }
}
