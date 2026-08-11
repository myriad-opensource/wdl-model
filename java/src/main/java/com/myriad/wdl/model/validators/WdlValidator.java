package com.myriad.wdl.model.validators;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.errors.WdlException;
import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.expressions.WdlArrayLiteral;
import com.myriad.wdl.model.processors.WdlProcessorBase;
import com.myriad.wdl.model.sections.WdlInput;
import com.myriad.wdl.model.sections.WdlOutput;
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
import com.myriad.wdl.model.types.WdlTypeInference;
import com.myriad.wdl.model.types.WdlTypeReferenceType;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;
import java.util.function.BiConsumer;

/**
 * Baseline semantic validator for Java WDL documents.
 *
 * <p>This validator performs the non-optional checks that should fail ordinary document loading,
 * including declaration assignability, required and private call input enforcement, member and
 * index access validation, and version-gated function availability.
 *
 * <p>Representative failures are covered by spec fixtures such as
 * {@code spec_examples/v1_3/private_declaration_fail.wdl},
 * {@code spec_examples/v1_3/select_first_empty_fail.wdl}, and
 * {@code spec_examples/v1_3/write_json_fail.wdl}.
 */
public class WdlValidator extends WdlProcessorBase {

  private final List<WdlSemanticError> errors = new ArrayList<>();
  private final Map<String, CallableContract> callableContracts = new HashMap<>();
  private final Map<String, Set<String>> structMembers = new HashMap<>();
  private final Map<String, Map<String, WdlType>> structMemberTypes = new HashMap<>();
  private final Map<String, EnumShape> enumShapes = new HashMap<>();
  private final Map<String, WdlType> enumValueTypes = new HashMap<>();
  private final Map<String, Set<String>> enumChoiceNames = new HashMap<>();

  private Map<String, WdlType> scopeTypes = new HashMap<>();
  private Map<String, Object> scopeValues = new HashMap<>();
  private Map<String, Set<String>> callOutputs = new HashMap<>();
  private Map<String, Map<String, WdlType>> callOutputTypes = new HashMap<>();
  private WdlExpressionValidator expressionValidator;
  private WdlVersion currentDocumentVersion;
  private boolean throwOnWarnings = true;

  /**
   * Validates a document and throws when errors exist, or when warning-only diagnostics exist and
   * {@link #isThrowOnWarnings()} is enabled.
   */
  public void validate(WdlDocument document) throws WdlException {
    errors.clear();
    currentDocumentVersion = document == null ? null : document.getWdlVersion();
    indexTopLevelContracts(document);
    processDocument(document);
    if (shouldThrowForCollectedDiagnostics()) {
      throw new WdlException(errors);
    }
  }

  /**
   * Controls whether warning-only validation results should throw.
   *
   * <p>This is mainly useful with {@link WdlLintingValidator}, where callers may want to collect
   * warnings without failing the current pass.
   */
  public final WdlValidator setThrowOnWarnings(boolean throwOnWarnings) {
    this.throwOnWarnings = throwOnWarnings;
    return this;
  }

  /** Returns the current warning throw policy. */
  public final boolean isThrowOnWarnings() {
    return throwOnWarnings;
  }

  private boolean shouldThrowForCollectedDiagnostics() {
    if (errors.isEmpty()) {
      return false;
    }
    boolean hasError =
        errors.stream().anyMatch(e -> e.severity() == WdlSemanticError.Severity.ERROR);
    if (hasError) {
      return true;
    }
    boolean hasWarning =
        errors.stream().anyMatch(e -> e.severity() == WdlSemanticError.Severity.WARNING);
    return throwOnWarnings && hasWarning;
  }

  private void indexTopLevelContracts(WdlDocument document) {
    callableContracts.clear();
    structMembers.clear();
    structMemberTypes.clear();
    enumShapes.clear();
    enumValueTypes.clear();
    enumChoiceNames.clear();

    if (document == null) {
      return;
    }

    validateImports(document);

    for (WdlDocument.WdlDocumentElement element : document.elements()) {
      if (element instanceof WdlTask) {
        indexLocalTask((WdlTask) element);
      } else if (element instanceof WdlWorkflow) {
        indexLocalWorkflow((WdlWorkflow) element);
      } else if (element instanceof WdlStruct) {
        indexLocalStruct((WdlStruct) element);
      } else if (element instanceof WdlEnum) {
        indexLocalEnum((WdlEnum) element);
      }
    }
  }

