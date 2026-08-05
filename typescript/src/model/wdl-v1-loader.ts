/** Core ANTLR-backed WDL 1.x parser and model builder. */
import {
  BaseErrorListener,
  CharStream,
  CommonTokenStream,
  type ParserRuleContext,
  type Token,
} from 'antlr4ng';

import {
  AdditiveExprOperationContext,
  BoundDeclarationContext,
  BracedCommandContext,
  CallAliasContext,
  CallExpressionContext,
  CallInputBlockContext,
  CallInputItemContext,
  CallStatementContext,
  CallTargetContext,
  ComparisonExprOperationContext,
  ConditionalElseClauseContext,
  ConditionalElseIfClauseContext,
  ConditionalStatementContext,
  DocumentContext,
  DocumentElementContext,
  EnumMultilineStringContext,
  EnumQuotedStringContext,
  EnumStringLiteralContext,
  EqualityExprOperationContext,
  ExpressionContext,
  GroupedExpressionContext,
  HintsItemTaskContext,
  HintsItemWorkflowContext,
  HintsSectionTaskContext,
  HintsSectionWorkflowContext,
  HintsValueTaskContext,
  HintsValueWorkflowContext,
  IfExpressionContext,
  ImportAliasContext,
  ImportMemberContext,
  ImportMembersContext,
  ImportStatementMembersContext,
  ImportStatementStandardContext,
  ImportStatementStarContext,
  ImportUriLiteralContext,
  InputSectionContext,
  LogicalAndExprOperationContext,
  LogicalOrExprOperationContext,
  MapLiteralContext,
  MapLiteralItemContext,
  MapTypeContext,
  ArrayLiteralContext,
  MetadataArrayContext,
  MetadataObjectContext,
  MetadataObjectItemContext,
  MetadataSectionContext,
  MetadataValueContext,
  MultilineStringCommandContext,
  MultilineStringContext,
  MultilineStringElementDoubleCloseAngleContext,
  MultilineStringElementDollarSignContext,
  MultilineStringElementEscapeContext,
  MultilineStringElementPlaceholderContext,
  MultilineStringElementSingleCloseAngleContext,
  MultilineStringElementTextContext,
  MultilineStringElementTildeContext,
  MultilineStringPlaceholderContext,
  MultiplicativeExprOperationContext,
  NullLiteralContext,
  NumberLiteralFloatContext,
  NumberLiteralIntContext,
  NumberLiteralSignedContext,
  ObjectLiteralContext,
  ObjectLiteralItemContext,
  OutputSectionContext,
  PairLiteralContext,
  PairTypeContext,
  ParameterMetadataSectionContext,
  PostfixExprArrayIndexContext,
  PostfixExprFieldContext,
  PowerExprOperationContext,
  PrimitiveTypeContext,
  QuotedStringContext,
  RequirementsItemContext,
  RequirementsSectionContext,
  RuntimeItemContext,
  RuntimeSectionContext,
  ScatterBodyContext,
  ScatterStatementContext,
  StrictIdentifierContext,
  StringElementDollarSignContext,
  StringElementEscapeContext,
  StringElementPlaceholderContext,
  StringElementTextContext,
  StringElementTildeContext,
  StringPlaceholderContext,
  StringPlaceholderExpressionContext,
  StringPlaceholderOptionFalseTrueContext,
  StringPlaceholderOptionSepDefaultContext,
  StringPlaceholderOptionTrueFalseContext,
  StringLiteralContext,
  StructDefinitionContext,
  StructItemMemberDeclarationContext,
  StructItemMetadataContext,
  StructItemParameterMetadataContext,
  StructLiteralContext,
  StructLiteralItemContext,
  TaskCommandSectionContext,
  TaskDeclarationContext,
  TaskDefinitionContext,
  TaskHintsSectionContext,
  TaskInputSectionContext,
  TaskMetadataSectionContext,
  TaskOutputSectionContext,
  TaskParameterMetadataSectionContext,
  TaskRequirementsSectionContext,
  TaskRuntimeSectionContext,
  TypeContext,
  TypeRefTypeContext,
  UnaryExprOperationContext,
  UnboundDeclarationContext,
  VariableContext,
  VersionStatementContext,
  WdlV1Parser,
  WorkflowCallStatementContext,
  WorkflowConditionalStatementContext,
  WorkflowDeclarationContext,
  WorkflowDefinitionContext,
  WorkflowHintsSectionContext,
  WorkflowInputSectionContext,
  WorkflowMetadataSectionContext,
  WorkflowOutputSectionContext,
  WorkflowParameterMetadataSectionContext,
  WorkflowScatterStatementContext,
  TaskHintValueArrayContext,
  TaskHintValueExpressionContext,
  WorkflowHintValueArrayContext,
  WorkflowHintValueBooleanContext,
  WorkflowHintValueNumberContext,
  WorkflowHintValueStringContext,
} from '../generated/antlr4/v1/WdlV1Parser.js';
import { WdlV1Lexer } from '../generated/antlr4/v1/WdlV1Lexer.js';
import { WdlDocument, type WdlDocumentElement } from './wdl-document.js';
import { WdlVersion } from './wdl-version.js';
import {
  WdlEnum,
  WdlEnumChoice,
  WdlStruct,
  WdlStructMember,
  WdlTask,
  WdlWorkflow,
} from './definitions/index.js';
import { AssertionError, WdlException, WdlSyntaxError } from './errors/index.js';
import {
  Delimiter,
  PlaceHolderSymbol,
  WdlArrayLiteral,
  WdlBinaryOperation,
  WdlBinaryOperator,
  WdlBooleanLiteral,
  type WdlExpression,
  WdlFloatLiteral,
  WdlFunctionCallOperation,
  WdlIndexAccessOperation,
  WdlIntLiteral,
  WdlMapEntry,
  WdlMapLiteral,
  WdlMemberAccessOperation,
  WdlNullLiteral,
  WdlObjectEntry,
  WdlObjectLiteral,
  WdlPairLiteral,
  WdlStringEscape,
  WdlStringLiteral,
  WdlStringPlaceholder,
  WdlStringPlaceholderOption,
  WdlStringPlaceholderOptionType,
  WdlStringText,
  WdlStringToken,
  WdlStructEntry,
  WdlStructLiteral,
  WdlTernaryOperation,
  WdlUnaryOperation,
  WdlUnaryOperator,
  WdlVariable,
} from './expressions/index.js';
import {
  WdlCommand,
  WdlInput,
  WdlMetadata,
  WdlMetadataEntry,
  WdlOutput,
  WdlParameterMetadata,
  WdlRequirementEntry,
  WdlRequirements,
  WdlRuntime,
  WdlRuntimeEntry,
  WdlTaskHint,
  WdlTaskHints,
  WdlWorkflowHint,
  WdlWorkflowHints,
} from './sections/index.js';
import {
  WdlBoundDeclaration,
  WdlCall,
  WdlCallInput,
  WdlConditional,
  WdlConditionalElseIf,
  WdlDeclaration,
  WdlImportMember,
  WdlImport,
  WdlImportMembers,
  WdlImportStandard,
  WdlImportStar,
  WdlScatter,
  type WdlStatement,
} from './statements/index.js';
import {
  WdlArrayType,
  WdlMapType,
  WdlPairType,
  WdlPrimitiveType,
  WdlType,
  WdlTypeReferenceType,
} from './types/index.js';
import { WdlImportResolverBase } from './resolvers/wdl-import-resolver-base.js';

