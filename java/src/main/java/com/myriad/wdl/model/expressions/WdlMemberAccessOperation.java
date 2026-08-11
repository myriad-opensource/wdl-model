package com.myriad.wdl.model.expressions;

import lombok.Getter;
import lombok.Setter;

public final class WdlMemberAccessOperation implements WdlExpression {
  @Getter @Setter private WdlExpression target;
  @Getter @Setter private String member;

  public WdlMemberAccessOperation() {}

  public WdlMemberAccessOperation(WdlExpression target, String memberName) {
    setTarget(target);
    setMember(memberName);
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.MEMBER_OP;
  }
}
