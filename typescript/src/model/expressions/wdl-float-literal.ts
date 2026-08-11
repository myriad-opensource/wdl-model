/** Floating-point literal expression nodes. */
import { WdlExpressionComponentType } from './wdl-expression.js';
import { WdlLiteral } from './wdl-literal.js';

/** Models a `Float` literal. */
export class WdlFloatLiteral extends WdlLiteral<number> {
  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.FLOAT_LIT;
  }
}