export interface WdlValidator {
  /** Validates a parsed WDL document. */
  validateDocument(document: WdlDocument): void;
}

/** Collects syntax diagnostics from the generated lexer and parser. */
class WdlErrorListener extends BaseErrorListener {
  public readonly syntaxErrors: WdlSyntaxError[] = [];

  public override syntaxError(
    _recognizer: unknown,
    _offendingSymbol: Token | null,
    line: number,
    charPositionInLine: number,
    msg: string,
    e: unknown,
  ): void {
    this.syntaxErrors.push(new WdlSyntaxError(msg, line, charPositionInLine, e));
  }

  public throwIfErrored(): void {
    if (this.syntaxErrors.length > 0) {
      throw new WdlException(this.syntaxErrors);
    }
  }
}

export class WdlV1Loader {
  /** Parses a WDL document from an `antlr4ng` character stream and optionally validates it. */
  public static load(
    input: CharStream,
    validator?: WdlValidator,
    importResolver?: WdlImportResolverBase,
    currentDocumentLocation?: string,
  ): WdlDocument {
    const lexer = new WdlV1Lexer(input);
    const parser = new WdlV1Parser(new CommonTokenStream(lexer));

    const errorListener = new WdlErrorListener();
    lexer.removeErrorListeners();
    parser.removeErrorListeners();
    lexer.addErrorListener(errorListener);
    parser.addErrorListener(errorListener);

    const documentContext = parser.document();
    errorListener.throwIfErrored();

    const document = this.buildDocument(documentContext, currentDocumentLocation);
    if (importResolver) {
      this.resolveImportsRecursive(document, importResolver, new Map<string, WdlDocument>());
    }
    if (validator) {
      validator.validateDocument(document);
    }
    return document;
  }

  /** Parses a WDL document from a source string and optionally validates it. */
  public static loadFromString(
    source: string,
    validator?: WdlValidator,
    sourceLocation?: string,
    importResolver?: WdlImportResolverBase,
  ): WdlDocument {
    return this.load(CharStream.fromString(source), validator, importResolver, sourceLocation);
  }

  private static buildDocument(
    ctx: DocumentContext,
    currentDocumentLocation?: string,
  ): WdlDocument {
    const versionCtx = ctx.versionStatement();
    const version = versionCtx ? this.buildVersion(versionCtx) : undefined;
    const document = new WdlDocument(version);
    document.setSourceLocation(currentDocumentLocation);
    for (const elementCtx of ctx.documentElement()) {
      const element = this.buildDocumentElement(elementCtx);
      if (element) {
        document.elements().push(element);
      }
    }
    return document;
  }

  private static resolveImportsRecursive(
    document: WdlDocument,
    importResolver: WdlImportResolverBase,
    loadedById: Map<string, WdlDocument>,
  ): void {
    const currentSourceLocation = document.getSourceLocation();
    if (currentSourceLocation) {
      loadedById.set(currentSourceLocation, document);
    }

    document.importedDocuments().clear();
    const currentLocation = document.getSourceLocation();
    for (const imp of document.importStatements()) {
      const sourceLiteral = imp.getSource();
      if (!sourceLiteral) continue;

      const importReference = this.extractStringLiteralText(sourceLiteral);
      const resolvedImportLocation = importResolver.resolveImportLocation(
        currentLocation,
        importReference,
      );
      const importIdentifier = resolvedImportLocation || importReference;
      imp.setImportIdentifier(importIdentifier);

      const importSourceText = importResolver.resolveImport(currentLocation, importReference);
      imp.setSourceText(importSourceText);

      let importedDocument = loadedById.get(importIdentifier);
      if (!importedDocument) {
        importedDocument = this.loadFromString(
          importSourceText,
          undefined,
          resolvedImportLocation,
          undefined,
        );
        loadedById.set(importIdentifier, importedDocument);
        this.resolveImportsRecursive(importedDocument, importResolver, loadedById);
      }

      document.importedDocuments().set(importIdentifier, importedDocument);
    }
  }

  private static extractStringLiteralText(sourceLiteral: WdlStringLiteral): string {
    const parts: string[] = [];
    for (const component of sourceLiteral.components()) {
      if (component instanceof WdlStringText) {
        parts.push(component.text ?? '');
      } else if (component instanceof WdlStringEscape) {
        parts.push(component.escapeText ?? '');
      } else {
        throw new AssertionError('Unsupported import URI element');
      }
    }
    return parts.join('');
  }

  private static buildVersion(ctx: VersionStatementContext): WdlVersion {
    return WdlVersion.fromString(ctx.FLOAT().getText());
  }

  private static buildDocumentElement(ctx: DocumentElementContext): WdlDocumentElement | undefined {
    const importCtx = ctx.importStatement();
    if (importCtx) return this.buildImport(importCtx);
    const structCtx = ctx.structDefinition();
    if (structCtx) return this.buildStruct(structCtx);
    const enumCtx = ctx.enumDefinition();
    if (enumCtx) return this.buildEnum(enumCtx);
    const taskCtx = ctx.taskDefinition();
    if (taskCtx) return this.buildTask(taskCtx);
    const workflowCtx = ctx.workflowDefinition();
    if (workflowCtx) return this.buildWorkflow(workflowCtx);
    return undefined;
  }

  private static buildImport(
    ctx: ParserRuleContext,
  ): WdlImportStandard | WdlImportMembers | WdlImportStar {
    if (ctx instanceof ImportStatementStandardContext) {
      const value = new WdlImportStandard();
      value.setSource(this.buildImportUriLiteral(ctx.importUriLiteral()));
      if (ctx.KEYWORD_AS() && ctx.strictIdentifier()) {
        value.setAlias(this.strictIdentifierText(ctx.strictIdentifier()!));
      }
      const aliasContexts = ctx.importAlias();
      for (const aliasCtx of aliasContexts) {
        value.members().push(this.buildImportAliasMember(aliasCtx));
      }
      return value;
    }
    if (ctx instanceof ImportStatementMembersContext) {
      const value = new WdlImportMembers();
      value.setSource(this.buildImportUriLiteral(ctx.importUriLiteral()));
      for (const memberCtx of ctx.importMembers().importMember()) {
        value.members().push(this.buildImportMember(memberCtx));
      }
      return value;
    }
    const value = new WdlImportStar();
    value.setSource(
      this.buildImportUriLiteral((ctx as ImportStatementStarContext).importUriLiteral()),
    );
    return value;
  }

  private static buildImportMember(ctx: ImportMemberContext): WdlImportMember {
    return new WdlImportMember(
      this.strictIdentifierText(ctx.strictIdentifier(0)!),
      ctx.strictIdentifier(1) ? this.strictIdentifierText(ctx.strictIdentifier(1)!) : undefined,
    );
  }

