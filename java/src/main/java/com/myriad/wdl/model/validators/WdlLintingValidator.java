package com.myriad.wdl.model.validators;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.base.WdlKeyValue;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlIndexAccessOperation;
import com.myriad.wdl.model.expressions.WdlMapLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral.WdlMapEntry;
import com.myriad.wdl.model.expressions.WdlMemberAccessOperation;
import com.myriad.wdl.model.expressions.WdlObjectLiteral;
import com.myriad.wdl.model.expressions.WdlObjectLiteral.WdlObjectEntry;
import com.myriad.wdl.model.expressions.WdlPairLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringComponent;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholderOption;
import com.myriad.wdl.model.expressions.WdlStructLiteral;
import com.myriad.wdl.model.expressions.WdlStructLiteral.WdlStructEntry;
import com.myriad.wdl.model.expressions.WdlTernaryOperation;
import com.myriad.wdl.model.expressions.WdlUnaryOperation;
import com.myriad.wdl.model.expressions.WdlVariable;
import com.myriad.wdl.model.sections.WdlCommand;
import com.myriad.wdl.model.sections.WdlHints;
import com.myriad.wdl.model.sections.WdlInput;
import com.myriad.wdl.model.sections.WdlOutput;
import com.myriad.wdl.model.sections.WdlRequirements;
import com.myriad.wdl.model.sections.WdlRuntime;
import com.myriad.wdl.model.statements.WdlCall;
import com.myriad.wdl.model.statements.WdlConditional;
import com.myriad.wdl.model.statements.WdlDeclaration;
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import com.myriad.wdl.model.statements.WdlImport;
import com.myriad.wdl.model.statements.WdlScatter;
import com.myriad.wdl.model.statements.WdlStatement;
import com.myriad.wdl.model.types.WdlArrayType;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPairType;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import com.myriad.wdl.model.types.WdlTypeReferenceType;
import java.util.HashSet;
import java.util.Set;

/**
 * Linting validator that extends static analysis with usage-oriented diagnostics.
 *
 * <p>The diagnostics produced here are warning-severity by default and are meant to answer
 * questions such as whether declarations, scatter variables, or call outputs are never used.
 * Callers can keep the diagnostics non-throwing with {@link #setThrowOnWarnings(boolean)}.
 */
public class WdlLintingValidator extends WdlStaticAnalysisValidator {

  @Override
  public void processDocument(WdlDocument node) {
    super.processDocument(node);
    lintDeprecatedDocumentFeatures(node);
  }

  @Override
  public void processWorkflow(WdlDocument ctx, WdlWorkflow node) {
    super.processWorkflow(ctx, node);
    lintDeprecatedWorkflowTypes(node);
    lintWorkflow(node);
  }

  @Override
  public void processTask(WdlDocument ctx, WdlTask node) {
    super.processTask(ctx, node);
    lintDeprecatedTaskFeatures(node);
    lintTask(node);
  }

  private void lintDeprecatedDocumentFeatures(WdlDocument document) {
    for (WdlImport imp : document.importStatements()) {
      String source = stringLiteralText(imp.getSource());
      if (source != null && source.startsWith("file://")) {
        addDeprecationWarning("Import source uses deprecated file:// URI: '" + source + "'");
      }
    }
  }

