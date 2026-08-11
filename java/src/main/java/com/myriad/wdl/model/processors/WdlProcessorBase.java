package com.myriad.wdl.model.processors;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.base.WdlKeyValue.WdlStringKeyValue;
import com.myriad.wdl.model.base.WdlNode;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlStruct.WdlStructElement;
import com.myriad.wdl.model.definitions.WdlStruct.WdlStructMember;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlTask.WdlTaskElement;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.definitions.WdlWorkflow.WdlWorkflowElement;
import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.expressions.WdlBinaryOperation;
import com.myriad.wdl.model.expressions.WdlExpression;
import com.myriad.wdl.model.expressions.WdlFunctionCallOperation;
import com.myriad.wdl.model.expressions.WdlIndexAccessOperation;
import com.myriad.wdl.model.expressions.WdlMapLiteral;
import com.myriad.wdl.model.expressions.WdlMemberAccessOperation;
import com.myriad.wdl.model.expressions.WdlObjectLiteral;
import com.myriad.wdl.model.expressions.WdlPairLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral;
import com.myriad.wdl.model.expressions.WdlStringLiteral.Delimiter;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringComponent;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringEscape;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringPlaceholder;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringText;
import com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringToken;
import com.myriad.wdl.model.expressions.WdlStructLiteral;
import com.myriad.wdl.model.expressions.WdlTernaryOperation;
import com.myriad.wdl.model.expressions.WdlUnaryOperation;
import com.myriad.wdl.model.expressions.WdlValueExpression;
import com.myriad.wdl.model.expressions.WdlVariable;
import com.myriad.wdl.model.sections.WdlCommand;
import com.myriad.wdl.model.sections.WdlHints.WdlTaskHints;
import com.myriad.wdl.model.sections.WdlHints.WdlWorkflowHints;
import com.myriad.wdl.model.sections.WdlInput;
import com.myriad.wdl.model.sections.WdlMetadataBase.WdlMetadata;
import com.myriad.wdl.model.sections.WdlMetadataBase.WdlParameterMetadata;
import com.myriad.wdl.model.sections.WdlOutput;
import com.myriad.wdl.model.sections.WdlRequirements;
import com.myriad.wdl.model.sections.WdlRuntime;
import com.myriad.wdl.model.statements.WdlCall;
import com.myriad.wdl.model.statements.WdlConditional;
import com.myriad.wdl.model.statements.WdlDeclaration;
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import com.myriad.wdl.model.statements.WdlImport;
import com.myriad.wdl.model.statements.WdlImport.WdlImportMember;
import com.myriad.wdl.model.statements.WdlImport.WdlImportMembers;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStandard;
import com.myriad.wdl.model.statements.WdlImport.WdlImportStar;
import com.myriad.wdl.model.statements.WdlScatter;
import com.myriad.wdl.model.types.WdlArrayType;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPairType;
import com.myriad.wdl.model.types.WdlPrimitiveType;
import com.myriad.wdl.model.types.WdlType;
import com.myriad.wdl.model.types.WdlTypeReferenceType;
import com.myriad.wdl.model.types.WdlTypeInference;
import java.net.URI;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Collectors;

/**
 * Default source-order traversal for the Java WDL object model.
 *
 * <p>This base class walks documents, definitions, sections, statements, expressions, and types in
 * a predictable order and exposes hook methods through {@link WdlProcessor}. It also includes
 * helpers for rendering model nodes back into WDL source text.
 */
public class WdlProcessorBase implements WdlProcessor {

  protected WdlDocument documentCtx;
  protected WdlVersion versionCtx;

