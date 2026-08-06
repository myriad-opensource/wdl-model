package com.myriad.wdl.model;

import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlEnum.WdlEnumChoice;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlStruct.WdlStructMember;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.errors.WdlSyntaxError;
import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlBinaryOperation.Operator;
import com.myriad.wdl.model.expressions.WdlBooleanLiteral;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFloatLiteral;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlIndexAccessOperation;
import com.myriad.wdl.model.expressions.WdlIntLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral;
import com.myriad.wdl.model.expressions.WdlMapLiteral.WdlMapEntry;
import com.myriad.wdl.model.expressions.WdlMemberAccessOperation;
import com.myriad.wdl.model.expressions.WdlNullLiteral;
import com.myriad.wdl.model.expressions.WdlNumberLiteral;
import com.myriad.wdl.model.expressions.WdlObjectLiteral;
import com.myriad.wdl.model.expressions.WdlObjectLiteral.WdlObjectEntry;
import com.myriad.wdl.model.expressions.WdlPairLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral.Delimiter;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringComponent;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringEscape;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder.PlaceHolderSymbol;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholderOption;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholderOption.Type;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringText;
import com.myriad.wdl.model.expressions.WdlStructLiteral;
import com.myriad.wdl.model.expressions.WdlStructLiteral.WdlStructEntry;
import com.myriad.wdl.model.expressions.WdlTernaryOperation;
import com.myriad.wdl.model.expressions.WdlUnaryOperation;
import com.myriad.wdl.model.expressions.WdlVariable;
import com.myriad.wdl.model.resolvers.WdlImportResolverApacheHttp;
import com.myriad.wdl.model.resolvers.WdlImportResolverBase;
import com.myriad.wdl.model.sections.WdlCommand;
import com.myriad.wdl.model.sections.WdlHints.WdlTaskHint;
import com.myriad.wdl.model.sections.WdlHints.WdlTaskHints;
import com.myriad.wdl.model.sections.WdlHints.WdlWorkflowHint;
import com.myriad.wdl.model.sections.WdlHints.WdlWorkflowHints;
import com.myriad.wdl.model.sections.WdlInput;
import com.myriad.wdl.model.sections.WdlMetadataBase.WdlMetadata;
import com.myriad.wdl.model.sections.WdlMetadataBase.WdlMetadataEntry;
import com.myriad.wdl.model.sections.WdlMetadataBase.WdlParameterMetadata;
import com.myriad.wdl.model.sections.WdlOutput;
import com.myriad.wdl.model.sections.WdlRequirements;
import com.myriad.wdl.model.sections.WdlRequirements.WdlRequirementEntry;
import com.myriad.wdl.model.sections.WdlRuntime;
import com.myriad.wdl.model.sections.WdlRuntime.WdlRuntimeEntry;
import com.myriad.wdl.model.statements.WdlCall;
import com.myriad.wdl.model.statements.WdlCall.WdlCallInput;
import com.myriad.wdl.model.statements.WdlConditional;
import com.myriad.wdl.model.statements.WdlConditional.WdlConditionalElseIf;
import com.myriad.wdl.model.statements.WdlDeclaration;
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import com.myriad.wdl.model.statements.WdlImport;
import com.myriad.wdl.model.statements.WdlImport.WdlImportMember;
import com.myriad.wdl.model.statements.WdlImport.WdlImportMembers;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStandard;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStar;
import com.myriad.wdl.model.statements.WdlScatter;
import com.myriad.wdl.model.statements.WdlStatement;
import com.myriad.wdl.model.types.WdlArrayType;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPairType;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import com.myriad.wdl.model.types.WdlTypeReferenceType;
import com.myriad.wdl.model.v1.grammar.WdlV1Lexer;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.AdditiveExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ArrayLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ArrayTypeContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.BooleanLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.BoundDeclarationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.BracedCommandContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CallAfterClauseContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CallAliasContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CallExpressionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CallInputBlockContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CallInputItemContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CallStatementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CallTargetContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.CommandSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ComparisonExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ConditionalElseClauseContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ConditionalElseIfClauseContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ConditionalStatementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.DocumentContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumDefinitionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.EqualityExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.HintsItemTaskContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.HintsItemWorkflowContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.HintsSectionTaskContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.HintsSectionWorkflowContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.IfExpressionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportAliasContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportMemberContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportMembersContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportStatementMembersContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportStatementStandardContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportStatementStarContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportUriElementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ImportUriLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.InputSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.LogicalAndExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.LogicalOrExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.MapLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.MapTypeContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.MetadataArrayContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.MetadataObjectContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.MultilineStringCommandContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.MultiplicativeExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.NoneLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.NullLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.NumberLiteralFloatContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.NumberLiteralIntContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.NumberLiteralSignedContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ObjectLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ObjectLiteralItemContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.OutputSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.PairLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.PairTypeContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.PostfixExprArrayIndexContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.PostfixExprFieldContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.PowerExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.PrimitiveTypeContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.RequirementsItemContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.RequirementsSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.RuntimeItemContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.RuntimeSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ScatterBodyContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.ScatterStatementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.StructDefinitionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.StructItemMemberDeclarationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.StructItemMetadataContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.StructItemParameterMetadataContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.StructLiteralContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.StructLiteralItemContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskCommandSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskDeclarationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskDefinitionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskHintsSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskInputSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskMetadataSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskOutputSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskParameterMetadataSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskRequirementsSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskRuntimeSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.TypeRefTypeContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.UnaryExprOperationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.UnboundDeclarationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.VersionStatementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowCallStatementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowConditionalStatementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowDeclarationContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowDefinitionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowHintsSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowInputSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowMetadataSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowOutputSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowParameterMetadataSectionContext;
import com.myriad.wdl.model.v1.grammar.WdlV1Parser.WorkflowScatterStatementContext;
import com.myriad.wdl.model.v1.grammar.WdlV1ParserBaseVisitor;
import com.myriad.wdl.model.validators.WdlValidator;
import java.io.File;
import java.io.IOException;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import org.antlr.v4.runtime.BaseErrorListener;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.CommonTokenFactory;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.RecognitionException;
import org.antlr.v4.runtime.Recognizer;

/**
 * Parser and model builder for WDL 1.x documents.
 *
 * <p>This class turns ANTLR parse trees into the library's Java object model. Validation is
 * optional: callers may load a document only, or load and validate in one step by supplying a
 * {@link WdlValidator}. The WDL 1.3 specification describes WDL as a "Workflow Description
 * Language" whose documents contain tasks, workflows, expressions, and user-defined types; this
 * loader is the entry point that materializes those concepts as Java objects.
 */
public class WdlV1Loader extends WdlV1ParserBaseVisitor<Void> {

  /** Parses a character stream into a {@link WdlDocument} without semantic validation. */
  public static WdlDocument load(CharStream input) throws WdlException {
    return load(input, null);
  }

  /** Parses a character stream into a {@link WdlDocument} and optionally validates the result. */
  public static WdlDocument load(CharStream input, WdlValidator validator) throws WdlException {
    return load(input, validator, null, null);
  }

  /**
   * Parses a character stream into a {@link WdlDocument}, optionally validates, and resolves
   * import source text using the provided resolver.
   */
  public static WdlDocument load(
      CharStream input,
      WdlValidator validator,
      WdlImportResolverBase importResolver,
      URI currentDocumentLocation)
      throws WdlException {
    WdlDocument document = parseDocument(input, currentDocumentLocation);

    if (importResolver != null) {
      resolveImportsRecursive(
          document, importResolver, new HashMap<>(), new ArrayDeque<>(), new HashSet<>());
    }
    if (validator != null) {
      validator.validate(document);
    }
    return document;
  }

  /** Parse a single WDL document from source text and attach the source location metadata. */
  private static WdlDocument parseDocument(CharStream input, URI currentDocumentLocation)
      throws WdlException {
    WdlV1Lexer lexer = new WdlV1Lexer(input);
    lexer.setTokenFactory(new CommonTokenFactory(true));
    WdlV1Parser parser = new WdlV1Parser(new CommonTokenStream(lexer));

    WdlErrorListener errorListener = new WdlErrorListener();
    lexer.removeErrorListeners();
    lexer.addErrorListener(errorListener);
    parser.removeErrorListeners();
    parser.addErrorListener(errorListener);

    WdlV1Parser.DocumentContext documentContext = parser.document();
    errorListener.throwIfErrored();

    WdlV1Loader builder = new WdlV1Loader();
    builder.visitDocument(documentContext);

    WdlDocument document = builder.getDocument();
    document.setSourceLocation(currentDocumentLocation);
    return document;
  }

  /** Parses a source string into a {@link WdlDocument} without semantic validation. */
  public static WdlDocument load(String sourceCode) throws WdlException {
    return load(sourceCode, (WdlValidator) null);
  }

