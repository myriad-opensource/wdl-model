"""Base expression nodes for the Python WDL object model."""

from __future__ import annotations

from enum import Enum

from wdl_model.model.base import WdlNode


class ComponentType(Enum):
    """High-level expression families used for traversal and validation dispatch."""

    VARIABLE = "VARIABLE"
    UNARY_OPERATION = "UNARY_OPERATION"
    BINARY_OPERATION = "BINARY_OPERATION"
    TERNARY_OPERATION = "TERNARY_OPERATION"
    LITERAL = "LITERAL"
    ARRAY_LITERAL = "ARRAY_LITERAL"
    MAP_LITERAL = "MAP_LITERAL"
    OBJECT_LITERAL = "OBJECT_LITERAL"
    PAIR_LITERAL = "PAIR_LITERAL"
    STRUCT_LITERAL = "STRUCT_LITERAL"
    FUNCTION_CALL = "FUNCTION_CALL"
    MEMBER_ACCESS = "MEMBER_ACCESS"
    INDEX_ACCESS = "INDEX_ACCESS"


class WdlExpression(WdlNode):
    """Base interface for WDL expressions.

    The WDL specification defines literals, variables, unary and binary operators, member access,
    function calls, indexing, object and struct literals, and ternary expressions.
    """

    def componentType(self) -> ComponentType:
        """Return the broad expression family for the current node."""
        raise NotImplementedError
