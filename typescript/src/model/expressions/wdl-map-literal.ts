/** Map literal expression nodes. */
import { WdlExpressionKeyValue } from '../base/wdl-key-value.js';
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Single key/value entry inside a WDL map literal. */
export class WdlMapEntry extends WdlExpressionKeyValue {}

/** Models a map literal such as `{"a": 1, "b": 2}`. */
export class WdlMapLiteral implements WdlExpression {
  private readonly entryValues: WdlMapEntry[] = [];

  /** Returns the ordered map entries contained in the literal. */
  public entries(): WdlMapEntry[] {
    return this.entryValues;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.MAP_LIT;
  }
}