  /** Parses a source string into a {@link WdlDocument} and optionally validates the result. */
  public static WdlDocument load(String sourceCode, WdlValidator validator) throws WdlException {
    return load(sourceCode, null, null, validator);
  }

  /**
   * Parses a source string with a known source location and resolves imports recursively using the
   * default resolver.
   */
  public static WdlDocument load(String sourceCode, URI sourceLocation) throws WdlException {
    return load(sourceCode, sourceLocation, null, null);
  }

  /**
   * Parses a source string with a known source location, optionally validates, and resolves
   * imports recursively using the default resolver.
   */
  public static WdlDocument load(String sourceCode, URI sourceLocation, WdlValidator validator)
      throws WdlException {
    return load(sourceCode, sourceLocation, null, validator);
  }

  /**
   * Parses a source string with a known source location and resolves imports recursively using the
   * provided resolver.
   */
  public static WdlDocument load(
      String sourceCode, URI sourceLocation, WdlImportResolverBase importResolver)
      throws WdlException {
    return load(sourceCode, sourceLocation, importResolver, null);
  }

  /**
   * Parses a source string into a {@link WdlDocument}, optionally validates, and resolves imports.
   */
  public static WdlDocument load(
      String sourceCode,
      URI sourceLocation,
      WdlImportResolverBase importResolver,
      WdlValidator validator)
      throws WdlException {
    org.antlr.v4.runtime.CharStream input = org.antlr.v4.runtime.CharStreams.fromString(sourceCode);
    WdlImportResolverBase resolver =
        sourceLocation != null
            ? (importResolver != null ? importResolver : new WdlImportResolverApacheHttp())
            : importResolver;
    return load(input, validator, resolver, sourceLocation);
  }

  /** Parses a UTF-8 file into a {@link WdlDocument} without semantic validation. */
  public static WdlDocument load(File file) throws WdlException, IOException {
    return load(file, null);
  }

  /** Parses a UTF-8 file into a {@link WdlDocument} and optionally validates the result. */
  public static WdlDocument load(File file, WdlValidator validator)
      throws WdlException, IOException {
    return load(file, validator, new WdlImportResolverApacheHttp());
  }

  /**
   * Parses a UTF-8 file into a {@link WdlDocument}, optionally validates, and resolves imports.
   */
  public static WdlDocument load(
      File file, WdlValidator validator, WdlImportResolverBase importResolver)
      throws WdlException, IOException {
    org.antlr.v4.runtime.CharStream input =
        org.antlr.v4.runtime.CharStreams.fromFileName(
            file.getAbsolutePath(), StandardCharsets.UTF_8);
    WdlImportResolverBase resolver =
        importResolver != null ? importResolver : new WdlImportResolverApacheHttp();
    return load(input, validator, resolver, file.toURI());
  }

  /**
   * Resolve imports depth-first and populate imported document maps on each visited document.
   */
  private static void resolveImportsRecursive(
      WdlDocument document,
      WdlImportResolverBase importResolver,
      Map<String, WdlDocument> loadedById,
      ArrayDeque<String> activeImportStack,
      Set<String> activeImportSet)
      throws WdlException {
    URI currentSourceLocation = document.getSourceLocation();
    String currentDocumentIdentifier =
        currentSourceLocation != null ? currentSourceLocation.toString() : null;
    if (currentDocumentIdentifier != null) {
      activeImportStack.addLast(currentDocumentIdentifier);
      activeImportSet.add(currentDocumentIdentifier);
      loadedById.putIfAbsent(currentDocumentIdentifier, document);
    }

    try {
      document.importedDocuments().clear();
      URI currentLocation = document.getSourceLocation();
      for (WdlImport imp : document.importStatements()) {
        WdlStringLiteral sourceLiteral = imp.getSource();
        if (sourceLiteral == null) {
          continue;
        }

        String importReference = extractStringLiteralText(sourceLiteral);
        URI resolvedImportLocation =
            importResolver.resolveImportLocation(currentLocation, importReference);
        String importIdentifier =
            resolvedImportLocation != null ? resolvedImportLocation.toString() : importReference;
        imp.setImportIdentifier(importIdentifier);

        if (activeImportSet.contains(importIdentifier)) {
          throw circularImportException(activeImportStack, importIdentifier);
        }

        String importSourceText = importResolver.resolveImport(currentLocation, importReference);
        imp.setSourceText(importSourceText);

        WdlDocument importedDocument = loadedById.get(importIdentifier);
        if (importedDocument == null) {
          importedDocument =
              parseDocument(
                  org.antlr.v4.runtime.CharStreams.fromString(importSourceText),
                  resolvedImportLocation);
          loadedById.put(importIdentifier, importedDocument);
          resolveImportsRecursive(
              importedDocument, importResolver, loadedById, activeImportStack, activeImportSet);
        }

        document.importedDocuments().put(importIdentifier, importedDocument);
      }
    } finally {
      if (currentDocumentIdentifier != null
          && currentDocumentIdentifier.equals(activeImportStack.peekLast())) {
        activeImportStack.removeLast();
        activeImportSet.remove(currentDocumentIdentifier);
      }
    }
  }

  private static WdlException circularImportException(
      ArrayDeque<String> activeImportStack, String importIdentifier) {
    List<String> cyclePath = new ArrayList<>(activeImportStack);
    cyclePath.add(importIdentifier);
    return new com.myriad.wdl.model.errors.WdlImportException(
        "Circular import detected: " + String.join(" -> ", cyclePath), importIdentifier);
  }

  /**
   * Reconstruct raw text from an import source literal while preserving escape token spelling.
   */
  private static String extractStringLiteralText(WdlStringLiteral sourceLiteral) {
    StringBuilder text = new StringBuilder();
    for (WdlStringComponent component : sourceLiteral.components()) {
      if (component instanceof WdlStringText) {
        text.append(((WdlStringText) component).getText());
      } else if (component instanceof WdlStringEscape) {
        text.append(((WdlStringEscape) component).getEscapeText());
      } else {
        throw new AssertionError("Unsupported import URI element");
      }
    }
    return text.toString();
  }

  /** Collects syntax errors from the lexer and parser before the model is built. */
  protected static class WdlErrorListener extends BaseErrorListener {
    private List<WdlSyntaxError> syntaxErrors = new ArrayList<>();

    @Override
    public void syntaxError(
        Recognizer<?, ?> recognizer,
        Object offendingSymbol,
        int line,
        int charPositionInLine,
        String msg,
        RecognitionException e) {
      syntaxErrors.add(new WdlSyntaxError(msg, line, charPositionInLine, e));
    }

    protected void throwIfErrored() throws WdlException {
      if (!syntaxErrors.isEmpty()) {
        throw new WdlException(syntaxErrors);
      }
    }
  }

  private ArrayDeque<WdlNode> stack = new ArrayDeque<>();

  /** Returns the finished root document after a successful visitor traversal. */
  public WdlDocument getDocument() {
    if (stack.isEmpty()) {
      // This should never happen
      throw new AssertionError("Stack is empty");
    }
    if (stack.size() != 1 && !(stack.peek() instanceof WdlDocument)) {
      // This should never happen
      throw new AssertionError("Stack does not contain exactly one WdlDocument");
    }
    return popWithType(WdlDocument.class);
  }

  private <T extends WdlNode> T popWithType(Class<T> expectedType) {
    try {
      return expectedType.cast(stack.pop());
    } catch (ClassCastException e) {
      throw new AssertionError(
          "Expected "
              + expectedType.getName()
              + " on stack not "
              + stack.peek().getClass().getName());
    }
  }

  private <T extends WdlNode> T peekWithType(Class<T> expectedType) {
    try {
      return expectedType.cast(stack.peek());
    } catch (ClassCastException e) {
      throw new AssertionError(
          "Expected "
              + expectedType.getName()
              + " on stack not "
              + stack.peek().getClass().getName());
    }
  }

  @SuppressWarnings("unchecked")
  private <T extends WdlNode> T findWithType(Class<T> expectedType) {
    WdlNode found =
        stack.stream()
            .filter(e -> expectedType.isInstance(e))
            .findFirst()
            .orElseThrow(() -> new IllegalStateException());
    return (T) found;
  }

  // =========================================================================
  // Document & Version
  // =========================================================================

  @Override
  public Void visitDocument(DocumentContext ctx) {
    stack.push(new WdlDocument());
    super.visitDocument(ctx);
    return null;
  }

  @Override
  public Void visitVersionStatement(VersionStatementContext ctx) {
    WdlDocument doc = peekWithType(WdlDocument.class);
    String version = ctx.FLOAT().getText();
    doc.setWdlVersion(WdlVersion.fromString(version));
    return null;
  }

  // =========================================================================
  // Imports
  // =========================================================================