  private static buildImportAliasMember(ctx: ImportAliasContext): WdlImportMember {
    return new WdlImportMember(
      this.strictIdentifierText(ctx.strictIdentifier(0)!),
      this.strictIdentifierText(ctx.strictIdentifier(1)!),
    );
  }

  private static buildImportUriLiteral(ctx: ImportUriLiteralContext): WdlStringLiteral {
    const value = new WdlStringLiteral(Delimiter.SINGLE_QUOTED);
    value.components().push(new WdlStringText(ctx.getText().replace(/^"|"$/g, '')));
    return value;
  }

  private static buildStruct(ctx: StructDefinitionContext): WdlStruct {
    const value = new WdlStruct(this.strictIdentifierText(ctx.strictIdentifier()));
    for (const item of ctx.structItem()) {
      if (item instanceof StructItemMemberDeclarationContext) {
        value.elements().push(this.buildStructMember(item.structDeclaration()));
      } else if (item instanceof StructItemMetadataContext) {
        value.elements().push(this.buildMetadataSection(item.metadataSection()));
      } else if (item instanceof StructItemParameterMetadataContext) {
        value.elements().push(this.buildParameterMetadataSection(item.parameterMetadataSection()));
      }
    }
    return value;
  }

  private static buildStructMember(
    ctx: BoundDeclarationContext | UnboundDeclarationContext | ParserRuleContext,
  ): WdlStructMember {
    const type =
      'type' in ctx && typeof (ctx as { type(): TypeContext }).type === 'function'
        ? this.buildType((ctx as { type(): TypeContext }).type())
        : undefined;
    const name =
      'strictIdentifier' in ctx &&
      typeof (ctx as { strictIdentifier(): StrictIdentifierContext }).strictIdentifier ===
        'function'
        ? this.strictIdentifierText(
            (ctx as { strictIdentifier(): StrictIdentifierContext }).strictIdentifier(),
          )
        : undefined;
    return new WdlStructMember(type, name);
  }

  private static buildEnum(ctx: any): WdlEnum {
    const valueType = ctx.enumTypeParameter?.()
      ? this.buildType(ctx.enumTypeParameter().type())
      : undefined;
    const value = new WdlEnum(this.strictIdentifierText(ctx.strictIdentifier()), valueType);
    for (const choice of ctx.enumChoice?.() ?? []) {
      const identifiers = choice.strictIdentifier?.();
      const name = Array.isArray(identifiers)
        ? this.strictIdentifierText(identifiers[0])
        : this.strictIdentifierText(identifiers);
      const expr = choice.enumLiteralExpression?.()
        ? this.buildEnumLiteralExpression(choice.enumLiteralExpression())
        : undefined;
      value.elements().push(new WdlEnumChoice(name, expr));
    }
    return value;
  }

  private static buildEnumLiteralExpression(ctx: any): WdlExpression {
    if (ctx.enumStringLiteral?.()) return this.buildEnumStringLiteral(ctx.enumStringLiteral());
    if (ctx.numberLiteralSigned?.())
      return this.buildNumberLiteralSigned(ctx.numberLiteralSigned());
    if (ctx.booleanLiteral?.())
      return new WdlBooleanLiteral(ctx.booleanLiteral().KEYWORD_TRUE() !== null);
    if (ctx.nullLiteral?.()) return new WdlNullLiteral(null);
    if (ctx.enumArrayLiteral?.()) {
      const value = new WdlArrayLiteral();
      for (const item of ctx.enumArrayLiteral().enumLiteralExpression())
        value.entries().push(this.buildEnumLiteralExpression(item));
      return value;
    }
    if (ctx.enumMapLiteral?.()) {
      const value = new WdlMapLiteral();
      for (const item of ctx.enumMapLiteral().enumMapLiteralItem()) {
        value
          .entries()
          .push(
            new WdlMapEntry(
              this.buildEnumLiteralExpression(item.enumLiteralExpression(0)!),
              this.buildEnumLiteralExpression(item.enumLiteralExpression(1)!),
            ),
          );
      }
      return value;
    }
    if (ctx.enumObjectLiteral?.()) {
      const value = new WdlObjectLiteral();
      for (const item of ctx.enumObjectLiteral().enumObjectLiteralItem()) {
        value
          .entries()
          .push(
            new WdlObjectEntry(
              this.strictIdentifierText(item.strictIdentifier()),
              this.buildEnumLiteralExpression(item.enumLiteralExpression()),
            ),
          );
      }
      return value;
    }
    if (ctx.enumStructLiteral?.()) {
      const value = new WdlStructLiteral(
        this.strictIdentifierText(ctx.enumStructLiteral().strictIdentifier()),
      );
      for (const item of ctx.enumStructLiteral().enumStructLiteralItem()) {
        value
          .entries()
          .push(
            new WdlStructEntry(
              this.strictIdentifierText(item.strictIdentifier()),
              this.buildEnumLiteralExpression(item.enumLiteralExpression()),
            ),
          );
      }
      return value;
    }
    if (ctx.enumPairLiteral?.()) {
      return new WdlPairLiteral(
        this.buildEnumLiteralExpression(ctx.enumPairLiteral().enumLiteralExpression(0)!),
        this.buildEnumLiteralExpression(ctx.enumPairLiteral().enumLiteralExpression(1)!),
      );
    }
    return new WdlStringLiteral();
  }

  private static buildEnumStringLiteral(ctx: EnumStringLiteralContext): WdlStringLiteral {
    const quoted = ctx.enumQuotedString();
    if (quoted) return this.buildEnumQuotedString(quoted);
    return this.buildEnumMultilineString(ctx.enumMultilineString()!);
  }

  private static buildEnumQuotedString(ctx: EnumQuotedStringContext): WdlStringLiteral {
    const value = new WdlStringLiteral(Delimiter.SINGLE_QUOTED);
    for (const element of ctx.enumStringElement()) {
      if (element.STRING_TEXT())
        value.components().push(new WdlStringText(element.STRING_TEXT()!.getText()));
      else if (element.STRING_ESCAPE())
        value.components().push(new WdlStringEscape(element.STRING_ESCAPE()!.getText()));
      else if (element.STRING_DOLLAR_SIGN())
        value.components().push(new WdlStringToken(element.STRING_DOLLAR_SIGN()!.getText()));
      else if (element.STRING_TILDE())
        value.components().push(new WdlStringToken(element.STRING_TILDE()!.getText()));
    }
    return value;
  }

