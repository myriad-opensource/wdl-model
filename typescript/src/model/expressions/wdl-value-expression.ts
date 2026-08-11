/** Expression types that carry a directly accessible concrete value. */
import type { WdlExpression } from './wdl-expression.js';

export interface WdlValueExpression<T> extends WdlExpression {
  /** Returns the concrete value represented by the expression. */
  getValue(): T | undefined;
}
