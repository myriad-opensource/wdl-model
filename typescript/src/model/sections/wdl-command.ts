/** Command section nodes. */
import type { WdlTaskElement } from '../definitions/wdl-task.js';
import type { WdlStringLiteral } from '../expressions/wdl-string-literal.js';

/** Models a task `command` section and its multiline/single-line form. */
export class WdlCommand implements WdlTaskElement {
  /** Creates a command section from its template literal and multiline flag. */
  public constructor(
    private commandTextValue?: WdlStringLiteral,
    private multilineValue = false,
  ) {}

  /** Returns the command template literal. */
  public getCommandText(): WdlStringLiteral | undefined {
    return this.commandTextValue;
  }
  /** Sets the command template literal. */
  public setCommandText(commandText: WdlStringLiteral | undefined): void {
    this.commandTextValue = commandText;
  }
  /** Returns whether the command used the multiline `<<< >>>` form. */
  public isMultiline(): boolean {
    return this.multilineValue;
  }
  /** Sets whether the command used the multiline `<<< >>>` form. */
  public setMultiline(multiline: boolean): void {
    this.multilineValue = multiline;
  }
}
