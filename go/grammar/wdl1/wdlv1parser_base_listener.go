// Code generated from ./WdlV1Parser.g4 by ANTLR 4.13.1. DO NOT EDIT.

package wdl1 // WdlV1Parser
import "github.com/antlr4-go/antlr/v4"

// BaseWdlV1ParserListener is a complete listener for a parse tree produced by WdlV1Parser.
type BaseWdlV1ParserListener struct{}

var _ WdlV1ParserListener = &BaseWdlV1ParserListener{}

// VisitTerminal is called when a terminal node is visited.
func (s *BaseWdlV1ParserListener) VisitTerminal(node antlr.TerminalNode) {}

// VisitErrorNode is called when an error node is visited.
func (s *BaseWdlV1ParserListener) VisitErrorNode(node antlr.ErrorNode) {}

// EnterEveryRule is called when any rule is entered.
func (s *BaseWdlV1ParserListener) EnterEveryRule(ctx antlr.ParserRuleContext) {}

// ExitEveryRule is called when any rule is exited.
func (s *BaseWdlV1ParserListener) ExitEveryRule(ctx antlr.ParserRuleContext) {}

// EnterDocument is called when production document is entered.
func (s *BaseWdlV1ParserListener) EnterDocument(ctx *DocumentContext) {}

// ExitDocument is called when production document is exited.
func (s *BaseWdlV1ParserListener) ExitDocument(ctx *DocumentContext) {}

// EnterVersionStatement is called when production versionStatement is entered.
func (s *BaseWdlV1ParserListener) EnterVersionStatement(ctx *VersionStatementContext) {}

// ExitVersionStatement is called when production versionStatement is exited.
func (s *BaseWdlV1ParserListener) ExitVersionStatement(ctx *VersionStatementContext) {}

// EnterDocumentElement is called when production documentElement is entered.
func (s *BaseWdlV1ParserListener) EnterDocumentElement(ctx *DocumentElementContext) {}

// ExitDocumentElement is called when production documentElement is exited.
func (s *BaseWdlV1ParserListener) ExitDocumentElement(ctx *DocumentElementContext) {}

// EnterImportStatementStandard is called when production importStatementStandard is entered.
func (s *BaseWdlV1ParserListener) EnterImportStatementStandard(ctx *ImportStatementStandardContext) {}

// ExitImportStatementStandard is called when production importStatementStandard is exited.
func (s *BaseWdlV1ParserListener) ExitImportStatementStandard(ctx *ImportStatementStandardContext) {}

// EnterImportStatementStar is called when production importStatementStar is entered.
func (s *BaseWdlV1ParserListener) EnterImportStatementStar(ctx *ImportStatementStarContext) {}

// ExitImportStatementStar is called when production importStatementStar is exited.
func (s *BaseWdlV1ParserListener) ExitImportStatementStar(ctx *ImportStatementStarContext) {}

// EnterImportStatementMembers is called when production importStatementMembers is entered.
func (s *BaseWdlV1ParserListener) EnterImportStatementMembers(ctx *ImportStatementMembersContext) {}

// ExitImportStatementMembers is called when production importStatementMembers is exited.
func (s *BaseWdlV1ParserListener) ExitImportStatementMembers(ctx *ImportStatementMembersContext) {}

// EnterImportMembers is called when production importMembers is entered.
func (s *BaseWdlV1ParserListener) EnterImportMembers(ctx *ImportMembersContext) {}

// ExitImportMembers is called when production importMembers is exited.
func (s *BaseWdlV1ParserListener) ExitImportMembers(ctx *ImportMembersContext) {}

// EnterImportMember is called when production importMember is entered.
func (s *BaseWdlV1ParserListener) EnterImportMember(ctx *ImportMemberContext) {}

// ExitImportMember is called when production importMember is exited.
func (s *BaseWdlV1ParserListener) ExitImportMember(ctx *ImportMemberContext) {}

// EnterImportUriLiteral is called when production importUriLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterImportUriLiteral(ctx *ImportUriLiteralContext) {}

// ExitImportUriLiteral is called when production importUriLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitImportUriLiteral(ctx *ImportUriLiteralContext) {}

// EnterImportUriElement is called when production importUriElement is entered.
func (s *BaseWdlV1ParserListener) EnterImportUriElement(ctx *ImportUriElementContext) {}

// ExitImportUriElement is called when production importUriElement is exited.
func (s *BaseWdlV1ParserListener) ExitImportUriElement(ctx *ImportUriElementContext) {}

// EnterImportAlias is called when production importAlias is entered.
func (s *BaseWdlV1ParserListener) EnterImportAlias(ctx *ImportAliasContext) {}

// ExitImportAlias is called when production importAlias is exited.
func (s *BaseWdlV1ParserListener) ExitImportAlias(ctx *ImportAliasContext) {}

// EnterStructDefinition is called when production structDefinition is entered.
func (s *BaseWdlV1ParserListener) EnterStructDefinition(ctx *StructDefinitionContext) {}

// ExitStructDefinition is called when production structDefinition is exited.
func (s *BaseWdlV1ParserListener) ExitStructDefinition(ctx *StructDefinitionContext) {}

// EnterStructItemMetadata is called when production structItemMetadata is entered.
func (s *BaseWdlV1ParserListener) EnterStructItemMetadata(ctx *StructItemMetadataContext) {}

// ExitStructItemMetadata is called when production structItemMetadata is exited.
func (s *BaseWdlV1ParserListener) ExitStructItemMetadata(ctx *StructItemMetadataContext) {}

// EnterStructItemParameterMetadata is called when production structItemParameterMetadata is entered.
func (s *BaseWdlV1ParserListener) EnterStructItemParameterMetadata(ctx *StructItemParameterMetadataContext) {
}

// ExitStructItemParameterMetadata is called when production structItemParameterMetadata is exited.
func (s *BaseWdlV1ParserListener) ExitStructItemParameterMetadata(ctx *StructItemParameterMetadataContext) {
}

// EnterStructItemMemberDeclaration is called when production structItemMemberDeclaration is entered.
func (s *BaseWdlV1ParserListener) EnterStructItemMemberDeclaration(ctx *StructItemMemberDeclarationContext) {
}

// ExitStructItemMemberDeclaration is called when production structItemMemberDeclaration is exited.
func (s *BaseWdlV1ParserListener) ExitStructItemMemberDeclaration(ctx *StructItemMemberDeclarationContext) {
}

// EnterStructDeclaration is called when production structDeclaration is entered.
func (s *BaseWdlV1ParserListener) EnterStructDeclaration(ctx *StructDeclarationContext) {}

// ExitStructDeclaration is called when production structDeclaration is exited.
func (s *BaseWdlV1ParserListener) ExitStructDeclaration(ctx *StructDeclarationContext) {}

// EnterEnumDefinition is called when production enumDefinition is entered.
func (s *BaseWdlV1ParserListener) EnterEnumDefinition(ctx *EnumDefinitionContext) {}

// ExitEnumDefinition is called when production enumDefinition is exited.
func (s *BaseWdlV1ParserListener) ExitEnumDefinition(ctx *EnumDefinitionContext) {}

// EnterEnumTypeParameter is called when production enumTypeParameter is entered.
func (s *BaseWdlV1ParserListener) EnterEnumTypeParameter(ctx *EnumTypeParameterContext) {}

// ExitEnumTypeParameter is called when production enumTypeParameter is exited.
func (s *BaseWdlV1ParserListener) ExitEnumTypeParameter(ctx *EnumTypeParameterContext) {}

