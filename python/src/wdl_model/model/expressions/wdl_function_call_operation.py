"""Function call expressions for the Python WDL object model.

This node models calls to WDL standard-library functions as well as engine-specific non-standard
functions. Validators use the built-in function catalog to enforce version availability and
signature expectations.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from enum import Enum

from .wdl_expression import ComponentType, WdlExpression
from ..wdl_version import WdlVersion


class WdlFunctionTypeHint(Enum):
    """Broad return/argument type hints used by the function catalog."""

    ANY = "ANY"
    ANY_OPTIONAL = "ANY_OPTIONAL"
    NUMBER = "NUMBER"
    BOOLEAN = "BOOLEAN"
    INT = "INT"
    FLOAT = "FLOAT"
    STRING = "STRING"
    STRING_OPTIONAL = "STRING_OPTIONAL"
    FILE = "FILE"
    DIRECTORY = "DIRECTORY"
    FILE_OR_DIRECTORY = "FILE_OR_DIRECTORY"
    OBJECT = "OBJECT"
    ARRAY_ANY = "ARRAY_ANY"
    ARRAY_FILE = "ARRAY_FILE"
    ARRAY_OPTIONAL_ANY = "ARRAY_OPTIONAL_ANY"
    ARRAY_INT = "ARRAY_INT"
    ARRAY_STRING = "ARRAY_STRING"
    ARRAY_OBJECT = "ARRAY_OBJECT"
    ARRAY_PAIR = "ARRAY_PAIR"
    ARRAY_ARRAY_ANY = "ARRAY_ARRAY_ANY"
    ARRAY_ARRAY_STRING = "ARRAY_ARRAY_STRING"
    MAP_ANY_ANY = "MAP_ANY_ANY"
    MAP_ANY_ARRAY = "MAP_ANY_ARRAY"
    MAP_STRING_STRING = "MAP_STRING_STRING"
    PAIR_ARRAY = "PAIR_ARRAY"


@dataclass(frozen=True)
class WdlFunctionSignature:
    """Return/argument signature hint for a function choice/variant."""

    returns: WdlFunctionTypeHint
    args: tuple[WdlFunctionTypeHint, ...]


@dataclass(frozen=True)
class _WdlFunctionMetadata:
    min_arity: int
    max_arity: int
    added_in: WdlVersion | None
    deprecated_in: WdlVersion | None
    removed_in: WdlVersion | None
    signatures: tuple[WdlFunctionSignature, ...]


class WdlFunction(Enum):
    """Catalog of standard-library functions described in the WDL specification."""

    FLOOR = "floor"
    CEIL = "ceil"
    ROUND = "round"
    MIN = "min"
    MAX = "max"
    SUB = "sub"
    STDOUT = "stdout"
    STDERR = "stderr"
    READ_LINES = "read_lines"
    READ_TSV = "read_tsv"
    READ_MAP = "read_map"
    READ_OBJECT = "read_object"
    READ_OBJECTS = "read_objects"
    READ_JSON = "read_json"
    READ_INT = "read_int"
    READ_FLOAT = "read_float"
    READ_STRING = "read_string"
    READ_BOOLEAN = "read_boolean"
    WRITE_LINES = "write_lines"
    WRITE_TSV = "write_tsv"
    WRITE_MAP = "write_map"
    WRITE_OBJECT = "write_object"
    WRITE_OBJECTS = "write_objects"
    WRITE_JSON = "write_json"
    GLOB = "glob"
    SIZE = "size"
    BASENAME = "basename"
    PREFIX = "prefix"
    SUFFIX = "suffix"
    QUOTE = "quote"
    SQUOTE = "squote"
    SEP = "sep"
    LENGTH = "length"
    RANGE = "range"
    CHUNK = "chunk"
    CROSS = "cross"
    ZIP = "zip"
    UNZIP = "unzip"
    TRANSPOSE = "transpose"
    FLATTEN = "flatten"
    SELECT_FIRST = "select_first"
    SELECT_ALL = "select_all"
    CONTAINS = "contains"
    CONTAINS_KEY = "contains_key"
    KEYS = "keys"
    VALUES = "values"
    AS_PAIRS = "as_pairs"
    AS_MAP = "as_map"
    COLLECT_BY_KEY = "collect_by_key"
    MATCHES = "matches"
    FIND = "find"
    DEFINED = "defined"
    JOIN_PATHS = "join_paths"
    VALUE = "value"
    NONSTANDARD = "nonstandard"

    def toWdlString(self) -> str:
        """Return the source-level function name."""
        return self.value

    @classmethod
    def fromWdlString(cls, wdlString: str | None) -> "WdlFunction":
        if wdlString is None:
            return cls.NONSTANDARD
        for fn in cls:
            if isinstance(fn.value, str) and fn.value == wdlString:
                return fn
        return cls.NONSTANDARD

    def getMinArity(self) -> int:
        return _FUNCTION_METADATA[self].min_arity

    def getMaxArity(self) -> int:
        return _FUNCTION_METADATA[self].max_arity

    def getAddedIn(self) -> WdlVersion | None:
        return _FUNCTION_METADATA[self].added_in

    def getDeprecatedIn(self) -> WdlVersion | None:
        return _FUNCTION_METADATA[self].deprecated_in

    def getRemovedIn(self) -> WdlVersion | None:
        return _FUNCTION_METADATA[self].removed_in

    def isVariadic(self) -> bool:
        return self.getMaxArity() < 0

    def supportsArity(self, arity: int) -> bool:
        if arity < self.getMinArity():
            return False
        return self.isVariadic() or arity <= self.getMaxArity()

    def getSignatures(self) -> list[WdlFunctionSignature]:
        return list(_FUNCTION_METADATA[self].signatures)


def _sig(
    returns: WdlFunctionTypeHint, *args: WdlFunctionTypeHint
) -> WdlFunctionSignature:
    return WdlFunctionSignature(returns=returns, args=tuple(args))


def _meta(
    min_arity: int,
    max_arity: int,
    *signatures: WdlFunctionSignature,
    added_in: WdlVersion | None = WdlVersion.V1_0,
    deprecated_in: WdlVersion | None = None,
    removed_in: WdlVersion | None = None,
) -> _WdlFunctionMetadata:
    return _WdlFunctionMetadata(
        min_arity=min_arity,
        max_arity=max_arity,
        added_in=added_in,
        deprecated_in=deprecated_in,
        removed_in=removed_in,
        signatures=tuple(signatures),
    )


T = WdlFunctionTypeHint
_FUNCTION_METADATA: dict[WdlFunction, _WdlFunctionMetadata] = {
    WdlFunction.FLOOR: _meta(1, 1, _sig(T.INT, T.FLOAT)),
    WdlFunction.CEIL: _meta(1, 1, _sig(T.INT, T.FLOAT)),
    WdlFunction.ROUND: _meta(1, 1, _sig(T.INT, T.FLOAT)),
    WdlFunction.MIN: _meta(
        2, 2, _sig(T.NUMBER, T.NUMBER, T.NUMBER), added_in=WdlVersion.V1_1
    ),
    WdlFunction.MAX: _meta(
        2, 2, _sig(T.NUMBER, T.NUMBER, T.NUMBER), added_in=WdlVersion.V1_1
    ),
    WdlFunction.SUB: _meta(3, 4, _sig(T.STRING, T.STRING, T.STRING, T.STRING)),
    WdlFunction.STDOUT: _meta(0, 0, _sig(T.FILE)),
    WdlFunction.STDERR: _meta(0, 0, _sig(T.FILE)),
    WdlFunction.READ_LINES: _meta(1, 1, _sig(T.ARRAY_STRING, T.FILE)),
    WdlFunction.READ_TSV: _meta(1, 2, _sig(T.ARRAY_ARRAY_STRING, T.FILE)),
    WdlFunction.READ_MAP: _meta(1, 1, _sig(T.MAP_STRING_STRING, T.FILE)),
    WdlFunction.READ_OBJECT: _meta(1, 1, _sig(T.OBJECT, T.FILE)),
    WdlFunction.READ_OBJECTS: _meta(1, 1, _sig(T.ARRAY_OBJECT, T.FILE)),
    WdlFunction.READ_JSON: _meta(1, 1, _sig(T.ANY, T.FILE)),
    WdlFunction.READ_INT: _meta(1, 1, _sig(T.INT, T.FILE)),
    WdlFunction.READ_FLOAT: _meta(1, 1, _sig(T.FLOAT, T.FILE)),
    WdlFunction.READ_STRING: _meta(1, 1, _sig(T.STRING, T.FILE)),
    WdlFunction.READ_BOOLEAN: _meta(1, 1, _sig(T.BOOLEAN, T.FILE)),
    WdlFunction.WRITE_LINES: _meta(1, 1, _sig(T.FILE, T.ARRAY_STRING)),
    WdlFunction.WRITE_TSV: _meta(1, 1, _sig(T.FILE, T.ARRAY_ARRAY_ANY)),
    WdlFunction.WRITE_MAP: _meta(1, 1, _sig(T.FILE, T.MAP_STRING_STRING)),
    WdlFunction.WRITE_OBJECT: _meta(1, 1, _sig(T.FILE, T.OBJECT)),
    WdlFunction.WRITE_OBJECTS: _meta(1, 1, _sig(T.FILE, T.ARRAY_OBJECT)),
    WdlFunction.WRITE_JSON: _meta(1, 1, _sig(T.FILE, T.ANY)),
    WdlFunction.GLOB: _meta(1, 1, _sig(T.ARRAY_FILE, T.STRING)),
    WdlFunction.SIZE: _meta(
        1,
        2,
        _sig(T.FLOAT, T.FILE_OR_DIRECTORY),
        _sig(T.FLOAT, T.ANY, T.STRING),
    ),
    WdlFunction.BASENAME: _meta(
        1,
        2,
        _sig(T.STRING, T.FILE_OR_DIRECTORY),
        _sig(T.STRING, T.STRING, T.STRING),
    ),
    WdlFunction.PREFIX: _meta(2, 2, _sig(T.ARRAY_STRING, T.STRING, T.ARRAY_ANY)),
    WdlFunction.SUFFIX: _meta(
        2,
        2,
        _sig(T.ARRAY_STRING, T.STRING, T.ARRAY_ANY),
        added_in=WdlVersion.V1_1,
    ),
    WdlFunction.QUOTE: _meta(
        1, 1, _sig(T.ARRAY_STRING, T.ARRAY_ANY), added_in=WdlVersion.V1_1
    ),
    WdlFunction.SQUOTE: _meta(
        1, 1, _sig(T.ARRAY_STRING, T.ARRAY_ANY), added_in=WdlVersion.V1_1
    ),
    WdlFunction.SEP: _meta(
        2, 2, _sig(T.STRING, T.STRING, T.ARRAY_ANY), added_in=WdlVersion.V1_1
    ),
    WdlFunction.LENGTH: _meta(1, 1, _sig(T.INT, T.ANY)),
    WdlFunction.RANGE: _meta(1, 1, _sig(T.ARRAY_INT, T.INT)),
    WdlFunction.CHUNK: _meta(
        2,
        2,
        _sig(T.ARRAY_ARRAY_ANY, T.ARRAY_ANY, T.INT),
        added_in=WdlVersion.V1_2,
    ),
    WdlFunction.CROSS: _meta(2, 2, _sig(T.ARRAY_PAIR, T.ARRAY_ANY, T.ARRAY_ANY)),
    WdlFunction.ZIP: _meta(2, 2, _sig(T.ARRAY_PAIR, T.ARRAY_ANY, T.ARRAY_ANY)),
    WdlFunction.UNZIP: _meta(
        1, 1, _sig(T.PAIR_ARRAY, T.ARRAY_PAIR), added_in=WdlVersion.V1_1
    ),
    WdlFunction.TRANSPOSE: _meta(1, 1, _sig(T.ARRAY_ARRAY_ANY, T.ARRAY_ARRAY_ANY)),
    WdlFunction.FLATTEN: _meta(1, 1, _sig(T.ARRAY_ANY, T.ARRAY_ARRAY_ANY)),
    WdlFunction.SELECT_FIRST: _meta(
        1,
        2,
        _sig(T.ANY, T.ARRAY_OPTIONAL_ANY),
        _sig(T.ANY, T.ARRAY_OPTIONAL_ANY, T.ANY),
    ),
    WdlFunction.SELECT_ALL: _meta(1, 1, _sig(T.ARRAY_ANY, T.ARRAY_OPTIONAL_ANY)),
    WdlFunction.CONTAINS: _meta(
        2,
        2,
        _sig(T.BOOLEAN, T.ARRAY_ANY, T.ANY),
        _sig(T.BOOLEAN, T.STRING, T.STRING),
        added_in=WdlVersion.V1_2,
    ),
    WdlFunction.CONTAINS_KEY: _meta(
        2,
        2,
        _sig(T.BOOLEAN, T.MAP_ANY_ANY, T.ANY),
        added_in=WdlVersion.V1_2,
    ),
    WdlFunction.KEYS: _meta(
        1, 1, _sig(T.ARRAY_ANY, T.MAP_ANY_ANY), added_in=WdlVersion.V1_1
    ),
    WdlFunction.VALUES: _meta(
        1, 1, _sig(T.ARRAY_ANY, T.MAP_ANY_ANY), added_in=WdlVersion.V1_2
    ),
    WdlFunction.AS_PAIRS: _meta(
        1, 1, _sig(T.ARRAY_PAIR, T.MAP_ANY_ANY), added_in=WdlVersion.V1_1
    ),
    WdlFunction.AS_MAP: _meta(
        1, 1, _sig(T.MAP_ANY_ANY, T.ARRAY_PAIR), added_in=WdlVersion.V1_1
    ),
    WdlFunction.COLLECT_BY_KEY: _meta(
        1,
        1,
        _sig(T.MAP_ANY_ARRAY, T.ARRAY_PAIR),
        added_in=WdlVersion.V1_1,
    ),
    WdlFunction.MATCHES: _meta(
        2, 2, _sig(T.BOOLEAN, T.STRING, T.STRING), added_in=WdlVersion.V1_2
    ),
    WdlFunction.FIND: _meta(
        2, 2, _sig(T.STRING_OPTIONAL, T.STRING, T.STRING), added_in=WdlVersion.V1_2
    ),
    WdlFunction.DEFINED: _meta(1, 1, _sig(T.BOOLEAN, T.ANY_OPTIONAL)),
    WdlFunction.JOIN_PATHS: _meta(
        2,
        -1,
        _sig(T.FILE_OR_DIRECTORY, T.FILE_OR_DIRECTORY, T.STRING),
        added_in=WdlVersion.V1_2,
    ),
    WdlFunction.VALUE: _meta(1, 1, _sig(T.ANY, T.ANY), added_in=WdlVersion.V1_3),
    WdlFunction.NONSTANDARD: _meta(0, -1, added_in=None),
}


@dataclass
class WdlFunctionCallOperation(WdlExpression):
    """Function call expression with an ordered list of argument expressions."""

    functionName: str | None = None
    function: WdlFunction = field(init=False, default=WdlFunction.NONSTANDARD)
    _arguments: deque[WdlExpression] = field(default_factory=deque)

    def __post_init__(self) -> None:
        self.setFunctionName(self.functionName)

    def setFunctionName(self, functionName: str | None) -> None:
        """Set the source-level function name and resolve the built-in catalog entry."""
        self.functionName = functionName
        self.function = WdlFunction.fromWdlString(functionName)

    def setFunction(self, function: WdlFunction | None) -> None:
        """Set the resolved built-in function entry and keep the source-level name in sync."""
        self.function = function or WdlFunction.NONSTANDARD
        if self.function != WdlFunction.NONSTANDARD:
            self.functionName = self.function.toWdlString()
        elif self.functionName is None:
            self.functionName = self.function.toWdlString()

    def getFunctionName(self) -> str | None:
        """Return the source-level function name."""
        return self.functionName

    def getFunction(self) -> WdlFunction:
        """Return the resolved built-in function entry."""
        return self.function

    def arguments(self) -> deque[WdlExpression]:
        """Return the ordered argument expressions supplied to the call."""
        return self._arguments

    def __str__(self) -> str:
        args = ", ".join(str(a) for a in self._arguments)
        return f"{self.functionName or ''}({args})"

    def componentType(self) -> ComponentType:
        """Return the broad expression family for the current node."""
        return ComponentType.FUNCTION_CALL