  @Override
  /** Walk the document root and dispatch to the appropriate element-level callbacks. */
  public void processDocument(WdlDocument node) {
    this.documentCtx = node;
    this.versionCtx = node.getWdlVersion();
    processVersion(node, node.getWdlVersion());
    for (WdlDocumentElement e : node.elements()) {
      if (e instanceof WdlImportStandard) {
        processImport(node, (WdlImportStandard) e);
      } else if (e instanceof WdlImportStar) {
        processImport(node, (WdlImportStar) e);
      } else if (e instanceof WdlImportMembers) {
        processImport(node, (WdlImportMembers) e);
      } else if (e instanceof WdlEnum) {
        processEnum(node, (WdlEnum) e);
      } else if (e instanceof WdlStruct) {
        processStruct(node, (WdlStruct) e);
      } else if (e instanceof WdlTask) {
        processTask(node, (WdlTask) e);
      } else if (e instanceof WdlWorkflow) {
        processWorkflow(node, (WdlWorkflow) e);
      } else {
        processUnexpectedNode(node, e);
      }
    }
  }

  @Override
  public void processVersion(WdlDocument ctx, WdlVersion node) {}

  @Override
  public void processImport(WdlDocument ctx, WdlImportStandard node) {}

  @Override
  public void processImport(WdlDocument ctx, WdlImportMembers node) {}

  @Override
  public void processImport(WdlDocument ctx, WdlImportStar node) {}

  @Override
  public void processEnum(WdlDocument ctx, WdlEnum node) {}

  @Override
  public void processStruct(WdlDocument ctx, WdlStruct node) {
    for (WdlStructElement e : node.elements()) {
      if (e instanceof WdlStructMember) {
        processStructMember(node, (WdlStructMember) e);
      } else if (e instanceof WdlMetadata) {
        processStructMetadata(node, (WdlMetadata) e);
      } else if (e instanceof WdlParameterMetadata) {
        processStructParameterMetadata(node, (WdlParameterMetadata) e);
      } else {
        processUnexpectedNode(node, e);
      }
    }
  }

  @Override
  public void processStructMember(WdlStruct ctx, WdlStructMember node) {}

  @Override
  public void processStructParameterMetadata(WdlStruct ctx, WdlParameterMetadata node) {}

  @Override
  public void processStructMetadata(WdlStruct ctx, WdlMetadata node) {}

  @Override
  public void processTask(WdlDocument ctx, WdlTask node) {
    for (WdlTaskElement e : node.elements()) {
      if (e instanceof WdlBoundDeclaration) {
        processTaskDeclaration(node, (WdlBoundDeclaration) e);
      } else if (e instanceof WdlInput) {
        processTaskInput(node, (WdlInput) e);
      } else if (e instanceof WdlOutput) {
        processTaskOutput(node, (WdlOutput) e);
      } else if (e instanceof WdlCommand) {
        processTaskCommand(node, (WdlCommand) e);
      } else if (e instanceof WdlMetadata) {
        processTaskMetadata(node, (WdlMetadata) e);
      } else if (e instanceof WdlParameterMetadata) {
        processTaskParameterMetadata(node, (WdlParameterMetadata) e);
      } else if (e instanceof WdlRequirements) {
        processTaskRequirements(node, (WdlRequirements) e);
      } else if (e instanceof WdlRuntime) {
        processTaskRuntime(node, (WdlRuntime) e);
      } else if (e instanceof WdlTaskHints) {
        processTaskHints(node, (WdlTaskHints) e);
      } else {
        processUnexpectedNode(node, e);
      }
    }
  }

  @Override
  public void processTaskDeclaration(WdlTask ctx, WdlBoundDeclaration node) {}

  @Override
  public void processTaskInput(WdlTask ctx, WdlInput node) {}

  @Override
  public void processTaskOutput(WdlTask ctx, WdlOutput node) {}

  @Override
  public void processTaskCommand(WdlTask ctx, WdlCommand node) {}

  @Override
  public void processTaskMetadata(WdlTask ctx, WdlMetadata node) {}

  @Override
  public void processTaskParameterMetadata(WdlTask ctx, WdlParameterMetadata node) {}

  @Override
  public void processTaskRequirements(WdlTask ctx, WdlRequirements node) {}

  @Override
  public void processTaskRuntime(WdlTask ctx, WdlRuntime node) {}