// EnterEnumChoice is called when production enumChoice is entered.
func (s *BaseWdlV1ParserListener) EnterEnumChoice(ctx *EnumChoiceContext) {}

// ExitEnumChoice is called when production enumChoice is exited.
func (s *BaseWdlV1ParserListener) ExitEnumChoice(ctx *EnumChoiceContext) {}

// EnterEnumLiteralExpression is called when production enumLiteralExpression is entered.
func (s *BaseWdlV1ParserListener) EnterEnumLiteralExpression(ctx *EnumLiteralExpressionContext) {}

// ExitEnumLiteralExpression is called when production enumLiteralExpression is exited.
func (s *BaseWdlV1ParserListener) ExitEnumLiteralExpression(ctx *EnumLiteralExpressionContext) {}

// EnterEnumStringLiteral is called when production enumStringLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterEnumStringLiteral(ctx *EnumStringLiteralContext) {}

// ExitEnumStringLiteral is called when production enumStringLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitEnumStringLiteral(ctx *EnumStringLiteralContext) {}

// EnterEnumQuotedString is called when production enumQuotedString is entered.
func (s *BaseWdlV1ParserListener) EnterEnumQuotedString(ctx *EnumQuotedStringContext) {}

// ExitEnumQuotedString is called when production enumQuotedString is exited.
func (s *BaseWdlV1ParserListener) ExitEnumQuotedString(ctx *EnumQuotedStringContext) {}

// EnterEnumStringElement is called when production enumStringElement is entered.
func (s *BaseWdlV1ParserListener) EnterEnumStringElement(ctx *EnumStringElementContext) {}

// ExitEnumStringElement is called when production enumStringElement is exited.
func (s *BaseWdlV1ParserListener) ExitEnumStringElement(ctx *EnumStringElementContext) {}

// EnterEnumMultilineString is called when production enumMultilineString is entered.
func (s *BaseWdlV1ParserListener) EnterEnumMultilineString(ctx *EnumMultilineStringContext) {}

// ExitEnumMultilineString is called when production enumMultilineString is exited.
func (s *BaseWdlV1ParserListener) ExitEnumMultilineString(ctx *EnumMultilineStringContext) {}

// EnterEnumMultilineStringElement is called when production enumMultilineStringElement is entered.
func (s *BaseWdlV1ParserListener) EnterEnumMultilineStringElement(ctx *EnumMultilineStringElementContext) {
}

// ExitEnumMultilineStringElement is called when production enumMultilineStringElement is exited.
func (s *BaseWdlV1ParserListener) ExitEnumMultilineStringElement(ctx *EnumMultilineStringElementContext) {
}

// EnterEnumArrayLiteral is called when production enumArrayLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterEnumArrayLiteral(ctx *EnumArrayLiteralContext) {}

// ExitEnumArrayLiteral is called when production enumArrayLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitEnumArrayLiteral(ctx *EnumArrayLiteralContext) {}

// EnterEnumMapLiteral is called when production enumMapLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterEnumMapLiteral(ctx *EnumMapLiteralContext) {}

// ExitEnumMapLiteral is called when production enumMapLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitEnumMapLiteral(ctx *EnumMapLiteralContext) {}

// EnterEnumMapLiteralItem is called when production enumMapLiteralItem is entered.
func (s *BaseWdlV1ParserListener) EnterEnumMapLiteralItem(ctx *EnumMapLiteralItemContext) {}

// ExitEnumMapLiteralItem is called when production enumMapLiteralItem is exited.
func (s *BaseWdlV1ParserListener) ExitEnumMapLiteralItem(ctx *EnumMapLiteralItemContext) {}

// EnterEnumObjectLiteral is called when production enumObjectLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterEnumObjectLiteral(ctx *EnumObjectLiteralContext) {}

// ExitEnumObjectLiteral is called when production enumObjectLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitEnumObjectLiteral(ctx *EnumObjectLiteralContext) {}

// EnterEnumObjectLiteralItem is called when production enumObjectLiteralItem is entered.
func (s *BaseWdlV1ParserListener) EnterEnumObjectLiteralItem(ctx *EnumObjectLiteralItemContext) {}

// ExitEnumObjectLiteralItem is called when production enumObjectLiteralItem is exited.
func (s *BaseWdlV1ParserListener) ExitEnumObjectLiteralItem(ctx *EnumObjectLiteralItemContext) {}

// EnterEnumStructLiteral is called when production enumStructLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterEnumStructLiteral(ctx *EnumStructLiteralContext) {}

// ExitEnumStructLiteral is called when production enumStructLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitEnumStructLiteral(ctx *EnumStructLiteralContext) {}

// EnterEnumStructLiteralItem is called when production enumStructLiteralItem is entered.
func (s *BaseWdlV1ParserListener) EnterEnumStructLiteralItem(ctx *EnumStructLiteralItemContext) {}

// ExitEnumStructLiteralItem is called when production enumStructLiteralItem is exited.
func (s *BaseWdlV1ParserListener) ExitEnumStructLiteralItem(ctx *EnumStructLiteralItemContext) {}

// EnterEnumPairLiteral is called when production enumPairLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterEnumPairLiteral(ctx *EnumPairLiteralContext) {}

// ExitEnumPairLiteral is called when production enumPairLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitEnumPairLiteral(ctx *EnumPairLiteralContext) {}

// EnterTaskDefinition is called when production taskDefinition is entered.
func (s *BaseWdlV1ParserListener) EnterTaskDefinition(ctx *TaskDefinitionContext) {}

// ExitTaskDefinition is called when production taskDefinition is exited.
func (s *BaseWdlV1ParserListener) ExitTaskDefinition(ctx *TaskDefinitionContext) {}

// EnterWorkflowDefinition is called when production workflowDefinition is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowDefinition(ctx *WorkflowDefinitionContext) {}

// ExitWorkflowDefinition is called when production workflowDefinition is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowDefinition(ctx *WorkflowDefinitionContext) {}

// EnterType is called when production type is entered.
func (s *BaseWdlV1ParserListener) EnterType(ctx *TypeContext) {}

// ExitType is called when production type is exited.
func (s *BaseWdlV1ParserListener) ExitType(ctx *TypeContext) {}

// EnterMapType is called when production mapType is entered.
func (s *BaseWdlV1ParserListener) EnterMapType(ctx *MapTypeContext) {}

// ExitMapType is called when production mapType is exited.
func (s *BaseWdlV1ParserListener) ExitMapType(ctx *MapTypeContext) {}

// EnterArrayType is called when production arrayType is entered.
func (s *BaseWdlV1ParserListener) EnterArrayType(ctx *ArrayTypeContext) {}

// ExitArrayType is called when production arrayType is exited.
func (s *BaseWdlV1ParserListener) ExitArrayType(ctx *ArrayTypeContext) {}

// EnterPairType is called when production pairType is entered.
func (s *BaseWdlV1ParserListener) EnterPairType(ctx *PairTypeContext) {}

// ExitPairType is called when production pairType is exited.
func (s *BaseWdlV1ParserListener) ExitPairType(ctx *PairTypeContext) {}

// EnterObjectType is called when production objectType is entered.
func (s *BaseWdlV1ParserListener) EnterObjectType(ctx *ObjectTypeContext) {}

// ExitObjectType is called when production objectType is exited.
func (s *BaseWdlV1ParserListener) ExitObjectType(ctx *ObjectTypeContext) {}

// EnterPrimitiveType is called when production primitiveType is entered.
func (s *BaseWdlV1ParserListener) EnterPrimitiveType(ctx *PrimitiveTypeContext) {}

