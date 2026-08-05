/** Unary operation expression nodes and operator kinds. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Supported unary operators from the WDL expression grammar. */
export enum WdlUnaryOperator {
  NOT = '!',
  NEGATIVE = '-',
}

/** Models a unary expression such as `!x` or `-n`. */
export class WdlUnaryOperation implements WdlExpression {
  /** Creates a unary expression from its operator and operand. */
  public constructor(
    private operatorValue?: WdlUnaryOperator,
    private operandValue?: WdlExpression,
  ) {}

  /** Returns the operator for the expression. */
  public getOperator(): WdlUnaryOperator | undefined {
    return this.operatorValue;
  }
  /** Sets the operator for the expression. */
  public setOperator(operator: WdlUnaryOperator | undefined): void {
    this.operatorValue = operator;
  }
  /** Returns the operand expression. */
  public getOperand(): WdlExpression | undefined {
    return this.operandValue;
  }
  /** Sets the operand expression. */
  public setOperand(operand: WdlExpression | undefined): void {
    this.operandValue = operand;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.UNARY_OP;
  }
}
