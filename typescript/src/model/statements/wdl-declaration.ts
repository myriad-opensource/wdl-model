/** Declaration statement nodes. */
import type { WdlTaskElement } from '../definitions/wdl-task.js';
import type { WdlWorkflowElement } from '../definitions/wdl-workflow.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';
import type { WdlSourceRange } from '../base/wdl-source-range.js';
import type { WdlType } from '../types/wdl-type.js';
import { WdlStatementComponentType, type WdlStatement } from './wdl-statement.js';

/** Models a typed WDL declaration, optionally marked as an environment variable. */
export class WdlDeclaration implements WdlStatement {
  private sourceRangeValue: WdlSourceRange | undefined;

  /** Creates a declaration from its type, name, and environment-variable flag. */
  public constructor(
    protected typeValue?: WdlType,
    protected nameValue?: string,
    protected environmentVariable = false,
  ) {}

  /** Returns the declared type. */
  public getType(): WdlType | undefined {
    return this.typeValue;
  }
  /** Sets the declared type. */
  public setType(type: WdlType | undefined): void {
    this.typeValue = type;
  }
  /** Returns the declared name. */
  public getName(): string | undefined {
    return this.nameValue;
  }
  /** Sets the declared name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }
  /** Returns whether the declaration uses the `env` modifier. */
  public isEnvironmentVariable(): boolean {
    return this.environmentVariable;
  }
  /** Sets whether the declaration uses the `env` modifier. */
  public setEnvironmentVariable(environmentVariable: boolean): void {
    this.environmentVariable = environmentVariable;
  }
  /** Returns the broad statement family for this node. */
  public componentType(): WdlStatementComponentType {
    return WdlStatementComponentType.DECLARATION;
  }
  /** Returns the source range of this declaration in the document, if set. */
  public getSourceRange(): WdlSourceRange | undefined {
    return this.sourceRangeValue;
  }
  /** Sets the source range of this declaration. */
  public setSourceRange(range: WdlSourceRange | undefined): void {
    this.sourceRangeValue = range;
  }
}

/** Models a declaration with an initializing expression. */
export class WdlBoundDeclaration
  extends WdlDeclaration
  implements WdlTaskElement, WdlWorkflowElement
{
  /** Creates a bound declaration from its type, name, and initializing expression. */
  public constructor(
    type?: WdlType,
    name?: string,
    private expressionValue?: WdlExpression,
  ) {
    super(type, name);
  }

  /** Returns the initializing expression. */
  public getExpression(): WdlExpression | undefined {
    return this.expressionValue;
  }
  /** Sets the initializing expression. */
  public setExpression(expression: WdlExpression | undefined): void {
    this.expressionValue = expression;
  }
}
