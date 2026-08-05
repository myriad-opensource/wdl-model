/** Linting validator for the TypeScript WDL model. */
import { WdlTask, WdlWorkflow } from '../definitions/index.js';
import {
  WdlArrayLiteral,
  WdlBinaryOperation,
  type WdlExpression,
  WdlFunctionCallOperation,
  WdlIndexAccessOperation,
  WdlMapLiteral,
  WdlMemberAccessOperation,
  WdlObjectLiteral,
  WdlPairLiteral,
  WdlStringLiteral,
  WdlStringPlaceholder,
  WdlStructLiteral,
  WdlTernaryOperation,
  WdlUnaryOperation,
  WdlVariable,
} from '../expressions/index.js';
import { WdlSemanticErrorCode } from '../errors/index.js';
import {
  WdlCommand,
  WdlInput,
  WdlOutput,
  WdlRequirements,
  WdlRuntime,
  WdlTaskHints,
  WdlWorkflowHints,
} from '../sections/index.js';
import {
  WdlBoundDeclaration,
  WdlCall,
  WdlConditional,
  WdlScatter,
  type WdlStatement,
} from '../statements/index.js';
import {
  WdlArrayType,
  WdlMapType,
  WdlPairType,
  WdlPrimitiveType,
  WdlType,
  WdlTypeReferenceType,
} from '../types/index.js';
import { WdlDocument } from '../wdl-document.js';
import { WdlStaticAnalysisSemanticValidator } from './wdl-static-analysis-semantic-validator.js';

type Usage = { usedVariables: Set<string>; usedCallOutputTargets: Set<string> };

export class WdlLintingSemanticValidator extends WdlStaticAnalysisSemanticValidator {
  /** Runs static-analysis validation and then emits document-level lint diagnostics. */
  public override processDocument(node: WdlDocument): void {
    super.processDocument(node);
    this.lintDeprecatedDocumentFeatures(node);
  }

  /** Runs static-analysis validation and then emits workflow lint diagnostics. */
  public override processWorkflow(ctx: WdlDocument, node: WdlWorkflow): void {
    super.processWorkflow(ctx, node);
    this.lintDeprecatedWorkflowTypes(node);
    this.lintWorkflow(node);
  }

  /** Runs static-analysis validation and then emits task lint diagnostics. */
  public override processTask(ctx: WdlDocument, node: WdlTask): void {
    super.processTask(ctx, node);
    this.lintDeprecatedTaskFeatures(node);
    this.lintTask(node);
  }

  private lintDeprecatedDocumentFeatures(document: WdlDocument): void {
    for (const imp of document.importStatements()) {
      const source = this.importSourceText(imp);
      if (source.startsWith('file://')) {
        this.addDeprecationWarning(`Import source uses deprecated file:// URI: '${source}'`);
      }
    }
  }

  private lintDeprecatedWorkflowTypes(workflow: WdlWorkflow): void {
    for (const element of workflow.elements()) {
      if (element instanceof WdlInput) {
        for (const declaration of element.elements()) {
          this.lintDeprecatedTypeUsage(
            declaration.getType(),
            'workflow input',
            declaration.getName(),
          );
        }
      } else if (element instanceof WdlBoundDeclaration) {
        this.lintDeprecatedTypeUsage(element.getType(), 'workflow declaration', element.getName());
      } else if (element instanceof WdlOutput) {
        for (const declaration of element.elements()) {
          this.lintDeprecatedTypeUsage(
            declaration.getType(),
            'workflow output',
            declaration.getName(),
          );
        }
      }
    }
  }

