/** Conditional statement nodes. */
import type { WdlNode } from '../base/wdl-node.js';
import type { WdlWorkflowElement } from '../definitions/wdl-workflow.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';
import { WdlStatementComponentType, type WdlStatement } from './wdl-statement.js';

/** Models a single `else if` branch inside a WDL conditional statement. */
export class WdlConditionalElseIf implements WdlNode {
  private readonly thenStatementValues: WdlStatement[] = [];

  /** Creates an `else if` branch from its optional condition expression. */
  public constructor(private conditionValue?: WdlExpression) {}
  /** Returns the branch condition expression. */
  public getCondition(): WdlExpression | undefined {
    return this.conditionValue;
  }
  /** Sets the branch condition expression. */
  public setCondition(condition: WdlExpression | undefined): void {
    this.conditionValue = condition;
  }
  /** Returns the ordered statements in this branch body. */
  public thenStatements(): WdlStatement[] {
    return this.thenStatementValues;
  }
}

/** Models a WDL `if` / `else if` / `else` statement block. */
export class WdlConditional implements WdlStatement, WdlWorkflowElement {
  private readonly thenStatementValues: WdlStatement[] = [];
  private readonly elseIfValues: WdlConditionalElseIf[] = [];
  private readonly elseStatementValues: WdlStatement[] = [];

  /** Creates a conditional statement from its optional condition expression. */
  public constructor(private conditionValue?: WdlExpression) {}
  /** Returns the condition expression for the `if` branch. */
  public getCondition(): WdlExpression | undefined {
    return this.conditionValue;
  }
  /** Sets the condition expression for the `if` branch. */
  public setCondition(condition: WdlExpression | undefined): void {
    this.conditionValue = condition;
  }
  /** Returns the ordered statements in the `then` branch. */
  public thenStatements(): WdlStatement[] {
    return this.thenStatementValues;
  }
  /** Returns the ordered `else if` branches. */
  public elseIfs(): WdlConditionalElseIf[] {
    return this.elseIfValues;
  }
  /** Returns the ordered statements in the `else` branch. */
  public elseStatements(): WdlStatement[] {
    return this.elseStatementValues;
  }
  /** Returns the broad statement family for this node. */
  public componentType(): WdlStatementComponentType {
    return WdlStatementComponentType.CONDITIONAL;
  }
}