  @Override
  public void processTaskHints(WdlTask ctx, WdlTaskHints node) {}

  @Override
  public void processWorkflow(WdlDocument ctx, WdlWorkflow node) {
    for (WdlWorkflowElement e : node.getElements()) {
      if (e instanceof WdlBoundDeclaration) {
        processWorkflowDeclaration(node, (WdlBoundDeclaration) e);
      } else if (e instanceof WdlCall) {
        processWorkflowCall(node, (WdlCall) e);
      } else if (e instanceof WdlConditional) {
        processWorkflowConditional(node, (WdlConditional) e);
      } else if (e instanceof WdlInput) {
        processWorkflowInput(node, (WdlInput) e);
      } else if (e instanceof WdlOutput) {
        processWorkflowOutput(node, (WdlOutput) e);
      } else if (e instanceof WdlMetadata) {
        processWorkflowMetadata(node, (WdlMetadata) e);
      } else if (e instanceof WdlParameterMetadata) {
        processWorkflowParameterMetadata(node, (WdlParameterMetadata) e);
      } else if (e instanceof WdlScatter) {
        processWorkflowScatter(node, (WdlScatter) e);
      } else if (e instanceof WdlWorkflowHints) {
        processWorkflowHints(node, (WdlWorkflowHints) e);
      } else {
        processUnexpectedNode(node, e);
      }
    }
  }

  @Override
  public void processWorkflowDeclaration(WdlWorkflow ctx, WdlBoundDeclaration node) {}

  @Override
  public void processWorkflowCall(WdlWorkflow ctx, WdlCall node) {}

  @Override
  public void processWorkflowConditional(WdlWorkflow ctx, WdlConditional node) {}

  @Override
  public void processWorkflowInput(WdlWorkflow ctx, WdlInput node) {}

  @Override
  public void processWorkflowOutput(WdlWorkflow ctx, WdlOutput node) {}

  @Override
  public void processWorkflowMetadata(WdlWorkflow ctx, WdlMetadata node) {}

  @Override
  public void processWorkflowParameterMetadata(WdlWorkflow ctx, WdlParameterMetadata node) {}

  @Override
  public void processWorkflowScatter(WdlWorkflow ctx, WdlScatter node) {}

  @Override
  public void processWorkflowHints(WdlWorkflow ctx, WdlWorkflowHints node) {}

  public void processUnexpectedNode(WdlNode ctx, WdlNode node) {
    throw new IllegalStateException(
        "Unexpected "
            + node.getClass().getSimpleName()
            + " node (context node:"
            + ctx.getClass().getSimpleName()
            + ")");
  }

  /**
   * Infer an enum's effective value type from its members.
   *
   * <p>If the enum has an explicit declared value type, that type is returned. Otherwise the
   * choice values are merged using WDL-compatible widening (for example Int + Float -> Float).
   * If no explicit values are present, String is assumed.
   */
  protected Optional<WdlType> inferEnumValueType(WdlEnum enumDef) {
    return WdlTypeInference.inferEnumValueType(enumDef);
  }

  /**
   * Infer the static literal type for expression forms that are self-typed.
   *
   * <p>This helper intentionally avoids scope-aware inference and is safe for structural model
   * queries in processors.
   */
  protected WdlType inferLiteralExpressionType(WdlExpression expression) {
    return WdlTypeInference.inferLiteralExpressionType(expression);
  }

  /**
   * Rich import-resolution result that captures both the local visible name and source origin.
   */
  public static final class ResolvedImport<T extends WdlNode> {
    private final String localName;
    private final String importedName;
    private final String importNamespace;
    private final WdlImport importStatement;
    private final WdlDocument importedDocument;
    private final T symbol;

    public ResolvedImport(
        String localName,
        String importedName,
        String importNamespace,
        WdlImport importStatement,
        WdlDocument importedDocument,
        T symbol) {
      this.localName = localName;
      this.importedName = importedName;
      this.importNamespace = importNamespace;
      this.importStatement = importStatement;
      this.importedDocument = importedDocument;
      this.symbol = symbol;
    }

