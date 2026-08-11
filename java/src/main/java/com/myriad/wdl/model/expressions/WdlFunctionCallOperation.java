package com.myriad.wdl.model.expressions;

import com.myriad.wdl.model.WdlVersion;
import java.util.ArrayDeque;
import java.util.Arrays;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import lombok.Getter;

/**
 * Function call expression.
 *
 * <p>This node models calls to the WDL standard library as well as engine-specific non-standard
 * functions. The built-in catalog is version-aware so validators can enforce when functions such
 * as {@code contains_key}, {@code join_paths}, and {@code value} become available.
 */
public final class WdlFunctionCallOperation implements WdlExpression {
  /** Catalog of standard-library functions described in the WDL specs. */
  public static enum WdlFunction {
    FLOOR("floor", 1, 1, sig(T.INT, T.FLOAT)),
    CEIL("ceil", 1, 1, sig(T.INT, T.FLOAT)),
    ROUND("round", 1, 1, sig(T.INT, T.FLOAT)),
    MIN("min", 2, 2, WdlVersion.V1_1, null, null, sig(T.NUMBER, T.NUMBER, T.NUMBER)),
    MAX("max", 2, 2, WdlVersion.V1_1, null, null, sig(T.NUMBER, T.NUMBER, T.NUMBER)),
    SUB("sub", 3, 4, sig(T.STRING, T.STRING, T.STRING, T.STRING)),

    STDOUT("stdout", 0, 0, sig(T.FILE)),
    STDERR("stderr", 0, 0, sig(T.FILE)),
    READ_LINES("read_lines", 1, 1, sig(T.ARRAY_STRING, T.FILE)),
    READ_TSV("read_tsv", 1, 2, sig(T.ARRAY_ARRAY_STRING, T.FILE)),
    READ_MAP("read_map", 1, 1, sig(T.MAP_STRING_STRING, T.FILE)),
    READ_OBJECT("read_object", 1, 1, sig(T.OBJECT, T.FILE)),
    READ_OBJECTS("read_objects", 1, 1, sig(T.ARRAY_OBJECT, T.FILE)),
    READ_JSON("read_json", 1, 1, sig(T.ANY, T.FILE)),
    READ_INT("read_int", 1, 1, sig(T.INT, T.FILE)),
    READ_FLOAT("read_float", 1, 1, sig(T.FLOAT, T.FILE)),
    READ_STRING("read_string", 1, 1, sig(T.STRING, T.FILE)),
    READ_BOOLEAN("read_boolean", 1, 1, sig(T.BOOLEAN, T.FILE)),

    WRITE_LINES("write_lines", 1, 1, sig(T.FILE, T.ARRAY_STRING)),
    WRITE_TSV("write_tsv", 1, 1, sig(T.FILE, T.ARRAY_ARRAY_ANY)),
    WRITE_MAP("write_map", 1, 1, sig(T.FILE, T.MAP_STRING_STRING)),
    WRITE_OBJECT("write_object", 1, 1, sig(T.FILE, T.OBJECT)),
    WRITE_OBJECTS("write_objects", 1, 1, sig(T.FILE, T.ARRAY_OBJECT)),
    WRITE_JSON("write_json", 1, 1, sig(T.FILE, T.ANY)),
    GLOB("glob", 1, 1, sig(T.ARRAY_FILE, T.STRING)),
    SIZE("size", 1, 2, sig(T.FLOAT, T.FILE_OR_DIRECTORY), sig(T.FLOAT, T.ANY, T.STRING)),

    BASENAME(
        "basename", 1, 2, sig(T.STRING, T.FILE_OR_DIRECTORY), sig(T.STRING, T.STRING, T.STRING)),
    PREFIX("prefix", 2, 2, sig(T.ARRAY_STRING, T.STRING, T.ARRAY_ANY)),
    SUFFIX("suffix", 2, 2, WdlVersion.V1_1, null, null, sig(T.ARRAY_STRING, T.STRING, T.ARRAY_ANY)),
    QUOTE("quote", 1, 1, WdlVersion.V1_1, null, null, sig(T.ARRAY_STRING, T.ARRAY_ANY)),
    SQUOTE("squote", 1, 1, WdlVersion.V1_1, null, null, sig(T.ARRAY_STRING, T.ARRAY_ANY)),
    SEP("sep", 2, 2, WdlVersion.V1_1, null, null, sig(T.STRING, T.STRING, T.ARRAY_ANY)),

