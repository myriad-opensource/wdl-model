/** Variable reference expression nodes. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Models a variable reference such as `x` or `sample_name`. */
export class WdlVariable implements WdlExpression {
  /** Creates a variable reference from its source-level name. */
  public constructor(private nameValue?: string) {}

  /** Returns the referenced variable name. */
  public getName(): string | undefined {
    return this.nameValue;
  }

  /** Sets the referenced variable name. */
  public setName(name: string | undefined): void {
    this.nameValue = name;
  }

  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.VARIABLE;
  }
}