  private void lintDeprecatedWorkflowTypes(WdlWorkflow workflow) {
    for (WdlWorkflow.WdlWorkflowElement element : workflow.getElements()) {
      if (element instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) element).elements()) {
          lintDeprecatedTypeUsage(declaration.getType(), "workflow input", declaration.getName());
        }
      } else if (element instanceof WdlBoundDeclaration) {
        WdlBoundDeclaration declaration = (WdlBoundDeclaration) element;
        lintDeprecatedTypeUsage(
            declaration.getType(), "workflow declaration", declaration.getName());
      } else if (element instanceof WdlOutput) {
        for (WdlBoundDeclaration declaration : ((WdlOutput) element).elements()) {
          lintDeprecatedTypeUsage(declaration.getType(), "workflow output", declaration.getName());
        }
      }
    }
  }

  private void lintDeprecatedTaskFeatures(WdlTask task) {
    for (WdlTask.WdlTaskElement element : task.elements()) {
      if (element instanceof WdlRuntime) {
        addDeprecationWarning(
            "Task '"
                + safeTaskName(task)
                + "' uses deprecated runtime section; use requirements/hints instead");
      } else if (element instanceof WdlRequirements) {
        for (WdlKeyValue<String, WdlExpression> entry : ((WdlRequirements) element).elements()) {
          if ("docker".equals(entry.getKey())) {
            addDeprecationWarning(
                "Task '"
                    + safeTaskName(task)
                    + "' uses deprecated requirements key 'docker'; use 'container'");
          }
        }
      } else if (element instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) element).elements()) {
          lintDeprecatedTypeUsage(declaration.getType(), "task input", declaration.getName());
        }
      } else if (element instanceof WdlBoundDeclaration) {
        WdlBoundDeclaration declaration = (WdlBoundDeclaration) element;
        lintDeprecatedTypeUsage(declaration.getType(), "task declaration", declaration.getName());
      } else if (element instanceof WdlOutput) {
        for (WdlBoundDeclaration declaration : ((WdlOutput) element).elements()) {
          lintDeprecatedTypeUsage(declaration.getType(), "task output", declaration.getName());
        }
      }
    }
  }

  private void lintDeprecatedTypeUsage(WdlType type, String scope, String name) {
    if (type == null) {
      return;
    }

    if (type instanceof WdlPrimitiveType
        && ((WdlPrimitiveType) type).primitiveType() == WdlPrimitiveType.Type.OBJECT) {
      addDeprecationWarning(
          "Deprecated Object type used in " + scope + " '" + safeName(name) + "'");
      return;
    }

    if (type instanceof WdlArrayType) {
      lintDeprecatedTypeUsage(((WdlArrayType) type).memberType(), scope, name);
      return;
    }
    if (type instanceof WdlMapType) {
      lintDeprecatedTypeUsage(((WdlMapType) type).keyType(), scope, name);
      lintDeprecatedTypeUsage(((WdlMapType) type).valueType(), scope, name);
      return;
    }
    if (type instanceof WdlPairType) {
      lintDeprecatedTypeUsage(((WdlPairType) type).leftType(), scope, name);
      lintDeprecatedTypeUsage(((WdlPairType) type).rightType(), scope, name);
      return;
    }
    if (type instanceof WdlTypeReferenceType) {
      if ("Object".equals(((WdlTypeReferenceType) type).referenceName())) {
        addDeprecationWarning(
            "Deprecated Object type used in " + scope + " '" + safeName(name) + "'");
      }
      return;
    }
  }

  private void lintWorkflow(WdlWorkflow workflow) {
    Set<String> declaredNames = new HashSet<>();
    Set<String> callNames = new HashSet<>();
    Usage usage = new Usage();

    for (WdlWorkflow.WdlWorkflowElement element : workflow.getElements()) {
      if (element instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) element).elements()) {
          if (declaration.getName() != null) {
            declaredNames.add(declaration.getName());
          }
          if (declaration instanceof WdlBoundDeclaration) {
            collectExpressionUsage(((WdlBoundDeclaration) declaration).getExpression(), usage);
          }
        }
      } else if (element instanceof WdlBoundDeclaration) {
        WdlBoundDeclaration declaration = (WdlBoundDeclaration) element;
        if (declaration.getName() != null) {
          declaredNames.add(declaration.getName());
        }
        collectExpressionUsage(declaration.getExpression(), usage);
      } else if (element instanceof WdlCall) {
        collectCallUsage((WdlCall) element, usage, callNames);
      } else if (element instanceof WdlScatter) {
        collectScatterUsage((WdlScatter) element, usage, declaredNames, callNames);
      } else if (element instanceof WdlConditional) {
        collectConditionalUsage((WdlConditional) element, usage, declaredNames, callNames);
      } else if (element instanceof WdlOutput) {
        for (WdlBoundDeclaration declaration : ((WdlOutput) element).elements()) {
          collectExpressionUsage(declaration.getExpression(), usage);
        }
      } else if (element instanceof WdlHints.WdlWorkflowHints) {
        for (WdlHints.WdlWorkflowHint hint : ((WdlHints.WdlWorkflowHints) element).elements()) {
          collectExpressionUsage(hint.getValue(), usage);
        }
      }
    }

    for (String name : declaredNames) {
      if (!usage.usedVariables.contains(name)) {
        addValidationError(
            WdlSemanticError.Code.LINT_UNUSED_WORKFLOW_DECLARATION,
            "Lint: workflow declaration '" + name + "' is never used");
      }
    }
    for (String callName : callNames) {
      if (!usage.usedCallOutputTargets.contains(callName)) {
        addValidationError(
            WdlSemanticError.Code.LINT_UNUSED_CALL_OUTPUT,
            "Lint: call '" + callName + "' outputs are never referenced");
      }
    }
  }

  private void lintTask(WdlTask task) {
    Set<String> declaredNames = new HashSet<>();
    Usage usage = new Usage();

    for (WdlTask.WdlTaskElement element : task.elements()) {
      if (element instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) element).elements()) {
          if (declaration.getName() != null) {
            declaredNames.add(declaration.getName());
          }
          if (declaration instanceof WdlBoundDeclaration) {
            collectExpressionUsage(((WdlBoundDeclaration) declaration).getExpression(), usage);
          }
        }
      } else if (element instanceof WdlBoundDeclaration) {
        WdlBoundDeclaration declaration = (WdlBoundDeclaration) element;
        if (declaration.getName() != null) {
          declaredNames.add(declaration.getName());
        }
        collectExpressionUsage(declaration.getExpression(), usage);
      } else if (element instanceof WdlOutput) {
        for (WdlBoundDeclaration declaration : ((WdlOutput) element).elements()) {
          collectExpressionUsage(declaration.getExpression(), usage);
        }
      } else if (element instanceof WdlCommand) {
        collectStringLiteralUsage(((WdlCommand) element).getCommandText(), usage);
      } else if (element instanceof WdlRuntime) {
        for (WdlRuntime.WdlRuntimeEntry entry : ((WdlRuntime) element).elements()) {
          collectExpressionUsage(entry.getValue(), usage);
        }
      } else if (element instanceof WdlRequirements) {
        for (WdlKeyValue<String, WdlExpression> entry : ((WdlRequirements) element).elements()) {
          collectExpressionUsage(entry.getValue(), usage);
        }
      } else if (element instanceof WdlHints.WdlTaskHints) {
        for (WdlHints.WdlTaskHint hint : ((WdlHints.WdlTaskHints) element).elements()) {
          collectExpressionUsage(hint.getValue(), usage);
        }
      }
    }

    for (String name : declaredNames) {
      if (!usage.usedVariables.contains(name)) {
        addValidationError(
            WdlSemanticError.Code.LINT_UNUSED_TASK_DECLARATION,
            "Lint: task declaration '" + name + "' is never used");
      }
    }
  }

  private void collectStatementsUsage(
      Iterable<WdlStatement> statements,
      Usage usage,
      Set<String> declaredNames,
      Set<String> callNames) {
    for (WdlStatement statement : statements) {
      if (statement instanceof WdlBoundDeclaration) {
        WdlBoundDeclaration declaration = (WdlBoundDeclaration) statement;
        if (declaration.getName() != null) {
          declaredNames.add(declaration.getName());
        }
        collectExpressionUsage(declaration.getExpression(), usage);
      } else if (statement instanceof WdlCall) {
        collectCallUsage((WdlCall) statement, usage, callNames);
      } else if (statement instanceof WdlScatter) {
        collectScatterUsage((WdlScatter) statement, usage, declaredNames, callNames);
      } else if (statement instanceof WdlConditional) {
        collectConditionalUsage((WdlConditional) statement, usage, declaredNames, callNames);
      }
    }
  }

  private void collectConditionalUsage(
      WdlConditional conditional, Usage usage, Set<String> declaredNames, Set<String> callNames) {
    collectExpressionUsage(conditional.getCondition(), usage);
    collectStatementsUsage(conditional.thenStatements(), usage, declaredNames, callNames);
    for (WdlConditional.WdlConditionalElseIf elseIf : conditional.elseIfs()) {
      collectExpressionUsage(elseIf.getCondition(), usage);
      collectStatementsUsage(elseIf.thenStatements(), usage, declaredNames, callNames);
    }
    collectStatementsUsage(conditional.elseStatements(), usage, declaredNames, callNames);
  }

  private void collectScatterUsage(
      WdlScatter scatter, Usage usage, Set<String> declaredNames, Set<String> callNames) {
    collectExpressionUsage(scatter.getCollection(), usage);
    String scatterVar = scatter.getName();
    if (scatterVar != null) {
      declaredNames.add(scatterVar);
    }

    Usage bodyUsage = new Usage();
    collectStatementsUsage(scatter.statements(), bodyUsage, declaredNames, callNames);
    usage.merge(bodyUsage);
    if (scatterVar != null && !bodyUsage.usedVariables.contains(scatterVar)) {
      addValidationError(
          WdlSemanticError.Code.LINT_UNUSED_SCATTER_VARIABLE,
          "Lint: scatter variable '" + scatterVar + "' is never used");
    }
  }

  private void collectCallUsage(WdlCall call, Usage usage, Set<String> callNames) {
    String target = call.targetPath().peekLast();
    String callName = call.getAlias() != null ? call.getAlias() : target;
    if (callName != null) {
      callNames.add(callName);
    }
    for (WdlCall.WdlCallInput input : call.inputs()) {
      collectExpressionUsage(input.getValue(), usage);
    }
  }

  private void collectExpressionUsage(WdlExpression expression, Usage usage) {
    if (expression == null) {
      return;
    }

    if (expression instanceof WdlVariable) {
      String name = ((WdlVariable) expression).getName();
      if (name != null) {
        usage.usedVariables.add(name);
      }
      return;
    }

    if (expression instanceof WdlBinaryOperation) {
      collectExpressionUsage(((WdlBinaryOperation) expression).getLeft(), usage);
      collectExpressionUsage(((WdlBinaryOperation) expression).getRight(), usage);
      return;
    }

    if (expression instanceof WdlUnaryOperation) {
      collectExpressionUsage(((WdlUnaryOperation) expression).getOperand(), usage);
      return;
    }

    if (expression instanceof WdlTernaryOperation) {
      collectExpressionUsage(((WdlTernaryOperation) expression).getCondition(), usage);
      collectExpressionUsage(((WdlTernaryOperation) expression).getTrueValue(), usage);
      collectExpressionUsage(((WdlTernaryOperation) expression).getFalseValue(), usage);
      return;
    }

    if (expression instanceof WdlFunctionCallOperation) {
      WdlFunctionCallOperation functionCall = (WdlFunctionCallOperation) expression;
      if (functionCall.getFunction() != null
          && functionCall.getFunction().getDeprecatedIn() != null
          && versionCtx != null
          && versionCtx.ordinal() >= functionCall.getFunction().getDeprecatedIn().ordinal()) {
        addDeprecationWarning(
            "Function '"
                + functionCall.getFunction().toWdlString()
                + "' is deprecated as of WDL "
                + functionCall.getFunction().getDeprecatedIn().getVersionString());
      }
      for (WdlExpression arg : ((WdlFunctionCallOperation) expression).arguments()) {
        collectExpressionUsage(arg, usage);
      }
      return;
    }

    if (expression instanceof WdlIndexAccessOperation) {
      collectExpressionUsage(((WdlIndexAccessOperation) expression).getTarget(), usage);
      collectExpressionUsage(((WdlIndexAccessOperation) expression).getIndex(), usage);
      return;
    }

    if (expression instanceof WdlMemberAccessOperation) {
      WdlExpression target = ((WdlMemberAccessOperation) expression).getTarget();
      if (target instanceof WdlVariable && ((WdlVariable) target).getName() != null) {
        usage.usedCallOutputTargets.add(((WdlVariable) target).getName());
      }
      collectExpressionUsage(target, usage);
      return;
    }

    if (expression instanceof WdlArrayLiteral) {
      for (WdlExpression item : ((WdlArrayLiteral) expression).entries()) {
        collectExpressionUsage(item, usage);
      }
      return;
    }

    if (expression instanceof WdlMapLiteral) {
      for (WdlMapEntry entry : ((WdlMapLiteral) expression).entries()) {
        collectExpressionUsage(entry.getKey(), usage);
        collectExpressionUsage(entry.getValue(), usage);
      }
      return;
    }

    if (expression instanceof WdlPairLiteral) {
      collectExpressionUsage(((WdlPairLiteral) expression).getLeft(), usage);
      collectExpressionUsage(((WdlPairLiteral) expression).getRight(), usage);
      return;
    }

    if (expression instanceof WdlObjectLiteral) {
      for (WdlObjectEntry entry : ((WdlObjectLiteral) expression).entries()) {
        collectExpressionUsage(entry.getValue(), usage);
      }
      return;
    }

    if (expression instanceof WdlStructLiteral) {
      for (WdlStructEntry entry : ((WdlStructLiteral) expression).entries()) {
        collectExpressionUsage(entry.getValue(), usage);
      }
      return;
    }

    if (expression instanceof WdlStringLiteral) {
      collectStringLiteralUsage((WdlStringLiteral) expression, usage);
    }
  }

  private void collectStringLiteralUsage(WdlStringLiteral stringLiteral, Usage usage) {
    if (stringLiteral == null) {
      return;
    }
    for (WdlStringComponent component : stringLiteral.components()) {
      if (!(component instanceof WdlStringPlaceholder)) {
        continue;
      }
      WdlStringPlaceholder placeholder = (WdlStringPlaceholder) component;
      collectExpressionUsage(placeholder.getExpression(), usage);
      WdlStringPlaceholderOption option = placeholder.getOption();
      if (option != null) {
        addDeprecationWarning("Expression placeholder options are deprecated");
        switch (option.getType()) {
          case SEP:
          case DEFAULT:
            collectStringLiteralUsage(option.getValue(), usage);
            break;
          case TRUE_FALSE:
          case FALSE_TRUE:
            collectStringLiteralUsage(option.getTrueValue(), usage);
            collectStringLiteralUsage(option.getFalseValue(), usage);
            break;
          default:
            break;
        }
      }
    }
  }

  private String stringLiteralText(WdlStringLiteral literal) {
    if (literal == null) {
      return null;
    }
    StringBuilder text = new StringBuilder();
    for (WdlStringComponent component : literal.components()) {
      if (component instanceof WdlStringLiteral.WdlStringText) {
        text.append(((WdlStringLiteral.WdlStringText) component).getText());
      } else if (component instanceof WdlStringLiteral.WdlStringEscape) {
        text.append(((WdlStringLiteral.WdlStringEscape) component).getEscapeText());
      }
    }
    return text.toString();
  }

  private void addDeprecationWarning(String message) {
    addValidationError(WdlSemanticError.Code.LINT_DEPRECATED_FEATURE, "Lint: " + message);
  }

  private String safeTaskName(WdlTask task) {
    return task.getName() == null ? "<unnamed-task>" : task.getName();
  }

  private String safeName(String name) {
    return name == null ? "<unnamed>" : name;
  }

  private static final class Usage {
    private final Set<String> usedVariables = new HashSet<>();
    private final Set<String> usedCallOutputTargets = new HashSet<>();

    private void merge(Usage other) {
      usedVariables.addAll(other.usedVariables);
      usedCallOutputTargets.addAll(other.usedCallOutputTargets);
    }
  }
}