    /** Returns the name visible in the importing document. */
    public String localName() {
      return localName;
    }

    /** Returns the original name in the imported document before aliasing. */
    public String importedName() {
      return importedName;
    }

    /** Returns the import namespace used for standard namespaced calls, if applicable. */
    public String importNamespace() {
      return importNamespace;
    }

    /** Returns the import statement that made this symbol visible. */
    public WdlImport importStatement() {
      return importStatement;
    }

    /** Returns the imported document that defines the resolved symbol. */
    public WdlDocument importedDocument() {
      return importedDocument;
    }

    /** Returns the resolved task, workflow, struct, or enum symbol. */
    public T symbol() {
      return symbol;
    }
  }

  /** Resolve task imports that are visible under the supplied call target (e.g. ns.task or task). */
  protected List<ResolvedImport<WdlTask>> resolveImportedTasks(
      WdlDocument context, String callTarget) {
    if (context == null || callTarget == null || callTarget.isBlank()) {
      return List.of();
    }

    List<ResolvedImport<WdlTask>> results = new ArrayList<>();
    boolean qualified = callTarget.contains(".");
    String namespacePart = qualified ? callTarget.substring(0, callTarget.indexOf('.')) : "";
    String memberPart = qualified ? callTarget.substring(callTarget.indexOf('.') + 1) : callTarget;

    for (WdlImport imp : context.importStatements()) {
      Optional<WdlDocument> importedOpt = resolveImportedDocument(context, imp);
      if (importedOpt.isEmpty()) {
        continue;
      }

      WdlDocument imported = importedOpt.get();
      if (imp instanceof WdlImportStandard) {
        String namespace = importNamespace((WdlImportStandard) imp);
        if (!qualified || !namespace.equals(namespacePart)) {
          continue;
        }
        for (WdlTask task : imported.tasks()) {
          if (Objects.equals(task.getName(), memberPart)) {
            results.add(
                new ResolvedImport<>(
                    namespace + "." + memberPart, memberPart, namespace, imp, imported, task));
          }
        }
      } else if (imp instanceof WdlImportStar) {
        if (qualified) {
          continue;
        }
        for (WdlTask task : imported.tasks()) {
          if (Objects.equals(task.getName(), memberPart)) {
            results.add(new ResolvedImport<>(memberPart, memberPart, null, imp, imported, task));
          }
        }
      } else if (imp instanceof WdlImportMembers) {
        if (qualified) {
          continue;
        }
        for (WdlImportMember member : ((WdlImportMembers) imp).members()) {
          String localName =
              member.getAlias() == null || member.getAlias().isBlank()
                  ? member.getMember()
                  : member.getAlias();
          if (!Objects.equals(localName, memberPart)) {
            continue;
          }
          for (WdlTask task : imported.tasks()) {
            if (Objects.equals(task.getName(), member.getMember())) {
              results.add(
                  new ResolvedImport<>(localName, member.getMember(), null, imp, imported, task));
            }
          }
        }
      }
    }

    return results;
  }

