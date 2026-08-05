/** Default source-order traversal and WDL rendering helpers for the TypeScript model. */
import { WdlDocument } from '../wdl-document.js';
import { WdlVersion } from '../wdl-version.js';
import type { WdlNode } from '../base/wdl-node.js';
import { WdlEnum } from '../definitions/wdl-enum.js';
import { WdlStruct, WdlStructMember } from '../definitions/wdl-struct.js';
import { WdlTask } from '../definitions/wdl-task.js';
import { WdlWorkflow } from '../definitions/wdl-workflow.js';
import {
  Delimiter,
  WdlArrayLiteral,
  WdlBinaryOperation,
  WdlBooleanLiteral,
  WdlExpression,
  WdlFloatLiteral,
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
  WdlVariable,
  WdlExpressionComponentType,
} from '../expressions/index.js';
import { WdlCommand } from '../sections/wdl-command.js';
import { WdlTaskHints, WdlWorkflowHints } from '../sections/wdl-hints.js';
import { WdlInput } from '../sections/wdl-input.js';
import { WdlMetadata, WdlParameterMetadata } from '../sections/wdl-metadata-base.js';
import { WdlOutput } from '../sections/wdl-output.js';
import { WdlRequirements } from '../sections/wdl-requirements.js';
import { WdlRuntime } from '../sections/wdl-runtime.js';
import { WdlCall } from '../statements/wdl-call.js';
import { WdlConditional } from '../statements/wdl-conditional.js';
import { WdlBoundDeclaration, WdlDeclaration } from '../statements/wdl-declaration.js';
import {
  WdlImport,
  WdlImportMember,
  WdlImportMembers,
  WdlImportStandard,
  WdlImportStar,
} from '../statements/wdl-import.js';
import { WdlScatter } from '../statements/wdl-scatter.js';
import {
  WdlArrayType,
  WdlMapType,
  WdlPairType,
  WdlPrimitiveType,
  WdlType,
  WdlTypeReferenceType,
} from '../types/index.js';
import type { WdlProcessor } from './wdl-processor.js';

export type ResolvedImport<TNode extends WdlNode> = {
  localName: string;
  importedName: string;
  importNamespace: string | undefined;
  importStatement: WdlImport;
  importedDocument: WdlDocument;
  symbol: TNode;
};

