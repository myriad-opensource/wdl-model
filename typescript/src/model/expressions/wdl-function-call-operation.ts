/** Function call expression nodes and WDL standard-library function metadata. */
import { WdlVersion } from '../wdl-version.js';
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Models a function call expression and its ordered arguments. */
export class WdlFunctionCallOperation implements WdlExpression {
  private resolvedFunction: WdlFunction = WdlFunction.NONSTANDARD;
  private readonly argumentValues: WdlExpression[] = [];

  /** Creates a function-call expression from its optional source-level function name. */
  public constructor(private functionNameValue?: string) {
    this.setFunctionName(functionNameValue);
  }

  /** Returns the source-level function name. */
  public getFunctionName(): string | undefined {
    return this.functionNameValue;
  }

  /** Sets the source-level function name and resolves the standard-library function metadata. */
  public setFunctionName(functionName: string | undefined): void {
    this.functionNameValue = functionName;
    this.resolvedFunction = WdlFunction.fromWdlString(functionName);
  }

  /** Returns the resolved standard-library function metadata. */
  public getFunction(): WdlFunction {
    return this.resolvedFunction;
  }

  /** Sets the resolved standard-library function metadata. */
  public setFunction(fn: WdlFunction | undefined): void {
    this.resolvedFunction = fn ?? WdlFunction.NONSTANDARD;
    if (this.resolvedFunction !== WdlFunction.NONSTANDARD) {
      this.functionNameValue = this.resolvedFunction.toWdlString();
    }
  }

  /** Returns the ordered argument expressions supplied to the call. */
  public arguments(): WdlExpression[] {
    return this.argumentValues;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.FUNC_OP;
  }
}

/** Signature metadata describing a function return type and ordered argument shapes. */
export class FunctionSignature {
  /** Creates a function signature from its return shape and ordered argument shapes. */
  public constructor(
    public readonly returns: WdlFunctionSignatureType,
    public readonly args: readonly WdlFunctionSignatureType[],
  ) {}
}

/** Broad signature-shape categories used by the validator layer. */
export enum WdlFunctionSignatureType {
  ANY = 'ANY',
  ANY_OPTIONAL = 'ANY_OPTIONAL',
  NUMBER = 'NUMBER',
  BOOLEAN = 'BOOLEAN',
  INT = 'INT',
  FLOAT = 'FLOAT',
  STRING = 'STRING',
  STRING_OPTIONAL = 'STRING_OPTIONAL',
  FILE = 'FILE',
  DIRECTORY = 'DIRECTORY',
  FILE_OR_DIRECTORY = 'FILE_OR_DIRECTORY',
  OBJECT = 'OBJECT',
  ARRAY_ANY = 'ARRAY_ANY',
  ARRAY_FILE = 'ARRAY_FILE',
  ARRAY_OPTIONAL_ANY = 'ARRAY_OPTIONAL_ANY',
  ARRAY_INT = 'ARRAY_INT',
  ARRAY_STRING = 'ARRAY_STRING',
  ARRAY_OBJECT = 'ARRAY_OBJECT',
  ARRAY_PAIR = 'ARRAY_PAIR',
  ARRAY_ARRAY_ANY = 'ARRAY_ARRAY_ANY',
  ARRAY_ARRAY_STRING = 'ARRAY_ARRAY_STRING',
  MAP_ANY_ANY = 'MAP_ANY_ANY',
  MAP_ANY_ARRAY = 'MAP_ANY_ARRAY',
  MAP_STRING_STRING = 'MAP_STRING_STRING',
  PAIR_ARRAY = 'PAIR_ARRAY',
}