    LENGTH("length", 1, 1, sig(T.INT, T.ANY)),
    RANGE("range", 1, 1, sig(T.ARRAY_INT, T.INT)),
    CHUNK("chunk", 2, 2, WdlVersion.V1_2, null, null, sig(T.ARRAY_ARRAY_ANY, T.ARRAY_ANY, T.INT)),
    CROSS("cross", 2, 2, sig(T.ARRAY_PAIR, T.ARRAY_ANY, T.ARRAY_ANY)),
    ZIP("zip", 2, 2, sig(T.ARRAY_PAIR, T.ARRAY_ANY, T.ARRAY_ANY)),
    UNZIP("unzip", 1, 1, WdlVersion.V1_1, null, null, sig(T.PAIR_ARRAY, T.ARRAY_PAIR)),
    TRANSPOSE("transpose", 1, 1, sig(T.ARRAY_ARRAY_ANY, T.ARRAY_ARRAY_ANY)),
    FLATTEN("flatten", 1, 1, sig(T.ARRAY_ANY, T.ARRAY_ARRAY_ANY)),

    SELECT_FIRST(
        "select_first",
        1,
        2,
        sig(T.ANY, T.ARRAY_OPTIONAL_ANY),
        sig(T.ANY, T.ARRAY_OPTIONAL_ANY, T.ANY)),
    SELECT_ALL("select_all", 1, 1, sig(T.ARRAY_ANY, T.ARRAY_OPTIONAL_ANY)),
    CONTAINS(
        "contains",
        2,
        2,
        WdlVersion.V1_2,
        null,
        null,
        sig(T.BOOLEAN, T.ARRAY_ANY, T.ANY),
        sig(T.BOOLEAN, T.STRING, T.STRING)),
    CONTAINS_KEY(
        "contains_key", 2, 2, WdlVersion.V1_2, null, null, sig(T.BOOLEAN, T.MAP_ANY_ANY, T.ANY)),
    KEYS("keys", 1, 1, WdlVersion.V1_1, null, null, sig(T.ARRAY_ANY, T.MAP_ANY_ANY)),
    VALUES("values", 1, 1, WdlVersion.V1_2, null, null, sig(T.ARRAY_ANY, T.MAP_ANY_ANY)),
    AS_PAIRS("as_pairs", 1, 1, WdlVersion.V1_1, null, null, sig(T.ARRAY_PAIR, T.MAP_ANY_ANY)),
    AS_MAP("as_map", 1, 1, WdlVersion.V1_1, null, null, sig(T.MAP_ANY_ANY, T.ARRAY_PAIR)),
    COLLECT_BY_KEY(
        "collect_by_key", 1, 1, WdlVersion.V1_1, null, null, sig(T.MAP_ANY_ARRAY, T.ARRAY_PAIR)),

    MATCHES("matches", 2, 2, WdlVersion.V1_2, null, null, sig(T.BOOLEAN, T.STRING, T.STRING)),
    FIND("find", 2, 2, WdlVersion.V1_2, null, null, sig(T.STRING_OPTIONAL, T.STRING, T.STRING)),
    DEFINED("defined", 1, 1, sig(T.BOOLEAN, T.ANY_OPTIONAL)),
    JOIN_PATHS(
        "join_paths",
        2,
        -1,
        WdlVersion.V1_2,
        null,
        null,
        sig(T.FILE_OR_DIRECTORY, T.FILE_OR_DIRECTORY, T.STRING)),
    VALUE("value", 1, 1, WdlVersion.V1_3, null, null, sig(T.ANY, T.ANY)),

    /** Engine-specific extension point for non-standard functions. */
    NONSTANDARD("nonstandard", 0, -1, (WdlVersion) null, (WdlVersion) null, (WdlVersion) null);

    /** Sentinel for an unbounded number of arguments. */
    public static final int UNBOUNDED = -1;

    private static final Map<String, WdlFunction> LOOKUP_BY_NAME;

    static {
      Map<String, WdlFunction> map = new LinkedHashMap<>();
      for (WdlFunction fn : values()) {
        map.put(fn.wdlName, fn);
      }
      LOOKUP_BY_NAME = Collections.unmodifiableMap(map);
    }

