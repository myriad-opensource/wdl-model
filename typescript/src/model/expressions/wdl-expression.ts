/** Base WDL expression abstractions. */
import type { WdlNode } from '../base/wdl-node.js';

/** High-level expression families used by traversal and validation code. */
export enum WdlExpressionComponentType {
  BOOL_LIT = 'BOOL_LIT',
  FLOAT_LIT = 'FLOAT_LIT',
  INT_LIT = 'INT_LIT',
  ARRAY_LIT = 'ARRAY_LIT',
  MAP_LIT = 'MAP_LIT',
  NULL_LIT = 'NULL_LIT',
  OBJ_LIT = 'OBJ_LIT',
  PAIR_LIT = 'PAIR_LIT',
  STR_LIT = 'STR_LIT',
  STRUCT_LIT = 'STRUCT_LIT',
  VARIABLE = 'VARIABLE',
  BINARY_OP = 'BINARY_OP',
  FUNC_OP = 'FUNC_OP',
  IDX_OP = 'IDX_OP',
  MEMBER_OP = 'MEMBER_OP',
  TERNARY_OP = 'TERNARY_OP',
  UNARY_OP = 'UNARY_OP',
}

/** Base interface implemented by all WDL expression nodes. */
export interface WdlExpression extends WdlNode {
  /** Returns the broad expression family for this node. */
  componentType(): WdlExpressionComponentType;
}