  /**
   * Resolve workflow imports that are visible under the supplied call target (e.g. ns.wf or wf).
   */
  protected List<ResolvedImport<WdlWorkflow>> resolveImportedWorkflows(
      WdlDocument context, String callTarget) {
    if (context == null || callTarget == null || callTarget.isBlank()) {
      return List.of();
    }

    List<ResolvedImport<WdlWorkflow>> results = new ArrayList<>();
    boolean qualified = callTarget.contains(".");
    String namespacePart = qualified ? callTarget.substring(0, callTarget.indexOf('.')) : "";
    String memberPart = qualified ? callTarget.substring(callTarget.indexOf('.') + 1) : callTarget;

    for (WdlImport imp : context.importStatements()) {
      Optional<WdlDocument> importedOpt = resolveImportedDocument(context, imp);
      if (importedOpt.isEmpty()) {
        continue;
      }

      WdlDocument imported = importedOpt.get();
      if (imp instanceof WdlImportStandard) {
        String namespace = importNamespace((WdlImportStandard) imp);
        if (!qualified || !namespace.equals(namespacePart)) {
          continue;
        }
        for (WdlWorkflow workflow : imported.workflows()) {
          if (Objects.equals(workflow.getName(), memberPart)) {
            results.add(
                new ResolvedImport<>(
                    namespace + "." + memberPart, memberPart, namespace, imp, imported, workflow));
          }
        }
      } else if (imp instanceof WdlImportStar) {
        if (qualified) {
          continue;
        }
        for (WdlWorkflow workflow : imported.workflows()) {
          if (Objects.equals(workflow.getName(), memberPart)) {
            results.add(
                new ResolvedImport<>(memberPart, memberPart, null, imp, imported, workflow));
          }
        }
      } else if (imp instanceof WdlImportMembers) {
        if (qualified) {
          continue;
        }
        for (WdlImportMember member : ((WdlImportMembers) imp).members()) {
          String localName =
              member.getAlias() == null || member.getAlias().isBlank()
                  ? member.getMember()
                  : member.getAlias();
          if (!Objects.equals(localName, memberPart)) {
            continue;
          }
          for (WdlWorkflow workflow : imported.workflows()) {
            if (Objects.equals(workflow.getName(), member.getMember())) {
              results.add(
                  new ResolvedImport<>(
                      localName, member.getMember(), null, imp, imported, workflow));
            }
          }
        }
      }
    }

    return results;
  }

  /** Resolve struct definitions copied into the current document namespace by imports. */
  protected List<ResolvedImport<WdlStruct>> resolveImportedStructs(
      WdlDocument context, String visibleTypeName) {
    return resolveImportedTypes(context, visibleTypeName, true);
  }

  /** Resolve enum definitions copied into the current document namespace by imports. */
  protected List<ResolvedImport<WdlEnum>> resolveImportedEnums(
      WdlDocument context, String visibleTypeName) {
    return resolveImportedTypes(context, visibleTypeName, false);
  }

  /**
   * Resolve imported type definitions by local visible name for either structs or enums.
   */
  private <T extends WdlNode> List<ResolvedImport<T>> resolveImportedTypes(
      WdlDocument context, String visibleTypeName, boolean structs) {
    if (context == null || visibleTypeName == null || visibleTypeName.isBlank()) {
      return List.of();
    }

    List<ResolvedImport<T>> results = new ArrayList<>();
    for (WdlImport imp : context.importStatements()) {
      Optional<WdlDocument> importedOpt = resolveImportedDocument(context, imp);
      if (importedOpt.isEmpty()) {
        continue;
      }
      WdlDocument imported = importedOpt.get();

      if (imp instanceof WdlImportStandard) {
        Map<String, String> aliases = importAliases((WdlImportStandard) imp);
        if (structs) {
          for (WdlStruct struct : imported.structs()) {
            String importedName = struct.getName();
            String localName = aliases.getOrDefault(importedName, importedName);
            if (Objects.equals(localName, visibleTypeName)) {
              results.add(
                  new ResolvedImport<>(localName, importedName, null, imp, imported, (T) struct));
            }
          }
        } else {
          for (WdlEnum en : imported.enums()) {
            String importedName = en.getName();
            String localName = aliases.getOrDefault(importedName, importedName);
            if (Objects.equals(localName, visibleTypeName)) {
              results.add(
                  new ResolvedImport<>(localName, importedName, null, imp, imported, (T) en));
            }
          }
        }
      } else if (imp instanceof WdlImportStar) {
        if (structs) {
          for (WdlStruct struct : imported.structs()) {
            if (Objects.equals(struct.getName(), visibleTypeName)) {
              results.add(
                  new ResolvedImport<>(
                      visibleTypeName, visibleTypeName, null, imp, imported, (T) struct));
            }
          }
        } else {
          for (WdlEnum en : imported.enums()) {
            if (Objects.equals(en.getName(), visibleTypeName)) {
              results.add(
                  new ResolvedImport<>(
                      visibleTypeName, visibleTypeName, null, imp, imported, (T) en));
            }
          }
        }
      } else if (imp instanceof WdlImportMembers) {
        for (WdlImportMember member : ((WdlImportMembers) imp).members()) {
          String localName =
              member.getAlias() == null || member.getAlias().isBlank()
                  ? member.getMember()
                  : member.getAlias();
          if (!Objects.equals(localName, visibleTypeName)) {
            continue;
          }
          if (structs) {
            for (WdlStruct struct : imported.structs()) {
              if (Objects.equals(struct.getName(), member.getMember())) {
                results.add(
                    new ResolvedImport<>(
                        localName, member.getMember(), null, imp, imported, (T) struct));
              }
            }
          } else {
            for (WdlEnum en : imported.enums()) {
              if (Objects.equals(en.getName(), member.getMember())) {
                results.add(
                    new ResolvedImport<>(
                        localName, member.getMember(), null, imp, imported, (T) en));
              }
            }
          }
        }
      }
    }

    return results;
  }