  private void validateImports(WdlDocument document) {
    Set<String> namespaces = new HashSet<>();
    Set<String> localTaskWorkflowNames = new HashSet<>();

    for (WdlDocument.WdlDocumentElement element : document.elements()) {
      if (element instanceof WdlTask && ((WdlTask) element).getName() != null) {
        localTaskWorkflowNames.add(((WdlTask) element).getName());
      } else if (element instanceof WdlWorkflow && ((WdlWorkflow) element).getName() != null) {
        localTaskWorkflowNames.add(((WdlWorkflow) element).getName());
      }
    }

    for (WdlImport imp : document.importStatements()) {
      Optional<WdlDocument> importedOpt = resolveImportedDocument(document, imp);
      if (importedOpt.isEmpty()) {
        continue;
      }
      WdlDocument imported = importedOpt.get();

      validateImportedVersionCompatibility(document, imported, imp);

      if (imp instanceof WdlImportStandard) {
        String namespace = importNamespace((WdlImportStandard) imp);
        if (namespace == null || namespace.isBlank()) {
          addError(
              WdlSemanticError.Code.GENERIC_SEMANTIC_ERROR,
              "Import namespace is empty for source '" + renderImportSource(imp) + "'");
        } else {
          if (!namespaces.add(namespace)) {
            addError(
                WdlSemanticError.Code.DUPLICATE_DEFINITION,
                "Duplicate import namespace '" + namespace + "'");
          }
          if (localTaskWorkflowNames.contains(namespace)) {
            addError(
                WdlSemanticError.Code.DUPLICATE_DEFINITION,
                "Import namespace '" + namespace + "' conflicts with local task/workflow name");
          }
        }
        validateStandardImportAliases((WdlImportStandard) imp, imported);
      } else if (imp instanceof WdlImportMembers) {
        validateMemberImportMembers((WdlImportMembers) imp, imported);
      }

      indexImportedCallables(document, imp, imported);
      indexImportedStructs(document, imp, imported);
      indexImportedEnums(document, imp, imported);
    }
  }

  private void validateImportedVersionCompatibility(
      WdlDocument importer, WdlDocument imported, WdlImport imp) {
    WdlVersion importingVersion = importer.getWdlVersion();
    WdlVersion importedVersion = imported.getWdlVersion();
    if (importingVersion == null || importedVersion == null) {
      return;
    }
    if (importingVersion.getMajor() != importedVersion.getMajor()
        || importedVersion.getMinor() > importingVersion.getMinor()) {
      addError(
          WdlSemanticError.Code.GENERIC_SEMANTIC_ERROR,
          "Imported document version "
              + importedVersion.getVersionString()
              + " is not compatible with importer version "
              + importingVersion.getVersionString()
              + " for source '"
              + renderImportSource(imp)
              + "'");
    }
  }

  private void validateStandardImportAliases(WdlImportStandard imp, WdlDocument imported) {
    Set<String> structNames = new HashSet<>();
    for (WdlStruct struct : imported.structs()) {
      if (struct.getName() != null) {
        structNames.add(struct.getName());
      }
    }
    Set<String> enumNames = new HashSet<>();
    for (WdlEnum en : imported.enums()) {
      if (en.getName() != null) {
        enumNames.add(en.getName());
      }
    }

    Set<String> aliasTargets = new HashSet<>();
    Set<String> aliasNames = new HashSet<>();
    for (WdlImportMember member : imp.members()) {
      String sourceName = member.getMember();
      if (sourceName == null || sourceName.isBlank()) {
        continue;
      }

      boolean exists = structNames.contains(sourceName) || enumNames.contains(sourceName);
      if (!exists) {
        addError(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "Import alias source '"
                + sourceName
                + "' is not a struct or enum in imported document '"
                + renderImportSource(imp)
                + "'");
      }
      if (!aliasTargets.add(sourceName)) {
        addError(
            WdlSemanticError.Code.DUPLICATE_DEFINITION,
            "Import alias source '" + sourceName + "' is aliased more than once");
      }

      String alias =
          member.getAlias() == null || member.getAlias().isBlank() ? sourceName : member.getAlias();
      if (!aliasNames.add(alias)) {
        addError(
            WdlSemanticError.Code.DUPLICATE_DEFINITION,
            "Import alias destination '" + alias + "' is used more than once");
      }
    }
  }

  private void validateMemberImportMembers(WdlImportMembers imp, WdlDocument imported) {
    Set<String> exported = new HashSet<>();
    imported.tasks().forEach(t -> addIfNonBlank(exported, t.getName()));
    imported.workflows().forEach(w -> addIfNonBlank(exported, w.getName()));
    imported.structs().forEach(s -> addIfNonBlank(exported, s.getName()));
    imported.enums().forEach(e -> addIfNonBlank(exported, e.getName()));

    Set<String> localNames = new HashSet<>();
    for (WdlImportMember member : imp.members()) {
      if (member.getMember() == null || member.getMember().isBlank()) {
        continue;
      }
      if (!exported.contains(member.getMember())) {
        addError(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "Import member '"
                + member.getMember()
                + "' does not exist in imported document '"
                + renderImportSource(imp)
                + "'");
      }
      String localName =
          member.getAlias() == null || member.getAlias().isBlank()
              ? member.getMember()
              : member.getAlias();
      if (!localNames.add(localName)) {
        addError(
            WdlSemanticError.Code.DUPLICATE_DEFINITION,
            "Import member local name '" + localName + "' is used more than once");
      }
    }
  }

