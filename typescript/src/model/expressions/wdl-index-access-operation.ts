/** Index access expression nodes. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Models index lookup such as `arr[0]` or `map[key]`. */
export class WdlIndexAccessOperation implements WdlExpression {
  /** Creates an index access from the target expression and index expression. */
  public constructor(
    private targetValue?: WdlExpression,
    private indexValue?: WdlExpression,
  ) {}

  /** Returns the indexed target expression. */
  public getTarget(): WdlExpression | undefined {
    return this.targetValue;
  }
  /** Sets the indexed target expression. */
  public setTarget(target: WdlExpression | undefined): void {
    this.targetValue = target;
  }
  /** Returns the index expression. */
  public getIndex(): WdlExpression | undefined {
    return this.indexValue;
  }
  /** Sets the index expression. */
  public setIndex(index: WdlExpression | undefined): void {
    this.indexValue = index;
  }
  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.IDX_OP;
  }
}