  /** Returns the namespace used for a standard import. */
  protected String importNamespace(WdlImportStandard imp) {
    if (imp.getAlias() != null && !imp.getAlias().isBlank()) {
      return imp.getAlias();
    }

    String source = importSourceText(imp);
    if (source.isBlank()) {
      return "";
    }

    String path = source;
    try {
      URI uri = URI.create(source);
      if (uri.getPath() != null && !uri.getPath().isBlank()) {
        path = uri.getPath();
      }
    } catch (RuntimeException ignored) {
      // Use the raw source literal when URI parsing fails.
    }

    int idx = path.lastIndexOf('/');
    String basename = idx >= 0 ? path.substring(idx + 1) : path;
    if (basename.endsWith(".wdl") && basename.length() > 4) {
      basename = basename.substring(0, basename.length() - 4);
    }
    return basename;
  }

  /** Resolve the imported document model for a specific import statement. */
  protected Optional<WdlDocument> resolveImportedDocument(WdlDocument context, WdlImport imp) {
    if (context == null || imp == null) {
      return Optional.empty();
    }
    String key = imp.getImportIdentifier();
    if (key == null || key.isBlank()) {
      return Optional.empty();
    }
    return Optional.ofNullable(context.importedDocuments().get(key));
  }

  private Map<String, String> importAliases(WdlImportStandard imp) {
    Map<String, String> aliases = new HashMap<>();
    for (WdlImportMember member : imp.members()) {
      if (member.getMember() == null || member.getMember().isBlank()) {
        continue;
      }
      String alias =
          member.getAlias() == null || member.getAlias().isBlank()
              ? member.getMember()
              : member.getAlias();
      aliases.put(member.getMember(), alias);
    }
    return aliases;
  }

  /**
   * Extract the raw import source text from the import literal components.
   */
  private String importSourceText(WdlImport imp) {
    if (imp == null || imp.getSource() == null) {
      return "";
    }
    StringBuilder text = new StringBuilder();
    for (WdlStringComponent component : imp.getSource().components()) {
      if (component instanceof WdlStringText) {
        text.append(((WdlStringText) component).getText());
      } else if (component instanceof WdlStringEscape) {
        text.append(((WdlStringEscape) component).getEscapeText());
      }
    }
    return text.toString();
  }

  /** Render a keyed expression entry back into WDL syntax using the supplied delimiter. */
  protected String keyValueToWdl(WdlStringKeyValue item, String delimiter) {
    if (item.getValue() == null) {
      return item.getKey();
    }
    return item.getKey() + delimiter + expressionToWdl(item.getValue());
  }

  protected String keyValueToWdl(WdlStringKeyValue item) {
    return keyValueToWdl(item, ": ");
  }

