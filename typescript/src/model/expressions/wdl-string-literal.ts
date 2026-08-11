/** String literal and placeholder expression nodes. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Source-level string delimiter forms supported by WDL. */
export enum Delimiter {
  SINGLE_QUOTED = '"',
  DOUBLE_ANGLE = '<<<>>>',
}

/** Component kinds that may appear inside a string literal. */
export enum WdlStringComponentType {
  TEXT = 'TEXT',
  ESCAPE = 'ESCAPE',
  TOKEN = 'TOKEN',
  PLACEHOLDER = 'PLACEHOLDER',
}

/** Base interface implemented by fragments inside a parsed string literal. */
export interface WdlStringComponent {
  /** Returns the broad string-component family for this node. */
  componentType(): WdlStringComponentType;
}

/** Plain text fragment inside a string literal. */
export class WdlStringText implements WdlStringComponent {
  /** Creates a text fragment from its source text. */
  public constructor(public text?: string) {}
  /** Returns the string-component family for this node. */
  public componentType(): WdlStringComponentType {
    return WdlStringComponentType.TEXT;
  }
}

/** Escape fragment inside a string literal. */
export class WdlStringEscape implements WdlStringComponent {
  /** Creates an escape fragment from its escaped source text. */
  public constructor(public escapeText?: string) {}
  /** Returns the string-component family for this node. */
  public componentType(): WdlStringComponentType {
    return WdlStringComponentType.ESCAPE;
  }
}

/** Token-preserving fragment inside a string literal. */
export class WdlStringToken implements WdlStringComponent {
  /** Creates a token fragment from its token text. */
  public constructor(public tokenText?: string) {}
  /** Returns the string-component family for this node. */
  public componentType(): WdlStringComponentType {
    return WdlStringComponentType.TOKEN;
  }
}

/** Supported placeholder sigils used in string interpolation. */
export enum PlaceHolderSymbol {
  TILDE = '~',
  DOLLAR = '$',
}

/** Supported placeholder option families from the WDL grammar. */
export enum WdlStringPlaceholderOptionType {
  DEFAULT = 'default',
  TRUE_FALSE = 'true_false',
}

/** Option block attached to a string placeholder. */
export class WdlStringPlaceholderOption {
  /** Creates placeholder options from the option type and optional string-literal payloads. */
  public constructor(
    public type: WdlStringPlaceholderOptionType,
    public value?: WdlStringLiteral,
    public trueValue?: WdlStringLiteral,
    public falseValue?: WdlStringLiteral,
  ) {}
}

/** Interpolated placeholder fragment inside a string literal. */
export class WdlStringPlaceholder implements WdlStringComponent {
  /** Creates a placeholder from its options, expression, and sigil. */
  public constructor(
    public option?: WdlStringPlaceholderOption,
    public expression?: WdlExpression,
    public symbol: PlaceHolderSymbol = PlaceHolderSymbol.TILDE,
  ) {}

  /** Returns the string-component family for this node. */
  public componentType(): WdlStringComponentType {
    return WdlStringComponentType.PLACEHOLDER;
  }
}

/** Models a WDL string literal assembled from ordered fragments. */
export class WdlStringLiteral implements WdlExpression {
  private readonly componentValues: WdlStringComponent[] = [];

  /** Creates a string literal from its delimiter form. */
  public constructor(public delimiter: Delimiter = Delimiter.SINGLE_QUOTED) {}

  /** Returns the ordered fragments that make up the string literal. */
  public components(): WdlStringComponent[] {
    return this.componentValues;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.STR_LIT;
  }
}