  private static buildEnumMultilineString(ctx: EnumMultilineStringContext): WdlStringLiteral {
    const value = new WdlStringLiteral(Delimiter.DOUBLE_ANGLE);
    for (const element of ctx.enumMultilineStringElement()) {
      if (element.MULTILINE_STRING_TEXT())
        value.components().push(new WdlStringText(element.MULTILINE_STRING_TEXT()!.getText()));
      else if (element.MULTILINE_STRING_ESCAPE())
        value.components().push(new WdlStringEscape(element.MULTILINE_STRING_ESCAPE()!.getText()));
      else if (element.MULTILINE_STRING_DOLLAR_SIGN())
        value
          .components()
          .push(new WdlStringToken(element.MULTILINE_STRING_DOLLAR_SIGN()!.getText()));
      else if (element.MULTILINE_STRING_TILDE())
        value.components().push(new WdlStringToken(element.MULTILINE_STRING_TILDE()!.getText()));
      else if (element.MULTILINE_STRING_DOUBLE_CLOSE_ANGLE())
        value
          .components()
          .push(new WdlStringToken(element.MULTILINE_STRING_DOUBLE_CLOSE_ANGLE()!.getText()));
      else if (element.MULTILINE_STRING_SINGLE_CLOSE_ANGLE())
        value
          .components()
          .push(new WdlStringToken(element.MULTILINE_STRING_SINGLE_CLOSE_ANGLE()!.getText()));
    }
    return value;
  }

  private static buildTask(ctx: TaskDefinitionContext): WdlTask {
    const value = new WdlTask(this.strictIdentifierText(ctx.strictIdentifier()));
    for (const element of ctx.taskElement()) {
      if (element instanceof TaskInputSectionContext)
        value.elements().push(this.buildInputSection(element.inputSection()));
      else if (element instanceof TaskOutputSectionContext)
        value.elements().push(this.buildOutputSection(element.outputSection()));
      else if (element instanceof TaskRuntimeSectionContext)
        value.elements().push(this.buildRuntimeSection(element.runtimeSection()));
      else if (element instanceof TaskRequirementsSectionContext)
        value.elements().push(this.buildRequirementsSection(element.requirementsSection()));
      else if (element instanceof TaskHintsSectionContext)
        value.elements().push(this.buildTaskHintsSection(element.hintsSectionTask()));
      else if (element instanceof TaskMetadataSectionContext)
        value.elements().push(this.buildMetadataSection(element.metadataSection()));
      else if (element instanceof TaskParameterMetadataSectionContext)
        value
          .elements()
          .push(this.buildParameterMetadataSection(element.parameterMetadataSection()));
      else if (element instanceof TaskCommandSectionContext)
        value.elements().push(this.buildCommandSection(element.commandSection()));
      else if (element instanceof TaskDeclarationContext)
        value.elements().push(this.buildBoundDeclaration(element.boundDeclaration()));
    }
    return value;
  }

  private static buildWorkflow(ctx: WorkflowDefinitionContext): WdlWorkflow {
    const value = new WdlWorkflow(this.strictIdentifierText(ctx.strictIdentifier()));
    for (const element of ctx.workflowElement()) {
      if (element instanceof WorkflowInputSectionContext)
        value.elements().push(this.buildInputSection(element.inputSection()));
      else if (element instanceof WorkflowOutputSectionContext)
        value.elements().push(this.buildOutputSection(element.outputSection()));
      else if (element instanceof WorkflowHintsSectionContext)
        value.elements().push(this.buildWorkflowHintsSection(element.hintsSectionWorkflow()));
      else if (element instanceof WorkflowMetadataSectionContext)
        value.elements().push(this.buildMetadataSection(element.metadataSection()));
      else if (element instanceof WorkflowParameterMetadataSectionContext)
        value
          .elements()
          .push(this.buildParameterMetadataSection(element.parameterMetadataSection()));
      else if (element instanceof WorkflowCallStatementContext)
        value.elements().push(this.buildCallStatement(element.callStatement()));
      else if (element instanceof WorkflowConditionalStatementContext)
        value.elements().push(this.buildConditionalStatement(element.conditionalStatement()));
      else if (element instanceof WorkflowScatterStatementContext)
        value.elements().push(this.buildScatterStatement(element.scatterStatement()));
      else if (element instanceof WorkflowDeclarationContext)
        value.elements().push(this.buildBoundDeclaration(element.boundDeclaration()));
    }
    return value;
  }

  private static buildInputSection(ctx: InputSectionContext): WdlInput {
    const value = new WdlInput();
    for (const declaration of ctx.declaration()) {
      value.elements().push(this.buildDeclaration(declaration));
    }
    return value;
  }

  private static buildOutputSection(ctx: OutputSectionContext): WdlOutput {
    const value = new WdlOutput();
    for (const declaration of ctx.boundDeclaration()) {
      value.elements().push(this.buildBoundDeclaration(declaration));
    }
    return value;
  }

  private static buildRuntimeSection(ctx: RuntimeSectionContext): WdlRuntime {
    const value = new WdlRuntime();
    for (const item of ctx.runtimeItem()) {
      value
        .elements()
        .push(
          new WdlRuntimeEntry(
            this.strictIdentifierText(item.strictIdentifier()),
            this.buildExpression(item.expression()),
          ),
        );
    }
    return value;
  }

  private static buildRequirementsSection(ctx: RequirementsSectionContext): WdlRequirements {
    const value = new WdlRequirements();
    for (const item of ctx.requirementsItem()) {
      value
        .elements()
        .push(
          new WdlRequirementEntry(
            this.strictIdentifierText(item.strictIdentifier()),
            this.buildExpression(item.expression()),
          ),
        );
    }
    return value;
  }

  private static buildTaskHintsSection(ctx: HintsSectionTaskContext): WdlTaskHints {
    const value = new WdlTaskHints();
    for (const item of ctx.hintsItemTask()) {
      value
        .elements()
        .push(
          new WdlTaskHint(
            this.strictIdentifierText(item.strictIdentifier()),
            this.buildTaskHintValue(item),
          ),
        );
    }
    return value;
  }

  private static buildWorkflowHintsSection(ctx: HintsSectionWorkflowContext): WdlWorkflowHints {
    const value = new WdlWorkflowHints();
    for (const item of ctx.hintsItemWorkflow()) {
      value
        .elements()
        .push(
          new WdlWorkflowHint(
            this.strictIdentifierText(item.strictIdentifier()),
            this.buildWorkflowHintValue(item),
          ),
        );
    }
    return value;
  }

  private static buildMetadataSection(ctx: MetadataSectionContext): WdlMetadata {
    const value = new WdlMetadata();
    for (const item of ctx.metadataObjectItem()) {
      value
        .elements()
        .push(
          new WdlMetadataEntry(
            this.dottedIdentifierText(item.dottedIdentifier()),
            this.buildMetadataValue(item.metadataValue()),
          ),
        );
    }
    return value;
  }

  private static buildParameterMetadataSection(
    ctx: ParameterMetadataSectionContext,
  ): WdlParameterMetadata {
    const value = new WdlParameterMetadata();
    for (const item of ctx.metadataObjectItem()) {
      value
        .elements()
        .push(
          new WdlMetadataEntry(
            this.dottedIdentifierText(item.dottedIdentifier()),
            this.buildMetadataValue(item.metadataValue()),
          ),
        );
    }
    return value;
  }

