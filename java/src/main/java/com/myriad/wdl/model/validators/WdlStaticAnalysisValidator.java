package com.myriad.wdl.model.validators;

import com.myriad.wdl.model.WdlDocument;
import com.myriad.wdl.model.WdlDocument.WdlDocumentElement;
import com.myriad.wdl.model.WdlVersion;
import com.myriad.wdl.model.definitions.WdlEnum;
import com.myriad.wdl.model.definitions.WdlStruct;
import com.myriad.wdl.model.definitions.WdlTask;
import com.myriad.wdl.model.definitions.WdlWorkflow;
import com.myriad.wdl.model.errors.WdlSemanticError;
import com.myriad.wdl.model.sections.WdlInput;
import com.myriad.wdl.model.sections.WdlOutput;
import com.myriad.wdl.model.statements.WdlCall;
import com.myriad.wdl.model.statements.WdlConditional;
import com.myriad.wdl.model.statements.WdlDeclaration;
import com.myriad.wdl.model.statements.WdlDeclaration.WdlBoundDeclaration;
import com.myriad.wdl.model.statements.WdlScatter;
import com.myriad.wdl.model.statements.WdlStatement;
import com.myriad.wdl.model.types.WdlArrayType;
import com.myriad.wdl.model.types.WdlMapType;
import com.myriad.wdl.model.types.WdlPairType;
import com.myriad.wdl.model.types.WdlType;
import com.myriad.wdl.model.types.WdlTypeReferenceType;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.function.BiConsumer;

/**
 * Static-analysis validator that extends {@link WdlValidator} with deterministic whole-document
 * checks.
 *
 * <p>This layer is intended for failures that are stronger than baseline semantic correctness but
 * still do not depend on runtime values. Typical examples include duplicate declarations, unknown
 * type references, unknown call targets, invalid after-dependencies, and nested workflow structure
 * defects. The synthetic tests in {@code WdlValidatorTest} demonstrate the intended coverage.
 */
public class WdlStaticAnalysisValidator extends WdlValidator {

  private final Set<String> knownCallableTargets = new HashSet<>();
  private final Set<String> knownTypeNames = new HashSet<>();

