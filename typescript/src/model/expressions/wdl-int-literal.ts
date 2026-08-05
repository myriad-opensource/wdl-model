/** Integer literal expression nodes. */
import { WdlExpressionComponentType } from './wdl-expression.js';
import { WdlLiteral } from './wdl-literal.js';

/** Models an `Int` literal. */
export class WdlIntLiteral extends WdlLiteral<number> {
  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.INT_LIT;
  }
}
