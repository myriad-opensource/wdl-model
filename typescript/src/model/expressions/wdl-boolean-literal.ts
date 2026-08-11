/** Boolean literal expression nodes. */
import { WdlExpressionComponentType } from './wdl-expression.js';
import { WdlLiteral } from './wdl-literal.js';

/** Models a `true` or `false` literal. */
export class WdlBooleanLiteral extends WdlLiteral<boolean> {
  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.BOOL_LIT;
  }
}
