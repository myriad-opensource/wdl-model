/** Struct literal expression nodes. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Single named entry inside a struct literal. */
export class WdlStructEntry extends WdlStringKeyValue {}

/** Models a struct literal such as `Person { name: "Ada" }`. */
export class WdlStructLiteral implements WdlExpression {
  private readonly entryValues: WdlStructEntry[] = [];

  /** Creates a struct literal from its target type name. */
  public constructor(private nameValue?: string) {}

  /** Returns the referenced struct type name. */
  public getName(): string | undefined {
    return this.nameValue;
  }

  /** Sets the referenced struct type name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }

  /** Returns the ordered struct field entries contained in the literal. */
  public entries(): WdlStructEntry[] {
    return this.entryValues;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.STRUCT_LIT;
  }
}
