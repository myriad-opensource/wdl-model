/** Enum definition nodes for the TypeScript WDL model. */
import { WdlStringKeyValue } from '../base/wdl-key-value.js';
import type { WdlSourceRange } from '../base/wdl-source-range.js';
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
  private sourceRangeValue: WdlSourceRange | undefined;

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
  /** Returns the source range of this enum in the document, if set. */
  public getSourceRange(): WdlSourceRange | undefined {
    return this.sourceRangeValue;
  }
  /** Sets the source range of this enum. */
  public setSourceRange(
    range: WdlSourceRange | undefined,
  ): void {
    this.sourceRangeValue = range;
  }

  /** Returns whether a choice with the supplied symbol exists. */
  public hasChoice(choiceName: string | undefined): boolean {
    return this.choice(choiceName) !== undefined;
  }

  /** Returns the enum choice by symbol, if present. */
  public choice(choiceName: string | undefined): WdlEnumChoice | undefined {
    if (!choiceName || !choiceName.trim()) return undefined;
    return this.elementValues.find((choice) => choice.getKey() === choiceName);
  }
}
