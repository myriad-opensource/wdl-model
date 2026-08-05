/** Base WDL type abstractions. */
import type { WdlNode } from '../base/wdl-node.js';

/** High-level type families used by traversal and validation code. */
export enum WdlTypeComponentType {
  PRIMITIVE = 'PRIMITIVE',
  TYPE_REFERENCE = 'TYPE_REFERENCE',
  ARRAY = 'ARRAY',
  PAIR = 'PAIR',
  MAP = 'MAP',
}

/** Base class for all WDL type nodes. */
export abstract class WdlType implements WdlNode {
  /** Creates a type node with an optional marker flag. */
  public constructor(private optional = false) {}

  /** Returns whether the type carries the WDL optional (`?`) marker. */
  public isOptional(): boolean {
    return this.optional;
  }

  /** Sets whether the type carries the WDL optional (`?`) marker. */
  public setOptional(optional: boolean): void {
    this.optional = optional;
  }

  /** Returns the broad type family for this node. */
  public abstract componentType(): WdlTypeComponentType;
}
