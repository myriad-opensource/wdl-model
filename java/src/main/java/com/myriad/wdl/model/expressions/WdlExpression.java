package com.myriad.wdl.model.expressions;

import com.myriad.wdl.model.base.WdlNode;

/**
 * Base interface for WDL expressions.
 *
 * <p>The WDL specification defines literals, variables, unary and binary operators, member access,
 * function calls, indexing, object and struct literals, and ternary expressions. Concrete
 * expression nodes implement this interface and identify their family with {@link #componentType()}.
 */
public interface WdlExpression extends WdlNode {
  /** High-level expression family used for traversal and validation dispatch. */
  public static enum ComponentType {
    BOOL_LIT,
    FLOAT_LIT,
    INT_LIT,
    ARRAY_LIT,
    MAP_LIT,
    NULL_LIT,
    OBJ_LIT,
    PAIR_LIT,
    STR_LIT,
    STRUCT_LIT,
    VARIABLE,
    BINARY_OP,
    FUNC_OP,
    IDX_OP,
    MEMBER_OP,
    TERNARY_OP,
    UNARY_OP,
  }

  /** Returns the broad category of this expression node. */
  public ComponentType componentType();
}