/** Catalog entry describing a WDL standard-library function. */
export class WdlFunction {
  public static readonly FLOOR = new WdlFunction('floor', 1, 1);
  public static readonly CEIL = new WdlFunction('ceil', 1, 1);
  public static readonly ROUND = new WdlFunction('round', 1, 1);
  public static readonly MIN = new WdlFunction('min', 2, 2, WdlVersion.V1_1);
  public static readonly MAX = new WdlFunction('max', 2, 2, WdlVersion.V1_1);
  public static readonly SUB = new WdlFunction('sub', 3, 4);
  public static readonly STDOUT = new WdlFunction('stdout', 0, 0);
  public static readonly STDERR = new WdlFunction('stderr', 0, 0);
  public static readonly READ_LINES = new WdlFunction('read_lines', 1, 1);
  public static readonly READ_TSV = new WdlFunction('read_tsv', 1, 2);
  public static readonly READ_MAP = new WdlFunction('read_map', 1, 1);
  public static readonly READ_OBJECT = new WdlFunction('read_object', 1, 1);
  public static readonly READ_OBJECTS = new WdlFunction('read_objects', 1, 1);
  public static readonly READ_JSON = new WdlFunction('read_json', 1, 1);
  public static readonly READ_INT = new WdlFunction('read_int', 1, 1);
  public static readonly READ_FLOAT = new WdlFunction('read_float', 1, 1);
  public static readonly READ_STRING = new WdlFunction('read_string', 1, 1);
  public static readonly READ_BOOLEAN = new WdlFunction('read_boolean', 1, 1);
  public static readonly WRITE_LINES = new WdlFunction('write_lines', 1, 1);
  public static readonly WRITE_TSV = new WdlFunction('write_tsv', 1, 1);
  public static readonly WRITE_MAP = new WdlFunction('write_map', 1, 1);
  public static readonly WRITE_OBJECT = new WdlFunction('write_object', 1, 1);
  public static readonly WRITE_OBJECTS = new WdlFunction('write_objects', 1, 1);
  public static readonly WRITE_JSON = new WdlFunction('write_json', 1, 1);
  public static readonly GLOB = new WdlFunction('glob', 1, 1);
  public static readonly SIZE = new WdlFunction('size', 1, 2);
  public static readonly BASENAME = new WdlFunction('basename', 1, 2);
  public static readonly PREFIX = new WdlFunction('prefix', 2, 2);
  public static readonly SUFFIX = new WdlFunction('suffix', 2, 2, WdlVersion.V1_1);
  public static readonly QUOTE = new WdlFunction('quote', 1, 1, WdlVersion.V1_1);
  public static readonly SQUOTE = new WdlFunction('squote', 1, 1, WdlVersion.V1_1);
  public static readonly SEP = new WdlFunction('sep', 2, 2, WdlVersion.V1_1);
  public static readonly LENGTH = new WdlFunction('length', 1, 1);
  public static readonly RANGE = new WdlFunction('range', 1, 1);
  public static readonly CHUNK = new WdlFunction('chunk', 2, 2, WdlVersion.V1_2);
  public static readonly CROSS = new WdlFunction('cross', 2, 2);
  public static readonly ZIP = new WdlFunction('zip', 2, 2);
  public static readonly UNZIP = new WdlFunction('unzip', 1, 1, WdlVersion.V1_1);
  public static readonly TRANSPOSE = new WdlFunction('transpose', 1, 1);
  public static readonly FLATTEN = new WdlFunction('flatten', 1, 1);
  public static readonly SELECT_FIRST = new WdlFunction('select_first', 1, 2, WdlVersion.V1_1);
  public static readonly SELECT_ALL = new WdlFunction('select_all', 1, 1, WdlVersion.V1_1);
  public static readonly CONTAINS = new WdlFunction('contains', 2, 2, WdlVersion.V1_2);
  public static readonly CONTAINS_KEY = new WdlFunction('contains_key', 2, 2, WdlVersion.V1_2);
  public static readonly KEYS = new WdlFunction('keys', 1, 1, WdlVersion.V1_1);
  public static readonly VALUES = new WdlFunction('values', 1, 1, WdlVersion.V1_2);
  public static readonly AS_PAIRS = new WdlFunction('as_pairs', 1, 1, WdlVersion.V1_1);
  public static readonly AS_MAP = new WdlFunction('as_map', 1, 1, WdlVersion.V1_1);
  public static readonly COLLECT_BY_KEY = new WdlFunction('collect_by_key', 1, 1, WdlVersion.V1_1);
  public static readonly MATCHES = new WdlFunction('matches', 2, 2, WdlVersion.V1_2);
  public static readonly FIND = new WdlFunction('find', 2, 2, WdlVersion.V1_2);
  public static readonly DEFINED = new WdlFunction('defined', 1, 1);
  public static readonly JOIN_PATHS = new WdlFunction('join_paths', 2, -1, WdlVersion.V1_2);
  public static readonly VALUE = new WdlFunction('value', 1, 1, WdlVersion.V1_3);
  public static readonly NONSTANDARD = new WdlFunction('nonstandard', 0, -1);