  private void addIfNonBlank(Set<String> names, String value) {
    if (value != null && !value.isBlank()) {
      names.add(value);
    }
  }

  private void indexLocalTask(WdlTask task) {
    if (task.getName() == null) {
      return;
    }
    CallableContract contract = buildTaskContract(task);
    indexCallableContract(task.getName(), contract, "task");
  }

  private void indexLocalWorkflow(WdlWorkflow workflow) {
    if (workflow.getName() == null) {
      return;
    }
    CallableContract contract = buildWorkflowContract(workflow);
    indexCallableContract(workflow.getName(), contract, "workflow");
  }

  private void indexImportedCallables(WdlDocument context, WdlImport imp, WdlDocument imported) {
    if (imp instanceof WdlImportStandard) {
      String namespace = importNamespace((WdlImportStandard) imp);
      if (namespace == null || namespace.isBlank()) {
        return;
      }
      for (WdlTask task : imported.tasks()) {
        if (task.getName() == null || task.getName().isBlank()) {
          continue;
        }
        indexCallableContract(
            namespace + "." + task.getName(), buildTaskContract(task), "imported task");
      }
      for (WdlWorkflow workflow : imported.workflows()) {
        if (workflow.getName() == null || workflow.getName().isBlank()) {
          continue;
        }
        indexCallableContract(
            namespace + "." + workflow.getName(),
            buildWorkflowContract(workflow),
            "imported workflow");
      }
      return;
    }

    if (imp instanceof WdlImportStar) {
      for (WdlTask task : imported.tasks()) {
        if (task.getName() == null || task.getName().isBlank()) {
          continue;
        }
        indexCallableContract(task.getName(), buildTaskContract(task), "imported task");
      }
      for (WdlWorkflow workflow : imported.workflows()) {
        if (workflow.getName() == null || workflow.getName().isBlank()) {
          continue;
        }
        indexCallableContract(
            workflow.getName(), buildWorkflowContract(workflow), "imported workflow");
      }
      return;
    }

    if (imp instanceof WdlImportMembers) {
      WdlImportMembers membersImport = (WdlImportMembers) imp;
      Map<String, WdlTask> taskByName = new HashMap<>();
      imported.tasks().forEach(t -> taskByName.put(t.getName(), t));
      Map<String, WdlWorkflow> workflowByName = new HashMap<>();
      imported.workflows().forEach(w -> workflowByName.put(w.getName(), w));

      for (WdlImportMember member : membersImport.members()) {
        String memberName = member.getMember();
        if (memberName == null || memberName.isBlank()) {
          continue;
        }
        String localName =
            member.getAlias() == null || member.getAlias().isBlank()
                ? memberName
                : member.getAlias();
        WdlTask task = taskByName.get(memberName);
        if (task != null) {
          indexCallableContract(localName, buildTaskContract(task), "imported task");
        }
        WdlWorkflow workflow = workflowByName.get(memberName);
        if (workflow != null) {
          indexCallableContract(localName, buildWorkflowContract(workflow), "imported workflow");
        }
      }
    }
  }

  private void indexImportedStructs(WdlDocument context, WdlImport imp, WdlDocument imported) {
    Map<String, String> typeAliases = importTypeAliasMap(imp);
    for (WdlStruct struct : imported.structs()) {
      if (struct.getName() == null || struct.getName().isBlank()) {
        continue;
      }
      String localName = resolveImportedTypeLocalName(imp, struct.getName());
      if (localName == null || localName.isBlank()) {
        continue;
      }
      StructShape incoming = toImportedStructShape(struct, typeAliases);
      StructShape existing = structShapeFor(localName);
      if (existing == null) {
        structMembers.put(localName, incoming.memberNamesSet());
        structMemberTypes.put(localName, incoming.memberTypesMap());
      } else if (!existing.isCompatibleWith(incoming)) {
        addError(
            WdlSemanticError.Code.TYPE_MISMATCH,
            "Imported struct '"
                + localName
                + "' has incompatible definitions across imports; use aliases to disambiguate");
      }
    }
  }

  private void indexImportedEnums(WdlDocument context, WdlImport imp, WdlDocument imported) {
    for (WdlEnum en : imported.enums()) {
      if (en.getName() == null || en.getName().isBlank()) {
        continue;
      }
      String localName = resolveImportedTypeLocalName(imp, en.getName());
      if (localName == null || localName.isBlank()) {
        continue;
      }
      EnumShape incoming = EnumShape.from(en, this::typeToWdl, this::expressionToWdl);
      EnumShape existing = enumShapes.get(localName);
      if (existing == null) {
        enumShapes.put(localName, incoming);
        enumValueTypes.put(localName, effectiveEnumValueType(en));
        enumChoiceNames.put(localName, enumChoiceNameSet(en));
      } else if (!existing.isCompatibleWith(incoming)) {
        addError(
            WdlSemanticError.Code.TYPE_MISMATCH,
            "Imported enum '"
                + localName
                + "' has incompatible definitions across imports; use aliases to disambiguate");
      } else {
        WdlType existingType = enumValueTypes.get(localName);
        WdlType incomingType = effectiveEnumValueType(en);
        if (existingType != null
            && incomingType != null
            && !Objects.equals(typeToWdl(existingType), typeToWdl(incomingType))) {
          addError(
              WdlSemanticError.Code.TYPE_MISMATCH,
              "Imported enum '"
                  + localName
                  + "' has incompatible value types across imports; use aliases to disambiguate");
        }
      }
    }
  }