// ExitPrimitiveType is called when production primitiveType is exited.
func (s *BaseWdlV1ParserListener) ExitPrimitiveType(ctx *PrimitiveTypeContext) {}

// EnterTypeRefType is called when production typeRefType is entered.
func (s *BaseWdlV1ParserListener) EnterTypeRefType(ctx *TypeRefTypeContext) {}

// ExitTypeRefType is called when production typeRefType is exited.
func (s *BaseWdlV1ParserListener) ExitTypeRefType(ctx *TypeRefTypeContext) {}

// EnterUnboundDeclaration is called when production unboundDeclaration is entered.
func (s *BaseWdlV1ParserListener) EnterUnboundDeclaration(ctx *UnboundDeclarationContext) {}

// ExitUnboundDeclaration is called when production unboundDeclaration is exited.
func (s *BaseWdlV1ParserListener) ExitUnboundDeclaration(ctx *UnboundDeclarationContext) {}

// EnterBoundDeclaration is called when production boundDeclaration is entered.
func (s *BaseWdlV1ParserListener) EnterBoundDeclaration(ctx *BoundDeclarationContext) {}

// ExitBoundDeclaration is called when production boundDeclaration is exited.
func (s *BaseWdlV1ParserListener) ExitBoundDeclaration(ctx *BoundDeclarationContext) {}

// EnterDeclaration is called when production declaration is entered.
func (s *BaseWdlV1ParserListener) EnterDeclaration(ctx *DeclarationContext) {}

// ExitDeclaration is called when production declaration is exited.
func (s *BaseWdlV1ParserListener) ExitDeclaration(ctx *DeclarationContext) {}

// EnterTaskInputSection is called when production taskInputSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskInputSection(ctx *TaskInputSectionContext) {}

// ExitTaskInputSection is called when production taskInputSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskInputSection(ctx *TaskInputSectionContext) {}

// EnterTaskCommandSection is called when production taskCommandSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskCommandSection(ctx *TaskCommandSectionContext) {}

// ExitTaskCommandSection is called when production taskCommandSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskCommandSection(ctx *TaskCommandSectionContext) {}

// EnterTaskOutputSection is called when production taskOutputSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskOutputSection(ctx *TaskOutputSectionContext) {}

// ExitTaskOutputSection is called when production taskOutputSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskOutputSection(ctx *TaskOutputSectionContext) {}

// EnterTaskRuntimeSection is called when production taskRuntimeSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskRuntimeSection(ctx *TaskRuntimeSectionContext) {}

// ExitTaskRuntimeSection is called when production taskRuntimeSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskRuntimeSection(ctx *TaskRuntimeSectionContext) {}

// EnterTaskRequirementsSection is called when production taskRequirementsSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskRequirementsSection(ctx *TaskRequirementsSectionContext) {}

// ExitTaskRequirementsSection is called when production taskRequirementsSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskRequirementsSection(ctx *TaskRequirementsSectionContext) {}

// EnterTaskHintsSection is called when production taskHintsSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskHintsSection(ctx *TaskHintsSectionContext) {}

// ExitTaskHintsSection is called when production taskHintsSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskHintsSection(ctx *TaskHintsSectionContext) {}

// EnterTaskMetadataSection is called when production taskMetadataSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskMetadataSection(ctx *TaskMetadataSectionContext) {}

// ExitTaskMetadataSection is called when production taskMetadataSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskMetadataSection(ctx *TaskMetadataSectionContext) {}

// EnterTaskParameterMetadataSection is called when production taskParameterMetadataSection is entered.
func (s *BaseWdlV1ParserListener) EnterTaskParameterMetadataSection(ctx *TaskParameterMetadataSectionContext) {
}

// ExitTaskParameterMetadataSection is called when production taskParameterMetadataSection is exited.
func (s *BaseWdlV1ParserListener) ExitTaskParameterMetadataSection(ctx *TaskParameterMetadataSectionContext) {
}

// EnterTaskDeclaration is called when production taskDeclaration is entered.
func (s *BaseWdlV1ParserListener) EnterTaskDeclaration(ctx *TaskDeclarationContext) {}

// ExitTaskDeclaration is called when production taskDeclaration is exited.
func (s *BaseWdlV1ParserListener) ExitTaskDeclaration(ctx *TaskDeclarationContext) {}

// EnterWorkflowInputSection is called when production workflowInputSection is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowInputSection(ctx *WorkflowInputSectionContext) {}

// ExitWorkflowInputSection is called when production workflowInputSection is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowInputSection(ctx *WorkflowInputSectionContext) {}

// EnterWorkflowOutputSection is called when production workflowOutputSection is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowOutputSection(ctx *WorkflowOutputSectionContext) {}

// ExitWorkflowOutputSection is called when production workflowOutputSection is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowOutputSection(ctx *WorkflowOutputSectionContext) {}

// EnterWorkflowHintsSection is called when production workflowHintsSection is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowHintsSection(ctx *WorkflowHintsSectionContext) {}

// ExitWorkflowHintsSection is called when production workflowHintsSection is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowHintsSection(ctx *WorkflowHintsSectionContext) {}

// EnterWorkflowConditionalStatement is called when production workflowConditionalStatement is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowConditionalStatement(ctx *WorkflowConditionalStatementContext) {
}

// ExitWorkflowConditionalStatement is called when production workflowConditionalStatement is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowConditionalStatement(ctx *WorkflowConditionalStatementContext) {
}

// EnterWorkflowScatterStatement is called when production workflowScatterStatement is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowScatterStatement(ctx *WorkflowScatterStatementContext) {
}

// ExitWorkflowScatterStatement is called when production workflowScatterStatement is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowScatterStatement(ctx *WorkflowScatterStatementContext) {
}

// EnterWorkflowCallStatement is called when production workflowCallStatement is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowCallStatement(ctx *WorkflowCallStatementContext) {}

// ExitWorkflowCallStatement is called when production workflowCallStatement is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowCallStatement(ctx *WorkflowCallStatementContext) {}

// EnterWorkflowMetadataSection is called when production workflowMetadataSection is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowMetadataSection(ctx *WorkflowMetadataSectionContext) {}

// ExitWorkflowMetadataSection is called when production workflowMetadataSection is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowMetadataSection(ctx *WorkflowMetadataSectionContext) {}

// EnterWorkflowParameterMetadataSection is called when production workflowParameterMetadataSection is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowParameterMetadataSection(ctx *WorkflowParameterMetadataSectionContext) {
}

// ExitWorkflowParameterMetadataSection is called when production workflowParameterMetadataSection is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowParameterMetadataSection(ctx *WorkflowParameterMetadataSectionContext) {
}

// EnterWorkflowDeclaration is called when production workflowDeclaration is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowDeclaration(ctx *WorkflowDeclarationContext) {}

// ExitWorkflowDeclaration is called when production workflowDeclaration is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowDeclaration(ctx *WorkflowDeclarationContext) {}

// EnterInputSection is called when production inputSection is entered.
func (s *BaseWdlV1ParserListener) EnterInputSection(ctx *InputSectionContext) {}

// ExitInputSection is called when production inputSection is exited.
func (s *BaseWdlV1ParserListener) ExitInputSection(ctx *InputSectionContext) {}

// EnterOutputSection is called when production outputSection is entered.
func (s *BaseWdlV1ParserListener) EnterOutputSection(ctx *OutputSectionContext) {}

// ExitOutputSection is called when production outputSection is exited.
func (s *BaseWdlV1ParserListener) ExitOutputSection(ctx *OutputSectionContext) {}

