/** Output section nodes. */
import type { WdlTaskElement } from '../definitions/wdl-task.js';
import type { WdlWorkflowElement } from '../definitions/wdl-workflow.js';
import type { WdlBoundDeclaration } from '../statements/wdl-declaration.js';

/** Models an explicit `output { ... }` section in a task or workflow. */
export class WdlOutput implements WdlTaskElement, WdlWorkflowElement {
  private readonly elementValues: WdlBoundDeclaration[] = [];
  /** Returns the ordered bound declarations contained in the output section. */
  public elements(): WdlBoundDeclaration[] {
    return this.elementValues;
  }
}