  private static buildCommandSection(ctx: any): WdlCommand {
    const multiline = ctx.multilineStringCommand?.();
    if (multiline) {
      return new WdlCommand(this.buildMultilineString(multiline), true);
    }
    return new WdlCommand(this.buildBracedCommand(ctx.bracedCommand()), false);
  }

  private static buildDeclaration(ctx: any): WdlDeclaration {
    const bound = ctx.boundDeclaration?.();
    if (bound) {
      return this.buildBoundDeclaration(bound);
    }
    return this.buildUnboundDeclaration(ctx.unboundDeclaration());
  }

  private static buildUnboundDeclaration(ctx: UnboundDeclarationContext): WdlDeclaration {
    const value = new WdlDeclaration(
      this.buildType(ctx.type()),
      this.strictIdentifierText(ctx.strictIdentifier()),
      ctx.KEYWORD_ENV() !== null,
    );
    return value;
  }

  private static buildBoundDeclaration(ctx: BoundDeclarationContext): WdlBoundDeclaration {
    const value = new WdlBoundDeclaration(
      this.buildType(ctx.type()),
      this.strictIdentifierText(ctx.strictIdentifier()),
      this.buildExpression(ctx.expression()),
    );
    value.setEnvironmentVariable(ctx.KEYWORD_ENV() !== null);
    return value;
  }

  private static buildCallStatement(ctx: CallStatementContext): WdlCall {
    const value = new WdlCall();
    for (const target of ctx.callTarget().strictIdentifier()) {
      value.targetPath().push(this.strictIdentifierText(target));
    }
    const alias = ctx.callAlias();
    if (alias) {
      value.setAlias(this.strictIdentifierText(alias.strictIdentifier()));
    }
    for (const after of ctx.callAfterClause()) {
      value.afterDependencies().push(this.strictIdentifierText(after.strictIdentifier()));
    }
    const inputBlock = ctx.callInputBlock();
    if (inputBlock) {
      value.setLegacyInputColonUsed(inputBlock.KEYWORD_INPUT() !== null);
      for (const item of inputBlock.callInputItem()) {
        const key = this.strictIdentifierText(item.strictIdentifier());
        const expression = item.expression()
          ? this.buildExpression(item.expression()!)
          : new WdlVariable(key);
        value.inputs().push(new WdlCallInput(key, expression));
      }
    }
    return value;
  }

  private static buildScatterStatement(ctx: ScatterStatementContext): WdlScatter {
    const value = new WdlScatter(
      this.strictIdentifierText(ctx.strictIdentifier()),
      this.buildExpression(ctx.expression()),
    );
    for (const statement of ctx.scatterBody().workflowStatement()) {
      value.statements().push(this.buildWorkflowStatement(statement));
    }
    return value;
  }

  private static buildConditionalStatement(ctx: ConditionalStatementContext): WdlConditional {
    const value = new WdlConditional(this.buildExpression(ctx.expression()));
    for (const statement of ctx.workflowStatement()) {
      value.thenStatements().push(this.buildWorkflowStatement(statement));
    }
    for (const elseIfCtx of ctx.conditionalElseIfClause()) {
      const elseIf = new WdlConditionalElseIf(this.buildExpression(elseIfCtx.expression()));
      for (const statement of elseIfCtx.workflowStatement()) {
        elseIf.thenStatements().push(this.buildWorkflowStatement(statement));
      }
      value.elseIfs().push(elseIf);
    }
    const elseClause = ctx.conditionalElseClause();
    if (elseClause) {
      for (const statement of elseClause.workflowStatement()) {
        value.elseStatements().push(this.buildWorkflowStatement(statement));
      }
    }
    return value;
  }

  private static buildWorkflowStatement(ctx: any): WdlStatement {
    if (ctx.boundDeclaration?.()) return this.buildBoundDeclaration(ctx.boundDeclaration());
    if (ctx.callStatement?.()) return this.buildCallStatement(ctx.callStatement());
    if (ctx.scatterStatement?.()) return this.buildScatterStatement(ctx.scatterStatement());
    return this.buildConditionalStatement(ctx.conditionalStatement());
  }

  private static buildType(ctx: TypeContext): WdlType {
    const primitive = ctx.primitiveType();
    if (primitive) return this.buildPrimitiveType(primitive);
    const typeRef = ctx.typeRefType();
    if (typeRef)
      return new WdlTypeReferenceType(
        this.strictIdentifierText(typeRef.strictIdentifier()),
        typeRef.QUESTION_MARK() !== null,
      );
    const arrayType = ctx.arrayType();
    if (arrayType)
      return new WdlArrayType(
        this.buildType(arrayType.type()),
        arrayType.PLUS() !== null,
        arrayType.QUESTION_MARK() !== null,
      );
    const pairType = ctx.pairType();
    if (pairType)
      return new WdlPairType(
        this.buildType(pairType.type_(0)!),
        this.buildType(pairType.type_(1)!),
        pairType.QUESTION_MARK() !== null,
      );
    const mapType = ctx.mapType();
    if (mapType)
      return new WdlMapType(
        this.buildPrimitiveType(mapType.primitiveType()),
        this.buildType(mapType.type()),
        mapType.QUESTION_MARK() !== null,
      );
    return new WdlTypeReferenceType('Object', true);
  }

  private static buildPrimitiveType(ctx: PrimitiveTypeContext): WdlPrimitiveType {
    const text = ctx.getText().replace(/\?$/, '');
    const optional = ctx.QUESTION_MARK() !== null;
    const mapping: Record<string, WdlPrimitiveType.Type> = {
      Boolean: WdlPrimitiveType.Type.BOOLEAN,
      Int: WdlPrimitiveType.Type.INT,
      Float: WdlPrimitiveType.Type.FLOAT,
      String: WdlPrimitiveType.Type.STRING,
      File: WdlPrimitiveType.Type.FILE,
      Directory: WdlPrimitiveType.Type.DIRECTORY,
      Object: WdlPrimitiveType.Type.OBJECT,
    };
    const primitiveType = mapping[text];
    if (primitiveType === undefined) {
      throw new AssertionError(`Unknown primitive type ${ctx.getText()}`);
    }
    return new WdlPrimitiveType(primitiveType, optional);
  }

  private static buildExpression(ctx: ExpressionContext): WdlExpression {
    return this.buildLogicalOrExpression(ctx.logicalOrExpression());
  }

  private static buildLogicalOrExpression(ctx: any): WdlExpression {
    if (ctx instanceof LogicalOrExprOperationContext) {
      return new WdlBinaryOperation(
        this.buildLogicalOrExpression(ctx.logicalOrExpression()),
        WdlBinaryOperator.OR,
        this.buildLogicalAndExpression(ctx.logicalAndExpression()),
      );
    }
    return this.buildLogicalAndExpression(ctx.logicalAndExpression());
  }

  private static buildLogicalAndExpression(ctx: any): WdlExpression {
    if (ctx instanceof LogicalAndExprOperationContext) {
      return new WdlBinaryOperation(
        this.buildLogicalAndExpression(ctx.logicalAndExpression()),
        WdlBinaryOperator.AND,
        this.buildEqualityExpression(ctx.equalityExpression()),
      );
    }
    return this.buildEqualityExpression(ctx.equalityExpression());
  }

