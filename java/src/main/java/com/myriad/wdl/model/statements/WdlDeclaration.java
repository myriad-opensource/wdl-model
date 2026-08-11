package com.myriad.wdl.model.statements;

import com.myriad.wdl.model.base.WdlSourceRange;
import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.types.WdlType;
import lombok.Getter;
import lombok.Setter;

/**
 * Base declaration statement.
 *
 * <p>WDL declarations introduce a typed name and may appear in tasks, workflows, inputs, outputs,
 * structs, and other scoped regions. A declaration may be unbound or may carry an initializing
 * expression via {@link WdlBoundDeclaration}.
 */
public class WdlDeclaration implements WdlStatement {
  @Getter @Setter protected WdlType type;
  @Getter @Setter protected String name;
  @Getter @Setter protected boolean environmentVariable;
  @Getter @Setter private WdlSourceRange sourceRange;

  public WdlDeclaration() {}

  public WdlDeclaration(String name) {
    setName(name);
  }

  public WdlDeclaration(WdlType type, String name) {
    this(name);
    setType(type);
  }

  @Override
  public String toString() {
    return getClass().getSimpleName();
  }

  @Override
  public ComponentType componentType() {
    return ComponentType.DECLARATION;
  }

  public static final class WdlBoundDeclaration extends WdlDeclaration
      implements WdlTaskElement, WdlWorkflowElement {
    @Getter @Setter private WdlExpression expression;

    public WdlBoundDeclaration() {}

    public WdlBoundDeclaration(String name) {
      setName(name);
    }

    public WdlBoundDeclaration(WdlType type, String name) {
      this(name);
      setType(type);
    }

    public WdlBoundDeclaration(WdlType type, String name, WdlExpression expression) {
      this(name);
      setType(type);
      setExpression(expression);
    }
  }
}