  @Override
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
    return new WdlStaticAnalysisExpressionValidator(
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

  @Override
  /**
   * Runs the baseline validator plus additional top-level symbol and type-reference checks before
   * the ordinary traversal begins.
   */
  public void validate(WdlDocument document) throws com.myriad.wdl.model.errors.WdlException {
    knownCallableTargets.clear();
    knownTypeNames.clear();
    Set<String> topLevelNames = new HashSet<>();
    for (WdlDocumentElement element : document.elements()) {
      if (element instanceof WdlTask && ((WdlTask) element).getName() != null) {
        String name = ((WdlTask) element).getName();
        knownCallableTargets.add(name);
        if (!topLevelNames.add("task:" + name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate task definition: '" + name + "'");
        }
      } else if (element instanceof WdlWorkflow && ((WdlWorkflow) element).getName() != null) {
        String name = ((WdlWorkflow) element).getName();
        knownCallableTargets.add(name);
        if (!topLevelNames.add("workflow:" + name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate workflow definition: '" + name + "'");
        }
      } else if (element instanceof WdlStruct && ((WdlStruct) element).getName() != null) {
        String name = ((WdlStruct) element).getName();
        knownTypeNames.add(name);
        if (!topLevelNames.add("struct:" + name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate struct definition: '" + name + "'");
        }
      } else if (element instanceof WdlEnum && ((WdlEnum) element).getName() != null) {
        String name = ((WdlEnum) element).getName();
        knownTypeNames.add(name);
        if (!topLevelNames.add("enum:" + name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate enum definition: '" + name + "'");
        }
      }
    }

    for (WdlDocumentElement element : document.elements()) {
      if (element instanceof WdlStruct && ((WdlStruct) element).getName() != null) {
        String structName = ((WdlStruct) element).getName();
        for (WdlStruct.WdlStructElement structElement : ((WdlStruct) element).elements()) {
          if (structElement instanceof WdlStruct.WdlStructMember) {
            validateKnownTypeReference(
                ((WdlStruct.WdlStructMember) structElement).getType(),
                "struct '"
                    + structName
                    + "' member '"
                    + ((WdlStruct.WdlStructMember) structElement).getName()
                    + "'");
          }
        }
      }
    }

    super.validate(document);
  }

  @Override
  public void processWorkflow(WdlDocument ctx, WdlWorkflow node) {
    Set<String> seenCallNames = new HashSet<>();
    Set<String> seenDeclarations = new HashSet<>();

    for (WdlWorkflow.WdlWorkflowElement element : node.getElements()) {
      if (element instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) element).elements()) {
          validateKnownTypeReference(
              declaration.getType(), "workflow input '" + declaration.getName() + "'");
          if (declaration.getName() != null && !seenDeclarations.add(declaration.getName())) {
            addValidationError(
                WdlSemanticError.Code.DUPLICATE_DEFINITION,
                "Duplicate workflow declaration: '" + declaration.getName() + "'");
          }
        }
      } else if (element instanceof WdlBoundDeclaration) {
        String name = ((WdlBoundDeclaration) element).getName();
        validateKnownTypeReference(
            ((WdlBoundDeclaration) element).getType(), "workflow declaration '" + name + "'");
        if (name != null && !seenDeclarations.add(name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate workflow declaration: '" + name + "'");
        }
      } else if (element instanceof WdlCall) {
        WdlCall call = (WdlCall) element;
        String target = call.targetPath().peekLast();
        String callName = call.getAlias() != null ? call.getAlias() : target;

        if (target != null && !knownCallableTargets.contains(target)) {
          addValidationError(
              WdlSemanticError.Code.UNKNOWN_REFERENCE,
              "Call target '" + target + "' is not defined");
        }
        if (callName != null && !seenCallNames.add(callName)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate call name in workflow: '" + callName + "'");
        }

        Set<String> callInputKeys = new HashSet<>();
        for (WdlCall.WdlCallInput callInput : call.inputs()) {
          String key = callInput.getKey();
          if (key != null && !callInputKeys.add(key)) {
            addValidationError(
                WdlSemanticError.Code.DUPLICATE_DEFINITION,
                "Duplicate call input key '"
                    + key
                    + "' in call '"
                    + (callName == null ? "<unnamed>" : callName)
                    + "'");
          }
        }

        for (String dep : call.afterDependencies()) {
          if (!seenCallNames.contains(dep)) {
            addValidationError(
                WdlSemanticError.Code.UNKNOWN_REFERENCE,
                "Call '"
                    + (callName == null ? "<unnamed>" : callName)
                    + "' has unknown or forward after dependency '"
                    + dep
                    + "'");
          }
        }
      }
    }

    validateNestedWorkflowStructure(node);

    super.processWorkflow(ctx, node);
  }

  @Override
  public void processTask(WdlDocument ctx, WdlTask node) {
    Set<String> taskDeclarationNames = new HashSet<>();

    for (WdlTask.WdlTaskElement element : node.elements()) {
      if (element instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) element).elements()) {
          validateKnownTypeReference(
              declaration.getType(),
              "task '" + node.getName() + "' input '" + declaration.getName() + "'");
          if (declaration.getName() != null && !taskDeclarationNames.add(declaration.getName())) {
            addValidationError(
                WdlSemanticError.Code.DUPLICATE_DEFINITION,
                "Duplicate task declaration in '"
                    + node.getName()
                    + "': '"
                    + declaration.getName()
                    + "'");
          }
        }
      } else if (element instanceof WdlBoundDeclaration) {
        String name = ((WdlBoundDeclaration) element).getName();
        validateKnownTypeReference(
            ((WdlBoundDeclaration) element).getType(),
            "task '" + node.getName() + "' declaration '" + name + "'");
        if (name != null && !taskDeclarationNames.add(name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate task declaration in '" + node.getName() + "': '" + name + "'");
        }
      } else if (element instanceof WdlOutput) {
        Set<String> outputNames = new HashSet<>();
        for (WdlBoundDeclaration declaration : ((WdlOutput) element).elements()) {
          validateKnownTypeReference(
              declaration.getType(),
              "task '" + node.getName() + "' output '" + declaration.getName() + "'");
          if (declaration.getName() != null && !outputNames.add(declaration.getName())) {
            addValidationError(
                WdlSemanticError.Code.DUPLICATE_DEFINITION,
                "Duplicate task output in '"
                    + node.getName()
                    + "': '"
                    + declaration.getName()
                    + "'");
          }
        }
      }
    }

    super.processTask(ctx, node);
  }

  private void validateKnownTypeReference(WdlType type, String location) {
    if (type == null) {
      return;
    }
    if (type instanceof WdlTypeReferenceType) {
      String ref = ((WdlTypeReferenceType) type).referenceName();
      if (ref != null && !knownTypeNames.contains(ref)) {
        addValidationError(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "Unknown type reference '" + ref + "' in " + location);
      }
      return;
    }
    if (type instanceof WdlArrayType) {
      validateKnownTypeReference(((WdlArrayType) type).memberType(), location);
      return;
    }
    if (type instanceof WdlMapType) {
      validateKnownTypeReference(((WdlMapType) type).keyType(), location);
      validateKnownTypeReference(((WdlMapType) type).valueType(), location);
      return;
    }
    if (type instanceof WdlPairType) {
      validateKnownTypeReference(((WdlPairType) type).leftType(), location);
      validateKnownTypeReference(((WdlPairType) type).rightType(), location);
    }
  }

  private void validateNestedWorkflowStructure(WdlWorkflow workflow) {
    Set<String> availableCalls = new HashSet<>();
    Set<String> namesInBlock = new HashSet<>();

    for (WdlWorkflow.WdlWorkflowElement element : workflow.getElements()) {
      if (element instanceof WdlInput) {
        for (WdlDeclaration declaration : ((WdlInput) element).elements()) {
          String name = declaration.getName();
          if (name != null && !namesInBlock.add(name)) {
            addValidationError(
                WdlSemanticError.Code.DUPLICATE_DEFINITION,
                "Duplicate workflow declaration: '" + name + "'");
          }
        }
      } else if (element instanceof WdlBoundDeclaration) {
        String name = ((WdlBoundDeclaration) element).getName();
        if (name != null && !namesInBlock.add(name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate workflow declaration: '" + name + "'");
        }
      } else if (element instanceof WdlCall) {
        WdlCall call = (WdlCall) element;
        validateCallStructure(call, namesInBlock, availableCalls);
      } else if (element instanceof WdlScatter) {
        WdlScatter scatter = (WdlScatter) element;
        String varName = scatter.getName();
        if (varName != null && !namesInBlock.add(varName)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate workflow declaration: '" + varName + "'");
        }
        validateNestedStatements(scatter.statements(), availableCalls, "scatter");
      } else if (element instanceof WdlConditional) {
        validateConditionalStructure((WdlConditional) element, availableCalls, "conditional");
      }
    }
  }

  private void validateNestedStatements(
      Iterable<WdlStatement> statements, Set<String> inheritedCalls, String contextLabel) {
    Set<String> namesInBlock = new HashSet<>();
    Set<String> availableCalls = new HashSet<>(inheritedCalls);

    for (WdlStatement statement : statements) {
      if (statement instanceof WdlBoundDeclaration) {
        String name = ((WdlBoundDeclaration) statement).getName();
        if (name != null && !namesInBlock.add(name)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate declaration in " + contextLabel + ": '" + name + "'");
        }
      } else if (statement instanceof WdlCall) {
        validateCallStructure((WdlCall) statement, namesInBlock, availableCalls);
      } else if (statement instanceof WdlScatter) {
        WdlScatter scatter = (WdlScatter) statement;
        String varName = scatter.getName();
        if (varName != null && !namesInBlock.add(varName)) {
          addValidationError(
              WdlSemanticError.Code.DUPLICATE_DEFINITION,
              "Duplicate declaration in " + contextLabel + ": '" + varName + "'");
        }
        validateNestedStatements(scatter.statements(), availableCalls, "scatter");
      } else if (statement instanceof WdlConditional) {
        validateConditionalStructure((WdlConditional) statement, availableCalls, "conditional");
      }
    }
  }

  private void validateConditionalStructure(
      WdlConditional conditional, Set<String> availableCalls, String contextLabel) {
    validateNestedStatements(conditional.thenStatements(), availableCalls, contextLabel + " then");
    for (WdlConditional.WdlConditionalElseIf elseIf : conditional.elseIfs()) {
      validateNestedStatements(elseIf.thenStatements(), availableCalls, contextLabel + " else-if");
    }
    validateNestedStatements(conditional.elseStatements(), availableCalls, contextLabel + " else");
  }

  private void validateCallStructure(
      WdlCall call, Set<String> namesInBlock, Set<String> availableCalls) {
    String target = call.targetPath().peekLast();
    String callName = call.getAlias() != null ? call.getAlias() : target;

    if (callName != null && !namesInBlock.add(callName)) {
      addValidationError(
          WdlSemanticError.Code.DUPLICATE_DEFINITION,
          "Duplicate call name in workflow: '" + callName + "'");
    }

    Set<String> callInputKeys = new HashSet<>();
    for (WdlCall.WdlCallInput callInput : call.inputs()) {
      String key = callInput.getKey();
      if (key != null && !callInputKeys.add(key)) {
        addValidationError(
            WdlSemanticError.Code.DUPLICATE_DEFINITION,
            "Duplicate call input key '"
                + key
                + "' in call '"
                + (callName == null ? "<unnamed>" : callName)
                + "'");
      }
    }

    for (String dep : call.afterDependencies()) {
      if (!availableCalls.contains(dep)) {
        addValidationError(
            WdlSemanticError.Code.UNKNOWN_REFERENCE,
            "Call '"
                + (callName == null ? "<unnamed>" : callName)
                + "' has unknown or forward after dependency '"
                + dep
                + "'");
      }
    }

    if (callName != null) {
      availableCalls.add(callName);
    }
  }
}