  private static buildEqualityExpression(ctx: any): WdlExpression {
    if (ctx instanceof EqualityExprOperationContext) {
      return new WdlBinaryOperation(
        this.buildEqualityExpression(ctx.equalityExpression()),
        ctx.EQUAL() ? WdlBinaryOperator.EQ : WdlBinaryOperator.NEQ,
        this.buildComparisonExpression(ctx.comparisonExpression()),
      );
    }
    return this.buildComparisonExpression(ctx.comparisonExpression());
  }

  private static buildComparisonExpression(ctx: any): WdlExpression {
    if (ctx instanceof ComparisonExprOperationContext) {
      let op: WdlBinaryOperator;
      if (ctx.LESS()) {
        op = WdlBinaryOperator.LT;
      } else if (ctx.LESS_EQUAL()) {
        op = WdlBinaryOperator.LTE;
      } else if (ctx.GREATER()) {
        op = WdlBinaryOperator.GT;
      } else if (ctx.GREATER_EQUAL()) {
        op = WdlBinaryOperator.GTE;
      } else {
        throw new AssertionError('Unknown comparison operator');
      }
      return new WdlBinaryOperation(
        this.buildComparisonExpression(ctx.comparisonExpression()),
        op,
        this.buildAdditiveExpression(ctx.additiveExpression()),
      );
    }
    return this.buildAdditiveExpression(ctx.additiveExpression());
  }

  private static buildAdditiveExpression(ctx: any): WdlExpression {
    if (ctx instanceof AdditiveExprOperationContext) {
      return new WdlBinaryOperation(
        this.buildAdditiveExpression(ctx.additiveExpression()),
        ctx.PLUS() ? WdlBinaryOperator.ADD : WdlBinaryOperator.SUBTRACT,
        this.buildMultiplicativeExpression(ctx.multiplicativeExpression()),
      );
    }
    return this.buildMultiplicativeExpression(ctx.multiplicativeExpression());
  }

  private static buildMultiplicativeExpression(ctx: any): WdlExpression {
    if (ctx instanceof MultiplicativeExprOperationContext) {
      let op: WdlBinaryOperator;
      if (ctx.ASTERISK()) {
        op = WdlBinaryOperator.MULTIPLY;
      } else if (ctx.SLASH()) {
        op = WdlBinaryOperator.DIVIDE;
      } else if (ctx.PERCENT()) {
        op = WdlBinaryOperator.MODULO;
      } else {
        throw new AssertionError('Unknown multiplicative operator');
      }
      return new WdlBinaryOperation(
        this.buildMultiplicativeExpression(ctx.multiplicativeExpression()),
        op,
        this.buildPowerExpression(ctx.powerExpression()),
      );
    }
    return this.buildPowerExpression(ctx.powerExpression());
  }

  private static buildPowerExpression(ctx: any): WdlExpression {
    if (ctx instanceof PowerExprOperationContext) {
      if (!ctx.EXPONENTIATION()) {
        throw new AssertionError('Unkown power operator');
      }
      return new WdlBinaryOperation(
        this.buildPowerExpression(ctx.powerExpression()),
        WdlBinaryOperator.POWER,
        this.buildUnaryExpression(ctx.unaryExpression()),
      );
    }
    return this.buildUnaryExpression(ctx.unaryExpression());
  }

  private static buildUnaryExpression(ctx: any): WdlExpression {
    if (ctx instanceof UnaryExprOperationContext) {
      let op: WdlUnaryOperator;
      if (ctx.MINUS()) {
        op = WdlUnaryOperator.NEGATIVE;
      } else if (ctx.EXCLAMATION()) {
        op = WdlUnaryOperator.NOT;
      } else {
        throw new AssertionError('Unknown unary operation');
      }
      return new WdlUnaryOperation(op, this.buildUnaryExpression(ctx.unaryExpression()));
    }
    return this.buildPostfixExpression(ctx.postfixExpression());
  }

  private static buildPostfixExpression(ctx: any): WdlExpression {
    if (ctx instanceof PostfixExprArrayIndexContext) {
      return new WdlIndexAccessOperation(
        this.buildPostfixExpression(ctx.postfixExpression()),
        this.buildExpression(ctx.expression()),
      );
    }
    if (ctx instanceof PostfixExprFieldContext) {
      return new WdlMemberAccessOperation(
        this.buildPostfixExpression(ctx.postfixExpression()),
        this.strictIdentifierText(ctx.strictIdentifier()),
      );
    }
    return this.buildPrimaryExpression(ctx.primaryExpression());
  }

  private static buildPrimaryExpression(ctx: any): WdlExpression {
    if (ctx.variable())
      return new WdlVariable(this.strictIdentifierText(ctx.variable().strictIdentifier()));
    if (ctx.noneLiteral() || ctx.nullLiteral?.()) return new WdlNullLiteral(null);
    if (ctx.booleanLiteral())
      return new WdlBooleanLiteral(ctx.booleanLiteral().KEYWORD_TRUE() !== null);
    if (ctx.numberLiteral()) return this.buildNumberLiteral(ctx.numberLiteral());
    if (ctx.stringLiteral()) return this.buildStringLiteral(ctx.stringLiteral()!);
    if (ctx.arrayLiteral()) return this.buildArrayLiteral(ctx.arrayLiteral());
    if (ctx.mapLiteral()) return this.buildMapLiteral(ctx.mapLiteral());
    if (ctx.objectLiteral()) return this.buildObjectLiteral(ctx.objectLiteral());
    if (ctx.structLiteral()) return this.buildStructLiteral(ctx.structLiteral());
    if (ctx.pairLiteral()) return this.buildPairLiteral(ctx.pairLiteral());
    if (ctx.groupedExpression()) return this.buildExpression(ctx.groupedExpression().expression());
    if (ctx.ifExpression()) return this.buildIfExpression(ctx.ifExpression());
    return this.buildCallExpression(ctx.callExpression());
  }

  private static buildNumberLiteral(ctx: any): WdlExpression {
    if (ctx instanceof NumberLiteralIntContext)
      return new WdlIntLiteral(Number(ctx.INTEGER().getText()));
    if (ctx instanceof NumberLiteralFloatContext)
      return new WdlFloatLiteral(Number(ctx.FLOAT().getText()));
    const inner = ctx.numberLiteral?.();
    if (inner) {
      const value = this.buildNumberLiteral(inner);
      if (ctx.MINUS()) {
        if (value instanceof WdlIntLiteral)
          return new WdlIntLiteral(-Number(value.getValue() ?? 0));
        if (value instanceof WdlFloatLiteral)
          return new WdlFloatLiteral(-Number(value.getValue() ?? 0));
      }
      return value;
    }
    return new WdlIntLiteral(0);
  }

  private static buildArrayLiteral(ctx: ArrayLiteralContext): WdlArrayLiteral {
    const value = new WdlArrayLiteral();
    for (const entry of ctx.expression()) value.entries().push(this.buildExpression(entry));
    return value;
  }