export class WdlProcessorBase implements WdlProcessor {
  /** Walks the document root and dispatches each top-level element in source order. */
  public processDocument(node: WdlDocument): void {
    const version = node.getWdlVersion();
    if (version) {
      this.processVersion(node, version);
    }
    for (const element of node.elements()) {
      if (element instanceof WdlImportStandard) this.processImportStandard(node, element);
      else if (element instanceof WdlImportMembers) this.processImportMembers(node, element);
      else if (element instanceof WdlImportStar) this.processImportStar(node, element);
      else if (element instanceof WdlEnum) this.processEnum(node, element);
      else if (element instanceof WdlStruct) this.processStruct(node, element);
      else if (element instanceof WdlTask) this.processTask(node, element);
      else if (element instanceof WdlWorkflow) this.processWorkflow(node, element);
      else this.processUnexpectedNode(node, element);
    }
  }
  /** Default no-op version hook. */
  public processVersion(_ctx: WdlDocument, _node: WdlVersion): void {}
  /** Default no-op standard import hook. */
  public processImportStandard(_ctx: WdlDocument, _node: WdlImportStandard): void {}
  /** Default no-op member import hook. */
  public processImportMembers(_ctx: WdlDocument, _node: WdlImportMembers): void {}
  /** Default no-op star import hook. */
  public processImportStar(_ctx: WdlDocument, _node: WdlImportStar): void {}
  /** Default no-op enum hook. */
  public processEnum(_ctx: WdlDocument, _node: WdlEnum): void {}
  /** Traverses struct members and metadata in source order. */
  public processStruct(_ctx: WdlDocument, node: WdlStruct): void {
    for (const e of node.elements()) {
      if (e instanceof WdlStructMember) this.processStructMember(node, e);
      else if (e instanceof WdlMetadata) this.processStructMetadata(node, e);
      else if (e instanceof WdlParameterMetadata) this.processStructParameterMetadata(node, e);
      else this.processUnexpectedNode(node, e);
    }
  }
  /** Default no-op struct member hook. */
  public processStructMember(_ctx: WdlStruct, _node: WdlStructMember): void {}
  /** Default no-op struct parameter metadata hook. */
  public processStructParameterMetadata(_ctx: WdlStruct, _node: WdlParameterMetadata): void {}
  /** Default no-op struct metadata hook. */
  public processStructMetadata(_ctx: WdlStruct, _node: WdlMetadata): void {}
  /** Traverses task elements in source order. */
  public processTask(_ctx: WdlDocument, node: WdlTask): void {
    for (const e of node.elements()) {
      if (e instanceof WdlBoundDeclaration) this.processTaskDeclaration(node, e);
      else if (e instanceof WdlInput) this.processTaskInput(node, e);
      else if (e instanceof WdlOutput) this.processTaskOutput(node, e);
      else if (e instanceof WdlCommand) this.processTaskCommand(node, e);
      else if (e instanceof WdlMetadata) this.processTaskMetadata(node, e);
      else if (e instanceof WdlParameterMetadata) this.processTaskParameterMetadata(node, e);
      else if (e instanceof WdlRequirements) this.processTaskRequirements(node, e);
      else if (e instanceof WdlRuntime) this.processTaskRuntime(node, e);
      else if (e instanceof WdlTaskHints) this.processTaskHints(node, e);
      else this.processUnexpectedNode(node, e);
    }
  }
  public processTaskDeclaration(_ctx: WdlTask, _node: WdlBoundDeclaration): void {}
  public processTaskInput(_ctx: WdlTask, _node: WdlInput): void {}
  public processTaskOutput(_ctx: WdlTask, _node: WdlOutput): void {}
  public processTaskCommand(_ctx: WdlTask, _node: WdlCommand): void {}
  public processTaskParameterMetadata(_ctx: WdlTask, _node: WdlParameterMetadata): void {}
  public processTaskMetadata(_ctx: WdlTask, _node: WdlMetadata): void {}
  public processTaskRequirements(_ctx: WdlTask, _node: WdlRequirements): void {}
  public processTaskRuntime(_ctx: WdlTask, _node: WdlRuntime): void {}
  public processTaskHints(_ctx: WdlTask, _node: WdlTaskHints): void {}
  /** Traverses workflow elements in source order. */
  public processWorkflow(_ctx: WdlDocument, node: WdlWorkflow): void {
    for (const e of node.elements()) {
      if (e instanceof WdlBoundDeclaration) this.processWorkflowDeclaration(node, e);
      else if (e instanceof WdlCall) this.processWorkflowCall(node, e);
      else if (e instanceof WdlConditional) this.processWorkflowConditional(node, e);
      else if (e instanceof WdlInput) this.processWorkflowInput(node, e);
      else if (e instanceof WdlOutput) this.processWorkflowOutput(node, e);
      else if (e instanceof WdlMetadata) this.processWorkflowMetadata(node, e);
      else if (e instanceof WdlParameterMetadata) this.processWorkflowParameterMetadata(node, e);
      else if (e instanceof WdlScatter) this.processWorkflowScatter(node, e);
      else if (e instanceof WdlWorkflowHints) this.processWorkflowHints(node, e);
      else this.processUnexpectedNode(node, e);
    }
  }
  public processWorkflowDeclaration(_ctx: WdlWorkflow, _node: WdlBoundDeclaration): void {}
  public processWorkflowInput(_ctx: WdlWorkflow, _node: WdlInput): void {}
  public processWorkflowOutput(_ctx: WdlWorkflow, _node: WdlOutput): void {}
  public processWorkflowMetadata(_ctx: WdlWorkflow, _node: WdlMetadata): void {}
  public processWorkflowParameterMetadata(_ctx: WdlWorkflow, _node: WdlParameterMetadata): void {}
  public processWorkflowCall(_ctx: WdlWorkflow, _node: WdlCall): void {}
  public processWorkflowConditional(_ctx: WdlWorkflow, _node: WdlConditional): void {}
  public processWorkflowScatter(_ctx: WdlWorkflow, _node: WdlScatter): void {}
  public processWorkflowHints(_ctx: WdlWorkflow, _node: WdlWorkflowHints): void {}
  /** Throws when traversal encounters an unexpected child node for the current context. */
  public processUnexpectedNode(ctx: WdlNode, node: WdlNode): never {
    throw new Error(`Unexpected node ${node.constructor.name} under ${ctx.constructor.name}`);
  }
  /** Renders a keyed expression pair back into WDL text. */
  public keyValueToWdl(
    item: { getKey(): string | undefined; getValue(): WdlExpression | undefined },
    delimiter = ': ',
  ): string {
    return item.getValue() === undefined
      ? (item.getKey() ?? '')
      : `${item.getKey() ?? ''}${delimiter}${this.expressionToWdl(item.getValue())}`;
  }
  /** Renders a declaration node back into WDL text. */
  public declarationToWdl(declaration: WdlDeclaration): string {
    let out = `${this.typeToWdl(declaration.getType())} ${declaration.getName() ?? ''}`.trim();
    if (declaration.isEnvironmentVariable()) out = `env ${out}`;
    if (declaration instanceof WdlBoundDeclaration && declaration.getExpression())
      out = `${out} = ${this.expressionToWdl(declaration.getExpression())}`;
    return out;
  }
  /** Renders an expression subtree back into WDL text. */
  public expressionToWdl(expr: WdlExpression | undefined): string {
    if (!expr) return '';
    switch (expr.componentType()) {
      case WdlExpressionComponentType.NULL_LIT:
        return 'None';
      case WdlExpressionComponentType.BOOL_LIT:
      case WdlExpressionComponentType.INT_LIT:
      case WdlExpressionComponentType.FLOAT_LIT:
        return String((expr as WdlBooleanLiteral | WdlIntLiteral | WdlFloatLiteral).getValue());
      case WdlExpressionComponentType.VARIABLE:
        return (expr as WdlVariable).getName() ?? '';
      case WdlExpressionComponentType.ARRAY_LIT:
        return `[${(expr as WdlArrayLiteral)
          .entries()
          .map((entry) => this.expressionToWdl(entry))
          .join(', ')}]`;
      case WdlExpressionComponentType.MAP_LIT:
        return `{${(expr as WdlMapLiteral)
          .entries()
          .map(
            (entry) =>
              `${this.expressionToWdl(entry.getKey())}: ${this.expressionToWdl(entry.getValue())}`,
          )
          .join(', ')}}`;
      case WdlExpressionComponentType.OBJ_LIT:
        return `{${(expr as WdlObjectLiteral)
          .entries()
          .map((entry) => `${entry.getKey() ?? ''}: ${this.expressionToWdl(entry.getValue())}`)
          .join(', ')}}`;
      case WdlExpressionComponentType.PAIR_LIT:
        return `(${this.expressionToWdl((expr as WdlPairLiteral).getLeft())}, ${this.expressionToWdl((expr as WdlPairLiteral).getRight())})`;
      case WdlExpressionComponentType.STR_LIT:
        return this.stringLiteralToWdl(expr as WdlStringLiteral, true);
      case WdlExpressionComponentType.STRUCT_LIT: {
        const value = expr as WdlStructLiteral;
        return `${value.getName() ?? ''} {${value
          .entries()
          .map((entry) => `${entry.getKey() ?? ''}: ${this.expressionToWdl(entry.getValue())}`)
          .join(', ')}}`;
      }
      case WdlExpressionComponentType.BINARY_OP: {
        const value = expr as WdlBinaryOperation;
        return `${this.expressionToWdl(value.getLeft())} ${value.getOperator() ?? ''} ${this.expressionToWdl(value.getRight())}`;
      }
      case WdlExpressionComponentType.UNARY_OP: {
        const value = expr as WdlUnaryOperation;
        return `${value.getOperator() ?? ''}${this.expressionToWdl(value.getOperand())}`;
      }
      case WdlExpressionComponentType.TERNARY_OP: {
        const value = expr as WdlTernaryOperation;
        return `if (${this.expressionToWdl(value.getCondition())}) ${this.expressionToWdl(value.getTrueValue())} else ${this.expressionToWdl(value.getFalseValue())}`;
      }
      case WdlExpressionComponentType.MEMBER_OP: {
        const value = expr as WdlMemberAccessOperation;
        return `${this.expressionToWdl(value.getTarget())}.${value.getMember() ?? ''}`;
      }
      case WdlExpressionComponentType.IDX_OP: {
        const value = expr as WdlIndexAccessOperation;
        return `${this.expressionToWdl(value.getTarget())}[${this.expressionToWdl(value.getIndex())}]`;
      }
      case WdlExpressionComponentType.FUNC_OP: {
        const value = expr as WdlFunctionCallOperation;
        return `${value.getFunctionName() ?? ''}(${value
          .arguments()
          .map((arg) => this.expressionToWdl(arg))
          .join(', ')})`;
      }
      default:
        return '';
    }
  }
  /** Renders a string literal back into WDL text. */
  public stringLiteralToWdl(
    literal: WdlStringLiteral | undefined,
    includeDelimiter: boolean,
  ): string {
    if (!literal) return '';
    const body = literal
      .components()
      .map((component) => {
        if (component instanceof WdlStringText) return component.text ?? '';
        if (component instanceof WdlStringEscape) return component.escapeText ?? '';
        if (component instanceof WdlStringToken) return component.tokenText ?? '';
        if (component instanceof WdlStringPlaceholder)
          return `${component.symbol}{${this.expressionToWdl(component.expression)}}`;
        return '';
      })
      .join('');
    if (!includeDelimiter) return body;
    return literal.delimiter === Delimiter.DOUBLE_ANGLE ? `<<<${body}>>>` : `"${body}"`;
  }
  /** Renders a type node back into WDL text. */
  public typeToWdl(type: WdlType | undefined): string {
    if (!type) return '';
    if (type instanceof WdlPrimitiveType)
      return `${type.primitiveType()}${type.isOptional() ? '?' : ''}`;
    if (type instanceof WdlTypeReferenceType)
      return `${type.referenceName()}${type.isOptional() ? '?' : ''}`;
    if (type instanceof WdlArrayType)
      return `Array[${this.typeToWdl(type.memberType())}]${type.isNonEmpty() ? '+' : ''}${type.isOptional() ? '?' : ''}`;
    if (type instanceof WdlPairType)
      return `Pair[${this.typeToWdl(type.leftType())}, ${this.typeToWdl(type.rightType())}]${type.isOptional() ? '?' : ''}`;
    if (type instanceof WdlMapType)
      return `Map[${this.typeToWdl(type.keyType())}, ${this.typeToWdl(type.valueType())}]${type.isOptional() ? '?' : ''}`;
    return '';
  }

