// Code generated from ./WdlV1Parser.g4 by ANTLR 4.13.1. DO NOT EDIT.

package wdl1 // WdlV1Parser
import "github.com/antlr4-go/antlr/v4"

// A complete Visitor for a parse tree produced by WdlV1Parser.
type WdlV1ParserVisitor interface {
	antlr.ParseTreeVisitor

	// Visit a parse tree produced by WdlV1Parser#document.
	VisitDocument(ctx *DocumentContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#versionStatement.
	VisitVersionStatement(ctx *VersionStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#documentElement.
	VisitDocumentElement(ctx *DocumentElementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importStatementStandard.
	VisitImportStatementStandard(ctx *ImportStatementStandardContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importStatementStar.
	VisitImportStatementStar(ctx *ImportStatementStarContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importStatementMembers.
	VisitImportStatementMembers(ctx *ImportStatementMembersContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importMembers.
	VisitImportMembers(ctx *ImportMembersContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importMember.
	VisitImportMember(ctx *ImportMemberContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importUriLiteral.
	VisitImportUriLiteral(ctx *ImportUriLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importUriElement.
	VisitImportUriElement(ctx *ImportUriElementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#importAlias.
	VisitImportAlias(ctx *ImportAliasContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#structDefinition.
	VisitStructDefinition(ctx *StructDefinitionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#structItemMetadata.
	VisitStructItemMetadata(ctx *StructItemMetadataContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#structItemParameterMetadata.
	VisitStructItemParameterMetadata(ctx *StructItemParameterMetadataContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#structItemMemberDeclaration.
	VisitStructItemMemberDeclaration(ctx *StructItemMemberDeclarationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#structDeclaration.
	VisitStructDeclaration(ctx *StructDeclarationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumDefinition.
	VisitEnumDefinition(ctx *EnumDefinitionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumTypeParameter.
	VisitEnumTypeParameter(ctx *EnumTypeParameterContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumChoice.
	VisitEnumChoice(ctx *EnumChoiceContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumLiteralExpression.
	VisitEnumLiteralExpression(ctx *EnumLiteralExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumStringLiteral.
	VisitEnumStringLiteral(ctx *EnumStringLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumQuotedString.
	VisitEnumQuotedString(ctx *EnumQuotedStringContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumStringElement.
	VisitEnumStringElement(ctx *EnumStringElementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumMultilineString.
	VisitEnumMultilineString(ctx *EnumMultilineStringContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumMultilineStringElement.
	VisitEnumMultilineStringElement(ctx *EnumMultilineStringElementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumArrayLiteral.
	VisitEnumArrayLiteral(ctx *EnumArrayLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumMapLiteral.
	VisitEnumMapLiteral(ctx *EnumMapLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumMapLiteralItem.
	VisitEnumMapLiteralItem(ctx *EnumMapLiteralItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumObjectLiteral.
	VisitEnumObjectLiteral(ctx *EnumObjectLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumObjectLiteralItem.
	VisitEnumObjectLiteralItem(ctx *EnumObjectLiteralItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumStructLiteral.
	VisitEnumStructLiteral(ctx *EnumStructLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumStructLiteralItem.
	VisitEnumStructLiteralItem(ctx *EnumStructLiteralItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#enumPairLiteral.
	VisitEnumPairLiteral(ctx *EnumPairLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskDefinition.
	VisitTaskDefinition(ctx *TaskDefinitionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowDefinition.
	VisitWorkflowDefinition(ctx *WorkflowDefinitionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#type.
	VisitType(ctx *TypeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#mapType.
	VisitMapType(ctx *MapTypeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#arrayType.
	VisitArrayType(ctx *ArrayTypeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#pairType.
	VisitPairType(ctx *PairTypeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#objectType.
	VisitObjectType(ctx *ObjectTypeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#primitiveType.
	VisitPrimitiveType(ctx *PrimitiveTypeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#typeRefType.
	VisitTypeRefType(ctx *TypeRefTypeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#unboundDeclaration.
	VisitUnboundDeclaration(ctx *UnboundDeclarationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#boundDeclaration.
	VisitBoundDeclaration(ctx *BoundDeclarationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#declaration.
	VisitDeclaration(ctx *DeclarationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskInputSection.
	VisitTaskInputSection(ctx *TaskInputSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskCommandSection.
	VisitTaskCommandSection(ctx *TaskCommandSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskOutputSection.
	VisitTaskOutputSection(ctx *TaskOutputSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskRuntimeSection.
	VisitTaskRuntimeSection(ctx *TaskRuntimeSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskRequirementsSection.
	VisitTaskRequirementsSection(ctx *TaskRequirementsSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskHintsSection.
	VisitTaskHintsSection(ctx *TaskHintsSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskMetadataSection.
	VisitTaskMetadataSection(ctx *TaskMetadataSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskParameterMetadataSection.
	VisitTaskParameterMetadataSection(ctx *TaskParameterMetadataSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskDeclaration.
	VisitTaskDeclaration(ctx *TaskDeclarationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowInputSection.
	VisitWorkflowInputSection(ctx *WorkflowInputSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowOutputSection.
	VisitWorkflowOutputSection(ctx *WorkflowOutputSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowHintsSection.
	VisitWorkflowHintsSection(ctx *WorkflowHintsSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowConditionalStatement.
	VisitWorkflowConditionalStatement(ctx *WorkflowConditionalStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowScatterStatement.
	VisitWorkflowScatterStatement(ctx *WorkflowScatterStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowCallStatement.
	VisitWorkflowCallStatement(ctx *WorkflowCallStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowMetadataSection.
	VisitWorkflowMetadataSection(ctx *WorkflowMetadataSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowParameterMetadataSection.
	VisitWorkflowParameterMetadataSection(ctx *WorkflowParameterMetadataSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowDeclaration.
	VisitWorkflowDeclaration(ctx *WorkflowDeclarationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#inputSection.
	VisitInputSection(ctx *InputSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#outputSection.
	VisitOutputSection(ctx *OutputSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#runtimeSection.
	VisitRuntimeSection(ctx *RuntimeSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#runtimeItem.
	VisitRuntimeItem(ctx *RuntimeItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#requirementsSection.
	VisitRequirementsSection(ctx *RequirementsSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#requirementsItem.
	VisitRequirementsItem(ctx *RequirementsItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsSectionTask.
	VisitHintsSectionTask(ctx *HintsSectionTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsItemTask.
	VisitHintsItemTask(ctx *HintsItemTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskHintValueExpression.
	VisitTaskHintValueExpression(ctx *TaskHintValueExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskHintValueHintsObject.
	VisitTaskHintValueHintsObject(ctx *TaskHintValueHintsObjectContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskHintValueInputObject.
	VisitTaskHintValueInputObject(ctx *TaskHintValueInputObjectContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskHintValueOutputObject.
	VisitTaskHintValueOutputObject(ctx *TaskHintValueOutputObjectContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskHintValueArray.
	VisitTaskHintValueArray(ctx *TaskHintValueArrayContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsTypedObjectTask.
	VisitHintsTypedObjectTask(ctx *HintsTypedObjectTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsObjectItemTask.
	VisitHintsObjectItemTask(ctx *HintsObjectItemTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#inputHintsObjectTask.
	VisitInputHintsObjectTask(ctx *InputHintsObjectTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#inputHintsItemTask.
	VisitInputHintsItemTask(ctx *InputHintsItemTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#outputHintsObjectTask.
	VisitOutputHintsObjectTask(ctx *OutputHintsObjectTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#outputHintsItemTask.
	VisitOutputHintsItemTask(ctx *OutputHintsItemTaskContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#taskHintsArray.
	VisitTaskHintsArray(ctx *TaskHintsArrayContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsSectionWorkflow.
	VisitHintsSectionWorkflow(ctx *HintsSectionWorkflowContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsItemWorkflow.
	VisitHintsItemWorkflow(ctx *HintsItemWorkflowContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueNumber.
	VisitWorkflowHintValueNumber(ctx *WorkflowHintValueNumberContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueString.
	VisitWorkflowHintValueString(ctx *WorkflowHintValueStringContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueBoolean.
	VisitWorkflowHintValueBoolean(ctx *WorkflowHintValueBooleanContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueObject.
	VisitWorkflowHintValueObject(ctx *WorkflowHintValueObjectContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueArray.
	VisitWorkflowHintValueArray(ctx *WorkflowHintValueArrayContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsObjectWorkflow.
	VisitHintsObjectWorkflow(ctx *HintsObjectWorkflowContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#hintsObjectItemWorkflow.
	VisitHintsObjectItemWorkflow(ctx *HintsObjectItemWorkflowContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowHintsArray.
	VisitWorkflowHintsArray(ctx *WorkflowHintsArrayContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#metadataSection.
	VisitMetadataSection(ctx *MetadataSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#parameterMetadataSection.
	VisitParameterMetadataSection(ctx *ParameterMetadataSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#metadataObject.
	VisitMetadataObject(ctx *MetadataObjectContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#metadataObjectItem.
	VisitMetadataObjectItem(ctx *MetadataObjectItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#metadataArray.
	VisitMetadataArray(ctx *MetadataArrayContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#metadataValue.
	VisitMetadataValue(ctx *MetadataValueContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#commandSection.
	VisitCommandSection(ctx *CommandSectionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringCommand.
	VisitMultilineStringCommand(ctx *MultilineStringCommandContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#bracedCommand.
	VisitBracedCommand(ctx *BracedCommandContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#workflowStatement.
	VisitWorkflowStatement(ctx *WorkflowStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#conditionalStatement.
	VisitConditionalStatement(ctx *ConditionalStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#conditionalElseIfClause.
	VisitConditionalElseIfClause(ctx *ConditionalElseIfClauseContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#conditionalElseClause.
	VisitConditionalElseClause(ctx *ConditionalElseClauseContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#scatterStatement.
	VisitScatterStatement(ctx *ScatterStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#scatterBody.
	VisitScatterBody(ctx *ScatterBodyContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#callStatement.
	VisitCallStatement(ctx *CallStatementContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#callTarget.
	VisitCallTarget(ctx *CallTargetContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#callAlias.
	VisitCallAlias(ctx *CallAliasContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#callAfterClause.
	VisitCallAfterClause(ctx *CallAfterClauseContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#callInputBlock.
	VisitCallInputBlock(ctx *CallInputBlockContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#callInputItem.
	VisitCallInputItem(ctx *CallInputItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#expression.
	VisitExpression(ctx *ExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#logicalOrExprOperation.
	VisitLogicalOrExprOperation(ctx *LogicalOrExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#logicalAndExprOperation.
	VisitLogicalAndExprOperation(ctx *LogicalAndExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#equalityExprOperation.
	VisitEqualityExprOperation(ctx *EqualityExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#comparisonExprOperation.
	VisitComparisonExprOperation(ctx *ComparisonExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#additiveExprOperation.
	VisitAdditiveExprOperation(ctx *AdditiveExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multiplicativeExprOperation.
	VisitMultiplicativeExprOperation(ctx *MultiplicativeExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#powerExprOperation.
	VisitPowerExprOperation(ctx *PowerExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#powerExprNone.
	VisitPowerExprNone(ctx *PowerExprNoneContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#unaryExprOperation.
	VisitUnaryExprOperation(ctx *UnaryExprOperationContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#unaryExprNone.
	VisitUnaryExprNone(ctx *UnaryExprNoneContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#postfixExprField.
	VisitPostfixExprField(ctx *PostfixExprFieldContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#postfixExprArrayIndex.
	VisitPostfixExprArrayIndex(ctx *PostfixExprArrayIndexContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#postfixExprNone.
	VisitPostfixExprNone(ctx *PostfixExprNoneContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#primaryExpression.
	VisitPrimaryExpression(ctx *PrimaryExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#variable.
	VisitVariable(ctx *VariableContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#nullLiteral.
	VisitNullLiteral(ctx *NullLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#noneLiteral.
	VisitNoneLiteral(ctx *NoneLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#booleanLiteral.
	VisitBooleanLiteral(ctx *BooleanLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#numberLiteralInt.
	VisitNumberLiteralInt(ctx *NumberLiteralIntContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#numberLiteralFloat.
	VisitNumberLiteralFloat(ctx *NumberLiteralFloatContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#numberLiteralSigned.
	VisitNumberLiteralSigned(ctx *NumberLiteralSignedContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#arrayLiteral.
	VisitArrayLiteral(ctx *ArrayLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#mapLiteral.
	VisitMapLiteral(ctx *MapLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#mapLiteralItem.
	VisitMapLiteralItem(ctx *MapLiteralItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#objectLiteral.
	VisitObjectLiteral(ctx *ObjectLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#objectLiteralItem.
	VisitObjectLiteralItem(ctx *ObjectLiteralItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#structLiteral.
	VisitStructLiteral(ctx *StructLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#structLiteralItem.
	VisitStructLiteralItem(ctx *StructLiteralItemContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#pairLiteral.
	VisitPairLiteral(ctx *PairLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#groupedExpression.
	VisitGroupedExpression(ctx *GroupedExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#ifExpression.
	VisitIfExpression(ctx *IfExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#callExpression.
	VisitCallExpression(ctx *CallExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringLiteral.
	VisitStringLiteral(ctx *StringLiteralContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#quotedString.
	VisitQuotedString(ctx *QuotedStringContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringElementText.
	VisitStringElementText(ctx *StringElementTextContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringElementEscape.
	VisitStringElementEscape(ctx *StringElementEscapeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringElementDollarSign.
	VisitStringElementDollarSign(ctx *StringElementDollarSignContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringElementTilde.
	VisitStringElementTilde(ctx *StringElementTildeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringElementPlaceholder.
	VisitStringElementPlaceholder(ctx *StringElementPlaceholderContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholder.
	VisitStringPlaceholder(ctx *StringPlaceholderContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineString.
	VisitMultilineString(ctx *MultilineStringContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementText.
	VisitMultilineStringElementText(ctx *MultilineStringElementTextContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementEscape.
	VisitMultilineStringElementEscape(ctx *MultilineStringElementEscapeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementDoubleCloseAngle.
	VisitMultilineStringElementDoubleCloseAngle(ctx *MultilineStringElementDoubleCloseAngleContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementSingleCloseAngle.
	VisitMultilineStringElementSingleCloseAngle(ctx *MultilineStringElementSingleCloseAngleContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementDollarSign.
	VisitMultilineStringElementDollarSign(ctx *MultilineStringElementDollarSignContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementTilde.
	VisitMultilineStringElementTilde(ctx *MultilineStringElementTildeContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementPlaceholder.
	VisitMultilineStringElementPlaceholder(ctx *MultilineStringElementPlaceholderContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#multilineStringPlaceholder.
	VisitMultilineStringPlaceholder(ctx *MultilineStringPlaceholderContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderExpression.
	VisitStringPlaceholderExpression(ctx *StringPlaceholderExpressionContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionSepDefault.
	VisitStringPlaceholderOptionSepDefault(ctx *StringPlaceholderOptionSepDefaultContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionTrueFalse.
	VisitStringPlaceholderOptionTrueFalse(ctx *StringPlaceholderOptionTrueFalseContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionFalseTrue.
	VisitStringPlaceholderOptionFalseTrue(ctx *StringPlaceholderOptionFalseTrueContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#strictIdentifier.
	VisitStrictIdentifier(ctx *StrictIdentifierContext) interface{}

	// Visit a parse tree produced by WdlV1Parser#dottedIdentifier.
	VisitDottedIdentifier(ctx *DottedIdentifierContext) interface{}
}