  private String resolveImportedTypeLocalName(WdlImport imp, String importedTypeName) {
    if (imp instanceof WdlImportStandard) {
      for (WdlImportMember alias : ((WdlImportStandard) imp).members()) {
        if (Objects.equals(alias.getMember(), importedTypeName)) {
          return alias.getAlias() == null || alias.getAlias().isBlank()
              ? importedTypeName
              : alias.getAlias();
        }
      }
      return importedTypeName;
    }

    if (imp instanceof WdlImportStar) {
      return importedTypeName;
    }

    if (imp instanceof WdlImportMembers) {
      for (WdlImportMember member : ((WdlImportMembers) imp).members()) {
        if (Objects.equals(member.getMember(), importedTypeName)) {
          return member.getAlias() == null || member.getAlias().isBlank()
              ? importedTypeName
              : member.getAlias();
        }
      }
      return null;
    }

    return null;
  }

  private StructShape structShapeFor(String structName) {
    Set<String> members = structMembers.get(structName);
    Map<String, WdlType> memberTypes = structMemberTypes.get(structName);
    if (members == null || memberTypes == null) {
      return null;
    }
    return StructShape.from(memberTypes, this::typeToWdl);
  }

  private void indexLocalStruct(WdlStruct struct) {
    if (struct.getName() == null || struct.getName().isBlank()) {
      return;
    }

    StructShape shape = toStructShape(struct);
    StructShape existing = structShapeFor(struct.getName());
    if (existing != null && !existing.isCompatibleWith(shape)) {
      addError(
          WdlSemanticError.Code.TYPE_MISMATCH,
          "Struct '"
              + struct.getName()
              + "' conflicts with imported struct definition; alias imported structs to"
              + " disambiguate");
      return;
    }
    structMembers.put(struct.getName(), shape.memberNamesSet());
    structMemberTypes.put(struct.getName(), shape.memberTypesMap());
  }

  private StructShape toStructShape(WdlStruct struct) {
    LinkedHashMap<String, String> orderedTypes = new LinkedHashMap<>();
    LinkedHashMap<String, WdlType> orderedTypeObjects = new LinkedHashMap<>();
    for (WdlStruct.WdlStructElement structElement : struct.elements()) {
      if (structElement instanceof WdlStruct.WdlStructMember) {
        WdlStruct.WdlStructMember member = (WdlStruct.WdlStructMember) structElement;
        if (member.getName() != null) {
          orderedTypes.put(member.getName(), typeToWdl(member.getType()));
          orderedTypeObjects.put(member.getName(), member.getType());
        }
      }
    }
    return new StructShape(orderedTypes, orderedTypeObjects);
  }

  private StructShape toImportedStructShape(WdlStruct struct, Map<String, String> typeAliases) {
    LinkedHashMap<String, String> orderedTypes = new LinkedHashMap<>();
    LinkedHashMap<String, WdlType> orderedTypeObjects = new LinkedHashMap<>();
    for (WdlStruct.WdlStructElement structElement : struct.elements()) {
      if (structElement instanceof WdlStruct.WdlStructMember) {
        WdlStruct.WdlStructMember member = (WdlStruct.WdlStructMember) structElement;
        if (member.getName() != null) {
          WdlType rewritten = rewriteTypeAliases(member.getType(), typeAliases);
          orderedTypes.put(member.getName(), typeToWdl(rewritten));
          orderedTypeObjects.put(member.getName(), rewritten);
        }
      }
    }
    return new StructShape(orderedTypes, orderedTypeObjects);
  }

  private Map<String, String> importTypeAliasMap(WdlImport imp) {
    Map<String, String> aliases = new HashMap<>();
    if (imp instanceof WdlImportStandard) {
      for (WdlImportMember member : ((WdlImportStandard) imp).members()) {
        if (member.getMember() == null || member.getMember().isBlank()) {
          continue;
        }
        String local =
            member.getAlias() == null || member.getAlias().isBlank()
                ? member.getMember()
                : member.getAlias();
        aliases.put(member.getMember(), local);
      }
    } else if (imp instanceof WdlImportMembers) {
      for (WdlImportMember member : ((WdlImportMembers) imp).members()) {
        if (member.getMember() == null || member.getMember().isBlank()) {
          continue;
        }
        String local =
            member.getAlias() == null || member.getAlias().isBlank()
                ? member.getMember()
                : member.getAlias();
        aliases.put(member.getMember(), local);
      }
    }
    return aliases;
  }