  /** Resolve imported tasks visible for a call target (for example `ns.task` or `task`). */
  public resolveImportedTasks(
    context: WdlDocument | undefined,
    callTarget: string | undefined,
  ): ResolvedImport<WdlTask>[] {
    return this.resolveImportedCallables(context, callTarget, (doc) => doc.tasks());
  }

  /** Resolve imported workflows visible for a call target. */
  public resolveImportedWorkflows(
    context: WdlDocument | undefined,
    callTarget: string | undefined,
  ): ResolvedImport<WdlWorkflow>[] {
    return this.resolveImportedCallables(context, callTarget, (doc) => doc.workflows());
  }

  /** Resolve imported struct definitions by local visible type name. */
  public resolveImportedStructs(
    context: WdlDocument | undefined,
    visibleTypeName: string | undefined,
  ): ResolvedImport<WdlStruct>[] {
    return this.resolveImportedStructTypes(context, visibleTypeName);
  }

  /** Resolve imported enum definitions by local visible type name. */
  public resolveImportedEnums(
    context: WdlDocument | undefined,
    visibleTypeName: string | undefined,
  ): ResolvedImport<WdlEnum>[] {
    return this.resolveImportedEnumTypes(context, visibleTypeName);
  }

  /** Resolve the imported document model for a specific import statement. */
  public resolveImportedDocument(
    context: WdlDocument | undefined,
    imp: WdlImport | undefined,
  ): WdlDocument | undefined {
    if (!context || !imp) return undefined;
    const key = imp.getImportIdentifier();
    if (!key || !key.trim()) return undefined;
    return context.importedDocuments().get(key);
  }

