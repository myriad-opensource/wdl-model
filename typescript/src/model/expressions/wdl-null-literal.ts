/** Null/None literal expression nodes. */
import { WdlExpressionComponentType } from './wdl-expression.js';
import { WdlLiteral } from './wdl-literal.js';

/** Models a WDL `None`/null literal. */
export class WdlNullLiteral extends WdlLiteral<null> {
  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.NULL_LIT;
  }
}