  @Override
  public Void visitImportStatementStandard(ImportStatementStandardContext ctx) {
    WdlImportStandard imp = new WdlImportStandard();
    stack.push(imp);
    super.visitImportStatementStandard(ctx);
    while (stack.peek() instanceof WdlImportMember) {
      imp.members().push(popWithType(WdlImportMember.class));
    }
    if (ctx.KEYWORD_AS() != null) {
      imp.setAlias(ctx.strictIdentifier().getText());
    }
    imp.setSource(popWithType(WdlStringLiteral.class));
    stack.pop();
    peekWithType(WdlDocument.class).elements().add(imp);
    return null;
  }

  @Override
  public Void visitImportStatementStar(ImportStatementStarContext ctx) {
    WdlImportStar imp = new WdlImportStar();
    stack.push(imp);
    super.visitImportStatementStar(ctx);
    imp.setSource(popWithType(WdlStringLiteral.class));
    stack.pop();
    peekWithType(WdlDocument.class).elements().add(imp);
    return null;
  }

  @Override
  public Void visitImportStatementMembers(ImportStatementMembersContext ctx) {
    WdlImportMembers imp = new WdlImportMembers();
    stack.push(imp);
    super.visitImportStatementMembers(ctx);
    imp.setSource(popWithType(WdlStringLiteral.class));
    stack.pop();
    peekWithType(WdlDocument.class).elements().add(imp);
    return null;
  }

  @Override
  public Void visitImportMembers(ImportMembersContext ctx) {
    WdlImportMembers imp = peekWithType(WdlImportMembers.class);
    super.visitImportMembers(ctx);
    while (stack.peek() != imp) {
      imp.members().push(popWithType(WdlImportMember.class));
    }
    return null;
  }

  @Override
  public Void visitImportMember(ImportMemberContext ctx) {
    WdlImportMember member = new WdlImportMember();
    stack.push(member);
    super.visitImportMember(ctx);
    if (ctx.KEYWORD_AS() != null) {
      member.setAlias(ctx.strictIdentifier(1).getText());
    }
    member.setMember(ctx.strictIdentifier(0).getText());
    return null;
  }

  @Override
  public Void visitImportUriLiteral(ImportUriLiteralContext ctx) {
    WdlStringLiteral str =
        new WdlStringLiteral(
            ctx.DOUBLE_QUOTE() != null ? Delimiter.DOUBLE_QUOTE : Delimiter.SINGLE_QUOTE);
    stack.push(str);
    super.visitImportUriLiteral(ctx);
    return null;
  }

  @Override
  public Void visitImportUriElement(ImportUriElementContext ctx) {
    WdlStringLiteral str = peekWithType(WdlStringLiteral.class);
    if (ctx.STRING_TEXT() != null) {
      str.components().push(new WdlStringText(ctx.STRING_TEXT().getText()));
    } else if (ctx.STRING_ESCAPE() != null) {
      str.components().push(new WdlStringEscape(ctx.STRING_ESCAPE().getText()));
    } else {
      // This should not happen
      throw new AssertionError("Unsupported import URI element");
    }
    super.visitImportUriElement(ctx);
    return null;
  }

  @Override
  public Void visitImportAlias(ImportAliasContext ctx) {
    WdlImportMember member = new WdlImportMember();
    stack.push(member);
    super.visitImportAlias(ctx);
    if (ctx.KEYWORD_AS() != null) {
      member.setAlias(ctx.strictIdentifier(1).getText());
    }
    member.setMember(ctx.strictIdentifier(0).getText());
    return null;
  }

  // =========================================================================
  // Struct Definitions
  // =========================================================================

  @Override
  public Void visitStructDefinition(StructDefinitionContext ctx) {
    WdlStruct struct = new WdlStruct(ctx.strictIdentifier().getText());
    stack.push(struct);
    super.visitStructDefinition(ctx);
    struct.setName(ctx.strictIdentifier().getText());
    stack.pop();
    peekWithType(WdlDocument.class).elements().add(struct);
    return null;
  }

  @Override
  public Void visitStructItemMemberDeclaration(StructItemMemberDeclarationContext ctx) {
    WdlStruct struct = findWithType(WdlStruct.class);
    WdlStructMember member = new WdlStructMember();
    stack.push(member);
    super.visitStructItemMemberDeclaration(ctx);
    member.setName(ctx.structDeclaration().strictIdentifier().getText());
    WdlType memberType = popWithType(WdlType.class);
    member.setType(memberType);
    stack.pop();
    struct.elements().addLast(member);
    return null;
  }

  @Override
  public Void visitStructItemMetadata(StructItemMetadataContext ctx) {
    WdlStruct struct = findWithType(WdlStruct.class);
    super.visitStructItemMetadata(ctx);
    struct.elements().addLast(popWithType(WdlMetadata.class));
    return null;
  }

  @Override
  public Void visitStructItemParameterMetadata(StructItemParameterMetadataContext ctx) {
    WdlStruct struct = findWithType(WdlStruct.class);
    super.visitStructItemParameterMetadata(ctx);
    struct.elements().addLast(popWithType(WdlParameterMetadata.class));
    return null;
  }

  // =========================================================================
  // Enum Definitions
  // =========================================================================

  @Override
  public Void visitEnumDefinition(EnumDefinitionContext ctx) {
    WdlEnum enumDef = new WdlEnum();
    stack.push(enumDef);
    super.visitEnumDefinition(ctx);
    while (stack.peek() instanceof WdlEnumChoice) {
      enumDef.elements().addFirst(popWithType(WdlEnumChoice.class));
    }
    if (ctx.enumTypeParameter() != null) {
      enumDef.setValueType(popWithType(WdlType.class));
    }
    enumDef.setName(ctx.strictIdentifier().getText());
    popWithType(WdlEnum.class);
    peekWithType(WdlDocument.class).elements().add(enumDef);
    return null;
  }

  @Override
  public Void visitEnumChoice(com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumChoiceContext ctx) {
    WdlEnumChoice choice = new WdlEnumChoice();
    stack.push(choice);
    super.visitEnumChoice(ctx);
    if (ctx.ASSIGNMENT() != null) {
      choice.setValue(popWithType(WdlExpression.class));
    }
    choice.setKey(ctx.strictIdentifier().getText());
    return null;
  }

  // =========================================================================
  // Declarations
  // =========================================================================

  @Override
  public Void visitUnboundDeclaration(UnboundDeclarationContext ctx) {
    WdlDeclaration decl = new WdlDeclaration();
    stack.push(decl);
    super.visitUnboundDeclaration(ctx);
    decl.setName(ctx.strictIdentifier().getText());
    decl.setType(popWithType(WdlType.class));
    decl.setEnvironmentVariable(ctx.KEYWORD_ENV() != null);
    return null;
  }

  @Override
  public Void visitBoundDeclaration(BoundDeclarationContext ctx) {
    WdlBoundDeclaration decl = new WdlBoundDeclaration();
    stack.push(decl);
    super.visitBoundDeclaration(ctx);
    decl.setExpression(popWithType(WdlExpression.class));
    decl.setName(ctx.strictIdentifier().getText());
    decl.setType(popWithType(WdlType.class));
    decl.setEnvironmentVariable(ctx.KEYWORD_ENV() != null);
    return null;
  }

  // =========================================================================
  // Input and Output Sections
  // =========================================================================

  @Override
  public Void visitInputSection(InputSectionContext ctx) {
    WdlInput section = new WdlInput();
    stack.push(section);
    super.visitInputSection(ctx);
    while (stack.peek() != section) {
      section.elements().push(popWithType(WdlDeclaration.class));
    }
    return null;
  }

  @Override
  public Void visitOutputSection(OutputSectionContext ctx) {
    WdlOutput section = new WdlOutput();
    stack.push(section);
    super.visitOutputSection(ctx);
    while (stack.peek() != section) {
      section.elements().push(popWithType(WdlBoundDeclaration.class));
    }
    return null;
  }

  // =========================================================================
  // Task Definitions
  // =========================================================================

  @Override
  public Void visitTaskDefinition(TaskDefinitionContext ctx) {
    WdlTask task = new WdlTask();
    stack.push(task);
    super.visitTaskDefinition(ctx);
    task.setName(ctx.strictIdentifier().getText());
    popWithType(WdlTask.class);
    peekWithType(WdlDocument.class).elements().addLast(task);
    return null;
  }

  @Override
  public Void visitTaskDeclaration(TaskDeclarationContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    super.visitTaskDeclaration(ctx);
    while (stack.peek() instanceof WdlBoundDeclaration) {
      task.elements().addLast(popWithType(WdlBoundDeclaration.class));
    }
    return null;
  }

  @Override
  public Void visitTaskInputSection(TaskInputSectionContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    super.visitTaskInputSection(ctx);
    task.elements().addLast(popWithType(WdlInput.class));
    return null;
  }

