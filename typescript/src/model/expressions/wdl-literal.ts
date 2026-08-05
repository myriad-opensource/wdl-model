/** Base literal expression abstraction. */
import type { WdlExpressionComponentType } from './wdl-expression.js';
import type { WdlValueExpression } from './wdl-value-expression.js';

/** Base class for scalar literal expressions that carry a concrete value. */
export abstract class WdlLiteral<T> implements WdlValueExpression<T> {
  /** Creates a literal from its underlying runtime value. */
  public constructor(private value?: T) {}

  /** Returns the literal value. */
  public getValue(): T | undefined {
    return this.value;
  }

  /** Sets the literal value. */
  public setValue(value: T | undefined): void {
    this.value = value;
  }

  /** Returns the broad expression family for this node. */
  public abstract componentType(): WdlExpressionComponentType;
}