  /** Return the namespace used for a standard import. */
  public importNamespace(imp: WdlImportStandard): string {
    const alias = imp.getAlias();
    if (alias && alias.trim()) return alias;
    const source = this.importSourceText(imp);
    if (!source.trim()) return '';
    let path = source;
    if (path.includes('/')) path = path.slice(path.lastIndexOf('/') + 1);
    if (path.endsWith('.wdl') && path.length > 4) path = path.slice(0, -4);
    return path;
  }

  /** Extract raw text for an import source literal. */
  public importSourceText(imp: WdlImport | undefined): string {
    if (!imp) return '';
    const source = imp.getSource();
    if (!source) return imp.getSourceText() ?? '';
    const chunks: string[] = [];
    for (const component of source.components()) {
      if (component instanceof WdlStringText) chunks.push(component.text ?? '');
      else if (component instanceof WdlStringEscape) chunks.push(component.escapeText ?? '');
    }
    return chunks.join('');
  }

  private resolveImportedCallables<TNode extends WdlTask | WdlWorkflow>(
    context: WdlDocument | undefined,
    callTarget: string | undefined,
    selector: (doc: WdlDocument) => TNode[],
  ): ResolvedImport<TNode>[] {
    if (!context || !callTarget || !callTarget.trim()) return [];

    const results: ResolvedImport<TNode>[] = [];
    const qualified = callTarget.includes('.');
    const [namespacePart, memberPart] = qualified
      ? [
          callTarget.slice(0, callTarget.indexOf('.')),
          callTarget.slice(callTarget.indexOf('.') + 1),
        ]
      : ['', callTarget];

    for (const imp of context.importStatements()) {
      const imported = this.resolveImportedDocument(context, imp);
      if (!imported) continue;

      if (imp instanceof WdlImportStandard) {
        const namespace = this.importNamespace(imp);
        if (!qualified || namespace !== namespacePart) continue;
        for (const node of selector(imported)) {
          const name = node.getName();
          if (name === memberPart) {
            results.push({
              localName: `${namespace}.${memberPart}`,
              importedName: memberPart,
              importNamespace: namespace,
              importStatement: imp,
              importedDocument: imported,
              symbol: node,
            });
          }
        }
      } else if (imp instanceof WdlImportStar) {
        if (qualified) continue;
        for (const node of selector(imported)) {
          const name = node.getName();
          if (name === memberPart) {
            results.push({
              localName: memberPart,
              importedName: memberPart,
              importNamespace: undefined,
              importStatement: imp,
              importedDocument: imported,
              symbol: node,
            });
          }
        }
      } else if (imp instanceof WdlImportMembers) {
        if (qualified) continue;
        for (const member of imp.members()) {
          const localName = member.getAlias() || member.getMember();
          if (localName !== memberPart) continue;
          for (const node of selector(imported)) {
            const name = node.getName();
            if (name === member.getMember()) {
              results.push({
                localName: localName ?? '',
                importedName: member.getMember() ?? '',
                importNamespace: undefined,
                importStatement: imp,
                importedDocument: imported,
                symbol: node,
              });
            }
          }
        }
      }
    }

    return results;
  }