  @Override
  public Void visitTaskOutputSection(TaskOutputSectionContext ctx) {
    WdlTask taskDef = findWithType(WdlTask.class);
    super.visitTaskOutputSection(ctx);
    taskDef.elements().addLast(popWithType(WdlOutput.class));
    return null;
  }

  @Override
  public Void visitTaskCommandSection(TaskCommandSectionContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    WdlCommand command = new WdlCommand();
    stack.push(command);
    super.visitTaskCommandSection(ctx);
    stack.pop();
    task.elements().addLast(command);
    return null;
  }

  @Override
  public Void visitCommandSection(CommandSectionContext ctx) {
    WdlCommand command = peekWithType(WdlCommand.class);
    super.visitCommandSection(ctx);
    command.setCommandText(popWithType(WdlStringLiteral.class));
    return null;
  }

  @Override
  public Void visitMultilineStringCommand(MultilineStringCommandContext ctx) {
    findWithType(WdlCommand.class).setMultiline(true);
    WdlStringLiteral cmdStr = new WdlStringLiteral();
    stack.push(cmdStr);
    super.visitMultilineStringCommand(ctx);
    while (stack.peek() != cmdStr) {
      cmdStr.components().push(popWithType(WdlStringComponent.class));
    }
    return null;
  }

  @Override
  public Void visitBracedCommand(BracedCommandContext ctx) {
    findWithType(WdlCommand.class).setMultiline(false);
    WdlStringLiteral cmdStr = new WdlStringLiteral();
    stack.push(cmdStr);
    super.visitBracedCommand(ctx);
    while (stack.peek() != cmdStr) {
      cmdStr.components().push(popWithType(WdlStringComponent.class));
    }
    return null;
  }

  @Override
  public Void visitMetadataSection(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.MetadataSectionContext ctx) {
    WdlMetadata section = new WdlMetadata();
    stack.push(section);
    super.visitMetadataSection(ctx);
    while (stack.peek() != section) {
      section.elements().addLast(popWithType(WdlMetadataEntry.class));
    }
    return null;
  }

  @Override
  public Void visitParameterMetadataSection(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.ParameterMetadataSectionContext ctx) {
    WdlParameterMetadata paramMetaSection = new WdlParameterMetadata();
    stack.push(paramMetaSection);
    super.visitParameterMetadataSection(ctx);
    while (stack.peek() != paramMetaSection) {
      paramMetaSection.elements().push(popWithType(WdlMetadataEntry.class));
    }
    return null;
  }

  @Override
  public Void visitMetadataObject(MetadataObjectContext ctx) {
    WdlObjectLiteral val = new WdlObjectLiteral();
    stack.push(val);
    super.visitMetadataObject(ctx);
    while (stack.peek() != val) {
      val.entries().push(popWithType(WdlObjectEntry.class));
    }
    return null;
  }

  @Override
  public Void visitMetadataObjectItem(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.MetadataObjectItemContext ctx) {
    WdlMetadataEntry entry = new WdlMetadataEntry();
    stack.push(entry);
    super.visitMetadataObjectItem(ctx);
    entry.setValue(popWithType(WdlExpression.class));
    entry.setKey(ctx.dottedIdentifier().getText());
    return null;
  }

  @Override
  public Void visitMetadataArray(MetadataArrayContext ctx) {
    WdlArrayLiteral val = new WdlArrayLiteral();
    stack.push(val);
    super.visitMetadataArray(ctx);
    while (stack.peek() != val) {
      val.entries().push(popWithType(WdlExpression.class));
    }
    return null;
  }

  @Override
  public Void visitTaskRuntimeSection(TaskRuntimeSectionContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    super.visitTaskRuntimeSection(ctx);
    while (stack.peek() instanceof WdlRuntime) {
      task.elements().addLast(popWithType(WdlRuntime.class));
    }
    return null;
  }

  @Override
  public Void visitRuntimeSection(RuntimeSectionContext ctx) {
    WdlRuntime runtimeSection = new WdlRuntime();
    stack.push(runtimeSection);
    super.visitRuntimeSection(ctx);
    while (stack.peek() != runtimeSection) {
      runtimeSection.elements().addLast(popWithType(WdlRuntimeEntry.class));
    }
    return null;
  }

  @Override
  public Void visitRuntimeItem(RuntimeItemContext ctx) {
    WdlRuntimeEntry entry = new WdlRuntimeEntry();
    stack.push(entry);
    super.visitRuntimeItem(ctx);
    entry.setValue(popWithType(WdlExpression.class));
    entry.setKey(ctx.strictIdentifier().getText());
    return null;
  }

  @Override
  public Void visitTaskRequirementsSection(TaskRequirementsSectionContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    super.visitTaskRequirementsSection(ctx);
    task.elements().addLast(popWithType(WdlRequirements.class));
    return null;
  }

  @Override
  public Void visitRequirementsSection(RequirementsSectionContext ctx) {
    WdlRequirements requirementsSection = new WdlRequirements();
    stack.push(requirementsSection);
    super.visitRequirementsSection(ctx);
    while (stack.peek() != requirementsSection) {
      requirementsSection.elements().addLast(popWithType(WdlRequirementEntry.class));
    }
    return null;
  }

  @Override
  public Void visitRequirementsItem(RequirementsItemContext ctx) {
    WdlRequirementEntry entry = new WdlRequirementEntry();
    stack.push(entry);
    super.visitRequirementsItem(ctx);
    entry.setValue(popWithType(WdlExpression.class));
    entry.setKey(ctx.strictIdentifier().getText());
    return null;
  }

  @Override
  public Void visitTaskHintsSection(TaskHintsSectionContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    super.visitTaskHintsSection(ctx);
    task.elements().addLast(popWithType(WdlTaskHints.class));
    return null;
  }

  @Override
  public Void visitHintsSectionTask(HintsSectionTaskContext ctx) {
    WdlTaskHints hints = new WdlTaskHints();
    stack.push(hints);
    super.visitHintsSectionTask(ctx);
    while (stack.peek() != hints) {
      hints.elements().addLast(popWithType(WdlTaskHint.class));
    }
    return null;
  }

  @Override
  public Void visitHintsItemTask(HintsItemTaskContext ctx) {
    WdlTaskHint hint = new WdlTaskHint();
    stack.push(hint);
    super.visitHintsItemTask(ctx);
    hint.setValue(popWithType(WdlExpression.class));
    hint.setKey(ctx.strictIdentifier().getText());
    return null;
  }

  @Override
  public Void visitHintsTypedObjectTask(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.HintsTypedObjectTaskContext ctx) {
    WdlObjectLiteral hintsObject = new WdlObjectLiteral();
    stack.push(hintsObject);
    super.visitHintsTypedObjectTask(ctx);
    while (stack.peek() != hintsObject) {
      hintsObject.entries().addLast(popWithType(WdlObjectEntry.class));
    }
    return null;
  }

  @Override
  public Void visitHintsObjectItemTask(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.HintsObjectItemTaskContext ctx) {
    WdlObjectEntry entry = new WdlObjectEntry();
    stack.push(entry);
    visit(ctx.hintsValueTask());
    entry.setValue(popWithType(WdlExpression.class));
    entry.setKey(ctx.dottedIdentifier().getText());
    return null;
  }

  @Override
  public Void visitInputHintsObjectTask(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.InputHintsObjectTaskContext ctx) {
    WdlObjectLiteral inputHintsObject = new WdlObjectLiteral();
    stack.push(inputHintsObject);
    super.visitInputHintsObjectTask(ctx);
    while (stack.peek() != inputHintsObject) {
      inputHintsObject.entries().addLast(popWithType(WdlObjectEntry.class));
    }
    return null;
  }

  @Override
  public Void visitInputHintsItemTask(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.InputHintsItemTaskContext ctx) {
    WdlObjectEntry entry = new WdlObjectEntry();
    stack.push(entry);
    visit(ctx.hintsTypedObjectTask());
    entry.setValue(popWithType(WdlExpression.class));
    entry.setKey(ctx.dottedIdentifier().getText());
    return null;
  }

  @Override
  public Void visitOutputHintsObjectTask(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.OutputHintsObjectTaskContext ctx) {
    WdlObjectLiteral outputHintsObject = new WdlObjectLiteral();
    stack.push(outputHintsObject);
    super.visitOutputHintsObjectTask(ctx);
    while (stack.peek() != outputHintsObject) {
      outputHintsObject.entries().push(popWithType(WdlObjectEntry.class));
    }
    return null;
  }

  @Override
  public Void visitOutputHintsItemTask(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.OutputHintsItemTaskContext ctx) {
    WdlObjectEntry entry = new WdlObjectEntry();
    stack.push(entry);
    visit(ctx.hintsTypedObjectTask());
    entry.setValue(popWithType(WdlExpression.class));
    entry.setKey(ctx.dottedIdentifier().getText());
    return null;
  }