  private lintDeprecatedTaskFeatures(task: WdlTask): void {
    for (const element of task.elements()) {
      if (element instanceof WdlRuntime) {
        this.addDeprecationWarning(
          `Task '${this.safeTaskName(task)}' uses deprecated runtime section; use requirements/hints instead`,
        );
      } else if (element instanceof WdlRequirements) {
        for (const entry of element.elements()) {
          if (entry.getKey() === 'docker') {
            this.addDeprecationWarning(
              `Task '${this.safeTaskName(task)}' uses deprecated requirements key 'docker'; use 'container'`,
            );
          }
        }
      } else if (element instanceof WdlInput) {
        for (const declaration of element.elements()) {
          this.lintDeprecatedTypeUsage(declaration.getType(), 'task input', declaration.getName());
        }
      } else if (element instanceof WdlBoundDeclaration) {
        this.lintDeprecatedTypeUsage(element.getType(), 'task declaration', element.getName());
      } else if (element instanceof WdlOutput) {
        for (const declaration of element.elements()) {
          this.lintDeprecatedTypeUsage(declaration.getType(), 'task output', declaration.getName());
        }
      }
    }
  }

  private lintDeprecatedTypeUsage(
    type: WdlType | undefined,
    scope: string,
    name: string | undefined,
  ): void {
    if (!type) return;

    if (type instanceof WdlPrimitiveType && type.primitiveType() === WdlPrimitiveType.Type.OBJECT) {
      this.addDeprecationWarning(
        `Deprecated Object type used in ${scope} '${this.safeName(name)}'`,
      );
      return;
    }

    if (type instanceof WdlArrayType) {
      this.lintDeprecatedTypeUsage(type.memberType(), scope, name);
      return;
    }
    if (type instanceof WdlMapType) {
      this.lintDeprecatedTypeUsage(type.keyType(), scope, name);
      this.lintDeprecatedTypeUsage(type.valueType(), scope, name);
      return;
    }
    if (type instanceof WdlPairType) {
      this.lintDeprecatedTypeUsage(type.leftType(), scope, name);
      this.lintDeprecatedTypeUsage(type.rightType(), scope, name);
      return;
    }
    if (type instanceof WdlTypeReferenceType && type.referenceName() === 'Object') {
      this.addDeprecationWarning(
        `Deprecated Object type used in ${scope} '${this.safeName(name)}'`,
      );
    }
  }

  private lintWorkflow(workflow: WdlWorkflow): void {
    const declaredNames = new Set<string>();
    const callNames = new Set<string>();
    const usage = this.newUsage();
    for (const element of workflow.elements()) {
      if (element instanceof WdlInput) {
        for (const declaration of element.elements()) {
          if (declaration.getName()) declaredNames.add(declaration.getName()!);
          if (declaration instanceof WdlBoundDeclaration)
            this.collectExpressionUsage(declaration.getExpression(), usage);
        }
      } else if (element instanceof WdlBoundDeclaration) {
        if (element.getName()) declaredNames.add(element.getName()!);
        this.collectExpressionUsage(element.getExpression(), usage);
      } else if (element instanceof WdlCall) this.collectCallUsage(element, usage, callNames);
      else if (element instanceof WdlScatter)
        this.collectScatterUsage(element, usage, declaredNames, callNames);
      else if (element instanceof WdlConditional)
        this.collectConditionalUsage(element, usage, declaredNames, callNames);
      else if (element instanceof WdlOutput)
        for (const declaration of element.elements())
          this.collectExpressionUsage(declaration.getExpression(), usage);
      else if (element instanceof WdlWorkflowHints)
        for (const hint of element.elements()) this.collectExpressionUsage(hint.getValue(), usage);
    }
    for (const name of declaredNames)
      if (!usage.usedVariables.has(name))
        this.addError(
          WdlSemanticErrorCode.LINT_UNUSED_WORKFLOW_DECLARATION,
          `Lint: workflow declaration '${name}' is never used`,
        );
    for (const callName of callNames)
      if (!usage.usedCallOutputTargets.has(callName))
        this.addError(
          WdlSemanticErrorCode.LINT_UNUSED_CALL_OUTPUT,
          `Lint: call '${callName}' outputs are never referenced`,
        );
  }

