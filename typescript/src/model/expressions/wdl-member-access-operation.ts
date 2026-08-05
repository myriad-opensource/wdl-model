/** Member access expression nodes. */
import { WdlExpressionComponentType, type WdlExpression } from './wdl-expression.js';

/** Models member lookup such as `call.output` or `struct.field`. */
export class WdlMemberAccessOperation implements WdlExpression {
  /** Creates a member access from the target expression and selected member name. */
  public constructor(
    private targetValue?: WdlExpression,
    private memberValue?: string,
  ) {}

  /** Returns the target expression whose member is being accessed. */
  public getTarget(): WdlExpression | undefined {
    return this.targetValue;
  }
  /** Sets the target expression whose member is being accessed. */
  public setTarget(target: WdlExpression | undefined): void {
    this.targetValue = target;
  }
  /** Returns the selected member name. */
  public getMember(): string | undefined {
    return this.memberValue;
  }
  /** Sets the selected member name. */
  public setMember(member: string | undefined): void {
    this.memberValue = member;
  }
  /** Returns the broad expression family for this node. */
  public componentType(): WdlExpressionComponentType {
    return WdlExpressionComponentType.MEMBER_OP;
  }
}