  @Override
  public Void visitTaskHintsArray(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.TaskHintsArrayContext ctx) {
    WdlArrayLiteral hintsArray = new WdlArrayLiteral();
    stack.push(hintsArray);
    super.visitTaskHintsArray(ctx);
    while (stack.peek() != hintsArray) {
      hintsArray.entries().push(popWithType(WdlExpression.class));
    }
    return null;
  }

  @Override
  public Void visitTaskMetadataSection(TaskMetadataSectionContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    super.visitTaskMetadataSection(ctx);
    task.elements().addLast(popWithType(WdlMetadata.class));
    return null;
  }

  @Override
  public Void visitTaskParameterMetadataSection(TaskParameterMetadataSectionContext ctx) {
    WdlTask task = findWithType(WdlTask.class);
    super.visitTaskParameterMetadataSection(ctx);
    task.elements().addLast(popWithType(WdlParameterMetadata.class));
    return null;
  }

  // =========================================================================
  // Workflow Definitions
  // =========================================================================

  @Override
  public Void visitWorkflowDefinition(WorkflowDefinitionContext ctx) {
    WdlWorkflow workflow = new WdlWorkflow(ctx.strictIdentifier().getText());
    stack.push(workflow);
    super.visitWorkflowDefinition(ctx);
    popWithType(WdlWorkflow.class);
    peekWithType(WdlDocument.class).elements().add(workflow);
    return null;
  }

  @Override
  public Void visitWorkflowDeclaration(WorkflowDeclarationContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowDeclaration(ctx);
    while (stack.peek() instanceof WdlBoundDeclaration) {
      workflow.getElements().addLast(popWithType(WdlBoundDeclaration.class));
    }
    return null;
  }

  @Override
  public Void visitWorkflowInputSection(WorkflowInputSectionContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowInputSection(ctx);
    workflow.getElements().addLast(popWithType(WdlInput.class));
    return null;
  }

  @Override
  public Void visitWorkflowOutputSection(WorkflowOutputSectionContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowOutputSection(ctx);
    workflow.getElements().addLast(popWithType(WdlOutput.class));
    return null;
  }

  @Override
  public Void visitWorkflowCallStatement(WorkflowCallStatementContext ctx) {
    super.visitWorkflowCallStatement(ctx);
    findWithType(WdlWorkflow.class).getElements().addLast(popWithType(WdlCall.class));
    return null;
  }

  @Override
  public Void visitCallStatement(CallStatementContext ctx) {
    WdlCall callStmt = new WdlCall();
    stack.push(callStmt);
    return super.visitCallStatement(ctx);
  }

  @Override
  public Void visitCallTarget(CallTargetContext ctx) {
    WdlCall callStmt = peekWithType(WdlCall.class);
    ctx.strictIdentifier().forEach(i -> callStmt.targetPath().addLast(i.getText()));
    return super.visitCallTarget(ctx);
  }

  @Override
  public Void visitCallAlias(CallAliasContext ctx) {
    WdlCall callStmt = peekWithType(WdlCall.class);
    callStmt.setAlias(ctx.strictIdentifier().getText());
    return super.visitCallAlias(ctx);
  }

  @Override
  public Void visitCallAfterClause(CallAfterClauseContext ctx) {
    WdlCall callStmt = peekWithType(WdlCall.class);
    callStmt.afterDependencies().addLast(ctx.strictIdentifier().getText());
    return super.visitCallAfterClause(ctx);
  }

  @Override
  public Void visitCallInputBlock(CallInputBlockContext ctx) {
    WdlCall call = peekWithType(WdlCall.class);
    call.setLegacyInputColonUsed(ctx.KEYWORD_INPUT() != null);
    super.visitCallInputBlock(ctx);
    while (stack.peek() instanceof WdlCallInput) {
      call.inputs().push(popWithType(WdlCallInput.class));
    }
    return null;
  }

  @Override
  public Void visitCallInputItem(CallInputItemContext ctx) {
    WdlCallInput callInput = new WdlCallInput(ctx.strictIdentifier().getText());
    super.visitCallInputItem(ctx);
    if (ctx.expression() != null) {
      callInput.setValue(popWithType(WdlExpression.class));
    }
    stack.push(callInput);
    return null;
  }

  @Override
  public Void visitWorkflowConditionalStatement(WorkflowConditionalStatementContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowConditionalStatement(ctx);
    WdlConditional condStmt = popWithType(WdlConditional.class);
    workflow.getElements().addLast(condStmt);
    return null;
  }

  @Override
  public Void visitConditionalStatement(ConditionalStatementContext ctx) {
    WdlConditional condStmt = new WdlConditional();
    stack.push(condStmt);
    super.visitConditionalStatement(ctx);
    for (int i = 0; i < ctx.workflowStatement().size(); i++) {
      condStmt.thenStatements().push(popWithType(WdlStatement.class));
    }
    condStmt.setCondition(popWithType(WdlExpression.class));
    return null;
  }

  @Override
  public Void visitConditionalElseIfClause(ConditionalElseIfClauseContext ctx) {
    WdlConditional condStmt = findWithType(WdlConditional.class);
    super.visitConditionalElseIfClause(ctx);
    WdlConditionalElseIf elseIfClause = new WdlConditionalElseIf();
    for (int i = 0; i < ctx.workflowStatement().size(); i++) {
      elseIfClause.thenStatements().push(popWithType(WdlStatement.class));
    }
    elseIfClause.setCondition(popWithType(WdlExpression.class));
    condStmt.elseIfs().addLast(elseIfClause);
    return null;
  }

  @Override
  public Void visitConditionalElseClause(ConditionalElseClauseContext ctx) {
    WdlConditional condStmt = findWithType(WdlConditional.class);
    super.visitConditionalElseClause(ctx);
    for (int i = 0; i < ctx.workflowStatement().size(); i++) {
      condStmt.elseStatements().push(popWithType(WdlStatement.class));
    }
    return null;
  }

  @Override
  public Void visitWorkflowScatterStatement(WorkflowScatterStatementContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowScatterStatement(ctx);
    WdlScatter scatterStmt = popWithType(WdlScatter.class);
    workflow.getElements().addLast(scatterStmt);
    return null;
  }

  @Override
  public Void visitScatterStatement(ScatterStatementContext ctx) {
    WdlScatter scatterStmt = new WdlScatter(ctx.strictIdentifier().getText());
    stack.push(scatterStmt);
    super.visitScatterStatement(ctx);
    scatterStmt.setCollection(popWithType(WdlExpression.class));
    return null;
  }

  @Override
  public Void visitScatterBody(ScatterBodyContext ctx) {
    WdlScatter scatterStmt = findWithType(WdlScatter.class);
    super.visitScatterBody(ctx);
    while (stack.peek() instanceof WdlStatement) {
      scatterStmt.statements().push(popWithType(WdlStatement.class));
    }
    return null;
  }

  @Override
  public Void visitWorkflowHintsSection(WorkflowHintsSectionContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowHintsSection(ctx);
    workflow.getElements().push(popWithType(WdlWorkflowHints.class));
    return null;
  }

  @Override
  public Void visitHintsSectionWorkflow(HintsSectionWorkflowContext ctx) {
    WdlWorkflowHints hints = new WdlWorkflowHints();
    stack.push(hints);
    super.visitHintsSectionWorkflow(ctx);
    while (stack.peek() != hints) {
      hints.elements().push(popWithType(WdlWorkflowHint.class));
    }
    return null;
  }

  @Override
  public Void visitHintsItemWorkflow(HintsItemWorkflowContext ctx) {
    WdlWorkflowHint hint = new WdlWorkflowHint(ctx.strictIdentifier().getText());
    stack.push(hint);
    super.visitHintsItemWorkflow(ctx);
    hint.setValue(popWithType(WdlExpression.class));
    return null;
  }

  @Override
  public Void visitWorkflowMetadataSection(WorkflowMetadataSectionContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowMetadataSection(ctx);
    workflow.getElements().addLast(popWithType(WdlMetadata.class));
    return null;
  }

  @Override
  public Void visitWorkflowParameterMetadataSection(WorkflowParameterMetadataSectionContext ctx) {
    WdlWorkflow workflow = findWithType(WdlWorkflow.class);
    super.visitWorkflowParameterMetadataSection(ctx);
    workflow.getElements().addLast(popWithType(WdlParameterMetadata.class));
    return null;
  }

  // =========================================================================
  // Types - all push to stack
  // =========================================================================

  @Override
  public Void visitMapType(MapTypeContext ctx) {
    super.visitMapType(ctx);
    WdlType valueType = popWithType(WdlType.class);
    WdlType keyType = popWithType(WdlType.class);
    boolean isOptional = ctx.QUESTION_MARK() != null;
    WdlMapType mapType = new WdlMapType(keyType, valueType, isOptional);
    stack.push(mapType);
    return null;
  }

