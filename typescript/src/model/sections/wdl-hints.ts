/** Hints section nodes. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import type { WdlTaskElement } from '../definitions/wdl-task.js';
import type { WdlWorkflowElement } from '../definitions/wdl-workflow.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';

/** Base class for a single keyed hint entry. */
export abstract class WdlHint extends WdlStringKeyValue {
  /** Creates a hint entry from its key and optional expression value. */
  public constructor(key?: string, value?: WdlExpression) {
    super(key, value);
  }
}

/** Task-scoped hint entry. */
export class WdlTaskHint extends WdlHint {}

/** Workflow-scoped hint entry. */
export class WdlWorkflowHint extends WdlHint implements WdlWorkflowElement {}

/** Base container for ordered hint entries. */
export class WdlHints<V extends WdlHint> {
  private readonly elementValues: V[] = [];
  /** Returns the ordered hints contained in the section. */
  public elements(): V[] {
    return this.elementValues;
  }
}

/** Task hints section. */
export class WdlTaskHints extends WdlHints<WdlTaskHint> implements WdlTaskElement {}

/** Workflow hints section. */
export class WdlWorkflowHints extends WdlHints<WdlWorkflowHint> implements WdlWorkflowElement {}