  private resolveImportedStructTypes(
    context: WdlDocument | undefined,
    visibleTypeName: string | undefined,
  ): ResolvedImport<WdlStruct>[] {
    if (!context || !visibleTypeName || !visibleTypeName.trim()) return [];

    const results: ResolvedImport<WdlStruct>[] = [];
    for (const imp of context.importStatements()) {
      const imported = this.resolveImportedDocument(context, imp);
      if (!imported) continue;

      const selectedNodes = imported.structs();

      if (imp instanceof WdlImportStandard) {
        const aliases = this.importAliases(imp);
        for (const node of selectedNodes) {
          const importedName = node.getName();
          if (!importedName) continue;
          const localName = aliases.get(importedName) ?? importedName;
          if (localName === visibleTypeName) {
            results.push({
              localName,
              importedName,
              importNamespace: undefined,
              importStatement: imp,
              importedDocument: imported,
              symbol: node,
            });
          }
        }
      } else if (imp instanceof WdlImportStar) {
        for (const node of selectedNodes) {
          const importedName = node.getName();
          if (importedName === visibleTypeName) {
            results.push({
              localName: visibleTypeName,
              importedName: visibleTypeName,
              importNamespace: undefined,
              importStatement: imp,
              importedDocument: imported,
              symbol: node,
            });
          }
        }
      } else if (imp instanceof WdlImportMembers) {
        for (const member of imp.members()) {
          const localName = member.getAlias() || member.getMember();
          if (localName !== visibleTypeName) continue;
          for (const node of selectedNodes) {
            const importedName = node.getName();
            if (importedName === member.getMember()) {
              results.push({
                localName: localName ?? '',
                importedName: member.getMember() ?? '',
                importNamespace: undefined,
                importStatement: imp,
                importedDocument: imported,
                symbol: node,
              });
            }
          }
        }
      }
    }

    return results;
  }