  @Override
  public Void visitArrayType(ArrayTypeContext ctx) {
    super.visitArrayType(ctx);
    WdlType elementType = popWithType(WdlType.class);
    boolean isNonEmpty = ctx.PLUS() != null;
    boolean isOptional = ctx.QUESTION_MARK() != null;
    WdlArrayType arrayType = new WdlArrayType(elementType, isNonEmpty, isOptional);
    stack.push(arrayType);
    return null;
  }

  @Override
  public Void visitPairType(PairTypeContext ctx) {
    super.visitPairType(ctx);
    WdlType rightType = popWithType(WdlType.class);
    WdlType leftType = popWithType(WdlType.class);
    boolean isOptional = ctx.QUESTION_MARK() != null;
    WdlPairType pairType = new WdlPairType(leftType, rightType, isOptional);
    stack.push(pairType);
    return null;
  }

  @Override
  public Void visitObjectType(com.myriad.wdl.model.v1.grammar.WdlV1Parser.ObjectTypeContext ctx) {
    boolean isOptional = ctx.QUESTION_MARK() != null;
    stack.push(new WdlTypeReferenceType("Object", isOptional));
    return null;
  }

  @Override
  public Void visitPrimitiveType(PrimitiveTypeContext ctx) {
    super.visitPrimitiveType(ctx);
    WdlPrimitiveType.Type type;

    if (ctx.KEYWORD_BOOLEAN_TYPE() != null) {
      type = WdlPrimitiveType.Type.BOOLEAN;
    } else if (ctx.KEYWORD_INT_TYPE() != null) {
      type = WdlPrimitiveType.Type.INT;
    } else if (ctx.KEYWORD_FLOAT_TYPE() != null) {
      type = WdlPrimitiveType.Type.FLOAT;
    } else if (ctx.KEYWORD_STRING_TYPE() != null) {
      type = WdlPrimitiveType.Type.STRING;
    } else if (ctx.KEYWORD_FILE_TYPE() != null) {
      type = WdlPrimitiveType.Type.FILE;
    } else if (ctx.KEYWORD_DIRECTORY_TYPE() != null) {
      type = WdlPrimitiveType.Type.DIRECTORY;
    } else {
      // This should never happen
      throw new AssertionError("Unknown primitive type " + ctx.getText());
    }

    boolean isOptional = ctx.QUESTION_MARK() != null;
    WdlPrimitiveType primitiveType = new WdlPrimitiveType(type, isOptional);
    stack.push(primitiveType);
    return null;
  }

  @Override
  public Void visitTypeRefType(TypeRefTypeContext ctx) {
    WdlTypeReferenceType type = new WdlTypeReferenceType();
    super.visitTypeRefType(ctx);
    type.setReferenceName(ctx.strictIdentifier().getText());
    type.setOptional(ctx.QUESTION_MARK() != null);
    stack.push(type);
    return null;
  }

  // =========================================================================
  // Expressions - all push to stack
  // =========================================================================

  @Override
  public Void visitNullLiteral(NullLiteralContext ctx) {
    super.visitNullLiteral(ctx);
    stack.push(new WdlNullLiteral());
    return null;
  }

  @Override
  public Void visitNoneLiteral(NoneLiteralContext ctx) {
    super.visitNoneLiteral(ctx);
    stack.push(new WdlNullLiteral());
    return null;
  }

  @Override
  public Void visitBooleanLiteral(BooleanLiteralContext ctx) {
    super.visitBooleanLiteral(ctx);
    boolean value = ctx.KEYWORD_TRUE() != null;
    stack.push(new WdlBooleanLiteral(value));
    return null;
  }

  @Override
  public Void visitNumberLiteralInt(NumberLiteralIntContext ctx) {
    super.visitNumberLiteralInt(ctx);
    long value = Long.parseLong(ctx.INTEGER().getText());
    stack.push(new WdlIntLiteral(value));
    return null;
  }

  @Override
  public Void visitNumberLiteralFloat(NumberLiteralFloatContext ctx) {
    super.visitNumberLiteralFloat(ctx);
    double value = Double.parseDouble(ctx.FLOAT().getText());
    stack.push(new WdlFloatLiteral(value));
    return null;
  }

  @Override
  public Void visitNumberLiteralSigned(NumberLiteralSignedContext ctx) {
    super.visitNumberLiteralSigned(ctx);
    if (ctx.MINUS() != null) {
      peekWithType(WdlNumberLiteral.class).negate();
    }
    return null;
  }

  @Override
  public Void visitQuotedString(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.QuotedStringContext ctx) {
    WdlStringLiteral expr =
        new WdlStringLiteral(
            ctx.SINGLE_QUOTE() != null
                ? WdlStringLiteral.Delimiter.SINGLE_QUOTE
                : WdlStringLiteral.Delimiter.DOUBLE_QUOTE);
    stack.push(expr);
    super.visitQuotedString(ctx);
    ArrayDeque<WdlStringComponent> components = new ArrayDeque<>();
    while (stack.peek() != expr) {
      components.addLast(popWithType(WdlStringComponent.class));
    }
    for (WdlStringComponent c : components) {
      expr.components().push(c);
    }
    return null;
  }

  @Override
  public Void visitMultilineString(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.MultilineStringContext ctx) {
    WdlStringLiteral expr = new WdlStringLiteral(WdlStringLiteral.Delimiter.MULTILINE);
    stack.push(expr);
    super.visitMultilineString(ctx);
    ArrayDeque<WdlStringComponent> components = new ArrayDeque<>();
    while (stack.peek() != expr) {
      components.addLast(popWithType(WdlStringComponent.class));
    }
    for (WdlStringComponent c : components) {
      expr.components().push(c);
    }
    return null;
  }

  @Override
  public Void visitStringElementText(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.StringElementTextContext ctx) {
    stack.push(new WdlStringText(ctx.STRING_TEXT().getText()));
    return null;
  }

  @Override
  public Void visitStringElementEscape(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.StringElementEscapeContext ctx) {
    stack.push(new WdlStringEscape(ctx.STRING_ESCAPE().getText()));
    return null;
  }

  @Override
  public Void visitStringPlaceholder(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.StringPlaceholderContext ctx) {
    PlaceHolderSymbol symbol;
    switch (ctx.STRING_PLACEHOLDER_START().getText()) {
      case "~{":
        symbol = PlaceHolderSymbol.TILDE;
        break;
      case "${":
        symbol = PlaceHolderSymbol.DOLLAR;
        break;
      default:
        // This should never happen
        throw new AssertionError(
            "Unknown PlaceHolderSymbol " + ctx.STRING_PLACEHOLDER_START().getText());
    }
    WdlStringPlaceholder placeholder = new WdlStringPlaceholder();
    placeholder.setSymbol(symbol);
    stack.push(placeholder);
    super.visitStringPlaceholder(ctx);
    WdlExpression placeholderExpr = popWithType(WdlExpression.class);
    placeholder.setExpression(placeholderExpr);
    while (stack.peek() != placeholder) {
      if (placeholder.getOption() != null) {
        // This should never happen
        throw new AssertionError("Placeholder option already set");
      }
      placeholder.setOption(popWithType(WdlStringPlaceholderOption.class));
    }
    return null;
  }

  @Override
  public Void visitStringPlaceholderOptionSepDefault(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.StringPlaceholderOptionSepDefaultContext ctx) {
    WdlStringPlaceholder placeholder = peekWithType(WdlStringPlaceholder.class);
    if (placeholder.getOption() != null) {
      // This should never happen
      throw new AssertionError("Placeholder option already set");
    }
    super.visitStringPlaceholderOptionSepDefault(ctx);
    WdlStringPlaceholderOption.Type type;
    switch (ctx.IDENTIFIER().getText()) {
      case "sep":
        type = Type.SEP;
        break;
      case "default":
        type = Type.DEFAULT;
        break;
      default:
        throw new IllegalArgumentException();
    }
    WdlStringLiteral value = popWithType(WdlStringLiteral.class);
    placeholder.setOption(new WdlStringPlaceholderOption(type, value));
    return null;
  }

  @Override
  public Void visitStringPlaceholderOptionTrueFalse(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.StringPlaceholderOptionTrueFalseContext ctx) {
    WdlStringPlaceholder placeholder = peekWithType(WdlStringPlaceholder.class);
    if (placeholder.getOption() != null) {
      // This should never happen
      throw new AssertionError("Placeholder option already set");
    }
    super.visitStringPlaceholderOptionTrueFalse(ctx);
    WdlStringLiteral falseStr = popWithType(WdlStringLiteral.class);
    WdlStringLiteral trueStr = popWithType(WdlStringLiteral.class);
    placeholder.setOption(new WdlStringPlaceholderOption(Type.TRUE_FALSE, trueStr, falseStr));
    return null;
  }

