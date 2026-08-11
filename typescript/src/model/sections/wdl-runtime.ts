/** Legacy runtime section nodes. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import type { WdlTaskElement } from '../definitions/wdl-task.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';

/** Single runtime entry such as `docker: "image"`. */
export class WdlRuntimeEntry extends WdlStringKeyValue {
  /** Creates a runtime entry from its key and optional expression value. */
  public constructor(key?: string, value?: WdlExpression) {
    super(key, value);
  }
}

/** Models a legacy task `runtime { ... }` section. */
export class WdlRuntime implements WdlTaskElement {
  private readonly elementValues: WdlRuntimeEntry[] = [];
  /** Returns the ordered runtime entries in the section. */
  public elements(): WdlRuntimeEntry[] {
    return this.elementValues;
  }
}