// EnterRuntimeSection is called when production runtimeSection is entered.
func (s *BaseWdlV1ParserListener) EnterRuntimeSection(ctx *RuntimeSectionContext) {}

// ExitRuntimeSection is called when production runtimeSection is exited.
func (s *BaseWdlV1ParserListener) ExitRuntimeSection(ctx *RuntimeSectionContext) {}

// EnterRuntimeItem is called when production runtimeItem is entered.
func (s *BaseWdlV1ParserListener) EnterRuntimeItem(ctx *RuntimeItemContext) {}

// ExitRuntimeItem is called when production runtimeItem is exited.
func (s *BaseWdlV1ParserListener) ExitRuntimeItem(ctx *RuntimeItemContext) {}

// EnterRequirementsSection is called when production requirementsSection is entered.
func (s *BaseWdlV1ParserListener) EnterRequirementsSection(ctx *RequirementsSectionContext) {}

// ExitRequirementsSection is called when production requirementsSection is exited.
func (s *BaseWdlV1ParserListener) ExitRequirementsSection(ctx *RequirementsSectionContext) {}

// EnterRequirementsItem is called when production requirementsItem is entered.
func (s *BaseWdlV1ParserListener) EnterRequirementsItem(ctx *RequirementsItemContext) {}

// ExitRequirementsItem is called when production requirementsItem is exited.
func (s *BaseWdlV1ParserListener) ExitRequirementsItem(ctx *RequirementsItemContext) {}

// EnterHintsSectionTask is called when production hintsSectionTask is entered.
func (s *BaseWdlV1ParserListener) EnterHintsSectionTask(ctx *HintsSectionTaskContext) {}

// ExitHintsSectionTask is called when production hintsSectionTask is exited.
func (s *BaseWdlV1ParserListener) ExitHintsSectionTask(ctx *HintsSectionTaskContext) {}

// EnterHintsItemTask is called when production hintsItemTask is entered.
func (s *BaseWdlV1ParserListener) EnterHintsItemTask(ctx *HintsItemTaskContext) {}

// ExitHintsItemTask is called when production hintsItemTask is exited.
func (s *BaseWdlV1ParserListener) ExitHintsItemTask(ctx *HintsItemTaskContext) {}

// EnterTaskHintValueExpression is called when production taskHintValueExpression is entered.
func (s *BaseWdlV1ParserListener) EnterTaskHintValueExpression(ctx *TaskHintValueExpressionContext) {}

// ExitTaskHintValueExpression is called when production taskHintValueExpression is exited.
func (s *BaseWdlV1ParserListener) ExitTaskHintValueExpression(ctx *TaskHintValueExpressionContext) {}

// EnterTaskHintValueHintsObject is called when production taskHintValueHintsObject is entered.
func (s *BaseWdlV1ParserListener) EnterTaskHintValueHintsObject(ctx *TaskHintValueHintsObjectContext) {
}

// ExitTaskHintValueHintsObject is called when production taskHintValueHintsObject is exited.
func (s *BaseWdlV1ParserListener) ExitTaskHintValueHintsObject(ctx *TaskHintValueHintsObjectContext) {
}

// EnterTaskHintValueInputObject is called when production taskHintValueInputObject is entered.
func (s *BaseWdlV1ParserListener) EnterTaskHintValueInputObject(ctx *TaskHintValueInputObjectContext) {
}

// ExitTaskHintValueInputObject is called when production taskHintValueInputObject is exited.
func (s *BaseWdlV1ParserListener) ExitTaskHintValueInputObject(ctx *TaskHintValueInputObjectContext) {
}

// EnterTaskHintValueOutputObject is called when production taskHintValueOutputObject is entered.
func (s *BaseWdlV1ParserListener) EnterTaskHintValueOutputObject(ctx *TaskHintValueOutputObjectContext) {
}

// ExitTaskHintValueOutputObject is called when production taskHintValueOutputObject is exited.
func (s *BaseWdlV1ParserListener) ExitTaskHintValueOutputObject(ctx *TaskHintValueOutputObjectContext) {
}

// EnterTaskHintValueArray is called when production taskHintValueArray is entered.
func (s *BaseWdlV1ParserListener) EnterTaskHintValueArray(ctx *TaskHintValueArrayContext) {}

// ExitTaskHintValueArray is called when production taskHintValueArray is exited.
func (s *BaseWdlV1ParserListener) ExitTaskHintValueArray(ctx *TaskHintValueArrayContext) {}

// EnterHintsTypedObjectTask is called when production hintsTypedObjectTask is entered.
func (s *BaseWdlV1ParserListener) EnterHintsTypedObjectTask(ctx *HintsTypedObjectTaskContext) {}

// ExitHintsTypedObjectTask is called when production hintsTypedObjectTask is exited.
func (s *BaseWdlV1ParserListener) ExitHintsTypedObjectTask(ctx *HintsTypedObjectTaskContext) {}

// EnterHintsObjectItemTask is called when production hintsObjectItemTask is entered.
func (s *BaseWdlV1ParserListener) EnterHintsObjectItemTask(ctx *HintsObjectItemTaskContext) {}

// ExitHintsObjectItemTask is called when production hintsObjectItemTask is exited.
func (s *BaseWdlV1ParserListener) ExitHintsObjectItemTask(ctx *HintsObjectItemTaskContext) {}

// EnterInputHintsObjectTask is called when production inputHintsObjectTask is entered.
func (s *BaseWdlV1ParserListener) EnterInputHintsObjectTask(ctx *InputHintsObjectTaskContext) {}

// ExitInputHintsObjectTask is called when production inputHintsObjectTask is exited.
func (s *BaseWdlV1ParserListener) ExitInputHintsObjectTask(ctx *InputHintsObjectTaskContext) {}

// EnterInputHintsItemTask is called when production inputHintsItemTask is entered.
func (s *BaseWdlV1ParserListener) EnterInputHintsItemTask(ctx *InputHintsItemTaskContext) {}

// ExitInputHintsItemTask is called when production inputHintsItemTask is exited.
func (s *BaseWdlV1ParserListener) ExitInputHintsItemTask(ctx *InputHintsItemTaskContext) {}

// EnterOutputHintsObjectTask is called when production outputHintsObjectTask is entered.
func (s *BaseWdlV1ParserListener) EnterOutputHintsObjectTask(ctx *OutputHintsObjectTaskContext) {}

// ExitOutputHintsObjectTask is called when production outputHintsObjectTask is exited.
func (s *BaseWdlV1ParserListener) ExitOutputHintsObjectTask(ctx *OutputHintsObjectTaskContext) {}

// EnterOutputHintsItemTask is called when production outputHintsItemTask is entered.
func (s *BaseWdlV1ParserListener) EnterOutputHintsItemTask(ctx *OutputHintsItemTaskContext) {}

// ExitOutputHintsItemTask is called when production outputHintsItemTask is exited.
func (s *BaseWdlV1ParserListener) ExitOutputHintsItemTask(ctx *OutputHintsItemTaskContext) {}

// EnterTaskHintsArray is called when production taskHintsArray is entered.
func (s *BaseWdlV1ParserListener) EnterTaskHintsArray(ctx *TaskHintsArrayContext) {}

// ExitTaskHintsArray is called when production taskHintsArray is exited.
func (s *BaseWdlV1ParserListener) ExitTaskHintsArray(ctx *TaskHintsArrayContext) {}

// EnterHintsSectionWorkflow is called when production hintsSectionWorkflow is entered.
func (s *BaseWdlV1ParserListener) EnterHintsSectionWorkflow(ctx *HintsSectionWorkflowContext) {}

