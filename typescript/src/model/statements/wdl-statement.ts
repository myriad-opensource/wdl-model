/** Base workflow and task statement abstractions. */
import type { WdlNode } from '../base/wdl-node.js';

/** High-level statement families used by traversal and validation code. */
export enum WdlStatementComponentType {
  CALL = 'CALL',
  CONDITIONAL = 'CONDITIONAL',
  DECLARATION = 'DECLARATION',
  SCATTER = 'SCATTER',
}

/** Base interface implemented by WDL statement nodes. */
export interface WdlStatement extends WdlNode {
  /** Returns the broad statement family for this node. */
  componentType(): WdlStatementComponentType;
}