  private resolveImportedEnumTypes(
    context: WdlDocument | undefined,
    visibleTypeName: string | undefined,
  ): ResolvedImport<WdlEnum>[] {
    if (!context || !visibleTypeName || !visibleTypeName.trim()) return [];

    const results: ResolvedImport<WdlEnum>[] = [];
    for (const imp of context.importStatements()) {
      const imported = this.resolveImportedDocument(context, imp);
      if (!imported) continue;

      const selectedNodes = imported.enums();

      if (imp instanceof WdlImportStandard) {
        const aliases = this.importAliases(imp);
        for (const node of selectedNodes) {
          const importedName = node.getName();
          if (!importedName) continue;
          const localName = aliases.get(importedName) ?? importedName;
          if (localName === visibleTypeName) {
            results.push({
              localName,
              importedName,
              importNamespace: undefined,
              importStatement: imp,
              importedDocument: imported,
              symbol: node,
            });
          }
        }
      } else if (imp instanceof WdlImportStar) {
        for (const node of selectedNodes) {
          const importedName = node.getName();
          if (importedName === visibleTypeName) {
            results.push({
              localName: visibleTypeName,
              importedName: visibleTypeName,
              importNamespace: undefined,
              importStatement: imp,
              importedDocument: imported,
              symbol: node,
            });
          }
        }
      } else if (imp instanceof WdlImportMembers) {
        for (const member of imp.members()) {
          const localName = member.getAlias() || member.getMember();
          if (localName !== visibleTypeName) continue;
          for (const node of selectedNodes) {
            const importedName = node.getName();
            if (importedName === member.getMember()) {
              results.push({
                localName: localName ?? '',
                importedName: member.getMember() ?? '',
                importNamespace: undefined,
                importStatement: imp,
                importedDocument: imported,
                symbol: node,
              });
            }
          }
        }
      }
    }

    return results;
  }

  private importAliases(imp: WdlImportStandard): Map<string, string> {
    const aliases = new Map<string, string>();
    for (const member of imp.members()) {
      const memberName = member.getMember();
      if (!memberName || !memberName.trim()) continue;
      aliases.set(memberName, member.getAlias() || memberName);
    }
    return aliases;
  }
}