// ExitHintsSectionWorkflow is called when production hintsSectionWorkflow is exited.
func (s *BaseWdlV1ParserListener) ExitHintsSectionWorkflow(ctx *HintsSectionWorkflowContext) {}

// EnterHintsItemWorkflow is called when production hintsItemWorkflow is entered.
func (s *BaseWdlV1ParserListener) EnterHintsItemWorkflow(ctx *HintsItemWorkflowContext) {}

// ExitHintsItemWorkflow is called when production hintsItemWorkflow is exited.
func (s *BaseWdlV1ParserListener) ExitHintsItemWorkflow(ctx *HintsItemWorkflowContext) {}

// EnterWorkflowHintValueNumber is called when production workflowHintValueNumber is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowHintValueNumber(ctx *WorkflowHintValueNumberContext) {}

// ExitWorkflowHintValueNumber is called when production workflowHintValueNumber is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowHintValueNumber(ctx *WorkflowHintValueNumberContext) {}

// EnterWorkflowHintValueString is called when production workflowHintValueString is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowHintValueString(ctx *WorkflowHintValueStringContext) {}

// ExitWorkflowHintValueString is called when production workflowHintValueString is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowHintValueString(ctx *WorkflowHintValueStringContext) {}

// EnterWorkflowHintValueBoolean is called when production workflowHintValueBoolean is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowHintValueBoolean(ctx *WorkflowHintValueBooleanContext) {
}

// ExitWorkflowHintValueBoolean is called when production workflowHintValueBoolean is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowHintValueBoolean(ctx *WorkflowHintValueBooleanContext) {
}

// EnterWorkflowHintValueObject is called when production workflowHintValueObject is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowHintValueObject(ctx *WorkflowHintValueObjectContext) {}

// ExitWorkflowHintValueObject is called when production workflowHintValueObject is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowHintValueObject(ctx *WorkflowHintValueObjectContext) {}

// EnterWorkflowHintValueArray is called when production workflowHintValueArray is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowHintValueArray(ctx *WorkflowHintValueArrayContext) {}

// ExitWorkflowHintValueArray is called when production workflowHintValueArray is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowHintValueArray(ctx *WorkflowHintValueArrayContext) {}

// EnterHintsObjectWorkflow is called when production hintsObjectWorkflow is entered.
func (s *BaseWdlV1ParserListener) EnterHintsObjectWorkflow(ctx *HintsObjectWorkflowContext) {}

// ExitHintsObjectWorkflow is called when production hintsObjectWorkflow is exited.
func (s *BaseWdlV1ParserListener) ExitHintsObjectWorkflow(ctx *HintsObjectWorkflowContext) {}

// EnterHintsObjectItemWorkflow is called when production hintsObjectItemWorkflow is entered.
func (s *BaseWdlV1ParserListener) EnterHintsObjectItemWorkflow(ctx *HintsObjectItemWorkflowContext) {}

// ExitHintsObjectItemWorkflow is called when production hintsObjectItemWorkflow is exited.
func (s *BaseWdlV1ParserListener) ExitHintsObjectItemWorkflow(ctx *HintsObjectItemWorkflowContext) {}

// EnterWorkflowHintsArray is called when production workflowHintsArray is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowHintsArray(ctx *WorkflowHintsArrayContext) {}

// ExitWorkflowHintsArray is called when production workflowHintsArray is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowHintsArray(ctx *WorkflowHintsArrayContext) {}

// EnterMetadataSection is called when production metadataSection is entered.
func (s *BaseWdlV1ParserListener) EnterMetadataSection(ctx *MetadataSectionContext) {}

// ExitMetadataSection is called when production metadataSection is exited.
func (s *BaseWdlV1ParserListener) ExitMetadataSection(ctx *MetadataSectionContext) {}

// EnterParameterMetadataSection is called when production parameterMetadataSection is entered.
func (s *BaseWdlV1ParserListener) EnterParameterMetadataSection(ctx *ParameterMetadataSectionContext) {
}

// ExitParameterMetadataSection is called when production parameterMetadataSection is exited.
func (s *BaseWdlV1ParserListener) ExitParameterMetadataSection(ctx *ParameterMetadataSectionContext) {
}

// EnterMetadataObject is called when production metadataObject is entered.
func (s *BaseWdlV1ParserListener) EnterMetadataObject(ctx *MetadataObjectContext) {}

// ExitMetadataObject is called when production metadataObject is exited.
func (s *BaseWdlV1ParserListener) ExitMetadataObject(ctx *MetadataObjectContext) {}

// EnterMetadataObjectItem is called when production metadataObjectItem is entered.
func (s *BaseWdlV1ParserListener) EnterMetadataObjectItem(ctx *MetadataObjectItemContext) {}

// ExitMetadataObjectItem is called when production metadataObjectItem is exited.
func (s *BaseWdlV1ParserListener) ExitMetadataObjectItem(ctx *MetadataObjectItemContext) {}

// EnterMetadataArray is called when production metadataArray is entered.
func (s *BaseWdlV1ParserListener) EnterMetadataArray(ctx *MetadataArrayContext) {}

// ExitMetadataArray is called when production metadataArray is exited.
func (s *BaseWdlV1ParserListener) ExitMetadataArray(ctx *MetadataArrayContext) {}

// EnterMetadataValue is called when production metadataValue is entered.
func (s *BaseWdlV1ParserListener) EnterMetadataValue(ctx *MetadataValueContext) {}

// ExitMetadataValue is called when production metadataValue is exited.
func (s *BaseWdlV1ParserListener) ExitMetadataValue(ctx *MetadataValueContext) {}

// EnterCommandSection is called when production commandSection is entered.
func (s *BaseWdlV1ParserListener) EnterCommandSection(ctx *CommandSectionContext) {}

// ExitCommandSection is called when production commandSection is exited.
func (s *BaseWdlV1ParserListener) ExitCommandSection(ctx *CommandSectionContext) {}

// EnterMultilineStringCommand is called when production multilineStringCommand is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringCommand(ctx *MultilineStringCommandContext) {}

// ExitMultilineStringCommand is called when production multilineStringCommand is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringCommand(ctx *MultilineStringCommandContext) {}

// EnterBracedCommand is called when production bracedCommand is entered.
func (s *BaseWdlV1ParserListener) EnterBracedCommand(ctx *BracedCommandContext) {}

// ExitBracedCommand is called when production bracedCommand is exited.
func (s *BaseWdlV1ParserListener) ExitBracedCommand(ctx *BracedCommandContext) {}

// EnterWorkflowStatement is called when production workflowStatement is entered.
func (s *BaseWdlV1ParserListener) EnterWorkflowStatement(ctx *WorkflowStatementContext) {}

// ExitWorkflowStatement is called when production workflowStatement is exited.
func (s *BaseWdlV1ParserListener) ExitWorkflowStatement(ctx *WorkflowStatementContext) {}

// EnterConditionalStatement is called when production conditionalStatement is entered.
func (s *BaseWdlV1ParserListener) EnterConditionalStatement(ctx *ConditionalStatementContext) {}

// ExitConditionalStatement is called when production conditionalStatement is exited.
func (s *BaseWdlV1ParserListener) ExitConditionalStatement(ctx *ConditionalStatementContext) {}

// EnterConditionalElseIfClause is called when production conditionalElseIfClause is entered.
func (s *BaseWdlV1ParserListener) EnterConditionalElseIfClause(ctx *ConditionalElseIfClauseContext) {}