  @Override
  public Void visitStringPlaceholderOptionFalseTrue(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.StringPlaceholderOptionFalseTrueContext ctx) {
    WdlStringPlaceholder placeholder = peekWithType(WdlStringPlaceholder.class);
    if (placeholder.getOption() != null) {
      // This should never happen
      throw new AssertionError("Placeholder option already set");
    }
    super.visitStringPlaceholderOptionFalseTrue(ctx);
    WdlStringLiteral trueStr = popWithType(WdlStringLiteral.class);
    WdlStringLiteral falseStr = popWithType(WdlStringLiteral.class);
    placeholder.setOption(new WdlStringPlaceholderOption(Type.FALSE_TRUE, trueStr, falseStr));
    return null;
  }

  @Override
  public Void visitMultilineStringElementText(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.MultilineStringElementTextContext ctx) {
    stack.push(new WdlStringText(ctx.MULTILINE_STRING_TEXT().getText()));
    return null;
  }

  @Override
  public Void visitMultilineStringElementEscape(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.MultilineStringElementEscapeContext ctx) {
    stack.push(new WdlStringEscape(ctx.MULTILINE_STRING_ESCAPE().getText()));
    return null;
  }

  @Override
  public Void visitMultilineStringPlaceholder(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.MultilineStringPlaceholderContext ctx) {
    PlaceHolderSymbol symbol;
    if (ctx.MULTILINE_STRING_TILDE_PLACEHOLDER_START() != null) {
      symbol = PlaceHolderSymbol.TILDE;
    } else if (ctx.MULTILINE_STRING_DOLLAR_PLACEHOLDER_START() != null) {
      symbol = PlaceHolderSymbol.DOLLAR;
    } else {
      // This should never happen
      throw new AssertionError("Unknown multiline placeholder symbol");
    }
    WdlStringPlaceholder placeholder = new WdlStringPlaceholder();
    placeholder.setSymbol(symbol);
    stack.push(placeholder);
    super.visitMultilineStringPlaceholder(ctx);
    WdlExpression placeholderExpr = popWithType(WdlExpression.class);
    placeholder.setExpression(placeholderExpr);
    return null;
  }

  @Override
  public Void visitEnumQuotedString(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumQuotedStringContext ctx) {
    WdlStringLiteral expr =
        new WdlStringLiteral(
            ctx.SINGLE_QUOTE() != null
                ? WdlStringLiteral.Delimiter.SINGLE_QUOTE
                : WdlStringLiteral.Delimiter.DOUBLE_QUOTE);
    stack.push(expr);
    super.visitEnumQuotedString(ctx);
    ArrayDeque<WdlStringComponent> components = new ArrayDeque<>();
    while (stack.peek() != expr) {
      components.addLast(popWithType(WdlStringComponent.class));
    }
    for (WdlStringComponent c : components) {
      expr.components().push(c);
    }
    return null;
  }

  @Override
  public Void visitEnumStringElement(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumStringElementContext ctx) {
    if (ctx.STRING_TEXT() != null) {
      stack.push(new WdlStringText(ctx.STRING_TEXT().getText()));
    } else if (ctx.STRING_ESCAPE() != null) {
      stack.push(new WdlStringEscape(ctx.STRING_ESCAPE().getText()));
    } else if (ctx.STRING_DOLLAR_SIGN() != null) {
      stack.push(new WdlStringText(ctx.STRING_DOLLAR_SIGN().getText()));
    } else if (ctx.STRING_TILDE() != null) {
      stack.push(new WdlStringText(ctx.STRING_TILDE().getText()));
    }
    return null;
  }

  @Override
  public Void visitEnumMultilineString(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumMultilineStringContext ctx) {
    WdlStringLiteral expr = new WdlStringLiteral(WdlStringLiteral.Delimiter.MULTILINE);
    stack.push(expr);
    super.visitEnumMultilineString(ctx);
    ArrayDeque<WdlStringComponent> components = new ArrayDeque<>();
    while (stack.peek() != expr) {
      components.addLast(popWithType(WdlStringComponent.class));
    }
    for (WdlStringComponent c : components) {
      expr.components().push(c);
    }
    return null;
  }

  @Override
  public Void visitEnumMultilineStringElement(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumMultilineStringElementContext ctx) {
    if (ctx.MULTILINE_STRING_TEXT() != null) {
      stack.push(new WdlStringText(ctx.MULTILINE_STRING_TEXT().getText()));
    } else if (ctx.MULTILINE_STRING_ESCAPE() != null) {
      stack.push(new WdlStringEscape(ctx.MULTILINE_STRING_ESCAPE().getText()));
    } else if (ctx.MULTILINE_STRING_DOUBLE_CLOSE_ANGLE() != null) {
      stack.push(new WdlStringText(ctx.MULTILINE_STRING_DOUBLE_CLOSE_ANGLE().getText()));
    } else if (ctx.MULTILINE_STRING_SINGLE_CLOSE_ANGLE() != null) {
      stack.push(new WdlStringText(ctx.MULTILINE_STRING_SINGLE_CLOSE_ANGLE().getText()));
    } else if (ctx.MULTILINE_STRING_DOLLAR_SIGN() != null) {
      stack.push(new WdlStringText(ctx.MULTILINE_STRING_DOLLAR_SIGN().getText()));
    } else if (ctx.MULTILINE_STRING_TILDE() != null) {
      stack.push(new WdlStringText(ctx.MULTILINE_STRING_TILDE().getText()));
    }
    return null;
  }

  @Override
  public Void visitEnumArrayLiteral(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumArrayLiteralContext ctx) {
    WdlArrayLiteral arrayLit = new WdlArrayLiteral();
    stack.push(arrayLit);
    super.visitEnumArrayLiteral(ctx);
    while (stack.peek() != arrayLit) {
      arrayLit.entries().push(popWithType(WdlExpression.class));
    }
    return null;
  }

  @Override
  public Void visitEnumMapLiteral(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumMapLiteralContext ctx) {
    WdlMapLiteral mapLit = new WdlMapLiteral();
    stack.push(mapLit);
    super.visitEnumMapLiteral(ctx);
    while (stack.peek() != mapLit) {
      WdlExpression val = popWithType(WdlExpression.class);
      WdlExpression key = popWithType(WdlExpression.class);
      mapLit.entries().push(new WdlMapEntry(key, val));
    }
    return null;
  }

  @Override
  public Void visitEnumObjectLiteral(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumObjectLiteralContext ctx) {
    WdlObjectLiteral objLit = new WdlObjectLiteral();
    stack.push(objLit);
    super.visitEnumObjectLiteral(ctx);
    while (stack.peek() != objLit) {
      objLit.entries().push(popWithType(WdlObjectEntry.class));
    }
    return null;
  }

  @Override
  public Void visitEnumObjectLiteralItem(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumObjectLiteralItemContext ctx) {
    WdlObjectEntry entry = new WdlObjectEntry(ctx.strictIdentifier().getText());
    stack.push(entry);
    super.visitEnumObjectLiteralItem(ctx);
    entry.setValue(popWithType(WdlExpression.class));
    return null;
  }

  @Override
  public Void visitEnumStructLiteral(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumStructLiteralContext ctx) {
    WdlStructLiteral structLit = new WdlStructLiteral(ctx.strictIdentifier().getText());
    stack.push(structLit);
    super.visitEnumStructLiteral(ctx);
    return null;
  }

  @Override
  public Void visitEnumStructLiteralItem(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumStructLiteralItemContext ctx) {
    WdlStructLiteral structLit = findWithType(WdlStructLiteral.class);
    WdlStructEntry entry = new WdlStructEntry(ctx.strictIdentifier().getText());
    stack.push(entry);
    super.visitEnumStructLiteralItem(ctx);
    entry.setValue(popWithType(WdlExpression.class));
    stack.pop();
    structLit.entries().add(entry);
    return null;
  }