  /** Returns all known catalog entries, including the non-standard sentinel. */
  public static values(): readonly WdlFunction[] {
    return [
      WdlFunction.FLOOR,
      WdlFunction.CEIL,
      WdlFunction.ROUND,
      WdlFunction.MIN,
      WdlFunction.MAX,
      WdlFunction.SUB,
      WdlFunction.STDOUT,
      WdlFunction.STDERR,
      WdlFunction.READ_LINES,
      WdlFunction.READ_TSV,
      WdlFunction.READ_MAP,
      WdlFunction.READ_OBJECT,
      WdlFunction.READ_OBJECTS,
      WdlFunction.READ_JSON,
      WdlFunction.READ_INT,
      WdlFunction.READ_FLOAT,
      WdlFunction.READ_STRING,
      WdlFunction.READ_BOOLEAN,
      WdlFunction.WRITE_LINES,
      WdlFunction.WRITE_TSV,
      WdlFunction.WRITE_MAP,
      WdlFunction.WRITE_OBJECT,
      WdlFunction.WRITE_OBJECTS,
      WdlFunction.WRITE_JSON,
      WdlFunction.GLOB,
      WdlFunction.SIZE,
      WdlFunction.BASENAME,
      WdlFunction.PREFIX,
      WdlFunction.SUFFIX,
      WdlFunction.QUOTE,
      WdlFunction.SQUOTE,
      WdlFunction.SEP,
      WdlFunction.LENGTH,
      WdlFunction.RANGE,
      WdlFunction.CHUNK,
      WdlFunction.CROSS,
      WdlFunction.ZIP,
      WdlFunction.UNZIP,
      WdlFunction.TRANSPOSE,
      WdlFunction.FLATTEN,
      WdlFunction.SELECT_FIRST,
      WdlFunction.SELECT_ALL,
      WdlFunction.CONTAINS,
      WdlFunction.CONTAINS_KEY,
      WdlFunction.KEYS,
      WdlFunction.VALUES,
      WdlFunction.AS_PAIRS,
      WdlFunction.AS_MAP,
      WdlFunction.COLLECT_BY_KEY,
      WdlFunction.MATCHES,
      WdlFunction.FIND,
      WdlFunction.DEFINED,
      WdlFunction.JOIN_PATHS,
      WdlFunction.VALUE,
      WdlFunction.NONSTANDARD,
    ];
  }

  /** Resolves a source-level function name to the matching catalog entry. */
  public static fromWdlString(wdlString?: string): WdlFunction {
    return WdlFunction.values().find((fn) => fn.wdlName === wdlString) ?? WdlFunction.NONSTANDARD;
  }

  /** Creates a function catalog entry. */
  private constructor(
    private readonly wdlName: string,
    private readonly minArity: number,
    private readonly maxArity: number,
    private readonly addedIn: WdlVersion | null = WdlVersion.V1_0,
  ) {}

  /** Returns the source-level function name. */
  public toWdlString(): string {
    return this.wdlName;
  }
  /** Returns the minimum supported arity. */
  public getMinArity(): number {
    return this.minArity;
  }
  /** Returns the maximum supported arity, or a negative number for variadic functions. */
  public getMaxArity(): number {
    return this.maxArity;
  }
  /** Returns the WDL version in which the function became available. */
  public getAddedIn(): WdlVersion | null {
    return this.addedIn;
  }
  /** Returns the WDL version in which the function was deprecated, when known. */
  public getDeprecatedIn(): WdlVersion | null {
    return null;
  }
  /** Returns the WDL version in which the function was removed, when known. */
  public getRemovedIn(): WdlVersion | null {
    return null;
  }
  /** Returns whether the function accepts an unbounded trailing arity. */
  public isVariadic(): boolean {
    return this.maxArity < 0;
  }
  /** Returns whether the supplied argument count is supported by this function. */
  public supportsArity(arity: number): boolean {
    if (arity < this.minArity) return false;
    return this.isVariadic() || arity <= this.maxArity;
  }
  /** Returns the known function signatures used by the validator layer. */
  public getSignatures(): readonly FunctionSignature[] {
    return [];
  }
}
