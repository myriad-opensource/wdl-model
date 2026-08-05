/** Ternary expression nodes. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Models a WDL `if cond then a else b` expression. */
export class WdlTernaryOperation implements WdlExpression {
  /** Creates a ternary expression from its condition, true branch, and false branch. */
  public constructor(
    private conditionValue?: WdlExpression,
    private trueValueValue?: WdlExpression,
    private falseValueValue?: WdlExpression,
  ) {}

  /** Returns the condition expression. */
  public getCondition(): WdlExpression | undefined {
    return this.conditionValue;
  }
  /** Sets the condition expression. */
  public setCondition(condition: WdlExpression | undefined): void {
    this.conditionValue = condition;
  }
  /** Returns the true-branch expression. */
  public getTrueValue(): WdlExpression | undefined {
    return this.trueValueValue;
  }
  /** Sets the true-branch expression. */
  public setTrueValue(value: WdlExpression | undefined): void {
    this.trueValueValue = value;
  }
  /** Returns the false-branch expression. */
  public getFalseValue(): WdlExpression | undefined {
    return this.falseValueValue;
  }
  /** Sets the false-branch expression. */
  public setFalseValue(value: WdlExpression | undefined): void {
    this.falseValueValue = value;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.TERNARY_OP;
  }
}