  @Override
  public Void visitEnumPairLiteral(
      com.myriad.wdl.model.v1.grammar.WdlV1Parser.EnumPairLiteralContext ctx) {
    WdlPairLiteral pairLit = new WdlPairLiteral();
    stack.push(pairLit);
    super.visitEnumPairLiteral(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    pairLit.setLeft(left);
    pairLit.setRight(right);
    return null;
  }

  @Override
  public Void visitArrayLiteral(ArrayLiteralContext ctx) {
    WdlArrayLiteral arrayLit = new WdlArrayLiteral();
    stack.push(arrayLit);
    super.visitArrayLiteral(ctx);
    while (stack.peek() != arrayLit) {
      WdlExpression elem = popWithType(WdlExpression.class);
      arrayLit.entries().push(elem);
    }
    return null;
  }

  @Override
  public Void visitMapLiteral(MapLiteralContext ctx) {
    WdlMapLiteral mapLit = new WdlMapLiteral();
    stack.push(mapLit);
    super.visitMapLiteral(ctx);
    while (stack.peek() != mapLit) {
      WdlExpression val = popWithType(WdlExpression.class);
      WdlExpression key = popWithType(WdlExpression.class);
      mapLit.entries().push(new WdlMapEntry(key, val));
    }
    return null;
  }

  @Override
  public Void visitStructLiteral(StructLiteralContext ctx) {
    WdlStructLiteral structLit = new WdlStructLiteral(ctx.strictIdentifier().getText());
    stack.push(structLit);
    super.visitStructLiteral(ctx);
    structLit.setName(ctx.strictIdentifier().getText());
    return null;
  }

  @Override
  public Void visitStructLiteralItem(StructLiteralItemContext ctx) {
    WdlStructLiteral structLit = findWithType(WdlStructLiteral.class);
    WdlStructEntry entry = new WdlStructEntry();
    stack.push(entry);
    super.visitStructLiteralItem(ctx);
    entry.setValue(popWithType(WdlExpression.class));
    entry.setKey(ctx.strictIdentifier().getText());
    stack.pop();
    structLit.entries().add(entry);
    return null;
  }

  @Override
  public Void visitObjectLiteral(ObjectLiteralContext ctx) {
    WdlObjectLiteral objLit = new WdlObjectLiteral();
    stack.push(objLit);
    super.visitObjectLiteral(ctx);
    while (stack.peek() != objLit) {
      objLit.entries().push(popWithType(WdlObjectEntry.class));
    }
    return null;
  }

  @Override
  public Void visitObjectLiteralItem(ObjectLiteralItemContext ctx) {
    WdlObjectEntry entry = new WdlObjectEntry();
    entry.setKey(ctx.strictIdentifier().getText());
    stack.push(entry);
    super.visitObjectLiteralItem(ctx);
    entry.setValue(popWithType(WdlExpression.class));
    return null;
  }

  @Override
  public Void visitPairLiteral(PairLiteralContext ctx) {
    WdlPairLiteral pairLit = new WdlPairLiteral();
    stack.push(pairLit);
    super.visitPairLiteral(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    pairLit.setLeft(left);
    pairLit.setRight(right);
    return null;
  }

  // =========================================================================
  // Binary Operators
  // =========================================================================

  @Override
  public Void visitLogicalOrExprOperation(LogicalOrExprOperationContext ctx) {
    super.visitLogicalOrExprOperation(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    WdlBinaryOperation expr = new WdlBinaryOperation(left, WdlBinaryOperation.Operator.OR, right);
    stack.push(expr);
    return null;
  }

  @Override
  public Void visitLogicalAndExprOperation(LogicalAndExprOperationContext ctx) {
    super.visitLogicalAndExprOperation(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    WdlBinaryOperation expr = new WdlBinaryOperation(left, WdlBinaryOperation.Operator.AND, right);
    stack.push(expr);
    return null;
  }

  @Override
  public Void visitEqualityExprOperation(EqualityExprOperationContext ctx) {
    super.visitEqualityExprOperation(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    WdlBinaryOperation.Operator op =
        ctx.EQUAL() != null ? WdlBinaryOperation.Operator.EQ : WdlBinaryOperation.Operator.NEQ;
    WdlBinaryOperation expr = new WdlBinaryOperation(left, op, right);
    stack.push(expr);
    return null;
  }

  @Override
  public Void visitComparisonExprOperation(ComparisonExprOperationContext ctx) {
    super.visitComparisonExprOperation(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    WdlBinaryOperation.Operator op;
    if (ctx.LESS() != null) {
      op = WdlBinaryOperation.Operator.LT;
    } else if (ctx.LESS_EQUAL() != null) {
      op = WdlBinaryOperation.Operator.LTE;
    } else if (ctx.GREATER() != null) {
      op = WdlBinaryOperation.Operator.GT;
    } else if (ctx.GREATER_EQUAL() != null) {
      op = WdlBinaryOperation.Operator.GTE;
    } else {
      // This should never happen
      throw new AssertionError("Unknown comparison operator");
    }
    WdlBinaryOperation expr = new WdlBinaryOperation(left, op, right);
    stack.push(expr);
    return null;
  }

  @Override
  public Void visitAdditiveExprOperation(AdditiveExprOperationContext ctx) {
    super.visitAdditiveExprOperation(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    WdlBinaryOperation.Operator op =
        ctx.PLUS() != null ? WdlBinaryOperation.Operator.ADD : WdlBinaryOperation.Operator.SUTRACT;
    WdlBinaryOperation expr = new WdlBinaryOperation(left, op, right);
    stack.push(expr);
    return null;
  }

  @Override
  public Void visitMultiplicativeExprOperation(MultiplicativeExprOperationContext ctx) {
    super.visitMultiplicativeExprOperation(ctx);
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    WdlBinaryOperation.Operator op;
    if (ctx.ASTERISK() != null) {
      op = WdlBinaryOperation.Operator.MULTIPLY;
    } else if (ctx.SLASH() != null) {
      op = WdlBinaryOperation.Operator.DIVIDE;
    } else if (ctx.PERCENT() != null) {
      op = WdlBinaryOperation.Operator.MODULO;
    } else {
      // This should never happen
      throw new AssertionError("Unknown multiplicative operator");
    }
    WdlBinaryOperation expr = new WdlBinaryOperation(left, op, right);
    stack.push(expr);
    return null;
  }

  @Override
  public Void visitPowerExprOperation(PowerExprOperationContext ctx) {
    super.visitPowerExprOperation(ctx);
    WdlBinaryOperation.Operator op;
    if (ctx.EXPONENTIATION() != null) {
      op = Operator.POWER;
    } else {
      // This should never happen
      throw new AssertionError("Unkown power operator");
    }
    WdlExpression right = popWithType(WdlExpression.class);
    WdlExpression left = popWithType(WdlExpression.class);
    WdlBinaryOperation expr = new WdlBinaryOperation(left, op, right);
    stack.push(expr);
    return null;
  }

  // =========================================================================
  // Unary Operators
  // =========================================================================

  @Override
  public Void visitUnaryExprOperation(UnaryExprOperationContext ctx) {
    super.visitUnaryExprOperation(ctx);
    WdlExpression operand = popWithType(WdlExpression.class);
    WdlUnaryOperation.Operator op;
    if (ctx.MINUS() != null) {
      op = WdlUnaryOperation.Operator.NEGATIVE;
    } else if (ctx.EXCLAMATION() != null) {
      op = WdlUnaryOperation.Operator.NOT;
    } else {
      // This should never happen
      throw new AssertionError("Unknown unary operation");
    }
    WdlUnaryOperation expr = new WdlUnaryOperation(op, operand);
    stack.push(expr);
    return null;
  }

  // =========================================================================
  // Postfix Expressions (array indexing, field access)
  // =========================================================================

  @Override
  public Void visitPostfixExprArrayIndex(PostfixExprArrayIndexContext ctx) {
    super.visitPostfixExprArrayIndex(ctx);
    WdlExpression indexExpr = popWithType(WdlExpression.class);
    WdlExpression targetExpr = popWithType(WdlExpression.class);
    WdlExpression result = new WdlIndexAccessOperation(targetExpr, indexExpr);
    stack.push(result);
    return null;
  }

  @Override
  public Void visitPostfixExprField(PostfixExprFieldContext ctx) {
    super.visitPostfixExprField(ctx);
    WdlExpression result =
        new WdlMemberAccessOperation(
            popWithType(WdlExpression.class), ctx.strictIdentifier().getText());
    stack.push(result);
    return null;
  }

  // =========================================================================
  // Primary Expressions
  // =========================================================================

  @Override
  public Void visitVariable(com.myriad.wdl.model.v1.grammar.WdlV1Parser.VariableContext ctx) {
    String varName = ctx.strictIdentifier().getText();
    WdlVariable expr = new WdlVariable(varName);
    stack.push(expr);
    return null;
  }

  // =========================================================================
  // Function Calls
  // =========================================================================

  @Override
  public Void visitCallExpression(CallExpressionContext ctx) {
    WdlFunctionCallOperation expr = new WdlFunctionCallOperation(ctx.strictIdentifier().getText());
    stack.push(expr);
    super.visitCallExpression(ctx);
    while (stack.peek() != expr) {
      expr.arguments().push(popWithType(WdlExpression.class));
    }
    return null;
  }

  // =========================================================================
  // Conditional Expressions
  // =========================================================================

  @Override
  public Void visitIfExpression(IfExpressionContext ctx) {
    WdlTernaryOperation expr = new WdlTernaryOperation();
    stack.push(expr);
    super.visitIfExpression(ctx);
    WdlExpression elseValue = popWithType(WdlExpression.class);
    WdlExpression thenValue = popWithType(WdlExpression.class);
    WdlExpression condition = popWithType(WdlExpression.class);
    expr.setCondition(condition);
    expr.setTrueValue(thenValue);
    expr.setFalseValue(elseValue);
    return null;
  }
}
