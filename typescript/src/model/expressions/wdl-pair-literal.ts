/** Pair literal expression nodes. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Models a pair literal such as `(left, right)`. */
export class WdlPairLiteral implements WdlExpression {
  /** Creates a pair literal from its left and right member expressions. */
  public constructor(
    private leftValue?: WdlExpression,
    private rightValue?: WdlExpression,
  ) {}

  /** Returns the left member expression. */
  public getLeft(): WdlExpression | undefined {
    return this.leftValue;
  }

  /** Sets the left member expression. */
  public setLeft(left: WdlExpression | undefined): void {
    this.leftValue = left;
  }

  /** Returns the right member expression. */
  public getRight(): WdlExpression | undefined {
    return this.rightValue;
  }

  /** Sets the right member expression. */
  public setRight(right: WdlExpression | undefined): void {
    this.rightValue = right;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.PAIR_LIT;
  }
}