  private WdlType rewriteTypeAliases(WdlType type, Map<String, String> aliases) {
    if (type == null || aliases == null || aliases.isEmpty()) {
      return type;
    }
    if (type instanceof WdlTypeReferenceType) {
      WdlTypeReferenceType ref = (WdlTypeReferenceType) type;
      String rewritten = aliases.getOrDefault(ref.referenceName(), ref.referenceName());
      return new WdlTypeReferenceType(rewritten, ref.isOptional());
    }
    if (type instanceof WdlArrayType) {
      WdlArrayType arr = (WdlArrayType) type;
      return new WdlArrayType(
          rewriteTypeAliases(arr.memberType(), aliases), arr.isOptional(), arr.isNonEmpty());
    }
    if (type instanceof WdlPairType) {
      WdlPairType pair = (WdlPairType) type;
      return new WdlPairType(
          rewriteTypeAliases(pair.leftType(), aliases),
          rewriteTypeAliases(pair.rightType(), aliases),
          pair.isOptional());
    }
    if (type instanceof WdlMapType) {
      WdlMapType map = (WdlMapType) type;
      return new WdlMapType(
          rewriteTypeAliases(map.keyType(), aliases),
          rewriteTypeAliases(map.valueType(), aliases),
          map.isOptional());
    }
    return type;
  }

  private void indexLocalEnum(WdlEnum en) {
    if (en.getName() == null || en.getName().isBlank()) {
      return;
    }
    EnumShape incoming = EnumShape.from(en, this::typeToWdl, this::expressionToWdl);
    EnumShape existing = enumShapes.get(en.getName());
    if (existing != null && !existing.isCompatibleWith(incoming)) {
      addError(
          WdlSemanticError.Code.TYPE_MISMATCH,
          "Enum '"
              + en.getName()
              + "' conflicts with imported enum definition; alias imported enums to disambiguate");
      return;
    }
    enumShapes.put(en.getName(), incoming);
    enumValueTypes.put(en.getName(), effectiveEnumValueType(en));
    enumChoiceNames.put(en.getName(), enumChoiceNameSet(en));
  }

  private WdlType effectiveEnumValueType(WdlEnum en) {
    return WdlTypeInference.inferEnumValueType(en)
        .orElseGet(() -> new WdlPrimitiveType(WdlPrimitiveType.Type.STRING, false));
  }

  private Set<String> enumChoiceNameSet(WdlEnum en) {
    Set<String> names = new LinkedHashSet<>();
    en.elements().forEach(choice -> addIfNonBlank(names, choice.getKey()));
    return names;
  }

