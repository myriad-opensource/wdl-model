/** Baseline semantic validator for the TypeScript WDL model. */
import { WdlDocument } from '../wdl-document.js';
import { WdlVersion } from '../wdl-version.js';
import { WdlEnum, WdlStruct, WdlTask, WdlWorkflow } from '../definitions/index.js';
import {
  WdlException,
  WdlSemanticError,
  WdlSemanticErrorCode,
  WdlSemanticSeverity,
} from '../errors/index.js';
import {
  WdlArrayLiteral,
  WdlBinaryOperation,
  WdlBinaryOperator,
  WdlBooleanLiteral,
  type WdlExpression,
  WdlFloatLiteral,
  WdlFunction,
  WdlFunctionCallOperation,
  WdlIndexAccessOperation,
  WdlIntLiteral,
  WdlMapLiteral,
  WdlMemberAccessOperation,
  WdlNullLiteral,
  WdlObjectLiteral,
  WdlPairLiteral,
  WdlStringEscape,
  WdlStringLiteral,
  WdlStringPlaceholder,
  WdlStringText,
  WdlStringToken,
  WdlStructLiteral,
  WdlTernaryOperation,
  WdlUnaryOperation,
  WdlUnaryOperator,
  WdlVariable,
} from '../expressions/index.js';
import { WdlInput, WdlOutput } from '../sections/index.js';
import {
  WdlCall,
  WdlConditional,
  WdlBoundDeclaration,
  WdlImport,
  WdlImportMembers,
  WdlImportStandard,
  WdlImportStar,
  WdlScatter,
  type WdlStatement,
} from '../statements/index.js';
import {
  WdlArrayType,
  inferEnumValueType,
  inferLiteralExpressionType,
  WdlMapType,
  WdlPairType,
  WdlPrimitiveType,
  WdlType,
  WdlTypeReferenceType,
} from '../types/index.js';
import { WdlProcessorBase } from '../processors/wdl-processor-base.js';

type TaskContract = {
  requiredInputs: Set<string>;
  inputTypes: Map<string, WdlType | undefined>;
  outputs: Set<string>;
  outputTypes: Map<string, WdlType | undefined>;
  privateDeclarations: Set<string>;
};

const UNKNOWN = Symbol('UNKNOWN');

export class WdlSemanticValidator extends WdlProcessorBase {
  protected readonly functionAddedIn = new Map<WdlFunction, WdlVersion>([
    [WdlFunction.MIN, WdlVersion.V1_1],
    [WdlFunction.MAX, WdlVersion.V1_1],
    [WdlFunction.SUFFIX, WdlVersion.V1_1],
    [WdlFunction.QUOTE, WdlVersion.V1_1],
    [WdlFunction.SQUOTE, WdlVersion.V1_1],
    [WdlFunction.SEP, WdlVersion.V1_1],
    [WdlFunction.UNZIP, WdlVersion.V1_1],
    [WdlFunction.SELECT_FIRST, WdlVersion.V1_1],
    [WdlFunction.SELECT_ALL, WdlVersion.V1_1],
    [WdlFunction.KEYS, WdlVersion.V1_1],
    [WdlFunction.AS_PAIRS, WdlVersion.V1_1],
    [WdlFunction.AS_MAP, WdlVersion.V1_1],
    [WdlFunction.COLLECT_BY_KEY, WdlVersion.V1_1],
    [WdlFunction.CHUNK, WdlVersion.V1_2],
    [WdlFunction.CONTAINS, WdlVersion.V1_2],
    [WdlFunction.CONTAINS_KEY, WdlVersion.V1_2],
    [WdlFunction.VALUES, WdlVersion.V1_2],
    [WdlFunction.MATCHES, WdlVersion.V1_2],
    [WdlFunction.FIND, WdlVersion.V1_2],
    [WdlFunction.JOIN_PATHS, WdlVersion.V1_2],
    [WdlFunction.VALUE, WdlVersion.V1_3],
  ]);

  protected errors: WdlSemanticError[] = [];
  protected taskContracts = new Map<string, TaskContract>();
  protected structMembers = new Map<string, Set<string>>();
  protected structMemberTypes = new Map<string, Map<string, WdlType | undefined>>();
  protected enumValueTypes = new Map<string, WdlType | undefined>();
  protected enumChoiceNames = new Map<string, Set<string>>();
  protected scopeTypes = new Map<string, WdlType | undefined>();
  protected scopeValues = new Map<string, unknown>();
  protected callOutputs = new Map<string, Set<string>>();
  protected callOutputTypes = new Map<string, Map<string, WdlType | undefined>>();
  protected documentVersion: WdlVersion | undefined;
  protected throwOnWarnings = true;

  /** Creates a semantic validator with the requested warning throw policy. */
  public constructor(throwOnWarnings = true) {
    super();
    this.throwOnWarnings = throwOnWarnings;
  }

  /** Validates a parsed WDL document and throws when the configured policy requires it. */
  public validateDocument(document: WdlDocument): void {
    this.errors = [];
    this.documentVersion = document.getWdlVersion();
    this.indexTopLevelContracts(document);
    this.validateImportDeclarations(document);
    this.processDocument(document);
    if (this.shouldThrowForCollectedDiagnostics()) {
      throw new WdlException(this.errors);
    }
  }

  /** Sets whether warning-only diagnostics should throw and returns `this` for chaining. */
  public setThrowOnWarnings(throwOnWarnings: boolean): this {
    this.throwOnWarnings = throwOnWarnings;
    return this;
  }

  /** Returns whether warning-only diagnostics are currently configured to throw. */
  public isThrowOnWarnings(): boolean {
    return this.throwOnWarnings;
  }

  /** Returns the diagnostics collected during the most recent validation pass. */
  public getErrors(): readonly WdlSemanticError[] {
    return this.errors;
  }

  protected shouldThrowForCollectedDiagnostics(): boolean {
    if (this.errors.length === 0) return false;
    const hasError = this.errors.some((error) => error.severity() === WdlSemanticSeverity.ERROR);
    if (hasError) return true;
    const hasWarning = this.errors.some(
      (error) => error.severity() === WdlSemanticSeverity.WARNING,
    );
    return this.throwOnWarnings && hasWarning;
  }

