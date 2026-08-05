/** Binary operation expression nodes and operator kinds. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Supported binary operators from the WDL expression grammar. */
export enum WdlBinaryOperator {
  OR = '||',
  AND = '&&',
  EQ = '==',
  NEQ = '!=',
  LT = '<',
  LTE = '<=',
  GT = '>',
  GTE = '>=',
  ADD = '+',
  SUBTRACT = '-',
  MULTIPLY = '*',
  DIVIDE = '/',
  MODULO = '%',
  POWER = '**',
}

/** Models a binary expression such as `a + b` or `x && y`. */
export class WdlBinaryOperation implements WdlExpression {
  /** Creates a binary expression from its left operand, operator, and right operand. */
  public constructor(
    private leftValue?: WdlExpression,
    private operatorValue?: WdlBinaryOperator,
    private rightValue?: WdlExpression,
  ) {}

  /** Returns the left operand expression. */
  public getLeft(): WdlExpression | undefined {
    return this.leftValue;
  }
  /** Sets the left operand expression. */
  public setLeft(left: WdlExpression | undefined): void {
    this.leftValue = left;
  }
  /** Returns the operator for the expression. */
  public getOperator(): WdlBinaryOperator | undefined {
    return this.operatorValue;
  }
  /** Sets the operator for the expression. */
  public setOperator(operator: WdlBinaryOperator | undefined): void {
    this.operatorValue = operator;
  }
  /** Returns the right operand expression. */
  public getRight(): WdlExpression | undefined {
    return this.rightValue;
  }
  /** Sets the right operand expression. */
  public setRight(right: WdlExpression | undefined): void {
    this.rightValue = right;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.BINARY_OP;
  }
}