  private static buildMapLiteral(ctx: MapLiteralContext): WdlMapLiteral {
    const value = new WdlMapLiteral();
    for (const item of ctx.mapLiteralItem()) {
      value
        .entries()
        .push(
          new WdlMapEntry(
            this.buildExpression(item.expression(0)!),
            this.buildExpression(item.expression(1)!),
          ),
        );
    }
    return value;
  }

  private static buildObjectLiteral(ctx: ObjectLiteralContext): WdlObjectLiteral {
    const value = new WdlObjectLiteral();
    for (const item of ctx.objectLiteralItem()) {
      value
        .entries()
        .push(
          new WdlObjectEntry(
            this.strictIdentifierText(item.strictIdentifier()),
            this.buildExpression(item.expression()),
          ),
        );
    }
    return value;
  }

  private static buildStructLiteral(ctx: StructLiteralContext): WdlStructLiteral {
    const value = new WdlStructLiteral(this.strictIdentifierText(ctx.strictIdentifier()));
    for (const item of ctx.structLiteralItem()) {
      value
        .entries()
        .push(
          new WdlStructEntry(
            this.strictIdentifierText(item.strictIdentifier()),
            this.buildExpression(item.expression()),
          ),
        );
    }
    return value;
  }

  private static buildPairLiteral(ctx: PairLiteralContext): WdlPairLiteral {
    return new WdlPairLiteral(
      this.buildExpression(ctx.expression(0)!),
      this.buildExpression(ctx.expression(1)!),
    );
  }

  private static buildIfExpression(ctx: IfExpressionContext): WdlTernaryOperation {
    return new WdlTernaryOperation(
      this.buildExpression(ctx.expression(0)!),
      this.buildExpression(ctx.expression(1)!),
      this.buildExpression(ctx.expression(2)!),
    );
  }

  private static buildCallExpression(ctx: CallExpressionContext): WdlFunctionCallOperation {
    const value = new WdlFunctionCallOperation(this.strictIdentifierText(ctx.strictIdentifier()));
    for (const arg of ctx.expression()) value.arguments().push(this.buildExpression(arg));
    return value;
  }

  private static buildStringLiteral(ctx: StringLiteralContext): WdlStringLiteral {
    const quoted = ctx.quotedString();
    if (quoted) return this.buildQuotedString(quoted);
    return this.buildMultilineString(ctx.multilineString()!);
  }

  private static buildQuotedString(ctx: QuotedStringContext): WdlStringLiteral {
    const value = new WdlStringLiteral(Delimiter.SINGLE_QUOTED);
    for (const element of ctx.stringElement()) {
      if (element instanceof StringElementTextContext)
        value.components().push(new WdlStringText(element.STRING_TEXT().getText()));
      else if (element instanceof StringElementEscapeContext)
        value.components().push(new WdlStringEscape(element.STRING_ESCAPE().getText()));
      else if (element instanceof StringElementDollarSignContext)
        value.components().push(new WdlStringToken(element.STRING_DOLLAR_SIGN().getText()));
      else if (element instanceof StringElementTildeContext)
        value.components().push(new WdlStringToken(element.STRING_TILDE().getText()));
      else if (element instanceof StringElementPlaceholderContext)
        value.components().push(this.buildStringPlaceholder(element.stringPlaceholder()));
    }
    return value;
  }

  private static buildMultilineString(
    ctx: MultilineStringContext | MultilineStringCommandContext,
  ): WdlStringLiteral {
    const value = new WdlStringLiteral(Delimiter.DOUBLE_ANGLE);
    for (const element of ctx.multilineStringElement()) {
      if (element instanceof MultilineStringElementTextContext)
        value.components().push(new WdlStringText(element.MULTILINE_STRING_TEXT().getText()));
      else if (element instanceof MultilineStringElementEscapeContext)
        value.components().push(new WdlStringEscape(element.MULTILINE_STRING_ESCAPE().getText()));
      else if (element instanceof MultilineStringElementDollarSignContext)
        value
          .components()
          .push(new WdlStringToken(element.MULTILINE_STRING_DOLLAR_SIGN().getText()));
      else if (element instanceof MultilineStringElementTildeContext)
        value.components().push(new WdlStringToken(element.MULTILINE_STRING_TILDE().getText()));
      else if (element instanceof MultilineStringElementDoubleCloseAngleContext)
        value
          .components()
          .push(new WdlStringToken(element.MULTILINE_STRING_DOUBLE_CLOSE_ANGLE().getText()));
      else if (element instanceof MultilineStringElementSingleCloseAngleContext)
        value
          .components()
          .push(new WdlStringToken(element.MULTILINE_STRING_SINGLE_CLOSE_ANGLE().getText()));
      else if (element instanceof MultilineStringElementPlaceholderContext)
        value
          .components()
          .push(this.buildMultilinePlaceholder(element.multilineStringPlaceholder()));
    }
    return value;
  }

  private static buildBracedCommand(ctx: BracedCommandContext): WdlStringLiteral {
    const value = new WdlStringLiteral(Delimiter.SINGLE_QUOTED);
    for (const element of ctx.stringElement()) {
      if (element instanceof StringElementTextContext)
        value.components().push(new WdlStringText(element.STRING_TEXT().getText()));
      else if (element instanceof StringElementEscapeContext)
        value.components().push(new WdlStringEscape(element.STRING_ESCAPE().getText()));
      else if (element instanceof StringElementDollarSignContext)
        value.components().push(new WdlStringToken(element.STRING_DOLLAR_SIGN().getText()));
      else if (element instanceof StringElementTildeContext)
        value.components().push(new WdlStringToken(element.STRING_TILDE().getText()));
      else if (element instanceof StringElementPlaceholderContext)
        value.components().push(this.buildStringPlaceholder(element.stringPlaceholder()));
    }
    return value;
  }

  private static buildStringPlaceholder(ctx: StringPlaceholderContext): WdlStringPlaceholder {
    const startText = ctx.STRING_PLACEHOLDER_START().getText();
    let symbol: PlaceHolderSymbol;
    if (startText === '~{') {
      symbol = PlaceHolderSymbol.TILDE;
    } else if (startText === '${') {
      symbol = PlaceHolderSymbol.DOLLAR;
    } else {
      throw new AssertionError(`Unknown PlaceHolderSymbol ${startText}`);
    }
    return new WdlStringPlaceholder(
      this.buildPlaceholderOption(ctx.stringPlaceholderExpression()),
      this.buildExpression(ctx.stringPlaceholderExpression().expression()),
      symbol,
    );
  }

  private static buildMultilinePlaceholder(
    ctx: MultilineStringPlaceholderContext,
  ): WdlStringPlaceholder {
    const tildeToken = ctx.MULTILINE_STRING_TILDE_PLACEHOLDER_START();
    const dollarToken = ctx.MULTILINE_STRING_DOLLAR_PLACEHOLDER_START();
    let symbol: PlaceHolderSymbol;
    if (tildeToken) {
      symbol = PlaceHolderSymbol.TILDE;
    } else if (dollarToken) {
      symbol = PlaceHolderSymbol.DOLLAR;
    } else {
      throw new AssertionError('Unknown multiline placeholder symbol');
    }
    return new WdlStringPlaceholder(
      this.buildPlaceholderOption(ctx.stringPlaceholderExpression()),
      this.buildExpression(ctx.stringPlaceholderExpression().expression()),
      symbol,
    );
  }