  private CallableContract buildTaskContract(WdlTask task) {
    Set<String> requiredInputs = new HashSet<>();
    Set<String> outputs = new HashSet<>();
    Set<String> privateDeclarations = new HashSet<>();
    Map<String, WdlType> inputTypes = new HashMap<>();
    Map<String, WdlType> outputTypes = new HashMap<>();

    for (WdlTask.WdlTaskElement taskElement : task.elements()) {
      if (taskElement instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) taskElement).elements()) {
          if (declaration.getName() != null) {
            inputTypes.put(declaration.getName(), declaration.getType());
            if (!(declaration instanceof WdlBoundDeclaration)
                && declaration.getType() != null
                && !declaration.getType().isOptional()) {
              requiredInputs.add(declaration.getName());
            }
          }
        }
      } else if (taskElement instanceof WdlOutput) {
        for (WdlBoundDeclaration declaration : ((WdlOutput) taskElement).elements()) {
          if (declaration.getName() != null) {
            outputs.add(declaration.getName());
            outputTypes.put(declaration.getName(), declaration.getType());
          }
        }
      } else if (taskElement instanceof WdlBoundDeclaration) {
        String name = ((WdlBoundDeclaration) taskElement).getName();
        if (name != null) {
          privateDeclarations.add(name);
        }
      }
    }

    return new CallableContract(
        requiredInputs, inputTypes, outputs, outputTypes, privateDeclarations);
  }

  private CallableContract buildWorkflowContract(WdlWorkflow workflow) {
    Set<String> requiredInputs = new HashSet<>();
    Set<String> outputs = new HashSet<>();
    Map<String, WdlType> inputTypes = new HashMap<>();
    Map<String, WdlType> outputTypes = new HashMap<>();

    for (WdlWorkflow.WdlWorkflowElement workflowElement : workflow.getElements()) {
      if (workflowElement instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) workflowElement).elements()) {
          if (declaration.getName() != null) {
            inputTypes.put(declaration.getName(), declaration.getType());
            if (!(declaration instanceof WdlBoundDeclaration)
                && declaration.getType() != null
                && !declaration.getType().isOptional()) {
              requiredInputs.add(declaration.getName());
            }
          }
        }
      } else if (workflowElement instanceof WdlOutput) {
        for (WdlBoundDeclaration declaration : ((WdlOutput) workflowElement).elements()) {
          if (declaration.getName() != null) {
            outputs.add(declaration.getName());
            outputTypes.put(declaration.getName(), declaration.getType());
          }
        }
      }
    }

    return new CallableContract(requiredInputs, inputTypes, outputs, outputTypes, Set.of());
  }

  private void indexCallableContract(String key, CallableContract contract, String kind) {
    if (key == null || key.isBlank() || contract == null) {
      return;
    }
    if (callableContracts.containsKey(key)) {
      addError(
          WdlSemanticError.Code.DUPLICATE_DEFINITION,
          "Conflicting " + kind + " definitions for callable name '" + key + "'");
      return;
    }
    callableContracts.put(key, contract);
  }

  private String renderImportSource(WdlImport imp) {
    if (imp == null || imp.getSource() == null) {
      return "";
    }
    StringBuilder text = new StringBuilder();
    imp.getSource()
        .components()
        .forEach(
            c -> {
              if (c instanceof com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringText) {
                text.append(
                    ((com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringText) c)
                        .getText());
              } else if (c
                  instanceof com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringEscape) {
                text.append(
                    ((com.myriad.wdl.model.expressions.WdlStringLiteral.WdlStringEscape) c)
                        .getEscapeText());
              }
            });
    return text.toString();
  }

  @Override
  public void processWorkflow(WdlDocument ctx, WdlWorkflow node) {
    Map<String, WdlType> prevTypes = scopeTypes;
    Map<String, Object> prevValues = scopeValues;
    Map<String, Set<String>> prevCallOutputs = callOutputs;
    Map<String, Map<String, WdlType>> prevCallOutputTypes = callOutputTypes;
    WdlExpressionValidator prevExpressionValidator = expressionValidator;

    scopeTypes = new HashMap<>();
    scopeValues = new HashMap<>();
    callOutputs = new HashMap<>();
    callOutputTypes = new HashMap<>();
    expressionValidator =
        createExpressionValidator(
            scopeTypes,
            scopeValues,
            callOutputs,
            callOutputTypes,
            structMembers,
            structMemberTypes,
          enumValueTypes,
          enumChoiceNames,
            currentDocumentVersion,
            this::addError);

    try {
      super.processWorkflow(ctx, node);
    } finally {
      scopeTypes = prevTypes;
      scopeValues = prevValues;
      callOutputs = prevCallOutputs;
      callOutputTypes = prevCallOutputTypes;
      expressionValidator = prevExpressionValidator;
    }
  }

  @Override
  public void processWorkflowInput(WdlWorkflow ctx, WdlInput node) {
    for (WdlDeclaration declaration : node.elements()) {
      if (declaration.getName() != null) {
        scopeTypes.put(declaration.getName(), declaration.getType());
      }
      if (declaration instanceof WdlBoundDeclaration && declaration.getName() != null) {
        WdlBoundDeclaration bound = (WdlBoundDeclaration) declaration;
        expressionValidator.validate(bound.getExpression());
        scopeValues.put(declaration.getName(), expressionValidator.evaluate(bound.getExpression()));
      }
    }
  }

  @Override
  public void processWorkflowDeclaration(WdlWorkflow ctx, WdlBoundDeclaration node) {
    validateBoundDeclaration(node);
  }

  @Override
  public void processWorkflowOutput(WdlWorkflow ctx, WdlOutput node) {
    for (WdlBoundDeclaration declaration : node.elements()) {
      validateBoundDeclaration(declaration);
    }
  }

  @Override
  public void processWorkflowCall(WdlWorkflow ctx, WdlCall node) {
    String qualifiedTarget = node.targetPathAsString();
    String fallbackTarget = node.targetPath().peekLast();
    CallableContract contract =
        qualifiedTarget == null || qualifiedTarget.isBlank()
            ? null
            : callableContracts.get(qualifiedTarget);
    if (contract == null && fallbackTarget != null && !fallbackTarget.isBlank()) {
      contract = callableContracts.get(fallbackTarget);
    }

    Set<String> providedInputs = new HashSet<>();

    for (WdlCallInput callInput : node.inputs()) {
      String key = callInput.getKey() == null ? "" : callInput.getKey();
      String rootName = key.contains(".") ? key.substring(0, key.indexOf('.')) : key;
      if (!rootName.isEmpty()) {
        providedInputs.add(rootName);
      }
      if (contract != null && contract.privateDeclarations.contains(rootName)) {
        addError(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "Call input '"
                + rootName
                + "' is private in callable '"
                + (qualifiedTarget == null ? "" : qualifiedTarget)
                + "'");
      }
      if (contract != null && !rootName.isEmpty() && !contract.inputTypes.containsKey(rootName)) {
        addError(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "Call input '"
                + rootName
                + "' does not exist in callable '"
                + (qualifiedTarget == null ? "" : qualifiedTarget)
                + "'");
      }
      expressionValidator.validate(callInput.getValue());
      if (contract != null && contract.inputTypes.containsKey(rootName)) {
        WdlType expected = contract.inputTypes.get(rootName);
        if (!expressionValidator.isAssignableFrom(expected, callInput.getValue())) {
          addError(
              WdlSemanticError.Code.TYPE_MISMATCH,
              "Call input '"
                  + rootName
                  + "' type is incompatible with callable '"
                  + (qualifiedTarget == null ? "" : qualifiedTarget)
                  + "' input type");
        }
      }
    }

    if (contract != null) {
      for (String requiredInput : contract.requiredInputs) {
        if (!providedInputs.contains(requiredInput)) {
          addError(
              WdlSemanticError.Code.UNKNOWN_REFERENCE,
              "Call to callable '"
                  + (qualifiedTarget == null ? "" : qualifiedTarget)
                  + "' is missing required input '"
                  + requiredInput
                  + "'");
        }
      }
    }

    String defaultCallName = fallbackTarget;
    String callName = node.getAlias() != null ? node.getAlias() : defaultCallName;
    if (callName != null) {
      callOutputs.put(callName, contract == null ? Set.of() : contract.outputs);
      callOutputTypes.put(callName, contract == null ? Map.of() : contract.outputTypes);
    }
  }

  @Override
  public void processWorkflowScatter(WdlWorkflow ctx, WdlScatter node) {
    expressionValidator.validate(node.getCollection());
    for (WdlStatement statement : node.statements()) {
      processWorkflowStatement(ctx, statement);
    }
  }

  @Override
  public void processWorkflowConditional(WdlWorkflow ctx, WdlConditional node) {
    expressionValidator.validate(node.getCondition());
    for (WdlStatement statement : node.thenStatements()) {
      processWorkflowStatement(ctx, statement);
    }
    for (WdlConditionalElseIf elseIf : node.elseIfs()) {
      expressionValidator.validate(elseIf.getCondition());
      for (WdlStatement statement : elseIf.thenStatements()) {
        processWorkflowStatement(ctx, statement);
      }
    }
    for (WdlStatement statement : node.elseStatements()) {
      processWorkflowStatement(ctx, statement);
    }
  }

  private void processWorkflowStatement(WdlWorkflow workflow, WdlStatement statement) {
    if (statement instanceof WdlBoundDeclaration) {
      processWorkflowDeclaration(workflow, (WdlBoundDeclaration) statement);
    } else if (statement instanceof WdlCall) {
      processWorkflowCall(workflow, (WdlCall) statement);
    } else if (statement instanceof WdlScatter) {
      processWorkflowScatter(workflow, (WdlScatter) statement);
    } else if (statement instanceof WdlConditional) {
      processWorkflowConditional(workflow, (WdlConditional) statement);
    }
  }

  private void validateBoundDeclaration(WdlBoundDeclaration declaration) {
    if (declaration.getName() != null) {
      scopeTypes.put(declaration.getName(), declaration.getType());
    }

    expressionValidator.validate(declaration.getExpression());
    if (!expressionValidator.isAssignableFrom(declaration.getType(), declaration.getExpression())) {
      addError(
          WdlSemanticError.Code.TYPE_MISMATCH,
          "Declaration '"
              + safeName(declaration.getName())
              + "' type is incompatible with expression");
    }

    if (declaration.getType() instanceof WdlArrayType) {
      WdlArrayType t = (WdlArrayType) declaration.getType();
      if (t.isNonEmpty() && declaration.getExpression() instanceof WdlArrayLiteral) {
        if (((WdlArrayLiteral) declaration.getExpression()).entries().isEmpty()) {
          addError(
              WdlSemanticError.Code.TYPE_MISMATCH,
              "Declaration '" + safeName(declaration.getName()) + "' requires a non-empty array");
        }
      }
    }

    if (declaration.getName() != null) {
      scopeValues.put(
          declaration.getName(), expressionValidator.evaluate(declaration.getExpression()));
    }
  }

  private String safeName(String value) {
    return value == null ? "<unnamed>" : value;
  }

  private void addError(String message) {
    addError(WdlSemanticError.Code.GENERIC_SEMANTIC_ERROR, message);
  }

  private void addError(WdlSemanticError.Code code, String message) {
    errors.add(new WdlSemanticError(code, message, 0, 0));
  }

  protected final void addValidationError(String message) {
    addError(message);
  }

  protected final void addValidationError(WdlSemanticError.Code code, String message) {
    addError(code, message);
  }

  protected WdlExpressionValidator createExpressionValidator(
      Map<String, WdlType> scopeTypes,
      Map<String, Object> scopeValues,
      Map<String, Set<String>> callOutputs,
      Map<String, Map<String, WdlType>> callOutputTypes,
      Map<String, Set<String>> structMembers,
      Map<String, Map<String, WdlType>> structMemberTypes,
    Map<String, WdlType> enumValueTypes,
    Map<String, Set<String>> enumChoiceNames,
      WdlVersion documentVersion,
      BiConsumer<WdlSemanticError.Code, String> addError) {
    return new WdlExpressionValidator(
        scopeTypes,
        scopeValues,
        callOutputs,
        callOutputTypes,
        structMembers,
        structMemberTypes,
      enumValueTypes,
      enumChoiceNames,
        documentVersion,
        addError);
  }

  private static final class CallableContract {
    private final Set<String> requiredInputs;
    private final Map<String, WdlType> inputTypes;
    private final Set<String> outputs;
    private final Map<String, WdlType> outputTypes;
    private final Set<String> privateDeclarations;

    private CallableContract(
        Set<String> requiredInputs,
        Map<String, WdlType> inputTypes,
        Set<String> outputs,
        Map<String, WdlType> outputTypes,
        Set<String> privateDeclarations) {
      this.requiredInputs = Objects.requireNonNull(requiredInputs, "requiredInputs");
      this.inputTypes = Objects.requireNonNull(inputTypes, "inputTypes");
      this.outputs = Objects.requireNonNull(outputs, "outputs");
      this.outputTypes = Objects.requireNonNull(outputTypes, "outputTypes");
      this.privateDeclarations = Objects.requireNonNull(privateDeclarations, "privateDeclarations");
    }

    private boolean isEquivalentTo(CallableContract other) {
      if (other == null) {
        return false;
      }
      return requiredInputs.equals(other.requiredInputs)
          && outputNames(outputs).equals(outputNames(other.outputs))
          && typeMapShape(inputTypes).equals(typeMapShape(other.inputTypes))
          && typeMapShape(outputTypes).equals(typeMapShape(other.outputTypes))
          && privateDeclarations.equals(other.privateDeclarations);
    }

    private Set<String> outputNames(Set<String> values) {
      return new HashSet<>(values);
    }

    private Map<String, String> typeMapShape(Map<String, WdlType> types) {
      Map<String, String> shape = new HashMap<>();
      for (Map.Entry<String, WdlType> e : types.entrySet()) {
        shape.put(e.getKey(), e.getValue() == null ? "<null>" : e.getValue().toString());
      }
      return shape;
    }
  }

  private static final class StructShape {
    private final LinkedHashMap<String, String> orderedMemberTypeWdl;
    private final LinkedHashMap<String, WdlType> orderedMemberTypes;

    private StructShape(
        LinkedHashMap<String, String> orderedMemberTypeWdl,
        LinkedHashMap<String, WdlType> orderedMemberTypes) {
      this.orderedMemberTypeWdl = orderedMemberTypeWdl;
      this.orderedMemberTypes = orderedMemberTypes;
    }

    private static StructShape from(
        Map<String, WdlType> memberTypes, java.util.function.Function<WdlType, String> toWdl) {
      LinkedHashMap<String, String> ordered = new LinkedHashMap<>();
      LinkedHashMap<String, WdlType> orderedTypes = new LinkedHashMap<>();
      for (Map.Entry<String, WdlType> entry : memberTypes.entrySet()) {
        ordered.put(
            entry.getKey(), entry.getValue() == null ? "<null>" : toWdl.apply(entry.getValue()));
        orderedTypes.put(entry.getKey(), entry.getValue());
      }
      return new StructShape(ordered, orderedTypes);
    }

    private boolean isCompatibleWith(StructShape other) {
      if (other == null) {
        return false;
      }
      return orderedMemberTypeWdl.equals(other.orderedMemberTypeWdl);
    }

    private Set<String> memberNamesSet() {
      return new LinkedHashSet<>(orderedMemberTypeWdl.keySet());
    }

    private Map<String, WdlType> memberTypesMap() {
      return new LinkedHashMap<>(orderedMemberTypes);
    }
  }

  private static final class EnumShape {
    private final String valueType;
    private final List<String> choices;

    private EnumShape(String valueType, List<String> choices) {
      this.valueType = valueType;
      this.choices = choices;
    }

    private static EnumShape from(
        WdlEnum en,
        java.util.function.Function<WdlType, String> typeToWdl,
        java.util.function.Function<com.myriad.wdl.model.expressions.WdlExpression, String>
            expressionToWdl) {
      String typeText =
          en.getValueType() == null ? "<implicit>" : typeToWdl.apply(en.getValueType());
      List<String> orderedChoices = new ArrayList<>();
      en.elements()
          .forEach(
              c -> {
                String value =
                    c.getValue() == null ? "<none>" : expressionToWdl.apply(c.getValue());
                orderedChoices.add(c.getKey() + "=" + value);
              });
      return new EnumShape(typeText, orderedChoices);
    }

    private boolean isCompatibleWith(EnumShape other) {
      if (other == null) {
        return false;
      }
      return Objects.equals(valueType, other.valueType) && choices.equals(other.choices);
    }
  }
}