// ExitConditionalElseIfClause is called when production conditionalElseIfClause is exited.
func (s *BaseWdlV1ParserListener) ExitConditionalElseIfClause(ctx *ConditionalElseIfClauseContext) {}

// EnterConditionalElseClause is called when production conditionalElseClause is entered.
func (s *BaseWdlV1ParserListener) EnterConditionalElseClause(ctx *ConditionalElseClauseContext) {}

// ExitConditionalElseClause is called when production conditionalElseClause is exited.
func (s *BaseWdlV1ParserListener) ExitConditionalElseClause(ctx *ConditionalElseClauseContext) {}

// EnterScatterStatement is called when production scatterStatement is entered.
func (s *BaseWdlV1ParserListener) EnterScatterStatement(ctx *ScatterStatementContext) {}

// ExitScatterStatement is called when production scatterStatement is exited.
func (s *BaseWdlV1ParserListener) ExitScatterStatement(ctx *ScatterStatementContext) {}

// EnterScatterBody is called when production scatterBody is entered.
func (s *BaseWdlV1ParserListener) EnterScatterBody(ctx *ScatterBodyContext) {}

// ExitScatterBody is called when production scatterBody is exited.
func (s *BaseWdlV1ParserListener) ExitScatterBody(ctx *ScatterBodyContext) {}

// EnterCallStatement is called when production callStatement is entered.
func (s *BaseWdlV1ParserListener) EnterCallStatement(ctx *CallStatementContext) {}

// ExitCallStatement is called when production callStatement is exited.
func (s *BaseWdlV1ParserListener) ExitCallStatement(ctx *CallStatementContext) {}

// EnterCallTarget is called when production callTarget is entered.
func (s *BaseWdlV1ParserListener) EnterCallTarget(ctx *CallTargetContext) {}

// ExitCallTarget is called when production callTarget is exited.
func (s *BaseWdlV1ParserListener) ExitCallTarget(ctx *CallTargetContext) {}

// EnterCallAlias is called when production callAlias is entered.
func (s *BaseWdlV1ParserListener) EnterCallAlias(ctx *CallAliasContext) {}

// ExitCallAlias is called when production callAlias is exited.
func (s *BaseWdlV1ParserListener) ExitCallAlias(ctx *CallAliasContext) {}

// EnterCallAfterClause is called when production callAfterClause is entered.
func (s *BaseWdlV1ParserListener) EnterCallAfterClause(ctx *CallAfterClauseContext) {}

// ExitCallAfterClause is called when production callAfterClause is exited.
func (s *BaseWdlV1ParserListener) ExitCallAfterClause(ctx *CallAfterClauseContext) {}

// EnterCallInputBlock is called when production callInputBlock is entered.
func (s *BaseWdlV1ParserListener) EnterCallInputBlock(ctx *CallInputBlockContext) {}

// ExitCallInputBlock is called when production callInputBlock is exited.
func (s *BaseWdlV1ParserListener) ExitCallInputBlock(ctx *CallInputBlockContext) {}

// EnterCallInputItem is called when production callInputItem is entered.
func (s *BaseWdlV1ParserListener) EnterCallInputItem(ctx *CallInputItemContext) {}

// ExitCallInputItem is called when production callInputItem is exited.
func (s *BaseWdlV1ParserListener) ExitCallInputItem(ctx *CallInputItemContext) {}

// EnterExpression is called when production expression is entered.
func (s *BaseWdlV1ParserListener) EnterExpression(ctx *ExpressionContext) {}

// ExitExpression is called when production expression is exited.
func (s *BaseWdlV1ParserListener) ExitExpression(ctx *ExpressionContext) {}

// EnterLogicalOrExprOperation is called when production logicalOrExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterLogicalOrExprOperation(ctx *LogicalOrExprOperationContext) {}

// ExitLogicalOrExprOperation is called when production logicalOrExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitLogicalOrExprOperation(ctx *LogicalOrExprOperationContext) {}

// EnterLogicalAndExprOperation is called when production logicalAndExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterLogicalAndExprOperation(ctx *LogicalAndExprOperationContext) {}

// ExitLogicalAndExprOperation is called when production logicalAndExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitLogicalAndExprOperation(ctx *LogicalAndExprOperationContext) {}

// EnterEqualityExprOperation is called when production equalityExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterEqualityExprOperation(ctx *EqualityExprOperationContext) {}

// ExitEqualityExprOperation is called when production equalityExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitEqualityExprOperation(ctx *EqualityExprOperationContext) {}

// EnterComparisonExprOperation is called when production comparisonExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterComparisonExprOperation(ctx *ComparisonExprOperationContext) {}

// ExitComparisonExprOperation is called when production comparisonExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitComparisonExprOperation(ctx *ComparisonExprOperationContext) {}

// EnterAdditiveExprOperation is called when production additiveExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterAdditiveExprOperation(ctx *AdditiveExprOperationContext) {}

// ExitAdditiveExprOperation is called when production additiveExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitAdditiveExprOperation(ctx *AdditiveExprOperationContext) {}

// EnterMultiplicativeExprOperation is called when production multiplicativeExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterMultiplicativeExprOperation(ctx *MultiplicativeExprOperationContext) {
}

// ExitMultiplicativeExprOperation is called when production multiplicativeExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitMultiplicativeExprOperation(ctx *MultiplicativeExprOperationContext) {
}

// EnterPowerExprOperation is called when production powerExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterPowerExprOperation(ctx *PowerExprOperationContext) {}

// ExitPowerExprOperation is called when production powerExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitPowerExprOperation(ctx *PowerExprOperationContext) {}

// EnterPowerExprNone is called when production powerExprNone is entered.
func (s *BaseWdlV1ParserListener) EnterPowerExprNone(ctx *PowerExprNoneContext) {}

// ExitPowerExprNone is called when production powerExprNone is exited.
func (s *BaseWdlV1ParserListener) ExitPowerExprNone(ctx *PowerExprNoneContext) {}

// EnterUnaryExprOperation is called when production unaryExprOperation is entered.
func (s *BaseWdlV1ParserListener) EnterUnaryExprOperation(ctx *UnaryExprOperationContext) {}

// ExitUnaryExprOperation is called when production unaryExprOperation is exited.
func (s *BaseWdlV1ParserListener) ExitUnaryExprOperation(ctx *UnaryExprOperationContext) {}

// EnterUnaryExprNone is called when production unaryExprNone is entered.
func (s *BaseWdlV1ParserListener) EnterUnaryExprNone(ctx *UnaryExprNoneContext) {}

// ExitUnaryExprNone is called when production unaryExprNone is exited.
func (s *BaseWdlV1ParserListener) ExitUnaryExprNone(ctx *UnaryExprNoneContext) {}

// EnterPostfixExprField is called when production postfixExprField is entered.
func (s *BaseWdlV1ParserListener) EnterPostfixExprField(ctx *PostfixExprFieldContext) {}

// ExitPostfixExprField is called when production postfixExprField is exited.
func (s *BaseWdlV1ParserListener) ExitPostfixExprField(ctx *PostfixExprFieldContext) {}

// EnterPostfixExprArrayIndex is called when production postfixExprArrayIndex is entered.
func (s *BaseWdlV1ParserListener) EnterPostfixExprArrayIndex(ctx *PostfixExprArrayIndexContext) {}

// ExitPostfixExprArrayIndex is called when production postfixExprArrayIndex is exited.
func (s *BaseWdlV1ParserListener) ExitPostfixExprArrayIndex(ctx *PostfixExprArrayIndexContext) {}