  private lintTask(task: WdlTask): void {
    const declaredNames = new Set<string>();
    const usage = this.newUsage();
    for (const element of task.elements()) {
      if (element instanceof WdlInput) {
        for (const declaration of element.elements()) {
          if (declaration.getName()) declaredNames.add(declaration.getName()!);
          if (declaration instanceof WdlBoundDeclaration)
            this.collectExpressionUsage(declaration.getExpression(), usage);
        }
      } else if (element instanceof WdlBoundDeclaration) {
        if (element.getName()) declaredNames.add(element.getName()!);
        this.collectExpressionUsage(element.getExpression(), usage);
      } else if (element instanceof WdlOutput)
        for (const declaration of element.elements())
          this.collectExpressionUsage(declaration.getExpression(), usage);
      else if (element instanceof WdlCommand)
        this.collectStringLiteralUsage(element.getCommandText(), usage);
      else if (element instanceof WdlRuntime)
        for (const entry of element.elements())
          this.collectExpressionUsage(entry.getValue(), usage);
      else if (element instanceof WdlRequirements)
        for (const entry of element.elements())
          this.collectExpressionUsage(entry.getValue(), usage);
      else if (element instanceof WdlTaskHints)
        for (const hint of element.elements()) this.collectExpressionUsage(hint.getValue(), usage);
    }
    for (const name of declaredNames)
      if (!usage.usedVariables.has(name))
        this.addError(
          WdlSemanticErrorCode.LINT_UNUSED_TASK_DECLARATION,
          `Lint: task declaration '${name}' is never used`,
        );
  }

