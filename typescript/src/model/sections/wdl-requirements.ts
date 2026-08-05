/** Requirements section nodes. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import type { WdlTaskElement } from '../definitions/wdl-task.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';

/** Single requirement entry such as `cpu: 2` or `container: "image"`. */
export class WdlRequirementEntry extends WdlStringKeyValue {
  /** Creates a requirement entry from its key and optional expression value. */
  public constructor(key?: string, value?: WdlExpression) {
    super(key, value);
  }
}

/** Models a task `requirements { ... }` section. */
export class WdlRequirements implements WdlTaskElement {
  private readonly elementValues: WdlRequirementEntry[] = [];
  /** Returns the ordered requirement entries in the section. */
  public elements(): WdlRequirementEntry[] {
    return this.elementValues;
  }
}
