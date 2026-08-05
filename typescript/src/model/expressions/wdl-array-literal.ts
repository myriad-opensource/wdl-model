/** WDL array literal expression nodes. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Models an ordered array literal such as `[1, 2, 3]`. */
export class WdlArrayLiteral implements WdlExpression {
  private readonly entryValues: WdlExpression[] = [];

  /** Returns the ordered element expressions contained in the literal. */
  public entries(): WdlExpression[] {
    return this.entryValues;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.ARRAY_LIT;
  }
}
