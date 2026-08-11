/** Deterministic static-analysis validator for the TypeScript WDL model. */
import { WdlDocument } from '../wdl-document.js';
import { WdlEnum, WdlStruct, WdlTask, WdlWorkflow } from '../definitions/index.js';
import {
  WdlBinaryOperation,
  WdlBinaryOperator,
  type WdlExpression,
  WdlFunction,
  WdlFunctionCallOperation,
  WdlTernaryOperation,
  WdlUnaryOperation,
  WdlUnaryOperator,
} from '../expressions/index.js';
import { WdlSemanticErrorCode } from '../errors/index.js';
import { WdlInput, WdlOutput } from '../sections/index.js';
import {
  WdlBoundDeclaration,
  WdlCall,
  WdlConditional,
  WdlImport,
  WdlImportMembers,
  WdlImportStandard,
  WdlImportStar,
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
import { WdlSemanticValidator } from './wdl-semantic-validator.js';

export class WdlStaticAnalysisSemanticValidator extends WdlSemanticValidator {
  // Signature and arity tables intentionally live in this class (not global
  // helpers) so language-level static policy is easy to evolve in one place.
  private readonly functionSignatures = new Map<WdlFunction, readonly string[][]>([
    [WdlFunction.FLOOR, [['NUMBER']]],
    [WdlFunction.CEIL, [['NUMBER']]],
    [WdlFunction.ROUND, [['NUMBER']]],
    [WdlFunction.MIN, [['NUMBER', 'NUMBER']]],
    [WdlFunction.MAX, [['NUMBER', 'NUMBER']]],
    [
      WdlFunction.SUB,
      [
        ['STRING', 'STRING', 'STRING'],
        ['STRING', 'STRING', 'STRING', 'STRING'],
      ],
    ],
    [WdlFunction.STDOUT, [[]]],
    [WdlFunction.STDERR, [[]]],
    [WdlFunction.READ_LINES, [['FILE']]],
    [WdlFunction.READ_MAP, [['FILE']]],
    [WdlFunction.READ_OBJECT, [['FILE']]],
    [WdlFunction.READ_OBJECTS, [['FILE']]],
    [WdlFunction.READ_JSON, [['ANY']]],
    [WdlFunction.READ_INT, [['FILE']]],
    [WdlFunction.READ_FLOAT, [['FILE']]],
    [WdlFunction.READ_STRING, [['FILE']]],
    [WdlFunction.READ_BOOLEAN, [['FILE']]],
    [WdlFunction.WRITE_LINES, [['ARRAY_STRING']]],
    [WdlFunction.WRITE_TSV, [['ARRAY_ARRAY_ANY']]],
    [WdlFunction.WRITE_MAP, [['MAP_STRING_STRING']]],
    [WdlFunction.WRITE_OBJECT, [['OBJECT']]],
    [WdlFunction.WRITE_OBJECTS, [['ARRAY_OBJECT']]],
    [WdlFunction.WRITE_JSON, [['ANY']]],
    [WdlFunction.GLOB, [['STRING']]],
    [WdlFunction.SIZE, [['FILE_OR_DIRECTORY'], ['ANY', 'STRING']]],
    [WdlFunction.BASENAME, [['FILE_OR_DIRECTORY'], ['STRING', 'STRING']]],
    [WdlFunction.PREFIX, [['STRING', 'ARRAY_ANY']]],
    [WdlFunction.SUFFIX, [['STRING', 'ARRAY_ANY']]],
    [WdlFunction.QUOTE, [['ARRAY_ANY']]],
    [WdlFunction.SQUOTE, [['ARRAY_ANY']]],
    [WdlFunction.SEP, [['STRING', 'ARRAY_ANY']]],
    [WdlFunction.LENGTH, [['ANY']]],
    [WdlFunction.RANGE, [['INT']]],
    [WdlFunction.CHUNK, [['ARRAY_ANY', 'INT']]],
    [WdlFunction.CROSS, [['ARRAY_ANY', 'ARRAY_ANY']]],
    [WdlFunction.ZIP, [['ARRAY_ANY', 'ARRAY_ANY']]],
    [WdlFunction.UNZIP, [['ARRAY_PAIR']]],
    [WdlFunction.TRANSPOSE, [['ARRAY_ARRAY_ANY']]],
    [WdlFunction.FLATTEN, [['ARRAY_ARRAY_ANY']]],
    [WdlFunction.SELECT_FIRST, [['ARRAY_OPTIONAL_ANY'], ['ARRAY_OPTIONAL_ANY', 'ANY']]],
    [WdlFunction.SELECT_ALL, [['ARRAY_OPTIONAL_ANY']]],
    [
      WdlFunction.CONTAINS,
      [
        ['ARRAY_ANY', 'ANY'],
        ['STRING', 'STRING'],
      ],
    ],
    [WdlFunction.CONTAINS_KEY, [['MAP_ANY_ANY', 'ANY']]],
    [WdlFunction.KEYS, [['MAP_ANY_ANY']]],
    [WdlFunction.VALUES, [['MAP_ANY_ANY']]],
    [WdlFunction.AS_PAIRS, [['MAP_ANY_ANY']]],
    [WdlFunction.AS_MAP, [['ARRAY_PAIR']]],
    [WdlFunction.COLLECT_BY_KEY, [['ARRAY_PAIR']]],
    [WdlFunction.MATCHES, [['STRING', 'STRING']]],
    [WdlFunction.FIND, [['STRING', 'STRING']]],
    [WdlFunction.DEFINED, [['ANY_OPTIONAL']]],
    [WdlFunction.JOIN_PATHS, [['FILE_OR_DIRECTORY', 'STRING']]],
    [WdlFunction.VALUE, [['ANY']]],
  ]);

  private readonly functionArity = new Map<WdlFunction, readonly [number, number | undefined]>([
    [WdlFunction.FLOOR, [1, 1]],
    [WdlFunction.CEIL, [1, 1]],
    [WdlFunction.ROUND, [1, 1]],
    [WdlFunction.MIN, [2, 2]],
    [WdlFunction.MAX, [2, 2]],
    [WdlFunction.SUB, [3, 4]],
    [WdlFunction.STDOUT, [0, 0]],
    [WdlFunction.STDERR, [0, 0]],
    [WdlFunction.READ_LINES, [1, 1]],
    [WdlFunction.READ_TSV, [1, 2]],
    [WdlFunction.READ_MAP, [1, 1]],
    [WdlFunction.READ_OBJECT, [1, 1]],
    [WdlFunction.READ_OBJECTS, [1, 1]],
    [WdlFunction.READ_JSON, [1, 1]],
    [WdlFunction.READ_INT, [1, 1]],
    [WdlFunction.READ_FLOAT, [1, 1]],
    [WdlFunction.READ_STRING, [1, 1]],
    [WdlFunction.READ_BOOLEAN, [1, 1]],
    [WdlFunction.WRITE_LINES, [1, 1]],
    [WdlFunction.WRITE_TSV, [1, 1]],
    [WdlFunction.WRITE_MAP, [1, 1]],
    [WdlFunction.WRITE_OBJECT, [1, 1]],
    [WdlFunction.WRITE_OBJECTS, [1, 1]],
    [WdlFunction.WRITE_JSON, [1, 1]],
    [WdlFunction.GLOB, [1, 1]],
    [WdlFunction.SIZE, [1, 2]],
    [WdlFunction.BASENAME, [1, 2]],
    [WdlFunction.PREFIX, [2, 2]],
    [WdlFunction.SUFFIX, [2, 2]],
    [WdlFunction.QUOTE, [1, 1]],
    [WdlFunction.SQUOTE, [1, 1]],
    [WdlFunction.SEP, [2, 2]],
    [WdlFunction.LENGTH, [1, 1]],
    [WdlFunction.RANGE, [1, 1]],
    [WdlFunction.CHUNK, [2, 2]],
    [WdlFunction.CROSS, [2, 2]],
    [WdlFunction.ZIP, [2, 2]],
    [WdlFunction.UNZIP, [1, 1]],
    [WdlFunction.TRANSPOSE, [1, 1]],
    [WdlFunction.FLATTEN, [1, 1]],
    [WdlFunction.SELECT_FIRST, [1, 2]],
    [WdlFunction.SELECT_ALL, [1, 1]],
    [WdlFunction.CONTAINS, [2, 2]],
    [WdlFunction.CONTAINS_KEY, [2, 2]],
    [WdlFunction.KEYS, [1, 1]],
    [WdlFunction.VALUES, [1, 1]],
    [WdlFunction.AS_PAIRS, [1, 1]],
    [WdlFunction.AS_MAP, [1, 1]],
    [WdlFunction.COLLECT_BY_KEY, [1, 1]],
    [WdlFunction.MATCHES, [2, 2]],
    [WdlFunction.FIND, [2, 2]],
    [WdlFunction.DEFINED, [1, 1]],
    [WdlFunction.JOIN_PATHS, [2, undefined]],
    [WdlFunction.VALUE, [1, 1]],
  ]);

  private knownCallableTargets = new Set<string>();
  private knownTypeNames = new Set<string>();

  /**
   * Runs static-analysis checks, then baseline semantic validation.
   *
   * Maintainer note:
   * - We pre-index top-level and imported names before traversal so reference
   *   checks are deterministic and order-independent.
   * - Duplicate keys include declaration kind (task/workflow/struct/enum) to
   *   avoid treating same-name cross-kind declarations as duplicates.
   */
  public override validateDocument(document: WdlDocument): void {
    this.knownCallableTargets = new Set();
    this.knownTypeNames = new Set();
    const topLevelNames = new Set<string>();
    for (const element of document.elements()) {
      if (element instanceof WdlTask && element.getName()) {
        const name = element.getName()!;
        this.knownCallableTargets.add(name);
        const key = `task:${name}`;
        if (topLevelNames.has(key))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate task definition: '${name}'`,
          );
        topLevelNames.add(key);
      } else if (element instanceof WdlWorkflow && element.getName()) {
        const name = element.getName()!;
        this.knownCallableTargets.add(name);
        const key = `workflow:${name}`;
        if (topLevelNames.has(key))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate workflow definition: '${name}'`,
          );
        topLevelNames.add(key);
      } else if (element instanceof WdlStruct && element.getName()) {
        const name = element.getName()!;
        this.knownTypeNames.add(name);
        const key = `struct:${name}`;
        if (topLevelNames.has(key))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate struct definition: '${name}'`,
          );
        topLevelNames.add(key);
      } else if (element instanceof WdlEnum && element.getName()) {
        const name = element.getName()!;
        this.knownTypeNames.add(name);
        const key = `enum:${name}`;
        if (topLevelNames.has(key))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate enum definition: '${name}'`,
          );
        topLevelNames.add(key);
      }
    }

    this.indexImportedTopLevelNames(document, topLevelNames);

    for (const element of document.elements()) {
      if (element instanceof WdlStruct && element.getName()) {
        for (const member of element.elements()) {
          if ('getType' in member && 'getName' in member && typeof member.getType === 'function') {
            const memberName = typeof member.getName === 'function' ? member.getName() : '';
            this.validateKnownTypeReference(
              member.getType(),
              `struct '${element.getName()}' member '${memberName ?? ''}'`,
            );
          }
        }
      }
    }
    super.validateDocument(document);
  }

  public override processWorkflow(ctx: WdlDocument, node: WdlWorkflow): void {
    const seenCallNames = new Set<string>();
    const seenDeclarations = new Set<string>();
    for (const element of node.elements()) {
      if (element instanceof WdlInput) {
        for (const declaration of element.elements()) {
          this.validateKnownTypeReference(
            declaration.getType(),
            `workflow input '${declaration.getName() ?? ''}'`,
          );
          const name = declaration.getName();
          if (name && seenDeclarations.has(name))
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Duplicate workflow declaration: '${name}'`,
            );
          else if (name) seenDeclarations.add(name);
        }
      } else if (element instanceof WdlBoundDeclaration) {
        const name = element.getName();
        this.validateKnownTypeReference(element.getType(), `workflow declaration '${name ?? ''}'`);
        if (name && seenDeclarations.has(name))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate workflow declaration: '${name}'`,
          );
        else if (name) seenDeclarations.add(name);
      } else if (element instanceof WdlCall) {
        const fullTarget = element.targetPath().join('.');
        const target = element.targetPath().at(-1);
        const callName = element.getAlias() ?? target;
        if (
          !this.knownCallableTargets.has(fullTarget) &&
          (!target || !this.knownCallableTargets.has(target))
        )
          this.addError(
            WdlSemanticErrorCode.UNKNOWN_REFERENCE,
            `Call target '${fullTarget || target}' is not defined`,
          );
        if (callName && seenCallNames.has(callName))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate call name in workflow: '${callName}'`,
          );
        else if (callName) seenCallNames.add(callName);
        const seenCallInputs = new Set<string>();
        for (const callInput of element.inputs()) {
          const key = callInput.getKey();
          if (!key) continue;
          if (seenCallInputs.has(key))
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Duplicate call input key '${key}' in call '${callName ?? '<unnamed>'}'`,
            );
          else seenCallInputs.add(key);
        }
        for (const dep of element.afterDependencies()) {
          if (!seenCallNames.has(dep))
            this.addError(
              WdlSemanticErrorCode.UNKNOWN_REFERENCE,
              `Call '${callName ?? '<unnamed>'}' has unknown or forward after dependency '${dep}'`,
            );
        }
      }
    }
    this.validateNestedWorkflowStructure(node);
    super.processWorkflow(ctx, node);
  }

  public override processImportStandard(ctx: WdlDocument, node: WdlImportStandard): void {
    this.validateImportBasics(ctx, node);

    const importedDocument = this.resolveImportedDocument(ctx, node);
    if (!importedDocument) return;

    const localVersion = ctx.getWdlVersion();
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

    for (const member of node.members()) {
      const memberName = member.getMember();
      if (!memberName) continue;
      if (!exported.has(memberName)) {
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Import alias refers to unknown member '${memberName}'`,
        );
      }
    }
  }

  public override processImportMembers(ctx: WdlDocument, node: WdlImportMembers): void {
    this.validateImportBasics(ctx, node);

    const importedDocument = this.resolveImportedDocument(ctx, node);
    if (!importedDocument) return;

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

    for (const member of node.members()) {
      const memberName = member.getMember();
      if (!memberName) continue;
      if (!exported.has(memberName)) {
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Import member '${memberName}' does not exist`,
        );
      }
    }
  }

  public override processImportStar(ctx: WdlDocument, node: WdlImportStar): void {
    this.validateImportBasics(ctx, node);
  }

  private validateImportBasics(ctx: WdlDocument, node: WdlImport): void {
    const importedDocument = this.resolveImportedDocument(ctx, node);
    if (!importedDocument) {
      const source = this.importSourceText(node) || '<unknown>';
      this.addError(
        WdlSemanticErrorCode.UNKNOWN_REFERENCE,
        `Unable to resolve import source '${source}'`,
      );
    }
  }

  private indexImportedTopLevelNames(document: WdlDocument, topLevelNames: Set<string>): void {
    const seenNamespaces = new Set<string>();
    const localTopLevelNames = new Set<string>();
    for (const element of document.elements()) {
      if (
        element instanceof WdlTask ||
        element instanceof WdlWorkflow ||
        element instanceof WdlStruct ||
        element instanceof WdlEnum
      ) {
        const name = element.getName();
        if (name) localTopLevelNames.add(name);
      }
    }

    for (const imp of document.importStatements()) {
      const importedDocument = this.resolveImportedDocument(document, imp);
      if (!importedDocument) continue;

      if (imp instanceof WdlImportStandard) {
        const namespace = this.importNamespace(imp);
        if (seenNamespaces.has(namespace)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate import namespace '${namespace}'`,
          );
        }
        seenNamespaces.add(namespace);
      }

      const callableNames = this.visibleImportedCallableNames(imp, importedDocument);
      for (const [visibleName] of callableNames.entries()) {
        this.knownCallableTargets.add(visibleName);
        if (!visibleName.includes('.')) {
          if (localTopLevelNames.has(visibleName)) {
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Imported callable '${visibleName}' conflicts with local definition`,
            );
          }
          const key = `callable:${visibleName}`;
          if (topLevelNames.has(key)) {
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Duplicate imported callable name '${visibleName}'`,
            );
          }
          topLevelNames.add(key);
        }
      }

      const structNames = this.visibleImportedStructNames(imp, importedDocument);
      for (const visibleName of structNames.keys()) {
        this.knownTypeNames.add(visibleName);
        if (localTopLevelNames.has(visibleName)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Imported type '${visibleName}' conflicts with local definition`,
          );
        }
      }

      const enumNames = this.visibleImportedEnums(imp, importedDocument);
      for (const visibleName of enumNames.keys()) {
        this.knownTypeNames.add(visibleName);
        if (localTopLevelNames.has(visibleName)) {
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Imported type '${visibleName}' conflicts with local definition`,
          );
        }
      }
    }
  }

  public override processTask(ctx: WdlDocument, node: WdlTask): void {
    const taskDeclarations = new Set<string>();
    for (const element of node.elements()) {
      if (element instanceof WdlInput) {
        for (const declaration of element.elements()) {
          this.validateKnownTypeReference(
            declaration.getType(),
            `task '${node.getName() ?? ''}' input '${declaration.getName() ?? ''}'`,
          );
          const name = declaration.getName();
          if (name && taskDeclarations.has(name))
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Duplicate task declaration in '${node.getName() ?? ''}': '${name}'`,
            );
          else if (name) taskDeclarations.add(name);
        }
      } else if (element instanceof WdlBoundDeclaration) {
        const name = element.getName();
        this.validateKnownTypeReference(
          element.getType(),
          `task '${node.getName() ?? ''}' declaration '${name ?? ''}'`,
        );
        if (name && taskDeclarations.has(name))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate task declaration in '${node.getName() ?? ''}': '${name}'`,
          );
        else if (name) taskDeclarations.add(name);
      } else if (element instanceof WdlOutput) {
        const outputNames = new Set<string>();
        for (const declaration of element.elements()) {
          this.validateKnownTypeReference(
            declaration.getType(),
            `task '${node.getName() ?? ''}' output '${declaration.getName() ?? ''}'`,
          );
          const name = declaration.getName();
          if (name && outputNames.has(name))
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Duplicate task output in '${node.getName() ?? ''}': '${name}'`,
            );
          else if (name) outputNames.add(name);
        }
      }
    }
    super.processTask(ctx, node);
  }

  protected override validateExpression(expr: WdlExpression | undefined): void {
    super.validateExpression(expr);
    if (!expr) return;
    if (expr instanceof WdlBinaryOperation) {
      const left = this.inferType(expr.getLeft());
      const right = this.inferType(expr.getRight());
      const op = expr.getOperator();
      if (
        (op === WdlBinaryOperator.OR || op === WdlBinaryOperator.AND) &&
        ((left && !this.isPrimitive(left, WdlPrimitiveType.Type.BOOLEAN)) ||
          (right && !this.isPrimitive(right, WdlPrimitiveType.Type.BOOLEAN)))
      )
        this.addError(
          WdlSemanticErrorCode.TYPE_MISMATCH,
          'Logical operators require Boolean operands',
        );
      if (
        [
          WdlBinaryOperator.MULTIPLY,
          WdlBinaryOperator.DIVIDE,
          WdlBinaryOperator.MODULO,
          WdlBinaryOperator.POWER,
          WdlBinaryOperator.SUBTRACT,
        ].includes(op!)
      ) {
        if ((left && !this.isNumeric(left)) || (right && !this.isNumeric(right)))
          this.addError(
            WdlSemanticErrorCode.TYPE_MISMATCH,
            'Numeric operator requires Int or Float operands',
          );
      }
      if (
        op === WdlBinaryOperator.ADD &&
        left &&
        right &&
        !(
          (this.isNumeric(left) && this.isNumeric(right)) ||
          this.isPrimitive(left, WdlPrimitiveType.Type.STRING) ||
          this.isPrimitive(right, WdlPrimitiveType.Type.STRING)
        )
      )
        this.addError(
          WdlSemanticErrorCode.TYPE_MISMATCH,
          "'+' requires numeric operands or string concatenation",
        );
      if (
        (op === WdlBinaryOperator.EQ || op === WdlBinaryOperator.NEQ) &&
        left &&
        right &&
        !this.isTypeAssignable(left, right) &&
        !this.isTypeAssignable(right, left)
      )
        this.addError(
          WdlSemanticErrorCode.TYPE_MISMATCH,
          'Equality comparison operands are incompatible',
        );
      if (
        [
          WdlBinaryOperator.LT,
          WdlBinaryOperator.LTE,
          WdlBinaryOperator.GT,
          WdlBinaryOperator.GTE,
        ].includes(op!) &&
        left &&
        right &&
        !this.areOrderComparable(left, right)
      )
        this.addError(
          WdlSemanticErrorCode.TYPE_MISMATCH,
          'Ordering comparison operands are incompatible',
        );
    }
    if (expr instanceof WdlUnaryOperation) {
      const operand = this.inferType(expr.getOperand());
      if (!operand) return;
      if (
        expr.getOperator() === WdlUnaryOperator.NOT &&
        !this.isPrimitive(operand, WdlPrimitiveType.Type.BOOLEAN)
      )
        this.addError(WdlSemanticErrorCode.TYPE_MISMATCH, "'!' requires a Boolean operand");
      if (expr.getOperator() === WdlUnaryOperator.NEGATIVE && !this.isNumeric(operand))
        this.addError(
          WdlSemanticErrorCode.TYPE_MISMATCH,
          "Unary '-' requires an Int or Float operand",
        );
    }
    if (expr instanceof WdlTernaryOperation) {
      const condition = this.inferType(expr.getCondition());
      if (condition && !this.isPrimitive(condition, WdlPrimitiveType.Type.BOOLEAN))
        this.addError(WdlSemanticErrorCode.TYPE_MISMATCH, 'Ternary condition must be Boolean');
      const trueType = this.inferType(expr.getTrueValue());
      const falseType = this.inferType(expr.getFalseValue());
      if (
        trueType &&
        falseType &&
        !this.isTypeAssignable(trueType, falseType) &&
        !this.isTypeAssignable(falseType, trueType)
      )
        this.addError(
          WdlSemanticErrorCode.TYPE_MISMATCH,
          'Ternary branches have incompatible types',
        );
    }
    if (expr instanceof WdlFunctionCallOperation) this.validateGenericFunctionCall(expr);
  }

  protected override validateFunctionCall(functionCall: WdlFunctionCallOperation): void {
    super.validateFunctionCall(functionCall);
    const fn = functionCall.getFunction();
    switch (fn) {
      case WdlFunction.KEYS:
        this.processKeys(functionCall);
        break;
      case WdlFunction.VALUES:
        this.processValues(functionCall);
        break;
      case WdlFunction.RANGE:
        this.processRange(functionCall);
        break;
      case WdlFunction.SELECT_ALL:
        this.processSelectAll(functionCall);
        break;
      case WdlFunction.READ_INT:
        this.validateSinglePathLikeArg('read_int', functionCall);
        break;
      case WdlFunction.READ_FLOAT:
        this.validateSinglePathLikeArg('read_float', functionCall);
        break;
      case WdlFunction.READ_STRING:
        this.validateSinglePathLikeArg('read_string', functionCall);
        break;
      case WdlFunction.READ_BOOLEAN:
        this.validateSinglePathLikeArg('read_boolean', functionCall);
        break;
      case WdlFunction.READ_LINES:
        this.validateSinglePathLikeArg('read_lines', functionCall);
        break;
      case WdlFunction.READ_TSV:
        this.validateSinglePathLikeArg('read_tsv', functionCall);
        break;
      case WdlFunction.READ_MAP:
        this.validateSinglePathLikeArg('read_map', functionCall);
        break;
      case WdlFunction.READ_OBJECT:
        this.validateSinglePathLikeArg('read_object', functionCall);
        break;
      case WdlFunction.READ_OBJECTS:
        this.validateSinglePathLikeArg('read_objects', functionCall);
        break;
      case WdlFunction.READ_JSON:
        this.validateSinglePathLikeArg('read_json', functionCall);
        break;
      case WdlFunction.GLOB:
        this.validateSinglePathLikeArg('glob', functionCall);
        break;
      default:
        break;
    }
  }

  protected processKeys(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1)
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'keys expects exactly 1 argument',
      );
    else {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (argType && !(argType instanceof WdlMapType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'keys expects a map argument',
        );
    }
  }
  protected processValues(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1)
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'values expects exactly 1 argument',
      );
    else {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (argType && !(argType instanceof WdlMapType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'values expects a map argument',
        );
    }
  }
  protected processRange(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1)
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'range expects exactly 1 argument',
      );
    else {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (
        argType &&
        !(
          argType instanceof WdlPrimitiveType &&
          argType.primitiveType() === WdlPrimitiveType.Type.INT
        )
      )
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'range expects an Int argument',
        );
    }
  }
  protected processSelectAll(functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1)
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        'select_all expects exactly 1 argument',
      );
    else {
      const argType = this.inferType(functionCall.arguments()[0]);
      if (argType && !(argType instanceof WdlArrayType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'select_all expects an array argument',
        );
    }
  }
  protected validateSinglePathLikeArg(name: string, functionCall: WdlFunctionCallOperation): void {
    if (functionCall.arguments().length !== 1) {
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        `${name} expects exactly 1 argument`,
      );
      return;
    }
    const argType = this.inferType(functionCall.arguments()[0]);
    if (argType && !this.isPathLikeType(argType))
      this.addError(
        WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
        `${name} expects a String/File/Directory argument`,
      );
  }
  protected isPathLikeType(type: WdlType): boolean {
    return (
      type instanceof WdlPrimitiveType &&
      [
        WdlPrimitiveType.Type.STRING,
        WdlPrimitiveType.Type.FILE,
        WdlPrimitiveType.Type.DIRECTORY,
      ].includes(type.primitiveType())
    );
  }
  protected isNumeric(type: WdlType | undefined): boolean {
    return (
      this.isPrimitive(type, WdlPrimitiveType.Type.INT) ||
      this.isPrimitive(type, WdlPrimitiveType.Type.FLOAT)
    );
  }
  protected areOrderComparable(left: WdlType, right: WdlType): boolean {
    return (
      (this.isNumeric(left) && this.isNumeric(right)) ||
      (this.isPrimitive(left, WdlPrimitiveType.Type.STRING) &&
        this.isPrimitive(right, WdlPrimitiveType.Type.STRING))
    );
  }

  protected validateGenericFunctionCall(functionCall: WdlFunctionCallOperation): void {
    const fn = functionCall.getFunction();
    if (fn === WdlFunction.NONSTANDARD) return;
    const argc = functionCall.arguments().length;
    const limits = this.functionArity.get(fn);
    if (limits) {
      const [minArity, maxArity] = limits;
      if (argc < minArity)
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          `${fn.toWdlString()} expects at least ${minArity} argument(s)`,
        );
      else if (maxArity !== undefined && argc > maxArity) {
        if (minArity === maxArity)
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            `${fn.toWdlString()} expects exactly ${minArity} argument(s)`,
          );
        else
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            `${fn.toWdlString()} expects between ${minArity} and ${maxArity} arguments`,
          );
      }
    }
    const signatures = this.functionSignatures.get(fn) ?? [];
    const sameAritySignatures = signatures.filter((sig) => sig.length === argc);
    if (sameAritySignatures.length > 0) {
      const anyCompatible = sameAritySignatures.some((sig) =>
        functionCall.arguments().every((arg, index) => {
          const argType = this.inferType(arg);
          return !argType || this.matchesSignatureType(argType, sig[index]!);
        }),
      );
      if (!anyCompatible)
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          `Argument types are incompatible for function '${fn.toWdlString()}'`,
        );
    }
    if ([WdlFunction.MIN, WdlFunction.MAX].includes(fn) && argc >= 2) {
      const left = this.inferType(functionCall.arguments()[0]);
      const right = this.inferType(functionCall.arguments()[1]);
      if ((left && !this.isNumeric(left)) || (right && !this.isNumeric(right)))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          `${fn.toWdlString()} expects numeric arguments`,
        );
    }
    if (fn === WdlFunction.SEP && argc === 2) {
      const delim = this.inferType(functionCall.arguments()[0]);
      const arr = this.inferType(functionCall.arguments()[1]);
      if (delim && !this.isPrimitive(delim, WdlPrimitiveType.Type.STRING))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'sep delimiter must be String',
        );
      if (arr && !(arr instanceof WdlArrayType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'sep second argument must be an Array',
        );
    }
    if ([WdlFunction.PREFIX, WdlFunction.SUFFIX].includes(fn) && argc === 2) {
      const s = this.inferType(functionCall.arguments()[0]);
      const arr = this.inferType(functionCall.arguments()[1]);
      if (s && !this.isPrimitive(s, WdlPrimitiveType.Type.STRING))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          `${fn.toWdlString()} first argument must be String`,
        );
      if (arr && !(arr instanceof WdlArrayType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          `${fn.toWdlString()} second argument must be Array`,
        );
    }
    if ([WdlFunction.QUOTE, WdlFunction.SQUOTE].includes(fn) && argc === 1) {
      const arr = this.inferType(functionCall.arguments()[0]);
      if (arr && !(arr instanceof WdlArrayType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          `${fn.toWdlString()} expects an Array argument`,
        );
    }
    if (fn === WdlFunction.TRANSPOSE && argc === 1) {
      const arr = this.inferType(functionCall.arguments()[0]);
      if (arr && (!(arr instanceof WdlArrayType) || !(arr.memberType() instanceof WdlArrayType)))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'transpose expects Array[Array[X]]',
        );
    }
    if (fn === WdlFunction.FLATTEN && argc === 1) {
      const arr = this.inferType(functionCall.arguments()[0]);
      if (arr && (!(arr instanceof WdlArrayType) || !(arr.memberType() instanceof WdlArrayType)))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'flatten expects Array[Array[X]]',
        );
    }
    if (fn === WdlFunction.CHUNK && argc === 2) {
      const arr = this.inferType(functionCall.arguments()[0]);
      const count = this.inferType(functionCall.arguments()[1]);
      if (arr && !(arr instanceof WdlArrayType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'chunk first argument must be Array',
        );
      if (count && !this.isPrimitive(count, WdlPrimitiveType.Type.INT))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'chunk second argument must be Int',
        );
    }
    if (fn === WdlFunction.CROSS && argc === 2) {
      const left = this.inferType(functionCall.arguments()[0]);
      const right = this.inferType(functionCall.arguments()[1]);
      if ((left && !(left instanceof WdlArrayType)) || (right && !(right instanceof WdlArrayType)))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'cross expects two array arguments',
        );
    }
    if (fn === WdlFunction.UNZIP && argc === 1) {
      const arr = this.inferType(functionCall.arguments()[0]);
      if (arr && (!(arr instanceof WdlArrayType) || !(arr.memberType() instanceof WdlPairType)))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'unzip expects Array[Pair[X,Y]]',
        );
    }
    if (fn === WdlFunction.AS_PAIRS && argc === 1) {
      const map = this.inferType(functionCall.arguments()[0]);
      if (map && !(map instanceof WdlMapType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'as_pairs expects a Map argument',
        );
    }
    if (fn === WdlFunction.COLLECT_BY_KEY && argc === 1) {
      const arr = this.inferType(functionCall.arguments()[0]);
      if (arr && (!(arr instanceof WdlArrayType) || !(arr.memberType() instanceof WdlPairType)))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'collect_by_key expects Array[Pair[K,V]]',
        );
    }
    if (fn === WdlFunction.MATCHES && argc === 2) {
      const left = this.inferType(functionCall.arguments()[0]);
      const right = this.inferType(functionCall.arguments()[1]);
      if (left && !this.isPrimitive(left, WdlPrimitiveType.Type.STRING))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'matches first argument must be String',
        );
      if (right && !this.isPrimitive(right, WdlPrimitiveType.Type.STRING))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'matches second argument must be String',
        );
    }
    if (fn === WdlFunction.FIND && argc === 2) {
      const left = this.inferType(functionCall.arguments()[0]);
      const right = this.inferType(functionCall.arguments()[1]);
      if (left && !this.isPrimitive(left, WdlPrimitiveType.Type.STRING))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'find first argument must be String',
        );
      if (right && !this.isPrimitive(right, WdlPrimitiveType.Type.STRING))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'find second argument must be String',
        );
    }
    if (fn === WdlFunction.CONTAINS && argc === 2) {
      const left = this.inferType(functionCall.arguments()[0]);
      const right = this.inferType(functionCall.arguments()[1]);
      if (left instanceof WdlArrayType && right) {
        const member = left.memberType();
        if (member && !this.isTypeAssignable(member, right))
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            'contains argument type is incompatible with array member type',
          );
      } else if (left) {
        if (!this.isPrimitive(left, WdlPrimitiveType.Type.STRING))
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            'contains first argument must be Array or String',
          );
        else if (right && !this.isPrimitive(right, WdlPrimitiveType.Type.STRING))
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            'contains second argument must be String when first argument is String',
          );
      }
    }
    if (fn === WdlFunction.CONTAINS_KEY && argc === 2) {
      const map = this.inferType(functionCall.arguments()[0]);
      const key = this.inferType(functionCall.arguments()[1]);
      if (map && !(map instanceof WdlMapType))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'contains_key first argument must be a Map',
        );
      else if (map instanceof WdlMapType && key) {
        const expected = map.keyType();
        if (expected && !this.isTypeAssignable(expected, key))
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            'contains_key key type is incompatible with map key type',
          );
      }
    }
    if (fn === WdlFunction.SIZE && argc >= 1) {
      const type = this.inferType(functionCall.arguments()[0]);
      if (type && !this.isPathLikeType(type))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'size first argument must be String/File/Directory',
        );
      if (argc >= 2) {
        const unit = this.inferType(functionCall.arguments()[1]);
        if (unit && !this.isPrimitive(unit, WdlPrimitiveType.Type.STRING))
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            'size second argument must be String',
          );
      }
    }
    if (fn === WdlFunction.BASENAME && argc >= 1) {
      const base = this.inferType(functionCall.arguments()[0]);
      if (base && !this.isPathLikeType(base))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'basename first argument must be String/File/Directory',
        );
      if (argc >= 2) {
        const suffix = this.inferType(functionCall.arguments()[1]);
        if (suffix && !this.isPrimitive(suffix, WdlPrimitiveType.Type.STRING))
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            'basename second argument must be String',
          );
      }
    }
    if (fn === WdlFunction.JOIN_PATHS && argc >= 2) {
      const first = this.inferType(functionCall.arguments()[0]);
      if (first && !this.isPathLikeType(first))
        this.addError(
          WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
          'join_paths first argument must be String/File/Directory',
        );
      for (const arg of functionCall.arguments().slice(1)) {
        const argType = this.inferType(arg);
        if (argType && !this.isPrimitive(argType, WdlPrimitiveType.Type.STRING)) {
          this.addError(
            WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            'join_paths arguments after the first must be String',
          );
          break;
        }
      }
    }
  }

  protected matchesSignatureType(actual: WdlType, sig: string): boolean {
    if (sig === 'ANY' || sig === 'ANY_OPTIONAL') return true;
    if (sig === 'NUMBER') return this.isNumeric(actual);
    if (sig === 'BOOLEAN') return this.isPrimitive(actual, WdlPrimitiveType.Type.BOOLEAN);
    if (sig === 'INT') return this.isPrimitive(actual, WdlPrimitiveType.Type.INT);
    if (sig === 'FLOAT') return this.isPrimitive(actual, WdlPrimitiveType.Type.FLOAT);
    if (sig === 'STRING' || sig === 'STRING_OPTIONAL')
      return this.isPrimitive(actual, WdlPrimitiveType.Type.STRING);
    if (sig === 'FILE') return this.isPrimitive(actual, WdlPrimitiveType.Type.FILE);
    if (sig === 'DIRECTORY') return this.isPrimitive(actual, WdlPrimitiveType.Type.DIRECTORY);
    if (sig === 'FILE_OR_DIRECTORY')
      return (
        this.isPrimitive(actual, WdlPrimitiveType.Type.FILE) ||
        this.isPrimitive(actual, WdlPrimitiveType.Type.DIRECTORY) ||
        this.isPrimitive(actual, WdlPrimitiveType.Type.STRING)
      );
    if (sig === 'OBJECT') return this.isPrimitive(actual, WdlPrimitiveType.Type.OBJECT);
    if (
      [
        'ARRAY_ANY',
        'ARRAY_FILE',
        'ARRAY_OPTIONAL_ANY',
        'ARRAY_INT',
        'ARRAY_STRING',
        'ARRAY_OBJECT',
        'ARRAY_PAIR',
        'ARRAY_ARRAY_ANY',
        'ARRAY_ARRAY_STRING',
      ].includes(sig)
    )
      return this.matchesArraySignature(actual, sig);
    if (['MAP_ANY_ANY', 'MAP_ANY_ARRAY', 'MAP_STRING_STRING'].includes(sig))
      return this.matchesMapSignature(actual, sig);
    if (sig === 'PAIR_ARRAY')
      return (
        actual instanceof WdlPairType &&
        actual.leftType() instanceof WdlArrayType &&
        actual.rightType() instanceof WdlArrayType
      );
    return true;
  }

  protected matchesArraySignature(actual: WdlType, sig: string): boolean {
    if (!(actual instanceof WdlArrayType)) return false;
    const member = actual.memberType();
    if (sig === 'ARRAY_ANY' || sig === 'ARRAY_OPTIONAL_ANY') return true;
    if (sig === 'ARRAY_FILE')
      return !!member && this.isPrimitive(member, WdlPrimitiveType.Type.FILE);
    if (sig === 'ARRAY_INT') return !!member && this.isPrimitive(member, WdlPrimitiveType.Type.INT);
    if (sig === 'ARRAY_STRING')
      return !!member && this.isPrimitive(member, WdlPrimitiveType.Type.STRING);
    if (sig === 'ARRAY_OBJECT')
      return !!member && this.isPrimitive(member, WdlPrimitiveType.Type.OBJECT);
    if (sig === 'ARRAY_PAIR') return member instanceof WdlPairType;
    if (sig === 'ARRAY_ARRAY_ANY') return member instanceof WdlArrayType;
    if (sig === 'ARRAY_ARRAY_STRING')
      return (
        member instanceof WdlArrayType &&
        !!member.memberType() &&
        this.isPrimitive(member.memberType(), WdlPrimitiveType.Type.STRING)
      );
    return true;
  }

  protected matchesMapSignature(actual: WdlType, sig: string): boolean {
    if (!(actual instanceof WdlMapType)) return false;
    if (sig === 'MAP_ANY_ANY') return true;
    if (sig === 'MAP_ANY_ARRAY') return actual.valueType() instanceof WdlArrayType;
    if (sig === 'MAP_STRING_STRING')
      return (
        this.isPrimitive(actual.keyType(), WdlPrimitiveType.Type.STRING) &&
        this.isPrimitive(actual.valueType(), WdlPrimitiveType.Type.STRING)
      );
    return true;
  }

  protected validateKnownTypeReference(type: WdlType | undefined, location: string): void {
    if (!type) return;
    if (type instanceof WdlTypeReferenceType) {
      const ref = type.referenceName();
      if (ref && !this.knownTypeNames.has(ref))
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Unknown type reference '${ref}' in ${location}`,
        );
      return;
    }
    if (type instanceof WdlArrayType) {
      this.validateKnownTypeReference(type.memberType(), location);
      return;
    }
    if (type instanceof WdlMapType) {
      this.validateKnownTypeReference(type.keyType(), location);
      this.validateKnownTypeReference(type.valueType(), location);
      return;
    }
    if (type instanceof WdlPairType) {
      this.validateKnownTypeReference(type.leftType(), location);
      this.validateKnownTypeReference(type.rightType(), location);
    }
  }

  protected validateNestedWorkflowStructure(workflow: WdlWorkflow): void {
    const availableCalls = new Set<string>();
    const namesInBlock = new Set<string>();
    for (const element of workflow.elements()) {
      if (element instanceof WdlInput) {
        for (const declaration of element.elements()) {
          const name = declaration.getName();
          if (name && namesInBlock.has(name))
            this.addError(
              WdlSemanticErrorCode.DUPLICATE_DEFINITION,
              `Duplicate workflow declaration: '${name}'`,
            );
          else if (name) namesInBlock.add(name);
        }
      } else if (element instanceof WdlBoundDeclaration) {
        const name = element.getName();
        if (name && namesInBlock.has(name))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate workflow declaration: '${name}'`,
          );
        else if (name) namesInBlock.add(name);
      } else if (element instanceof WdlCall)
        this.validateCallStructure(element, namesInBlock, availableCalls);
      else if (element instanceof WdlScatter) {
        const varName = element.getName();
        if (varName && namesInBlock.has(varName))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate workflow declaration: '${varName}'`,
          );
        else if (varName) namesInBlock.add(varName);
        this.validateNestedStatements(element.statements(), availableCalls, 'scatter');
      } else if (element instanceof WdlConditional)
        this.validateConditionalStructure(element, availableCalls, 'conditional');
    }
  }

  protected validateNestedStatements(
    statements: readonly WdlStatement[],
    inheritedCalls: Set<string>,
    contextLabel: string,
  ): void {
    const namesInBlock = new Set<string>();
    const availableCalls = new Set(inheritedCalls);
    for (const statement of statements) {
      if (statement instanceof WdlBoundDeclaration) {
        const name = statement.getName();
        if (name && namesInBlock.has(name))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate declaration in ${contextLabel}: '${name}'`,
          );
        else if (name) namesInBlock.add(name);
      } else if (statement instanceof WdlCall)
        this.validateCallStructure(statement, namesInBlock, availableCalls);
      else if (statement instanceof WdlScatter) {
        const varName = statement.getName();
        if (varName && namesInBlock.has(varName))
          this.addError(
            WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            `Duplicate declaration in ${contextLabel}: '${varName}'`,
          );
        else if (varName) namesInBlock.add(varName);
        this.validateNestedStatements(statement.statements(), availableCalls, 'scatter');
      } else if (statement instanceof WdlConditional)
        this.validateConditionalStructure(statement, availableCalls, 'conditional');
    }
  }

  protected validateConditionalStructure(
    conditional: WdlConditional,
    availableCalls: Set<string>,
    contextLabel: string,
  ): void {
    this.validateNestedStatements(
      conditional.thenStatements(),
      availableCalls,
      `${contextLabel} then`,
    );
    for (const elseIf of conditional.elseIfs())
      this.validateNestedStatements(
        elseIf.thenStatements(),
        availableCalls,
        `${contextLabel} else-if`,
      );
    this.validateNestedStatements(
      conditional.elseStatements(),
      availableCalls,
      `${contextLabel} else`,
    );
  }

  protected validateCallStructure(
    call: WdlCall,
    namesInBlock: Set<string>,
    availableCalls: Set<string>,
  ): void {
    const target = call.targetPath().at(-1);
    const callName = call.getAlias() ?? target;
    if (callName && namesInBlock.has(callName))
      this.addError(
        WdlSemanticErrorCode.DUPLICATE_DEFINITION,
        `Duplicate call name in workflow: '${callName}'`,
      );
    else if (callName) namesInBlock.add(callName);
    const seenCallInputs = new Set<string>();
    for (const callInput of call.inputs()) {
      const key = callInput.getKey();
      if (!key) continue;
      if (seenCallInputs.has(key))
        this.addError(
          WdlSemanticErrorCode.DUPLICATE_DEFINITION,
          `Duplicate call input key '${key}' in call '${callName ?? '<unnamed>'}'`,
        );
      else seenCallInputs.add(key);
    }
    for (const dep of call.afterDependencies()) {
      if (!availableCalls.has(dep))
        this.addError(
          WdlSemanticErrorCode.UNKNOWN_REFERENCE,
          `Call '${callName ?? '<unnamed>'}' has unknown or forward after dependency '${dep}'`,
        );
    }
    if (callName) availableCalls.add(callName);
  }
}