  private static buildPlaceholderOption(
    ctx: StringPlaceholderExpressionContext,
  ): WdlStringPlaceholderOption | undefined {
    const option = ctx.stringPlaceholderOption(0);
    if (!option) return undefined;
    if (option instanceof StringPlaceholderOptionSepDefaultContext) {
      const identifier = option.IDENTIFIER().getText();
      if (identifier !== 'sep' && identifier !== 'default') {
        throw new TypeError('Unsupported placeholder option');
      }
      return new WdlStringPlaceholderOption(
        WdlStringPlaceholderOptionType.DEFAULT,
        this.buildStringLiteral(option.stringLiteral()),
      );
    }
    if (option instanceof StringPlaceholderOptionTrueFalseContext) {
      return new WdlStringPlaceholderOption(
        WdlStringPlaceholderOptionType.TRUE_FALSE,
        undefined,
        this.buildStringLiteral(option.stringLiteral(0)!),
        this.buildStringLiteral(option.stringLiteral(1)!),
      );
    }
    return new WdlStringPlaceholderOption(
      WdlStringPlaceholderOptionType.TRUE_FALSE,
      undefined,
      this.buildStringLiteral(
        (option as StringPlaceholderOptionFalseTrueContext).stringLiteral(1)!,
      ),
      this.buildStringLiteral(
        (option as StringPlaceholderOptionFalseTrueContext).stringLiteral(0)!,
      ),
    );
  }

  private static buildMetadataValue(ctx: MetadataValueContext): WdlExpression {
    if (ctx.stringLiteral()) return this.buildStringLiteral(ctx.stringLiteral()!);
    if (ctx.booleanLiteral())
      return new WdlBooleanLiteral(ctx.booleanLiteral()!.KEYWORD_TRUE() !== null);
    if (ctx.nullLiteral()) return new WdlNullLiteral(null);
    if (ctx.numberLiteralSigned()) return this.buildNumberLiteralSigned(ctx.numberLiteralSigned()!);
    if (ctx.metadataArray()) return this.buildMetadataArray(ctx.metadataArray()!);
    return this.buildMetadataObject(ctx.metadataObject()!);
  }

  private static buildNumberLiteralSigned(ctx: NumberLiteralSignedContext): WdlExpression {
    const inner = this.buildNumberLiteral(ctx.numberLiteral());
    if (ctx.MINUS()) {
      if (inner instanceof WdlIntLiteral) return new WdlIntLiteral(-Number(inner.getValue() ?? 0));
      if (inner instanceof WdlFloatLiteral)
        return new WdlFloatLiteral(-Number(inner.getValue() ?? 0));
    }
    return inner;
  }

  private static buildMetadataArray(ctx: MetadataArrayContext): WdlArrayLiteral {
    const value = new WdlArrayLiteral();
    for (const item of ctx.metadataValue()) value.entries().push(this.buildMetadataValue(item));
    return value;
  }

  private static buildMetadataObject(ctx: MetadataObjectContext): WdlObjectLiteral {
    const value = new WdlObjectLiteral();
    for (const item of ctx.metadataObjectItem()) {
      value
        .entries()
        .push(
          new WdlObjectEntry(
            this.dottedIdentifierText(item.dottedIdentifier()),
            this.buildMetadataValue(item.metadataValue()),
          ),
        );
    }
    return value;
  }

  private static buildTaskHintValue(ctx: HintsItemTaskContext): WdlExpression {
    const valueCtx = ctx.hintsValueTask();
    if (valueCtx instanceof TaskHintValueExpressionContext) {
      return this.buildExpression(valueCtx.expression());
    }
    if (valueCtx instanceof TaskHintValueArrayContext) {
      const array = new WdlArrayLiteral();
      for (const item of valueCtx.taskHintsArray().hintsValueTask()) {
        array.entries().push(this.buildTaskHintValueFromValue(item));
      }
      return array;
    }
    return new WdlVariable(valueCtx.getText());
  }

  private static buildWorkflowHintValue(ctx: HintsItemWorkflowContext): WdlExpression {
    const valueCtx = ctx.hintsValueWorkflow();
    if (valueCtx instanceof WorkflowHintValueNumberContext)
      return this.buildNumberLiteralSigned(valueCtx.numberLiteralSigned());
    if (valueCtx instanceof WorkflowHintValueStringContext)
      return this.buildStringLiteral(valueCtx.stringLiteral());
    if (valueCtx instanceof WorkflowHintValueBooleanContext)
      return new WdlBooleanLiteral(valueCtx.booleanLiteral().KEYWORD_TRUE() !== null);
    if (valueCtx instanceof WorkflowHintValueArrayContext) {
      const array = new WdlArrayLiteral();
      for (const item of valueCtx.workflowHintsArray().hintsValueWorkflow()) {
        array.entries().push(this.buildWorkflowHintValueFromValue(item));
      }
      return array;
    }
    return new WdlVariable(valueCtx.getText());
  }

  private static buildTaskHintValueFromValue(valueCtx: HintsValueTaskContext): WdlExpression {
    if (valueCtx instanceof TaskHintValueExpressionContext)
      return this.buildExpression(valueCtx.expression());
    if (valueCtx instanceof TaskHintValueArrayContext) {
      const array = new WdlArrayLiteral();
      for (const item of valueCtx.taskHintsArray().hintsValueTask())
        array.entries().push(this.buildTaskHintValueFromValue(item));
      return array;
    }
    return new WdlVariable(valueCtx.getText());
  }

  private static buildWorkflowHintValueFromValue(
    valueCtx: HintsValueWorkflowContext,
  ): WdlExpression {
    if (valueCtx instanceof WorkflowHintValueNumberContext)
      return this.buildNumberLiteralSigned(valueCtx.numberLiteralSigned());
    if (valueCtx instanceof WorkflowHintValueStringContext)
      return this.buildStringLiteral(valueCtx.stringLiteral());
    if (valueCtx instanceof WorkflowHintValueBooleanContext)
      return new WdlBooleanLiteral(valueCtx.booleanLiteral().KEYWORD_TRUE() !== null);
    if (valueCtx instanceof WorkflowHintValueArrayContext) {
      const array = new WdlArrayLiteral();
      for (const item of valueCtx.workflowHintsArray().hintsValueWorkflow())
        array.entries().push(this.buildWorkflowHintValueFromValue(item));
      return array;
    }
    return new WdlVariable(valueCtx.getText());
  }

  private static strictIdentifierText(ctx: StrictIdentifierContext): string {
    return ctx.getText();
  }

  private static dottedIdentifierText(ctx: { getText(): string }): string {
    return ctx.getText();
  }
}