  /** Render a declaration node back into WDL syntax. */
  protected String declarationToWdl(WdlDeclaration declartaion) {
    StringBuilder str = new StringBuilder();
    str.append(typeToWdl(declartaion.getType())).append(" ").append(declartaion.getName());
    if (declartaion instanceof WdlBoundDeclaration) {
      str.append(" = ")
          .append(expressionToWdl(((WdlBoundDeclaration) declartaion).getExpression()));
    }
    return str.toString();
  }

  /** Render an expression subtree back into WDL syntax. */
  protected String expressionToWdl(WdlExpression expr) {
    switch (expr.componentType()) {
      case BOOL_LIT:
      case INT_LIT:
      case FLOAT_LIT:
        return Objects.toString(((WdlValueExpression<?>) expr).getValue());
      case ARRAY_LIT:
        return "["
            + String.join(
                ", ",
                ((WdlArrayLiteral) expr)
                    .entries().stream().map(ae -> expressionToWdl(ae)).collect(Collectors.toList()))
            + "]";
      case MAP_LIT:
        return "{"
            + String.join(
                ", ",
                ((WdlMapLiteral) expr)
                    .entries().stream()
                        .map(
                            me ->
                                expressionToWdl(me.getKey())
                                    + ": "
                                    + expressionToWdl(me.getValue()))
                        .collect(Collectors.toList()))
            + "}";
      case NULL_LIT:
        return "None";
      case OBJ_LIT:
        {
          WdlObjectLiteral e = (WdlObjectLiteral) expr;
          return "{"
              + String.join(
                  ", ",
                  e.entries().stream()
                      .map(oe -> oe.getKey() + ": " + expressionToWdl(oe.getValue()))
                      .collect(Collectors.toList()))
              + "}";
        }
      case PAIR_LIT:
        return "("
            + expressionToWdl(((WdlPairLiteral) expr).getLeft())
            + ", "
            + expressionToWdl(((WdlPairLiteral) expr).getRight())
            + ")";
      case STR_LIT:
        return stringLiteralToWdl((WdlStringLiteral) expr, true);
      case STRUCT_LIT:
        {
          WdlStructLiteral e = (WdlStructLiteral) expr;
          return e.getName()
              + " {"
              + String.join(
                  ", ",
                  e.entries().stream()
                      .map(se -> se.getKey() + ": " + expressionToWdl(se.getValue()))
                      .collect(Collectors.toList()))
              + "}";
        }
      case VARIABLE:
        return ((WdlVariable) expr).getName();
      case BINARY_OP:
        {
          WdlBinaryOperation e = (WdlBinaryOperation) expr;
          return expressionToWdl(e.getLeft())
              + " "
              + e.getOperator().getWdlString()
              + " "
              + expressionToWdl(e.getRight());
        }
      case FUNC_OP:
        {
          WdlFunctionCallOperation e = (WdlFunctionCallOperation) expr;
          return e.getFunctionName()
              + "("
              + String.join(
                  ", ",
                  e.arguments().stream().map(a -> expressionToWdl(a)).collect(Collectors.toList()))
              + ")";
        }
      case IDX_OP:
        {
          WdlIndexAccessOperation e = (WdlIndexAccessOperation) expr;
          return expressionToWdl(e.getTarget()) + "[" + expressionToWdl(e.getIndex()) + "]";
        }
      case MEMBER_OP:
        {
          WdlMemberAccessOperation e = (WdlMemberAccessOperation) expr;
          return expressionToWdl(e.getTarget()) + "." + e.getMember();
        }
      case TERNARY_OP:
        {
          WdlTernaryOperation e = (WdlTernaryOperation) expr;
          return "if ("
              + expressionToWdl(e.getCondition())
              + ") "
              + expressionToWdl(e.getTrueValue())
              + " else "
              + expressionToWdl(e.getFalseValue());
        }
      case UNARY_OP:
        {
          WdlUnaryOperation e = (WdlUnaryOperation) expr;
          return e.getOperator().getWdlString() + expressionToWdl(e.getOperand());
        }
      default:
        throw new IllegalStateException(
            "Unhandled expression component type: " + expr.componentType());
    }
  }