  protected indexTopLevelContracts(document: WdlDocument): void {
    this.taskContracts.clear();
    this.structMembers.clear();
    this.structMemberTypes.clear();
    this.enumValueTypes.clear();
    this.enumChoiceNames.clear();

    for (const element of document.elements()) {
      if (element instanceof WdlTask) {
        const taskName = element.getName();
        if (taskName) {
          this.taskContracts.set(taskName, this.buildTaskContract(element));
        }
      } else if (element instanceof WdlWorkflow) {
        const workflowName = element.getName();
        if (workflowName) {
          this.taskContracts.set(workflowName, this.buildWorkflowContract(element));
        }
      } else if (element instanceof WdlStruct) {
        const structName = element.getName();
        if (!structName) continue;
        const members = new Set<string>();
        const memberTypes = new Map<string, WdlType | undefined>();
        for (const structElement of element.elements()) {
          if ('getName' in structElement && typeof structElement.getName === 'function') {
            const memberName = structElement.getName();
            if (memberName) {
              members.add(memberName);
              memberTypes.set(
                memberName,
                'getType' in structElement && typeof structElement.getType === 'function'
                  ? structElement.getType()
                  : undefined,
              );
            }
          }
        }
        this.structMembers.set(structName, members);
        this.structMemberTypes.set(structName, memberTypes);
      } else if (element instanceof WdlEnum) {
        const enumName = element.getName();
        if (!enumName) continue;
        this.enumValueTypes.set(
          enumName,
          inferEnumValueType(element) ?? new WdlPrimitiveType(WdlPrimitiveType.Type.STRING),
        );
        this.enumChoiceNames.set(
          enumName,
          new Set(
            element
              .elements()
              .map((choice) => choice.getKey())
              .filter((name): name is string => Boolean(name && name.trim())),
          ),
        );
      }
    }

    this.indexImportedTopLevelContracts(document);
  }

  public override processWorkflow(ctx: WdlDocument, node: WdlWorkflow): void {
    const prevTypes = this.scopeTypes;
    const prevValues = this.scopeValues;
    const prevCallOutputs = this.callOutputs;
    const prevCallOutputTypes = this.callOutputTypes;

    this.scopeTypes = new Map();
    this.scopeValues = new Map();
    this.callOutputs = new Map();
    this.callOutputTypes = new Map();

    try {
      super.processWorkflow(ctx, node);
    } finally {
      this.scopeTypes = prevTypes;
      this.scopeValues = prevValues;
      this.callOutputs = prevCallOutputs;
      this.callOutputTypes = prevCallOutputTypes;
    }
  }

  public override processWorkflowInput(_ctx: WdlWorkflow, node: WdlInput): void {
    for (const decl of node.elements()) {
      const name = decl.getName();
      if (name) this.scopeTypes.set(name, decl.getType());
      if (decl instanceof WdlBoundDeclaration && name) {
        this.validateExpression(decl.getExpression());
        this.scopeValues.set(name, this.evaluate(decl.getExpression()));
      }
    }
  }

  public override processWorkflowDeclaration(_ctx: WdlWorkflow, node: WdlBoundDeclaration): void {
    this.validateBoundDeclaration(node);
  }

  public override processWorkflowOutput(_ctx: WdlWorkflow, node: WdlOutput): void {
    for (const decl of node.elements()) this.validateBoundDeclaration(decl);
  }

  public override processWorkflowCall(_ctx: WdlWorkflow, node: WdlCall): void {
    const targetPath = node.targetPath();
    const target = targetPath.at(-1);
    const fullTarget = targetPath.join('.');
    const contract = this.resolveCallableContract(fullTarget, target);
    const providedInputs = new Set<string>();

    for (const callInput of node.inputs()) {
      const key = callInput.getKey() ?? '';
      const rootName = key.includes('.') ? key.slice(0, key.indexOf('.')) : key;
      if (rootName) providedInputs.add(rootName);

      if (contract && contract.privateDeclarations.has(rootName)) {
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Call input '${rootName}' is private in callable '${fullTarget || target}'`,
        );
      }
      if (contract && rootName && !contract.inputTypes.has(rootName)) {
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Call input '${rootName}' does not exist in callable '${fullTarget || target}'`,
        );
      }

      this.validateExpression(callInput.getValue());
      if (contract && contract.inputTypes.has(rootName)) {
        const expected = contract.inputTypes.get(rootName);
        if (!this.isAssignableFrom(expected, callInput.getValue())) {
          this.addError(
            WdlSemanticErrorCode.TYPE_MISMATCH,
            `Call input '${rootName}' type is incompatible with callable '${fullTarget || target}' input type`,
          );
        }
      }
    }

    if (contract) {
      for (const required of contract.requiredInputs) {
        if (!providedInputs.has(required)) {
          this.addError(
            WdlSemanticErrorCode.UNKNOWN_REFERENCE,
            `Call to '${fullTarget || target}' is missing required input '${required}'`,
          );
        }
      }
    }