// EnterPostfixExprNone is called when production postfixExprNone is entered.
func (s *BaseWdlV1ParserListener) EnterPostfixExprNone(ctx *PostfixExprNoneContext) {}

// ExitPostfixExprNone is called when production postfixExprNone is exited.
func (s *BaseWdlV1ParserListener) ExitPostfixExprNone(ctx *PostfixExprNoneContext) {}

// EnterPrimaryExpression is called when production primaryExpression is entered.
func (s *BaseWdlV1ParserListener) EnterPrimaryExpression(ctx *PrimaryExpressionContext) {}

// ExitPrimaryExpression is called when production primaryExpression is exited.
func (s *BaseWdlV1ParserListener) ExitPrimaryExpression(ctx *PrimaryExpressionContext) {}

// EnterVariable is called when production variable is entered.
func (s *BaseWdlV1ParserListener) EnterVariable(ctx *VariableContext) {}

// ExitVariable is called when production variable is exited.
func (s *BaseWdlV1ParserListener) ExitVariable(ctx *VariableContext) {}

// EnterNullLiteral is called when production nullLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterNullLiteral(ctx *NullLiteralContext) {}

// ExitNullLiteral is called when production nullLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitNullLiteral(ctx *NullLiteralContext) {}

// EnterNoneLiteral is called when production noneLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterNoneLiteral(ctx *NoneLiteralContext) {}

// ExitNoneLiteral is called when production noneLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitNoneLiteral(ctx *NoneLiteralContext) {}

// EnterBooleanLiteral is called when production booleanLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterBooleanLiteral(ctx *BooleanLiteralContext) {}

// ExitBooleanLiteral is called when production booleanLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitBooleanLiteral(ctx *BooleanLiteralContext) {}

// EnterNumberLiteralInt is called when production numberLiteralInt is entered.
func (s *BaseWdlV1ParserListener) EnterNumberLiteralInt(ctx *NumberLiteralIntContext) {}

// ExitNumberLiteralInt is called when production numberLiteralInt is exited.
func (s *BaseWdlV1ParserListener) ExitNumberLiteralInt(ctx *NumberLiteralIntContext) {}

// EnterNumberLiteralFloat is called when production numberLiteralFloat is entered.
func (s *BaseWdlV1ParserListener) EnterNumberLiteralFloat(ctx *NumberLiteralFloatContext) {}

// ExitNumberLiteralFloat is called when production numberLiteralFloat is exited.
func (s *BaseWdlV1ParserListener) ExitNumberLiteralFloat(ctx *NumberLiteralFloatContext) {}

// EnterNumberLiteralSigned is called when production numberLiteralSigned is entered.
func (s *BaseWdlV1ParserListener) EnterNumberLiteralSigned(ctx *NumberLiteralSignedContext) {}

// ExitNumberLiteralSigned is called when production numberLiteralSigned is exited.
func (s *BaseWdlV1ParserListener) ExitNumberLiteralSigned(ctx *NumberLiteralSignedContext) {}

// EnterArrayLiteral is called when production arrayLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterArrayLiteral(ctx *ArrayLiteralContext) {}

// ExitArrayLiteral is called when production arrayLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitArrayLiteral(ctx *ArrayLiteralContext) {}

// EnterMapLiteral is called when production mapLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterMapLiteral(ctx *MapLiteralContext) {}

// ExitMapLiteral is called when production mapLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitMapLiteral(ctx *MapLiteralContext) {}

// EnterMapLiteralItem is called when production mapLiteralItem is entered.
func (s *BaseWdlV1ParserListener) EnterMapLiteralItem(ctx *MapLiteralItemContext) {}

// ExitMapLiteralItem is called when production mapLiteralItem is exited.
func (s *BaseWdlV1ParserListener) ExitMapLiteralItem(ctx *MapLiteralItemContext) {}

// EnterObjectLiteral is called when production objectLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterObjectLiteral(ctx *ObjectLiteralContext) {}

// ExitObjectLiteral is called when production objectLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitObjectLiteral(ctx *ObjectLiteralContext) {}

// EnterObjectLiteralItem is called when production objectLiteralItem is entered.
func (s *BaseWdlV1ParserListener) EnterObjectLiteralItem(ctx *ObjectLiteralItemContext) {}

// ExitObjectLiteralItem is called when production objectLiteralItem is exited.
func (s *BaseWdlV1ParserListener) ExitObjectLiteralItem(ctx *ObjectLiteralItemContext) {}

// EnterStructLiteral is called when production structLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterStructLiteral(ctx *StructLiteralContext) {}

// ExitStructLiteral is called when production structLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitStructLiteral(ctx *StructLiteralContext) {}

// EnterStructLiteralItem is called when production structLiteralItem is entered.
func (s *BaseWdlV1ParserListener) EnterStructLiteralItem(ctx *StructLiteralItemContext) {}

// ExitStructLiteralItem is called when production structLiteralItem is exited.
func (s *BaseWdlV1ParserListener) ExitStructLiteralItem(ctx *StructLiteralItemContext) {}

// EnterPairLiteral is called when production pairLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterPairLiteral(ctx *PairLiteralContext) {}

// ExitPairLiteral is called when production pairLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitPairLiteral(ctx *PairLiteralContext) {}

// EnterGroupedExpression is called when production groupedExpression is entered.
func (s *BaseWdlV1ParserListener) EnterGroupedExpression(ctx *GroupedExpressionContext) {}

// ExitGroupedExpression is called when production groupedExpression is exited.
func (s *BaseWdlV1ParserListener) ExitGroupedExpression(ctx *GroupedExpressionContext) {}

// EnterIfExpression is called when production ifExpression is entered.
func (s *BaseWdlV1ParserListener) EnterIfExpression(ctx *IfExpressionContext) {}

// ExitIfExpression is called when production ifExpression is exited.
func (s *BaseWdlV1ParserListener) ExitIfExpression(ctx *IfExpressionContext) {}

// EnterCallExpression is called when production callExpression is entered.
func (s *BaseWdlV1ParserListener) EnterCallExpression(ctx *CallExpressionContext) {}

// ExitCallExpression is called when production callExpression is exited.
func (s *BaseWdlV1ParserListener) ExitCallExpression(ctx *CallExpressionContext) {}

// EnterStringLiteral is called when production stringLiteral is entered.
func (s *BaseWdlV1ParserListener) EnterStringLiteral(ctx *StringLiteralContext) {}

// ExitStringLiteral is called when production stringLiteral is exited.
func (s *BaseWdlV1ParserListener) ExitStringLiteral(ctx *StringLiteralContext) {}

// EnterQuotedString is called when production quotedString is entered.
func (s *BaseWdlV1ParserListener) EnterQuotedString(ctx *QuotedStringContext) {}

// ExitQuotedString is called when production quotedString is exited.
func (s *BaseWdlV1ParserListener) ExitQuotedString(ctx *QuotedStringContext) {}

// EnterStringElementText is called when production stringElementText is entered.
func (s *BaseWdlV1ParserListener) EnterStringElementText(ctx *StringElementTextContext) {}

// ExitStringElementText is called when production stringElementText is exited.
func (s *BaseWdlV1ParserListener) ExitStringElementText(ctx *StringElementTextContext) {}

// EnterStringElementEscape is called when production stringElementEscape is entered.
func (s *BaseWdlV1ParserListener) EnterStringElementEscape(ctx *StringElementEscapeContext) {}

// ExitStringElementEscape is called when production stringElementEscape is exited.
func (s *BaseWdlV1ParserListener) ExitStringElementEscape(ctx *StringElementEscapeContext) {}

