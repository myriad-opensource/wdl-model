/** Enum definition nodes for the TypeScript WDL model. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import type { WdlExpression } from '../expressions/wdl-expression.js';
import type { WdlType } from '../types/wdl-type.js';

/** Single enum choice with a symbolic name and optional explicit value expression. */
export class WdlEnumChoice extends WdlStringKeyValue {
  /** Creates an enum choice from its key and optional explicit value expression. */
  public constructor(key?: string, value?: WdlExpression) {
    super(key, value);
  }
}

/** Models a WDL enum definition. */
export class WdlEnum {
  private readonly elementValues: WdlEnumChoice[] = [];

  /** Creates an enum from its optional name and optional explicit value type. */
  public constructor(
    private nameValue?: string,
    private valueTypeValue?: WdlType,
  ) {}

  /** Returns the declared enum name. */
  public getName(): string | undefined {
    return this.nameValue;
  }
  /** Sets the declared enum name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }
  /** Returns the explicit enum value type when one is declared. */
  public getValueType(): WdlType | undefined {
    return this.valueTypeValue;
  }
  /** Sets the explicit enum value type. */
  public setValueType(valueType: WdlType | undefined): void {
    this.valueTypeValue = valueType;
  }
  /** Returns the ordered enum choices. */
  public elements(): WdlEnumChoice[] {
    return this.elementValues;
  }
}
