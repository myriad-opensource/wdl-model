// Code generated from ./WdlV1Parser.g4 by ANTLR 4.13.1. DO NOT EDIT.

package wdl1 // WdlV1Parser
import "github.com/antlr4-go/antlr/v4"

type BaseWdlV1ParserVisitor struct {
	*antlr.BaseParseTreeVisitor
}

func (v *BaseWdlV1ParserVisitor) VisitDocument(ctx *DocumentContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitVersionStatement(ctx *VersionStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitDocumentElement(ctx *DocumentElementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportStatementStandard(ctx *ImportStatementStandardContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportStatementStar(ctx *ImportStatementStarContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportStatementMembers(ctx *ImportStatementMembersContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportMembers(ctx *ImportMembersContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportMember(ctx *ImportMemberContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportUriLiteral(ctx *ImportUriLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportUriElement(ctx *ImportUriElementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitImportAlias(ctx *ImportAliasContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStructDefinition(ctx *StructDefinitionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStructItemMetadata(ctx *StructItemMetadataContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStructItemParameterMetadata(ctx *StructItemParameterMetadataContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStructItemMemberDeclaration(ctx *StructItemMemberDeclarationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStructDeclaration(ctx *StructDeclarationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumDefinition(ctx *EnumDefinitionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumTypeParameter(ctx *EnumTypeParameterContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumChoice(ctx *EnumChoiceContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumLiteralExpression(ctx *EnumLiteralExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumStringLiteral(ctx *EnumStringLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumQuotedString(ctx *EnumQuotedStringContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumStringElement(ctx *EnumStringElementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumMultilineString(ctx *EnumMultilineStringContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumMultilineStringElement(ctx *EnumMultilineStringElementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumArrayLiteral(ctx *EnumArrayLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumMapLiteral(ctx *EnumMapLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumMapLiteralItem(ctx *EnumMapLiteralItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumObjectLiteral(ctx *EnumObjectLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumObjectLiteralItem(ctx *EnumObjectLiteralItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumStructLiteral(ctx *EnumStructLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumStructLiteralItem(ctx *EnumStructLiteralItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEnumPairLiteral(ctx *EnumPairLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskDefinition(ctx *TaskDefinitionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowDefinition(ctx *WorkflowDefinitionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitType(ctx *TypeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMapType(ctx *MapTypeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitArrayType(ctx *ArrayTypeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPairType(ctx *PairTypeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitObjectType(ctx *ObjectTypeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPrimitiveType(ctx *PrimitiveTypeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTypeRefType(ctx *TypeRefTypeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitUnboundDeclaration(ctx *UnboundDeclarationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitBoundDeclaration(ctx *BoundDeclarationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitDeclaration(ctx *DeclarationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskInputSection(ctx *TaskInputSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskCommandSection(ctx *TaskCommandSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskOutputSection(ctx *TaskOutputSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskRuntimeSection(ctx *TaskRuntimeSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskRequirementsSection(ctx *TaskRequirementsSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskHintsSection(ctx *TaskHintsSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskMetadataSection(ctx *TaskMetadataSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskParameterMetadataSection(ctx *TaskParameterMetadataSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskDeclaration(ctx *TaskDeclarationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowInputSection(ctx *WorkflowInputSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowOutputSection(ctx *WorkflowOutputSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowHintsSection(ctx *WorkflowHintsSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowConditionalStatement(ctx *WorkflowConditionalStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowScatterStatement(ctx *WorkflowScatterStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowCallStatement(ctx *WorkflowCallStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowMetadataSection(ctx *WorkflowMetadataSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowParameterMetadataSection(ctx *WorkflowParameterMetadataSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowDeclaration(ctx *WorkflowDeclarationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitInputSection(ctx *InputSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitOutputSection(ctx *OutputSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitRuntimeSection(ctx *RuntimeSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitRuntimeItem(ctx *RuntimeItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitRequirementsSection(ctx *RequirementsSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitRequirementsItem(ctx *RequirementsItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsSectionTask(ctx *HintsSectionTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsItemTask(ctx *HintsItemTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskHintValueExpression(ctx *TaskHintValueExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskHintValueHintsObject(ctx *TaskHintValueHintsObjectContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskHintValueInputObject(ctx *TaskHintValueInputObjectContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskHintValueOutputObject(ctx *TaskHintValueOutputObjectContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskHintValueArray(ctx *TaskHintValueArrayContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsTypedObjectTask(ctx *HintsTypedObjectTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsObjectItemTask(ctx *HintsObjectItemTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitInputHintsObjectTask(ctx *InputHintsObjectTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitInputHintsItemTask(ctx *InputHintsItemTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitOutputHintsObjectTask(ctx *OutputHintsObjectTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitOutputHintsItemTask(ctx *OutputHintsItemTaskContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitTaskHintsArray(ctx *TaskHintsArrayContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsSectionWorkflow(ctx *HintsSectionWorkflowContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsItemWorkflow(ctx *HintsItemWorkflowContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowHintValueNumber(ctx *WorkflowHintValueNumberContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowHintValueString(ctx *WorkflowHintValueStringContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowHintValueBoolean(ctx *WorkflowHintValueBooleanContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowHintValueObject(ctx *WorkflowHintValueObjectContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowHintValueArray(ctx *WorkflowHintValueArrayContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsObjectWorkflow(ctx *HintsObjectWorkflowContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitHintsObjectItemWorkflow(ctx *HintsObjectItemWorkflowContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowHintsArray(ctx *WorkflowHintsArrayContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMetadataSection(ctx *MetadataSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitParameterMetadataSection(ctx *ParameterMetadataSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMetadataObject(ctx *MetadataObjectContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMetadataObjectItem(ctx *MetadataObjectItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMetadataArray(ctx *MetadataArrayContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMetadataValue(ctx *MetadataValueContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCommandSection(ctx *CommandSectionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringCommand(ctx *MultilineStringCommandContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitBracedCommand(ctx *BracedCommandContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitWorkflowStatement(ctx *WorkflowStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitConditionalStatement(ctx *ConditionalStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitConditionalElseIfClause(ctx *ConditionalElseIfClauseContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitConditionalElseClause(ctx *ConditionalElseClauseContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitScatterStatement(ctx *ScatterStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitScatterBody(ctx *ScatterBodyContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCallStatement(ctx *CallStatementContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCallTarget(ctx *CallTargetContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCallAlias(ctx *CallAliasContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCallAfterClause(ctx *CallAfterClauseContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCallInputBlock(ctx *CallInputBlockContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCallInputItem(ctx *CallInputItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitExpression(ctx *ExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitLogicalOrExprOperation(ctx *LogicalOrExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitLogicalAndExprOperation(ctx *LogicalAndExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitEqualityExprOperation(ctx *EqualityExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitComparisonExprOperation(ctx *ComparisonExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitAdditiveExprOperation(ctx *AdditiveExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultiplicativeExprOperation(ctx *MultiplicativeExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPowerExprOperation(ctx *PowerExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPowerExprNone(ctx *PowerExprNoneContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitUnaryExprOperation(ctx *UnaryExprOperationContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitUnaryExprNone(ctx *UnaryExprNoneContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPostfixExprField(ctx *PostfixExprFieldContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPostfixExprArrayIndex(ctx *PostfixExprArrayIndexContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPostfixExprNone(ctx *PostfixExprNoneContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPrimaryExpression(ctx *PrimaryExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitVariable(ctx *VariableContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitNullLiteral(ctx *NullLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitNoneLiteral(ctx *NoneLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitBooleanLiteral(ctx *BooleanLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitNumberLiteralInt(ctx *NumberLiteralIntContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitNumberLiteralFloat(ctx *NumberLiteralFloatContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitNumberLiteralSigned(ctx *NumberLiteralSignedContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitArrayLiteral(ctx *ArrayLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMapLiteral(ctx *MapLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMapLiteralItem(ctx *MapLiteralItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitObjectLiteral(ctx *ObjectLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitObjectLiteralItem(ctx *ObjectLiteralItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStructLiteral(ctx *StructLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStructLiteralItem(ctx *StructLiteralItemContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitPairLiteral(ctx *PairLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitGroupedExpression(ctx *GroupedExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitIfExpression(ctx *IfExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitCallExpression(ctx *CallExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringLiteral(ctx *StringLiteralContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitQuotedString(ctx *QuotedStringContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringElementText(ctx *StringElementTextContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringElementEscape(ctx *StringElementEscapeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringElementDollarSign(ctx *StringElementDollarSignContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringElementTilde(ctx *StringElementTildeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringElementPlaceholder(ctx *StringElementPlaceholderContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringPlaceholder(ctx *StringPlaceholderContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineString(ctx *MultilineStringContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringElementText(ctx *MultilineStringElementTextContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringElementEscape(ctx *MultilineStringElementEscapeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringElementDoubleCloseAngle(ctx *MultilineStringElementDoubleCloseAngleContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringElementSingleCloseAngle(ctx *MultilineStringElementSingleCloseAngleContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringElementDollarSign(ctx *MultilineStringElementDollarSignContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringElementTilde(ctx *MultilineStringElementTildeContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringElementPlaceholder(ctx *MultilineStringElementPlaceholderContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitMultilineStringPlaceholder(ctx *MultilineStringPlaceholderContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringPlaceholderExpression(ctx *StringPlaceholderExpressionContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringPlaceholderOptionSepDefault(ctx *StringPlaceholderOptionSepDefaultContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringPlaceholderOptionTrueFalse(ctx *StringPlaceholderOptionTrueFalseContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStringPlaceholderOptionFalseTrue(ctx *StringPlaceholderOptionFalseTrueContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitStrictIdentifier(ctx *StrictIdentifierContext) interface{} {
	return v.VisitChildren(ctx)
}

func (v *BaseWdlV1ParserVisitor) VisitDottedIdentifier(ctx *DottedIdentifierContext) interface{} {
	return v.VisitChildren(ctx)
}