  private newUsage(): Usage {
    return { usedVariables: new Set(), usedCallOutputTargets: new Set() };
  }
  private mergeUsage(target: Usage, other: Usage): void {
    for (const v of other.usedVariables) target.usedVariables.add(v);
    for (const v of other.usedCallOutputTargets) target.usedCallOutputTargets.add(v);
  }
  private collectStatementsUsage(
    statements: readonly WdlStatement[],
    usage: Usage,
    declaredNames: Set<string>,
    callNames: Set<string>,
  ): void {
    for (const statement of statements) {
      if (statement instanceof WdlBoundDeclaration) {
        if (statement.getName()) declaredNames.add(statement.getName()!);
        this.collectExpressionUsage(statement.getExpression(), usage);
      } else if (statement instanceof WdlCall) this.collectCallUsage(statement, usage, callNames);
      else if (statement instanceof WdlScatter)
        this.collectScatterUsage(statement, usage, declaredNames, callNames);
      else if (statement instanceof WdlConditional)
        this.collectConditionalUsage(statement, usage, declaredNames, callNames);
    }
  }
  private collectConditionalUsage(
    conditional: WdlConditional,
    usage: Usage,
    declaredNames: Set<string>,
    callNames: Set<string>,
  ): void {
    this.collectExpressionUsage(conditional.getCondition(), usage);
    this.collectStatementsUsage(conditional.thenStatements(), usage, declaredNames, callNames);
    for (const elseIf of conditional.elseIfs()) {
      this.collectExpressionUsage(elseIf.getCondition(), usage);
      this.collectStatementsUsage(elseIf.thenStatements(), usage, declaredNames, callNames);
    }
    this.collectStatementsUsage(conditional.elseStatements(), usage, declaredNames, callNames);
  }
  private collectScatterUsage(
    scatter: WdlScatter,
    usage: Usage,
    declaredNames: Set<string>,
    callNames: Set<string>,
  ): void {
    this.collectExpressionUsage(scatter.getCollection(), usage);
    const scatterVar = scatter.getName();
    if (scatterVar) declaredNames.add(scatterVar);
    const bodyUsage = this.newUsage();
    this.collectStatementsUsage(scatter.statements(), bodyUsage, declaredNames, callNames);
    this.mergeUsage(usage, bodyUsage);
    if (scatterVar && !bodyUsage.usedVariables.has(scatterVar))
      this.addError(
        WdlSemanticErrorCode.LINT_UNUSED_SCATTER_VARIABLE,
        `Lint: scatter variable '${scatterVar}' is never used`,
      );
  }
  private collectCallUsage(call: WdlCall, usage: Usage, callNames: Set<string>): void {
    const target = call.targetPath().at(-1);
    const callName = call.getAlias() ?? target;
    if (callName) callNames.add(callName);
    for (const callInput of call.inputs()) this.collectExpressionUsage(callInput.getValue(), usage);
  }
  private collectExpressionUsage(expr: WdlExpression | undefined, usage: Usage): void {
    if (!expr) return;
    if (expr instanceof WdlVariable) {
      const name = expr.getName();
      if (name) usage.usedVariables.add(name);
      return;
    }
    if (expr instanceof WdlBinaryOperation) {
      this.collectExpressionUsage(expr.getLeft(), usage);
      this.collectExpressionUsage(expr.getRight(), usage);
      return;
    }
    if (expr instanceof WdlUnaryOperation) {
      this.collectExpressionUsage(expr.getOperand(), usage);
      return;
    }
    if (expr instanceof WdlTernaryOperation) {
      this.collectExpressionUsage(expr.getCondition(), usage);
      this.collectExpressionUsage(expr.getTrueValue(), usage);
      this.collectExpressionUsage(expr.getFalseValue(), usage);
      return;
    }
    if (expr instanceof WdlFunctionCallOperation) {
      for (const arg of expr.arguments()) this.collectExpressionUsage(arg, usage);
      return;
    }
    if (expr instanceof WdlIndexAccessOperation) {
      this.collectExpressionUsage(expr.getTarget(), usage);
      this.collectExpressionUsage(expr.getIndex(), usage);
      return;
    }
    if (expr instanceof WdlMemberAccessOperation) {
      const target = expr.getTarget();
      if (target instanceof WdlVariable && target.getName())
        usage.usedCallOutputTargets.add(target.getName()!);
      this.collectExpressionUsage(target, usage);
      return;
    }
    if (expr instanceof WdlArrayLiteral) {
      for (const item of expr.entries()) this.collectExpressionUsage(item, usage);
      return;
    }
    if (expr instanceof WdlMapLiteral) {
      for (const entry of expr.entries()) {
        this.collectExpressionUsage(entry.getKey(), usage);
        this.collectExpressionUsage(entry.getValue(), usage);
      }
      return;
    }
    if (expr instanceof WdlPairLiteral) {
      this.collectExpressionUsage(expr.getLeft(), usage);
      this.collectExpressionUsage(expr.getRight(), usage);
      return;
    }
    if (expr instanceof WdlObjectLiteral) {
      for (const entry of expr.entries()) this.collectExpressionUsage(entry.getValue(), usage);
      return;
    }
    if (expr instanceof WdlStructLiteral) {
      for (const entry of expr.entries()) this.collectExpressionUsage(entry.getValue(), usage);
      return;
    }
    if (expr instanceof WdlStringLiteral) this.collectStringLiteralUsage(expr, usage);
  }
  private collectStringLiteralUsage(
    stringLiteral: WdlStringLiteral | undefined,
    usage: Usage,
  ): void {
    if (!stringLiteral) return;
    for (const component of stringLiteral.components()) {
      if (!(component instanceof WdlStringPlaceholder)) continue;
      this.collectExpressionUsage(component.expression, usage);
      const option = component.option;
      if (!option) continue;
      this.addDeprecationWarning('Expression placeholder options are deprecated');
      this.collectStringLiteralUsage(option.value, usage);
      this.collectStringLiteralUsage(option.trueValue, usage);
      this.collectStringLiteralUsage(option.falseValue, usage);
    }
  }

  private addDeprecationWarning(message: string): void {
    this.addError(WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE, `Lint: ${message}`);
  }

  private safeTaskName(task: WdlTask): string {
    return task.getName() ?? '<unnamed-task>';
  }

  private safeName(name: string | undefined): string {
    return name ?? '<unnamed>';
  }
}