// EnterStringElementDollarSign is called when production stringElementDollarSign is entered.
func (s *BaseWdlV1ParserListener) EnterStringElementDollarSign(ctx *StringElementDollarSignContext) {}

// ExitStringElementDollarSign is called when production stringElementDollarSign is exited.
func (s *BaseWdlV1ParserListener) ExitStringElementDollarSign(ctx *StringElementDollarSignContext) {}

// EnterStringElementTilde is called when production stringElementTilde is entered.
func (s *BaseWdlV1ParserListener) EnterStringElementTilde(ctx *StringElementTildeContext) {}

// ExitStringElementTilde is called when production stringElementTilde is exited.
func (s *BaseWdlV1ParserListener) ExitStringElementTilde(ctx *StringElementTildeContext) {}

// EnterStringElementPlaceholder is called when production stringElementPlaceholder is entered.
func (s *BaseWdlV1ParserListener) EnterStringElementPlaceholder(ctx *StringElementPlaceholderContext) {
}

// ExitStringElementPlaceholder is called when production stringElementPlaceholder is exited.
func (s *BaseWdlV1ParserListener) ExitStringElementPlaceholder(ctx *StringElementPlaceholderContext) {
}

// EnterStringPlaceholder is called when production stringPlaceholder is entered.
func (s *BaseWdlV1ParserListener) EnterStringPlaceholder(ctx *StringPlaceholderContext) {}

// ExitStringPlaceholder is called when production stringPlaceholder is exited.
func (s *BaseWdlV1ParserListener) ExitStringPlaceholder(ctx *StringPlaceholderContext) {}

// EnterMultilineString is called when production multilineString is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineString(ctx *MultilineStringContext) {}

// ExitMultilineString is called when production multilineString is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineString(ctx *MultilineStringContext) {}

// EnterMultilineStringElementText is called when production multilineStringElementText is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringElementText(ctx *MultilineStringElementTextContext) {
}

// ExitMultilineStringElementText is called when production multilineStringElementText is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringElementText(ctx *MultilineStringElementTextContext) {
}

// EnterMultilineStringElementEscape is called when production multilineStringElementEscape is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringElementEscape(ctx *MultilineStringElementEscapeContext) {
}

// ExitMultilineStringElementEscape is called when production multilineStringElementEscape is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringElementEscape(ctx *MultilineStringElementEscapeContext) {
}

// EnterMultilineStringElementDoubleCloseAngle is called when production multilineStringElementDoubleCloseAngle is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringElementDoubleCloseAngle(ctx *MultilineStringElementDoubleCloseAngleContext) {
}

// ExitMultilineStringElementDoubleCloseAngle is called when production multilineStringElementDoubleCloseAngle is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringElementDoubleCloseAngle(ctx *MultilineStringElementDoubleCloseAngleContext) {
}

// EnterMultilineStringElementSingleCloseAngle is called when production multilineStringElementSingleCloseAngle is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringElementSingleCloseAngle(ctx *MultilineStringElementSingleCloseAngleContext) {
}

// ExitMultilineStringElementSingleCloseAngle is called when production multilineStringElementSingleCloseAngle is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringElementSingleCloseAngle(ctx *MultilineStringElementSingleCloseAngleContext) {
}

// EnterMultilineStringElementDollarSign is called when production multilineStringElementDollarSign is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringElementDollarSign(ctx *MultilineStringElementDollarSignContext) {
}

// ExitMultilineStringElementDollarSign is called when production multilineStringElementDollarSign is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringElementDollarSign(ctx *MultilineStringElementDollarSignContext) {
}

// EnterMultilineStringElementTilde is called when production multilineStringElementTilde is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringElementTilde(ctx *MultilineStringElementTildeContext) {
}

// ExitMultilineStringElementTilde is called when production multilineStringElementTilde is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringElementTilde(ctx *MultilineStringElementTildeContext) {
}

// EnterMultilineStringElementPlaceholder is called when production multilineStringElementPlaceholder is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringElementPlaceholder(ctx *MultilineStringElementPlaceholderContext) {
}

// ExitMultilineStringElementPlaceholder is called when production multilineStringElementPlaceholder is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringElementPlaceholder(ctx *MultilineStringElementPlaceholderContext) {
}

// EnterMultilineStringPlaceholder is called when production multilineStringPlaceholder is entered.
func (s *BaseWdlV1ParserListener) EnterMultilineStringPlaceholder(ctx *MultilineStringPlaceholderContext) {
}

// ExitMultilineStringPlaceholder is called when production multilineStringPlaceholder is exited.
func (s *BaseWdlV1ParserListener) ExitMultilineStringPlaceholder(ctx *MultilineStringPlaceholderContext) {
}

// EnterStringPlaceholderExpression is called when production stringPlaceholderExpression is entered.
func (s *BaseWdlV1ParserListener) EnterStringPlaceholderExpression(ctx *StringPlaceholderExpressionContext) {
}

// ExitStringPlaceholderExpression is called when production stringPlaceholderExpression is exited.
func (s *BaseWdlV1ParserListener) ExitStringPlaceholderExpression(ctx *StringPlaceholderExpressionContext) {
}

// EnterStringPlaceholderOptionSepDefault is called when production stringPlaceholderOptionSepDefault is entered.
func (s *BaseWdlV1ParserListener) EnterStringPlaceholderOptionSepDefault(ctx *StringPlaceholderOptionSepDefaultContext) {
}

// ExitStringPlaceholderOptionSepDefault is called when production stringPlaceholderOptionSepDefault is exited.
func (s *BaseWdlV1ParserListener) ExitStringPlaceholderOptionSepDefault(ctx *StringPlaceholderOptionSepDefaultContext) {
}

// EnterStringPlaceholderOptionTrueFalse is called when production stringPlaceholderOptionTrueFalse is entered.
func (s *BaseWdlV1ParserListener) EnterStringPlaceholderOptionTrueFalse(ctx *StringPlaceholderOptionTrueFalseContext) {
}

// ExitStringPlaceholderOptionTrueFalse is called when production stringPlaceholderOptionTrueFalse is exited.
func (s *BaseWdlV1ParserListener) ExitStringPlaceholderOptionTrueFalse(ctx *StringPlaceholderOptionTrueFalseContext) {
}

// EnterStringPlaceholderOptionFalseTrue is called when production stringPlaceholderOptionFalseTrue is entered.
func (s *BaseWdlV1ParserListener) EnterStringPlaceholderOptionFalseTrue(ctx *StringPlaceholderOptionFalseTrueContext) {
}

// ExitStringPlaceholderOptionFalseTrue is called when production stringPlaceholderOptionFalseTrue is exited.
func (s *BaseWdlV1ParserListener) ExitStringPlaceholderOptionFalseTrue(ctx *StringPlaceholderOptionFalseTrueContext) {
}

// EnterStrictIdentifier is called when production strictIdentifier is entered.
func (s *BaseWdlV1ParserListener) EnterStrictIdentifier(ctx *StrictIdentifierContext) {}

// ExitStrictIdentifier is called when production strictIdentifier is exited.
func (s *BaseWdlV1ParserListener) ExitStrictIdentifier(ctx *StrictIdentifierContext) {}

// EnterDottedIdentifier is called when production dottedIdentifier is entered.
func (s *BaseWdlV1ParserListener) EnterDottedIdentifier(ctx *DottedIdentifierContext) {}

// ExitDottedIdentifier is called when production dottedIdentifier is exited.
func (s *BaseWdlV1ParserListener) ExitDottedIdentifier(ctx *DottedIdentifierContext) {}
