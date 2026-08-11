/** Input section nodes. */
import type { WdlTaskElement } from '../definitions/wdl-task.js';
import type { WdlWorkflowElement } from '../definitions/wdl-workflow.js';
import type { WdlDeclaration } from '../statements/wdl-declaration.js';

/** Models an explicit `input { ... }` section in a task or workflow. */
export class WdlInput implements WdlTaskElement, WdlWorkflowElement {
  private readonly elementValues: WdlDeclaration[] = [];
  /** Returns the ordered declarations contained in the input section. */
  public elements(): WdlDeclaration[] {
    return this.elementValues;
  }
}