  protected String stringLiteralToWdl(WdlStringLiteral strLit, boolean quote) {
    String startQuote = "";
    String endQuote = "";
    if (quote) {
      if (strLit.getDelimiter() == Delimiter.SINGLE_QUOTE) {
        startQuote = "'";
        endQuote = "'";
      } else if (strLit.getDelimiter() == Delimiter.DOUBLE_QUOTE) {
        startQuote = "\"";
        endQuote = "\"";
      } else if (strLit.getDelimiter() == Delimiter.MULTILINE) {
        startQuote = ">>>";
        endQuote = "<<<";
      }
    }
    StringBuilder str = new StringBuilder(startQuote);
    for (WdlStringComponent component : strLit.components()) {
      switch (component.componentType()) {
        case TEXT:
          str.append(((WdlStringText) component).getText());
          break;
        case ESC:
          str.append(((WdlStringEscape) component).getEscapeText());
          break;
        case SPECIAL:
          str.append(((WdlStringToken) component).getTokenText());
          break;
        case PLACEHOLDER:
          {
            WdlStringPlaceholder s = (WdlStringPlaceholder) component;
            str.append(s.getSymbol().getWdlString()).append("{");
            if (s.getOption() != null) {
              switch (s.getOption().getType()) {
                case TRUE_FALSE:
                  str.append("true=")
                      .append(stringLiteralToWdl(s.getOption().getTrueValue(), true))
                      .append(" false=")
                      .append(stringLiteralToWdl(s.getOption().getFalseValue(), true))
                      .append(" ");
                  break;
                case FALSE_TRUE:
                  str.append("false=")
                      .append(stringLiteralToWdl(s.getOption().getFalseValue(), true))
                      .append(" true=")
                      .append(stringLiteralToWdl(s.getOption().getTrueValue(), true))
                      .append(" ");
                  break;
                case SEP:
                  str.append("sep=")
                      .append(stringLiteralToWdl(s.getOption().getValue(), true))
                      .append(" ");
                  break;
                case DEFAULT:
                  str.append("default=")
                      .append(stringLiteralToWdl(s.getOption().getValue(), true))
                      .append(" ");
                  break;
                default:
                  throw new IllegalStateException();
              }
            }
            str.append(expressionToWdl(s.getExpression()));
            str.append("}");
            break;
          }
        default:
          throw new IllegalStateException(
              "Unhandled string component type: " + component.componentType());
      }
    }
    return str.append(endQuote).toString();
  }

  protected String typeToWdl(WdlType type) {
    StringBuilder str = new StringBuilder();
    switch (type.componentType()) {
      case PRIMITIVE:
        {
          str.append(((WdlPrimitiveType) type).primitiveType().toWdlString());
          break;
        }
      case ARRAY:
        {
          WdlArrayType t = (WdlArrayType) type;
          str.append("Array[").append(typeToWdl(t.memberType())).append("]");
          if (t.isNonEmpty()) {
            str.append("+");
          }
          break;
        }
      case TYPEREF:
        {
          str.append(((WdlTypeReferenceType) type).referenceName());
          break;
        }
      case PAIR:
        {
          WdlPairType t = (WdlPairType) type;
          str.append("Pair[")
              .append(typeToWdl(t.leftType()))
              .append(",")
              .append(typeToWdl(t.rightType()))
              .append("]");
          break;
        }
      case MAP:
        {
          WdlMapType t = (WdlMapType) type;
          str.append("Map[")
              .append(typeToWdl(t.keyType()))
              .append(", ")
              .append(typeToWdl(t.valueType()))
              .append("]");
          break;
        }
      default:
        throw new IllegalStateException("Unhandled string component type: " + type.componentType());
    }
    str.append(type.isOptional() ? "?" : "");
    return str.toString();
  }
}