    const callName = node.getAlias() ?? target;
    if (callName) {
      this.callOutputs.set(callName, contract ? new Set(contract.outputs) : new Set());
      this.callOutputTypes.set(callName, contract ? new Map(contract.outputTypes) : new Map());
    }
  }

  protected resolveCallableContract(
    fullTarget: string | undefined,
    targetName: string | undefined,
  ): TaskContract | undefined {
    if (fullTarget && this.taskContracts.has(fullTarget)) {
      return this.taskContracts.get(fullTarget);
    }
    if (targetName && this.taskContracts.has(targetName)) {
      return this.taskContracts.get(targetName);
    }
    return undefined;
  }

  protected buildTaskContract(task: WdlTask): TaskContract {
    const requiredInputs = new Set<string>();
    const outputs = new Set<string>();
    const privateDeclarations = new Set<string>();
    const inputTypes = new Map<string, WdlType | undefined>();
    const outputTypes = new Map<string, WdlType | undefined>();

    for (const taskElement of task.elements()) {
      if (taskElement instanceof WdlInput) {
        for (const decl of taskElement.elements()) {
          const name = decl.getName();
          if (name) {
            inputTypes.set(name, decl.getType());
            if (
              !(decl instanceof WdlBoundDeclaration) &&
              decl.getType() &&
              !decl.getType()!.isOptional()
            ) {
              requiredInputs.add(name);
            }
          }
        }
      } else if (taskElement instanceof WdlOutput) {
        for (const decl of taskElement.elements()) {
          const name = decl.getName();
          if (name) {
            outputs.add(name);
            outputTypes.set(name, decl.getType());
          }
        }
      } else if (taskElement instanceof WdlBoundDeclaration) {
        const name = taskElement.getName();
        if (name) privateDeclarations.add(name);
      }
    }

    return {
      requiredInputs,
      inputTypes,
      outputs,
      outputTypes,
      privateDeclarations,
    };
  }

  protected buildWorkflowContract(workflow: WdlWorkflow): TaskContract {
    const requiredInputs = new Set<string>();
    const outputs = new Set<string>();
    const privateDeclarations = new Set<string>();
    const inputTypes = new Map<string, WdlType | undefined>();
    const outputTypes = new Map<string, WdlType | undefined>();

    for (const workflowElement of workflow.elements()) {
      if (workflowElement instanceof WdlInput) {
        for (const decl of workflowElement.elements()) {
          const name = decl.getName();
          if (name) {
            inputTypes.set(name, decl.getType());
            if (
              !(decl instanceof WdlBoundDeclaration) &&
              decl.getType() &&
              !decl.getType()!.isOptional()
            ) {
              requiredInputs.add(name);
            }
          }
        }
      } else if (workflowElement instanceof WdlOutput) {
        for (const decl of workflowElement.elements()) {
          const name = decl.getName();
          if (name) {
            outputs.add(name);
            outputTypes.set(name, decl.getType());
          }
        }
      } else if (workflowElement instanceof WdlBoundDeclaration) {
        const name = workflowElement.getName();
        if (name) privateDeclarations.add(name);
      }
    }

    return {
      requiredInputs,
      inputTypes,
      outputs,
      outputTypes,
      privateDeclarations,
    };
  }

  protected indexImportedTopLevelContracts(document: WdlDocument): void {
    const seenImportedCallables = new Set<string>();
    const seenImportedTypes = new Set<string>();
    const localTopLevelNames = new Set<string>();
    for (const element of document.elements()) {
      if (!('getName' in element) || typeof element.getName !== 'function') continue;
      const name = element.getName();
      if (name) localTopLevelNames.add(name);
    }

    for (const imp of document.importStatements()) {
      const importedDocument = this.resolveImportedDocument(document, imp);
      if (!importedDocument) continue;

      const localVersion = document.getWdlVersion();
      const importedVersion = importedDocument.getWdlVersion();
      if (localVersion && importedVersion && localVersion !== importedVersion) {
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Imported document version mismatch: ${localVersion.getVersionString()} vs ${importedVersion.getVersionString()}`,
        );
      }

      const exported = new Set<string>([
        ...importedDocument
          .tasks()
          .map((task) => task.getName())
          .filter(Boolean),
        ...importedDocument
          .workflows()
          .map((workflow) => workflow.getName())
          .filter(Boolean),
        ...importedDocument
          .structs()
          .map((struct) => struct.getName())
          .filter(Boolean),
        ...importedDocument
          .enums()
          .map((enumDef) => enumDef.getName())
          .filter(Boolean),
      ] as string[]);

      if (imp instanceof WdlImportMembers || imp instanceof WdlImportStandard) {
        const seenLocalAliases = new Set<string>();
        for (const member of imp.members()) {
          const memberName = member.getMember();
          if (!memberName) continue;
          if (!exported.has(memberName)) {
            this.addError(
              WdlSemanticErrorCode.UNKNOWN_REFERENCE,
              `Import member '${memberName}' does not exist`,
            );
          }
          const localAlias = member.getAlias() || memberName;
          if (seenLocalAliases.has(localAlias)) {
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Duplicate import member alias '${localAlias}'`,
            );
          }
          seenLocalAliases.add(localAlias);
          if (localTopLevelNames.has(localAlias)) {
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Imported member '${localAlias}' conflicts with local definition`,
            );
          }
        }
      }

      const callableNames = this.visibleImportedCallableNames(imp, importedDocument);
      for (const [visibleName, callable] of callableNames.entries()) {
        if (seenImportedCallables.has(visibleName)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate imported callable name '${visibleName}'`,
          );
        }
        seenImportedCallables.add(visibleName);
        if (localTopLevelNames.has(visibleName) && !visibleName.includes('.')) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Imported callable '${visibleName}' conflicts with local definition`,
          );
        }
        if (callable instanceof WdlTask)
          this.taskContracts.set(visibleName, this.buildTaskContract(callable));
        else this.taskContracts.set(visibleName, this.buildWorkflowContract(callable));
      }

      const visibleStructNames = this.visibleImportedStructNames(imp, importedDocument);
      for (const [visibleName, struct] of visibleStructNames.entries()) {
        if (seenImportedTypes.has(visibleName)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate imported type name '${visibleName}'`,
          );
        }
        seenImportedTypes.add(visibleName);
        if (localTopLevelNames.has(visibleName)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Imported type '${visibleName}' conflicts with local definition`,
          );
        }
        const members = new Set<string>();
        const memberTypes = new Map<string, WdlType | undefined>();
        for (const member of struct.elements()) {
          if (!('getName' in member) || typeof member.getName !== 'function') continue;
          const memberName = member.getName();
          if (!memberName) continue;
          members.add(memberName);
          memberTypes.set(
            memberName,
            'getType' in member && typeof member.getType === 'function'
              ? member.getType()
              : undefined,
          );
        }
        this.structMembers.set(visibleName, members);
        this.structMemberTypes.set(visibleName, memberTypes);
      }

      const visibleEnums = this.visibleImportedEnums(imp, importedDocument);
      for (const [visibleName, enumDef] of visibleEnums.entries()) {
        if (seenImportedTypes.has(visibleName)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate imported type name '${visibleName}'`,
          );
        }
        seenImportedTypes.add(visibleName);
        if (localTopLevelNames.has(visibleName)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Imported type '${visibleName}' conflicts with local definition`,
          );
        }
        this.enumValueTypes.set(
          visibleName,
          inferEnumValueType(enumDef) ?? new WdlPrimitiveType(WdlPrimitiveType.Type.STRING),
        );
        this.enumChoiceNames.set(
          visibleName,
          new Set(
            enumDef
              .elements()
              .map((choice) => choice.getKey())
              .filter((name): name is string => Boolean(name && name.trim())),
          ),
        );
      }
    }
  }

  protected visibleImportedCallableNames(
    imp: WdlImport,
    importedDocument: WdlDocument,
  ): Map<string, WdlTask | WdlWorkflow> {
    const visible = new Map<string, WdlTask | WdlWorkflow>();
    const callables = [...importedDocument.tasks(), ...importedDocument.workflows()];

    if (imp instanceof WdlImportStandard) {
      const namespace = this.importNamespace(imp);
      for (const callable of callables) {
        const name = callable.getName();
        if (!name) continue;
        visible.set(`${namespace}.${name}`, callable);
      }
      for (const member of imp.members()) {
        const memberName = member.getMember();
        if (!memberName) continue;
        const callable = callables.find((entry) => entry.getName() === memberName);
        if (!callable) continue;
        const localName = member.getAlias() || memberName;
        visible.set(localName, callable);
      }
      return visible;
    }

    if (imp instanceof WdlImportStar) {
      for (const callable of callables) {
        const name = callable.getName();
        if (!name) continue;
        visible.set(name, callable);
      }
      return visible;
    }

    if (imp instanceof WdlImportMembers) {
      for (const member of imp.members()) {
        const memberName = member.getMember();
        if (!memberName) continue;
        const callable = callables.find((entry) => entry.getName() === memberName);
        if (!callable) continue;
        const localName = member.getAlias() || memberName;
        visible.set(localName, callable);
      }
    }
    return visible;
  }

  protected visibleImportedStructNames(
    imp: WdlImport,
    importedDocument: WdlDocument,
  ): Map<string, WdlStruct> {
    const visible = new Map<string, WdlStruct>();
    const structs = importedDocument.structs();

    if (imp instanceof WdlImportStandard) {
      const aliases = new Map<string, string>();
      for (const member of imp.members()) {
        const memberName = member.getMember();
        if (!memberName) continue;
        aliases.set(memberName, member.getAlias() || memberName);
      }
      for (const struct of structs) {
        const name = struct.getName();
        if (!name) continue;
        visible.set(aliases.get(name) ?? name, struct);
      }
      return visible;
    }

    if (imp instanceof WdlImportStar) {
      for (const struct of structs) {
        const name = struct.getName();
        if (!name) continue;
        visible.set(name, struct);
      }
      return visible;
    }

    if (imp instanceof WdlImportMembers) {
      for (const member of imp.members()) {
        const memberName = member.getMember();
        if (!memberName) continue;
        const struct = structs.find((entry) => entry.getName() === memberName);
        if (!struct) continue;
        const localName = member.getAlias() || memberName;
        visible.set(localName, struct);
      }
    }
    return visible;
  }

  protected visibleImportedEnums(imp: WdlImport, importedDocument: WdlDocument): Map<string, WdlEnum> {
    const visible = new Map<string, WdlEnum>();
    const enums = importedDocument.enums();

    if (imp instanceof WdlImportStandard) {
      const aliases = new Map<string, string>();
      for (const member of imp.members()) {
        const memberName = member.getMember();
        if (!memberName) continue;
        aliases.set(memberName, member.getAlias() || memberName);
      }
      for (const enumDef of enums) {
        const name = enumDef.getName();
        if (!name) continue;
        visible.set(aliases.get(name) ?? name, enumDef);
      }
      return visible;
    }

    if (imp instanceof WdlImportStar) {
      for (const enumDef of enums) {
        const name = enumDef.getName();
        if (name) visible.set(name, enumDef);
      }
      return visible;
    }

    if (imp instanceof WdlImportMembers) {
      for (const member of imp.members()) {
        const memberName = member.getMember();
        if (!memberName) continue;
        const enumDef = enums.find((entry) => entry.getName() === memberName);
        if (!enumDef) continue;
        visible.set(member.getAlias() || memberName, enumDef);
      }
    }
    return visible;
  }

  public override processWorkflowScatter(ctx: WdlWorkflow, node: WdlScatter): void {
    this.validateExpression(node.getCollection());
    for (const statement of node.statements()) this.processWorkflowStatement(ctx, statement);
  }

  public override processWorkflowConditional(ctx: WdlWorkflow, node: WdlConditional): void {
    this.validateExpression(node.getCondition());
    for (const statement of node.thenStatements()) this.processWorkflowStatement(ctx, statement);
    for (const elseIf of node.elseIfs()) {
      this.validateExpression(elseIf.getCondition());
      for (const statement of elseIf.thenStatements())
        this.processWorkflowStatement(ctx, statement);
    }
    for (const statement of node.elseStatements()) this.processWorkflowStatement(ctx, statement);
  }

  protected processWorkflowStatement(workflow: WdlWorkflow, statement: WdlStatement): void {
    if (statement instanceof WdlBoundDeclaration)
      this.processWorkflowDeclaration(workflow, statement);
    else if (statement instanceof WdlCall) this.processWorkflowCall(workflow, statement);
    else if (statement instanceof WdlScatter) this.processWorkflowScatter(workflow, statement);
    else if (statement instanceof WdlConditional)
      this.processWorkflowConditional(workflow, statement);
  }

  protected validateBoundDeclaration(node: WdlBoundDeclaration): void {
    const name = node.getName();
    if (name) this.scopeTypes.set(name, node.getType());

    this.validateExpression(node.getExpression());
    if (!this.isAssignableFrom(node.getType(), node.getExpression())) {
      this.addError(
        WdlSemanticErrorCode.TYPE_MISMATCH,
        `Declaration '${name ?? '<unnamed>'}' type is incompatible with expression`,
      );
    }

    const type = node.getType();
    const expression = node.getExpression();
    if (
      type instanceof WdlArrayType &&
      type.isNonEmpty() &&
      expression instanceof WdlArrayLiteral &&
      expression.entries().length === 0
    ) {
      this.addError(
        WdlSemanticErrorCode.TYPE_MISMATCH,
        `Declaration '${name ?? '<unnamed>'}' requires a non-empty array`,
      );
    }

    if (name) this.scopeValues.set(name, this.evaluate(expression));
  }

  protected validateExpression(expr: WdlExpression | undefined): void {
    if (!expr) return;

    if (expr instanceof WdlFunctionCallOperation) {
      this.validateFunctionVersionAvailability(expr);
      this.validateFunctionCall(expr);
      for (const arg of expr.arguments()) this.validateExpression(arg);
      return;
    }

    if (expr instanceof WdlIndexAccessOperation) {
      this.validateExpression(expr.getTarget());
      this.validateExpression(expr.getIndex());
      const targetValue = this.evaluate(expr.getTarget());
      const indexValue = this.evaluate(expr.getIndex());
      if (Array.isArray(targetValue) && typeof indexValue === 'number') {
        if (indexValue < 0 || indexValue >= targetValue.length)
          this.addError(WdlSemanticErrorCode.UNKNOWN_REFERENCE, 'Array index out of bounds');
      } else if (targetValue instanceof Map && indexValue !== UNKNOWN) {
        if (!targetValue.has(indexValue))
          this.addError(
            WdlSemanticErrorCode.UNKNOWN_REFERENCE,
            `Map key does not exist: ${String(indexValue)}`,
          );
      }
      return;
    }

    if (expr instanceof WdlMemberAccessOperation) {
      this.validateExpression(expr.getTarget());
      const targetExpr = expr.getTarget();
      if (targetExpr instanceof WdlVariable) {
        const targetName = targetExpr.getName();
        const memberName = expr.getMember() ?? '';
        if (targetName && this.callOutputs.has(targetName)) {
          if (!this.callOutputs.get(targetName)!.has(memberName))
            this.addError(
              WdlSemanticErrorCode.UNKNOWN_REFERENCE,
              `'${memberName}' is not an output field of call '${targetName}'`,
            );
        } else if (targetName && this.scopeTypes.has(targetName)) {
          const declaredType = this.scopeTypes.get(targetName);
          if (declaredType instanceof WdlTypeReferenceType) {
            const structName = declaredType.referenceName();
            const members = this.structMembers.get(structName);
            if (members && !members.has(memberName))
              this.addError(
                WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                `Field '${memberName}' does not exist in struct '${structName}'`,
              );
          }
        }
      }
      return;
    }

    if (expr instanceof WdlArrayLiteral) {
      for (const entry of expr.entries()) this.validateExpression(entry);
      return;
    }
    if (expr instanceof WdlMapLiteral) {
      for (const entry of expr.entries()) {
        this.validateExpression(entry.getKey());
        this.validateExpression(entry.getValue());
      }
      return;
    }
    if (expr instanceof WdlPairLiteral) {
      this.validateExpression(expr.getLeft());
      this.validateExpression(expr.getRight());
      return;
    }
    if (expr instanceof WdlObjectLiteral) {
      for (const entry of expr.entries()) this.validateExpression(entry.getValue());
      return;
    }
    if (expr instanceof WdlStructLiteral) {
      for (const entry of expr.entries()) this.validateExpression(entry.getValue());
      return;
    }
    if (expr instanceof WdlStringLiteral) {
      for (const component of expr.components()) {
        if (component instanceof WdlStringPlaceholder) {
          this.validateExpression(component.expression);
          const option = component.option;
          if (option) {
            this.validateExpression(option.value);
            this.validateExpression(option.trueValue);
            this.validateExpression(option.falseValue);
          }
        }
      }
      return;
    }

    if (expr instanceof WdlBinaryOperation) {
      this.validateExpression(expr.getLeft());
      this.validateExpression(expr.getRight());
      return;
    }
    if (expr instanceof WdlUnaryOperation) {
      this.validateExpression(expr.getOperand());
      return;
    }
    if (expr instanceof WdlTernaryOperation) {
      this.validateExpression(expr.getCondition());
      this.validateExpression(expr.getTrueValue());
      this.validateExpression(expr.getFalseValue());
    }
  }

  protected validateFunctionCall(functionCall: WdlFunctionCallOperation): void {
    const fn = functionCall.getFunction();
    switch (fn) {
      case WdlFunction.SELECT_FIRST:
        this.processSelectFirst(functionCall);
        break;
      case WdlFunction.AS_MAP:
        this.processAsMap(functionCall);
        break;
      case WdlFunction.ZIP:
        this.processZip(functionCall);
        break;
      case WdlFunction.WRITE_JSON:
        this.processWriteJson(functionCall);
        break;
      default:
        break;
    }
  }

  protected evaluate(expr: WdlExpression | undefined): unknown {
    if (!expr) return UNKNOWN;
    if (expr instanceof WdlNullLiteral) return null;
    if ('getValue' in expr && typeof expr.getValue === 'function') return expr.getValue();
    if (expr instanceof WdlStringLiteral) {
      const textParts: string[] = [];
      for (const component of expr.components()) {
        if (component instanceof WdlStringText && component.text !== undefined)
          textParts.push(component.text);
        else if (component instanceof WdlStringEscape && component.escapeText !== undefined)
          textParts.push(component.escapeText);
        else if (component instanceof WdlStringToken && component.tokenText !== undefined)
          textParts.push(component.tokenText);
        else return UNKNOWN;
      }
      return textParts.join('');
    }
    if (expr instanceof WdlVariable) {
      const name = expr.getName();
      if (!name) return UNKNOWN;
      if (name === 'None') return null;
      return this.scopeValues.get(name) ?? UNKNOWN;
    }
    if (expr instanceof WdlArrayLiteral) return expr.entries().map((entry) => this.evaluate(entry));
    if (expr instanceof WdlPairLiteral) {
      const left = this.evaluate(expr.getLeft());
      const right = this.evaluate(expr.getRight());
      if (left === UNKNOWN || right === UNKNOWN) return UNKNOWN;
      return [left, right] as const;
    }
    if (expr instanceof WdlMapLiteral) {
      const out = new Map<unknown, unknown>();
      for (const entry of expr.entries()) {
        const key = this.evaluate(entry.getKey());
        const value = this.evaluate(entry.getValue());
        if (key === UNKNOWN) return UNKNOWN;
        out.set(key, value);
      }
      return out;
    }
    return UNKNOWN;
  }

  protected inferType(expr: WdlExpression | undefined): WdlType | undefined {
    if (!expr) return undefined;
    if (
      expr instanceof WdlIntLiteral ||
      expr instanceof WdlFloatLiteral ||
      expr instanceof WdlBooleanLiteral ||
      expr instanceof WdlStringLiteral ||
      expr instanceof WdlObjectLiteral ||
      expr instanceof WdlStructLiteral ||
      expr instanceof WdlNullLiteral
    ) {
      const literalType = inferLiteralExpressionType(expr);
      if (literalType) return literalType;
      if (expr instanceof WdlStructLiteral) return new WdlTypeReferenceType('Object');
    }
    if (expr instanceof WdlIntLiteral) return new WdlPrimitiveType(WdlPrimitiveType.Type.INT);
    if (expr instanceof WdlFloatLiteral) return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
    if (expr instanceof WdlBooleanLiteral)
      return new WdlPrimitiveType(WdlPrimitiveType.Type.BOOLEAN);
    if (expr instanceof WdlStringLiteral) return new WdlPrimitiveType(WdlPrimitiveType.Type.STRING);
    if (expr instanceof WdlNullLiteral) return undefined;
    if (expr instanceof WdlVariable) {
      const name = expr.getName();
      if (name === 'None') return undefined;
      return name ? this.scopeTypes.get(name) : undefined;
    }
    if (expr instanceof WdlArrayLiteral) {
      let memberType: WdlType | undefined;
      for (const item of expr.entries())
        memberType = this.mergeTypes(memberType, this.inferType(item));
      return new WdlArrayType(memberType);
    }
    if (expr instanceof WdlPairLiteral) {
      const left = this.inferType(expr.getLeft());
      const right = this.inferType(expr.getRight());
      return left && right ? new WdlPairType(left, right) : undefined;
    }
    if (expr instanceof WdlMapLiteral) {
      let keyType: WdlType | undefined;
      let valueType: WdlType | undefined;
      for (const entry of expr.entries()) {
        keyType = this.mergeTypes(keyType, this.inferType(entry.getKey()));
        valueType = this.mergeTypes(valueType, this.inferType(entry.getValue()));
      }
      return new WdlMapType(keyType, valueType);
    }
    if (expr instanceof WdlIndexAccessOperation) {
      const targetType = this.inferType(expr.getTarget());
      if (targetType instanceof WdlArrayType) return targetType.memberType();
      if (targetType instanceof WdlMapType) return targetType.valueType();
      return undefined;
    }
    if (expr instanceof WdlMemberAccessOperation) {
      const targetExpr = expr.getTarget();
      if (targetExpr instanceof WdlVariable) {
        const targetName = targetExpr.getName();
        const memberName = expr.getMember() ?? '';
        if (targetName && this.callOutputTypes.has(targetName))
          return this.callOutputTypes.get(targetName)!.get(memberName);
        if (targetName) {
          const targetType = this.scopeTypes.get(targetName);
          if (targetType instanceof WdlTypeReferenceType) {
            return this.structMemberTypes.get(targetType.referenceName())?.get(memberName);
          }
        }
      }
      return undefined;
    }
    if (expr instanceof WdlFunctionCallOperation) return this.inferFunctionType(expr);
    if (expr instanceof WdlUnaryOperation) {
      if (expr.getOperator() === WdlUnaryOperator.NOT)
        return new WdlPrimitiveType(WdlPrimitiveType.Type.BOOLEAN);
      if (expr.getOperator() === WdlUnaryOperator.NEGATIVE)
        return this.inferType(expr.getOperand());
    }
    if (expr instanceof WdlBinaryOperation) {
      const op = expr.getOperator();
      if (
        [
          WdlBinaryOperator.OR,
          WdlBinaryOperator.AND,
          WdlBinaryOperator.EQ,
          WdlBinaryOperator.NEQ,
          WdlBinaryOperator.LT,
          WdlBinaryOperator.LTE,
          WdlBinaryOperator.GT,
          WdlBinaryOperator.GTE,
        ].includes(op!)
      ) {
        return new WdlPrimitiveType(WdlPrimitiveType.Type.BOOLEAN);
      }
      if (
        [
          WdlBinaryOperator.ADD,
          WdlBinaryOperator.SUBTRACT,
          WdlBinaryOperator.MULTIPLY,
          WdlBinaryOperator.DIVIDE,
          WdlBinaryOperator.MODULO,
          WdlBinaryOperator.POWER,
        ].includes(op!)
      ) {
        const left = this.inferType(expr.getLeft());
        const right = this.inferType(expr.getRight());
        if (
          this.isPrimitive(left, WdlPrimitiveType.Type.FLOAT) ||
          this.isPrimitive(right, WdlPrimitiveType.Type.FLOAT)
        )
          return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
        if (
          this.isPrimitive(left, WdlPrimitiveType.Type.INT) &&
          this.isPrimitive(right, WdlPrimitiveType.Type.INT)
        )
          return new WdlPrimitiveType(WdlPrimitiveType.Type.INT);
        if (
          op === WdlBinaryOperator.ADD &&
          (this.isPrimitive(left, WdlPrimitiveType.Type.STRING) ||
            this.isPrimitive(right, WdlPrimitiveType.Type.STRING))
        )
          return new WdlPrimitiveType(WdlPrimitiveType.Type.STRING);
      }
    }
    if (expr instanceof WdlTernaryOperation)
      return this.mergeTypes(
        this.inferType(expr.getTrueValue()),
        this.inferType(expr.getFalseValue()),
      );
    return undefined;
  }

  protected validateFunctionVersionAvailability(functionCall: WdlFunctionCallOperation): void {
    const fn = functionCall.getFunction();
    if (fn === WdlFunction.NONSTANDARD || !this.documentVersion) return;
    const addedIn = this.functionAddedIn.get(fn);
    if (!addedIn) return;
    if (this.versionTuple(this.documentVersion) < this.versionTuple(addedIn)) {
      this.addError(
        WdlSemanticErrorCode.FUNCTION_NOT_AVAILABLE_IN_VERSION,
        `Function '${fn.toWdlString()}' is not available in WDL ${this.documentVersion.getVersionString()} (added in ${addedIn.getVersionString()})`,
      );
    }
  }

  protected versionTuple(version: WdlVersion): number {
    return version.major * 100 + version.minor;
  }

  protected inferFunctionType(functionCall: WdlFunctionCallOperation): WdlType | undefined {
    const fn = functionCall.getFunction();
    if (
      [
        WdlFunction.DEFINED,
        WdlFunction.CONTAINS,
        WdlFunction.CONTAINS_KEY,
        WdlFunction.MATCHES,
      ].includes(fn)
    )
      return new WdlPrimitiveType(WdlPrimitiveType.Type.BOOLEAN);
    if ([WdlFunction.LENGTH, WdlFunction.READ_INT].includes(fn))
      return new WdlPrimitiveType(WdlPrimitiveType.Type.INT);
    if (fn === WdlFunction.READ_FLOAT) return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
    if (
      [
        WdlFunction.READ_STRING,
        WdlFunction.STDOUT,
        WdlFunction.STDERR,
        WdlFunction.WRITE_LINES,
        WdlFunction.WRITE_TSV,
        WdlFunction.WRITE_MAP,
        WdlFunction.WRITE_OBJECT,
        WdlFunction.WRITE_OBJECTS,
        WdlFunction.WRITE_JSON,
        WdlFunction.BASENAME,
        WdlFunction.PREFIX,
        WdlFunction.SUFFIX,
        WdlFunction.QUOTE,
        WdlFunction.SQUOTE,
        WdlFunction.SEP,
      ].includes(fn)
    )
      return new WdlPrimitiveType(WdlPrimitiveType.Type.STRING);
    if (fn === WdlFunction.READ_BOOLEAN) return new WdlPrimitiveType(WdlPrimitiveType.Type.BOOLEAN);
    if ([WdlFunction.READ_LINES, WdlFunction.GLOB].includes(fn))
      return new WdlArrayType(new WdlPrimitiveType(WdlPrimitiveType.Type.STRING));
    if (fn === WdlFunction.RANGE)
      return new WdlArrayType(new WdlPrimitiveType(WdlPrimitiveType.Type.INT));
    if (fn === WdlFunction.SELECT_FIRST && functionCall.arguments()[0]) {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (argType instanceof WdlArrayType) return argType.memberType();
    }
    if (fn === WdlFunction.ZIP && functionCall.arguments().length >= 2) {
      const left = this.inferType(functionCall.arguments()[0]);
      const right = this.inferType(functionCall.arguments()[1]);
      if (
        left instanceof WdlArrayType &&
        right instanceof WdlArrayType &&
        left.memberType() &&
        right.memberType()
      )
        return new WdlArrayType(new WdlPairType(left.memberType(), right.memberType()));
    }
    if (fn === WdlFunction.AS_MAP && functionCall.arguments()[0]) {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (argType instanceof WdlArrayType) {
        const pair = argType.memberType();
        if (!(pair instanceof WdlPairType)) return undefined;
        return new WdlMapType(pair.leftType(), pair.rightType());
      }
    }
    if (fn === WdlFunction.KEYS && functionCall.arguments()[0]) {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (argType instanceof WdlMapType) return new WdlArrayType(argType.keyType());
    }
    if (fn === WdlFunction.VALUES && functionCall.arguments()[0]) {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (argType instanceof WdlMapType) return new WdlArrayType(argType.valueType());
    }
    return undefined;
  }

  protected isAssignableFrom(
    expected: WdlType | undefined,
    expr: WdlExpression | undefined,
  ): boolean {
    if (!expected || !expr) return true;
    if (expr instanceof WdlNullLiteral) return expected.isOptional();
    if (expr instanceof WdlVariable && expr.getName() === 'None') return expected.isOptional();

    if (expected instanceof WdlArrayType && expr instanceof WdlArrayLiteral) {
      const memberType = expected.memberType();
      for (const entry of expr.entries()) {
        if (!this.isAssignableFrom(memberType, entry)) return false;
      }
      return true;
    }
    if (expected instanceof WdlMapType && expr instanceof WdlMapLiteral) {
      for (const entry of expr.entries()) {
        if (!this.isAssignableFrom(expected.keyType(), entry.getKey())) return false;
        if (!this.isAssignableFrom(expected.valueType(), entry.getValue())) return false;
      }
      return true;
    }
    if (expected instanceof WdlPairType && expr instanceof WdlPairLiteral) {
      return (
        this.isAssignableFrom(expected.leftType(), expr.getLeft()) &&
        this.isAssignableFrom(expected.rightType(), expr.getRight())
      );
    }

    if (this.isEnumTypeReference(expected)) {
      const evaluated = this.evaluate(expr);
      if (typeof evaluated === 'string') {
        const choices = this.enumChoiceNames.get(this.typeReferenceName(expected));
        if (choices && choices.size > 0) return choices.has(evaluated);
      }
    }

    const actual = this.inferType(expr);
    if (!actual) return true;
    return this.isTypeAssignable(expected, actual);
  }

  protected validateImportDeclarations(document: WdlDocument): void {
    const seenNamespaces = new Set<string>();
    const localTopLevelNames = new Set<string>();
    for (const element of document.elements()) {
      if (!('getName' in element) || typeof element.getName !== 'function') continue;
      const name = element.getName();
      if (name) localTopLevelNames.add(name);
    }

    for (const imp of document.importStatements()) {
      const importedDocument = this.resolveImportedDocument(document, imp);
      if (!importedDocument) {
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Unable to resolve import source '${this.importSourceText(imp) || '<unknown>'}'`,
        );
        continue;
      }

      if (imp instanceof WdlImportStandard) {
        const namespace = this.importNamespace(imp);
        if (seenNamespaces.has(namespace)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate import namespace '${namespace}'`,
          );
        }
        seenNamespaces.add(namespace);
        if (localTopLevelNames.has(namespace)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Import namespace '${namespace}' conflicts with local definition`,
          );
        }
      }
    }
  }

  protected isTypeAssignable(expected: WdlType | undefined, actual: WdlType | undefined): boolean {
    if (!expected || !actual) return true;
    if (!expected.isOptional() && actual.isOptional()) return false;
    if (expected.componentType() !== actual.componentType()) {
      if (
        this.isPrimitive(expected, WdlPrimitiveType.Type.FLOAT) &&
        this.isPrimitive(actual, WdlPrimitiveType.Type.INT)
      )
        return true;
      if (
        this.isPrimitive(expected, WdlPrimitiveType.Type.FILE) &&
        this.isPrimitive(actual, WdlPrimitiveType.Type.STRING)
      )
        return true;
      if (
        this.isPrimitive(expected, WdlPrimitiveType.Type.DIRECTORY) &&
        this.isPrimitive(actual, WdlPrimitiveType.Type.STRING)
      )
        return true;
      if (
        this.isPrimitive(expected, WdlPrimitiveType.Type.STRING) &&
        this.isEnumTypeReference(actual)
      )
        return true;
      if (
        this.isEnumTypeReference(expected) &&
        this.isPrimitive(actual, WdlPrimitiveType.Type.STRING)
      )
        return true;
      if (expected instanceof WdlMapType && this.isStructTypeReference(actual)) {
        const members = this.structMemberTypes.get(this.typeReferenceName(actual));
        if (!members || members.size === 0) return false;
        for (const memberType of members.values()) {
          if (!this.isTypeAssignable(expected.valueType(), memberType)) return false;
        }
        return true;
      }
      return false;
    }
    if (expected instanceof WdlPrimitiveType && actual instanceof WdlPrimitiveType)
      {
        if (
          expected.primitiveType() === WdlPrimitiveType.Type.FILE &&
          actual.primitiveType() === WdlPrimitiveType.Type.STRING
        )
          return true;
        if (
          expected.primitiveType() === WdlPrimitiveType.Type.DIRECTORY &&
          actual.primitiveType() === WdlPrimitiveType.Type.STRING
        )
          return true;
        return expected.primitiveType() === actual.primitiveType();
      }
    if (expected instanceof WdlArrayType && actual instanceof WdlArrayType)
      return this.isTypeAssignable(expected.memberType(), actual.memberType());
    if (expected instanceof WdlMapType && actual instanceof WdlMapType)
      return (
        this.isTypeAssignable(expected.keyType(), actual.keyType()) &&
        this.isTypeAssignable(expected.valueType(), actual.valueType())
      );
    if (expected instanceof WdlPairType && actual instanceof WdlPairType)
      return (
        this.isTypeAssignable(expected.leftType(), actual.leftType()) &&
        this.isTypeAssignable(expected.rightType(), actual.rightType())
      );
    if (expected instanceof WdlTypeReferenceType && actual instanceof WdlTypeReferenceType) {
      if (expected.referenceName() === actual.referenceName()) return true;
      if (this.isStructTypeReference(expected) && this.isStructTypeReference(actual)) {
        return this.areStructReferencesCompatible(
          expected.referenceName(),
          actual.referenceName(),
          new Set<string>(),
        );
      }
      return false;
    }
    return true;
  }

  protected isEnumTypeReference(type: WdlType | undefined): boolean {
    return type instanceof WdlTypeReferenceType && this.enumValueTypes.has(this.typeReferenceName(type));
  }

  protected isStructTypeReference(type: WdlType | undefined): boolean {
    return type instanceof WdlTypeReferenceType && this.structMemberTypes.has(this.typeReferenceName(type));
  }

  protected typeReferenceName(type: WdlType | undefined): string {
    if (!(type instanceof WdlTypeReferenceType)) return '';
    return type.referenceName();
  }

  protected areStructReferencesCompatible(
    expectedStructName: string | undefined,
    actualStructName: string | undefined,
    visitingPairs: Set<string>,
  ): boolean {
    if (expectedStructName === actualStructName) return true;
    if (!expectedStructName || !actualStructName) return false;

    const pairKey = `${expectedStructName}<=${actualStructName}`;
    if (visitingPairs.has(pairKey)) return true;
    visitingPairs.add(pairKey);

    const expectedMembers = this.structMemberTypes.get(expectedStructName);
    const actualMembers = this.structMemberTypes.get(actualStructName);
    if (!expectedMembers || !actualMembers) return false;

    for (const [memberName, expectedMemberType] of expectedMembers.entries()) {
      if (!actualMembers.has(memberName)) return false;
      const actualMemberType = actualMembers.get(memberName);
      if (this.isStructTypeReference(expectedMemberType) && this.isStructTypeReference(actualMemberType)) {
        if (
          !this.areStructReferencesCompatible(
            this.typeReferenceName(expectedMemberType),
            this.typeReferenceName(actualMemberType),
            visitingPairs,
          )
        )
          return false;
        continue;
      }
      if (!this.isTypeAssignable(expectedMemberType, actualMemberType)) return false;
    }

    return true;
  }

  protected mergeTypes(
    current: WdlType | undefined,
    next: WdlType | undefined,
  ): WdlType | undefined {
    if (!next) return current;
    if (!current) return next;
    if (this.isTypeAssignable(current, next)) return current;
    if (this.isTypeAssignable(next, current)) return next;
    if (
      this.isPrimitive(current, WdlPrimitiveType.Type.INT) &&
      this.isPrimitive(next, WdlPrimitiveType.Type.FLOAT)
    )
      return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
    if (
      this.isPrimitive(current, WdlPrimitiveType.Type.FLOAT) &&
      this.isPrimitive(next, WdlPrimitiveType.Type.INT)
    )
      return new WdlPrimitiveType(WdlPrimitiveType.Type.FLOAT);
    return undefined;
  }

  protected isPrimitive(type: WdlType | undefined, primitive: WdlPrimitiveType.Type): boolean {
    return type instanceof WdlPrimitiveType && type.primitiveType() === primitive;
  }

  protected processSelectFirst(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1) {
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'select_first expects exactly 1 argument',
      );
      return;
    }
    const firstArg = functionCall.arguments()[0];
    const argType = this.inferType(firstArg);
    if (argType && !(argType instanceof WdlArrayType))
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'select_first expects an array argument',
      );
    if (firstArg instanceof WdlArrayLiteral && firstArg.entries().length === 0) {
      this.addError(WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS, 'select_first array is empty');
      return;
    }
    const value = this.evaluate(firstArg);
    if (Array.isArray(value)) {
      if (value.length === 0)
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'select_first array is empty',
        );
      else if (value.every((item) => item === null))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'select_first array contains only None values',
        );
    }
  }

  protected processAsMap(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1) {
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'as_map expects exactly 1 argument',
      );
      return;
    }
    const firstArg = functionCall.arguments()[0];
    const firstArgType = this.inferType(firstArg);
    if (
      firstArgType &&
      (!(firstArgType instanceof WdlArrayType) ||
        !(firstArgType.memberType() instanceof WdlPairType))
    )
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'as_map expects Array[Pair[K,V]]',
      );
    if (!(firstArg instanceof WdlArrayLiteral)) return;
    const seen = new Set<unknown>();
    for (const entry of firstArg.entries()) {
      if (!(entry instanceof WdlPairLiteral)) continue;
      const key = this.evaluate(entry.getLeft());
      if (key === UNKNOWN) continue;
      if (seen.has(key)) {
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          `as_map has duplicate key: ${String(key)}`,
        );
        return;
      }
      seen.add(key);
    }
  }

  protected processZip(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 2) {
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'zip expects exactly 2 arguments',
      );
      return;
    }
    const leftType = this.inferType(functionCall.arguments()[0]);
    const rightType = this.inferType(functionCall.arguments()[1]);
    if (
      (leftType && !(leftType instanceof WdlArrayType)) ||
      (rightType && !(rightType instanceof WdlArrayType))
    )
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'zip expects two array arguments',
      );
    const left = this.evaluate(functionCall.arguments()[0]);
    const right = this.evaluate(functionCall.arguments()[1]);
    if (Array.isArray(left) && Array.isArray(right) && left.length !== right.length)
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'zip arguments must have the same length',
      );
  }

  protected processWriteJson(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1) {
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'write_json expects exactly 1 argument',
      );
      return;
    }
    if (this.containsNonStringMapKey(functionCall.arguments()[0]))
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'write_json argument contains a map with non-string keys',
      );
  }

  protected containsNonStringMapKey(expr: WdlExpression | undefined): boolean {
    if (!expr) return false;
    if (expr instanceof WdlMapLiteral) {
      for (const entry of expr.entries()) {
        const keyValue = this.evaluate(entry.getKey());
        if (typeof keyValue !== 'string') return true;
        if (this.containsNonStringMapKey(entry.getValue())) return true;
      }
      return false;
    }
    if (expr instanceof WdlArrayLiteral)
      return expr.entries().some((item) => this.containsNonStringMapKey(item));
    if (expr instanceof WdlPairLiteral)
      return (
        this.containsNonStringMapKey(expr.getLeft()) ||
        this.containsNonStringMapKey(expr.getRight())
      );
    if (expr instanceof WdlObjectLiteral)
      return expr.entries().some((entry) => this.containsNonStringMapKey(entry.getValue()));
    if (expr instanceof WdlStructLiteral)
      return expr.entries().some((entry) => this.containsNonStringMapKey(entry.getValue()));
    if (expr instanceof WdlVariable) {
      const value = this.scopeValues.get(expr.getName() ?? '');
      return value !== undefined && this.containsNonStringMapKeyInValue(value);
    }
    return false;
  }

  protected containsNonStringMapKeyInValue(value: unknown): boolean {
    if (value instanceof Map) {
      for (const [key, nested] of value.entries()) {
        if (typeof key !== 'string') return true;
        if (this.containsNonStringMapKeyInValue(nested)) return true;
      }
      return false;
    }
    if (Array.isArray(value))
      return value.some((item) => this.containsNonStringMapKeyInValue(item));
    return false;
  }

  protected addError(code: WdlSemanticErrorCode, message: string): void {
    this.errors.push(new WdlSemanticError(code, message, 0, 0));
  }
}
