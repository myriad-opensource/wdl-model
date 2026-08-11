/** Object literal expression nodes. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Single named entry inside an object literal. */
export class WdlObjectEntry extends WdlStringKeyValue {}

/** Models a WDL object literal. */
export class WdlObjectLiteral implements WdlExpression {
  private readonly entryValues: WdlObjectEntry[] = [];

  /** Returns the ordered object entries contained in the literal. */
  public entries(): WdlObjectEntry[] {
    return this.entryValues;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.OBJ_LIT;
  }
}