    private final String wdlName;
    private final int minArity;
    private final int maxArity;
    private final WdlVersion addedIn;
    private final WdlVersion deprecatedIn;
    private final WdlVersion removedIn;
    private final List<FunctionSignature> signatures;

    WdlFunction(String wdlName, int minArity, int maxArity, FunctionSignature... signatures) {
      this(wdlName, minArity, maxArity, WdlVersion.V1_0, null, null, signatures);
    }

    WdlFunction(
        String wdlName,
        int minArity,
        int maxArity,
        WdlVersion addedIn,
        WdlVersion deprecatedIn,
        WdlVersion removedIn,
        FunctionSignature... signatures) {
      this.wdlName = wdlName;
      this.minArity = minArity;
      this.maxArity = maxArity;
      this.addedIn = addedIn;
      this.deprecatedIn = deprecatedIn;
      this.removedIn = removedIn;
      this.signatures = Collections.unmodifiableList(Arrays.asList(signatures));
    }

    public String toWdlString() {
      return wdlName;
    }

    public int getMinArity() {
      return minArity;
    }

    public int getMaxArity() {
      return maxArity;
    }

    public WdlVersion getAddedIn() {
      return addedIn;
    }

    public WdlVersion getDeprecatedIn() {
      return deprecatedIn;
    }

    public WdlVersion getRemovedIn() {
      return removedIn;
    }

    public boolean isVariadic() {
      return maxArity < 0;
    }

    public boolean supportsArity(int arity) {
      if (arity < minArity) {
        return false;
      }
      return isVariadic() || arity <= maxArity;
    }

    public List<FunctionSignature> getSignatures() {
      return signatures;
    }

    public static WdlFunction fromWdlString(String wdlString) {
      return LOOKUP_BY_NAME.getOrDefault(wdlString, NONSTANDARD);
    }

    private static FunctionSignature sig(T returns, T... args) {
      return new FunctionSignature(returns, args);
    }

    /**
     * Broad type hints used for function signatures.
     *
     * <p>This is intentionally descriptive rather than a full type checker model.
     */
    public enum T {
      ANY,
      ANY_OPTIONAL,
      NUMBER,
      BOOLEAN,
      INT,
      FLOAT,
      STRING,
      STRING_OPTIONAL,
      FILE,
      DIRECTORY,
      FILE_OR_DIRECTORY,
      OBJECT,
      ARRAY_ANY,
      ARRAY_FILE,
      ARRAY_OPTIONAL_ANY,
      ARRAY_INT,
      ARRAY_STRING,
      ARRAY_OBJECT,
      ARRAY_PAIR,
      ARRAY_ARRAY_ANY,
      ARRAY_ARRAY_STRING,
      MAP_ANY_ANY,
      MAP_ANY_ARRAY,
      MAP_STRING_STRING,
      PAIR_ARRAY;
    }

    /** Return/argument signature hint for a function choice/variant. */
    public static final class FunctionSignature {
      private final T returns;
      private final List<T> args;

      FunctionSignature(T returns, T... args) {
        this.returns = returns;
        this.args = Collections.unmodifiableList(Arrays.asList(args));
      }

      public T getReturns() {
        return returns;
      }

      public List<T> getArgs() {
        return args;
      }
    }
  }

  @Getter private String functionName;
  @Getter private WdlFunction function = WdlFunction.NONSTANDARD;
  private final ArrayDeque<WdlExpression> arguments = new ArrayDeque<>();

  public WdlFunctionCallOperation() {}

  public WdlFunctionCallOperation(String functionName) {
    setFunctionName(functionName);
  }

  /** Sets the source-level function name and updates the built-in function catalog mapping. */
  public void setFunctionName(String functionName) {
    this.functionName = functionName;
    this.function =
        functionName == null ? WdlFunction.NONSTANDARD : WdlFunction.fromWdlString(functionName);
  }

  /** Sets the resolved built-in function entry and keeps the source-level function name in sync. */
  public void setFunction(WdlFunction function) {
    this.function = function == null ? WdlFunction.NONSTANDARD : function;
    if (this.function != WdlFunction.NONSTANDARD) {
      this.functionName = this.function.toWdlString();
    } else if (this.functionName == null) {
      this.functionName = this.function.toWdlString();
    }
  }

  /** Returns the ordered argument expressions supplied to the function call. */
  public ArrayDeque<WdlExpression> arguments() {
    return arguments;
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.FUNC_OP;
  }
}
