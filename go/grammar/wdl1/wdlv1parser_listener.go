// Code generated from ./WdlV1Parser.g4 by ANTLR 4.13.1. DO NOT EDIT.

package wdl1 // WdlV1Parser
import "github.com/antlr4-go/antlr/v4"

// WdlV1ParserListener is a complete listener for a parse tree produced by WdlV1Parser.
type WdlV1ParserListener interface {
	antlr.ParseTreeListener

	// EnterDocument is called when entering the document production.
	EnterDocument(c *DocumentContext)

	// EnterVersionStatement is called when entering the versionStatement production.
	EnterVersionStatement(c *VersionStatementContext)

	// EnterDocumentElement is called when entering the documentElement production.
	EnterDocumentElement(c *DocumentElementContext)

	// EnterImportStatementStandard is called when entering the importStatementStandard production.
	EnterImportStatementStandard(c *ImportStatementStandardContext)

	// EnterImportStatementStar is called when entering the importStatementStar production.
	EnterImportStatementStar(c *ImportStatementStarContext)

	// EnterImportStatementMembers is called when entering the importStatementMembers production.
	EnterImportStatementMembers(c *ImportStatementMembersContext)

	// EnterImportMembers is called when entering the importMembers production.
	EnterImportMembers(c *ImportMembersContext)

	// EnterImportMember is called when entering the importMember production.
	EnterImportMember(c *ImportMemberContext)

	// EnterImportUriLiteral is called when entering the importUriLiteral production.
	EnterImportUriLiteral(c *ImportUriLiteralContext)

	// EnterImportUriElement is called when entering the importUriElement production.
	EnterImportUriElement(c *ImportUriElementContext)

	// EnterImportAlias is called when entering the importAlias production.
	EnterImportAlias(c *ImportAliasContext)

	// EnterStructDefinition is called when entering the structDefinition production.
	EnterStructDefinition(c *StructDefinitionContext)

	// EnterStructItemMetadata is called when entering the structItemMetadata production.
	EnterStructItemMetadata(c *StructItemMetadataContext)

	// EnterStructItemParameterMetadata is called when entering the structItemParameterMetadata production.
	EnterStructItemParameterMetadata(c *StructItemParameterMetadataContext)

	// EnterStructItemMemberDeclaration is called when entering the structItemMemberDeclaration production.
	EnterStructItemMemberDeclaration(c *StructItemMemberDeclarationContext)

	// EnterStructDeclaration is called when entering the structDeclaration production.
	EnterStructDeclaration(c *StructDeclarationContext)

	// EnterEnumDefinition is called when entering the enumDefinition production.
	EnterEnumDefinition(c *EnumDefinitionContext)

	// EnterEnumTypeParameter is called when entering the enumTypeParameter production.
	EnterEnumTypeParameter(c *EnumTypeParameterContext)

	// EnterEnumChoice is called when entering the enumChoice production.
	EnterEnumChoice(c *EnumChoiceContext)

	// EnterEnumLiteralExpression is called when entering the enumLiteralExpression production.
	EnterEnumLiteralExpression(c *EnumLiteralExpressionContext)

	// EnterEnumStringLiteral is called when entering the enumStringLiteral production.
	EnterEnumStringLiteral(c *EnumStringLiteralContext)

	// EnterEnumQuotedString is called when entering the enumQuotedString production.
	EnterEnumQuotedString(c *EnumQuotedStringContext)

	// EnterEnumStringElement is called when entering the enumStringElement production.
	EnterEnumStringElement(c *EnumStringElementContext)

	// EnterEnumMultilineString is called when entering the enumMultilineString production.
	EnterEnumMultilineString(c *EnumMultilineStringContext)

	// EnterEnumMultilineStringElement is called when entering the enumMultilineStringElement production.
	EnterEnumMultilineStringElement(c *EnumMultilineStringElementContext)

	// EnterEnumArrayLiteral is called when entering the enumArrayLiteral production.
	EnterEnumArrayLiteral(c *EnumArrayLiteralContext)

	// EnterEnumMapLiteral is called when entering the enumMapLiteral production.
	EnterEnumMapLiteral(c *EnumMapLiteralContext)

	// EnterEnumMapLiteralItem is called when entering the enumMapLiteralItem production.
	EnterEnumMapLiteralItem(c *EnumMapLiteralItemContext)

	// EnterEnumObjectLiteral is called when entering the enumObjectLiteral production.
	EnterEnumObjectLiteral(c *EnumObjectLiteralContext)

	// EnterEnumObjectLiteralItem is called when entering the enumObjectLiteralItem production.
	EnterEnumObjectLiteralItem(c *EnumObjectLiteralItemContext)

	// EnterEnumStructLiteral is called when entering the enumStructLiteral production.
	EnterEnumStructLiteral(c *EnumStructLiteralContext)

	// EnterEnumStructLiteralItem is called when entering the enumStructLiteralItem production.
	EnterEnumStructLiteralItem(c *EnumStructLiteralItemContext)

	// EnterEnumPairLiteral is called when entering the enumPairLiteral production.
	EnterEnumPairLiteral(c *EnumPairLiteralContext)

	// EnterTaskDefinition is called when entering the taskDefinition production.
	EnterTaskDefinition(c *TaskDefinitionContext)

	// EnterWorkflowDefinition is called when entering the workflowDefinition production.
	EnterWorkflowDefinition(c *WorkflowDefinitionContext)

	// EnterType is called when entering the type production.
	EnterType(c *TypeContext)

	// EnterMapType is called when entering the mapType production.
	EnterMapType(c *MapTypeContext)

	// EnterArrayType is called when entering the arrayType production.
	EnterArrayType(c *ArrayTypeContext)

	// EnterPairType is called when entering the pairType production.
	EnterPairType(c *PairTypeContext)

	// EnterObjectType is called when entering the objectType production.
	EnterObjectType(c *ObjectTypeContext)

	// EnterPrimitiveType is called when entering the primitiveType production.
	EnterPrimitiveType(c *PrimitiveTypeContext)

	// EnterTypeRefType is called when entering the typeRefType production.
	EnterTypeRefType(c *TypeRefTypeContext)

	// EnterUnboundDeclaration is called when entering the unboundDeclaration production.
	EnterUnboundDeclaration(c *UnboundDeclarationContext)

	// EnterBoundDeclaration is called when entering the boundDeclaration production.
	EnterBoundDeclaration(c *BoundDeclarationContext)

	// EnterDeclaration is called when entering the declaration production.
	EnterDeclaration(c *DeclarationContext)

	// EnterTaskInputSection is called when entering the taskInputSection production.
	EnterTaskInputSection(c *TaskInputSectionContext)

	// EnterTaskCommandSection is called when entering the taskCommandSection production.
	EnterTaskCommandSection(c *TaskCommandSectionContext)

	// EnterTaskOutputSection is called when entering the taskOutputSection production.
	EnterTaskOutputSection(c *TaskOutputSectionContext)

	// EnterTaskRuntimeSection is called when entering the taskRuntimeSection production.
	EnterTaskRuntimeSection(c *TaskRuntimeSectionContext)

	// EnterTaskRequirementsSection is called when entering the taskRequirementsSection production.
	EnterTaskRequirementsSection(c *TaskRequirementsSectionContext)

	// EnterTaskHintsSection is called when entering the taskHintsSection production.
	EnterTaskHintsSection(c *TaskHintsSectionContext)

	// EnterTaskMetadataSection is called when entering the taskMetadataSection production.
	EnterTaskMetadataSection(c *TaskMetadataSectionContext)

	// EnterTaskParameterMetadataSection is called when entering the taskParameterMetadataSection production.
	EnterTaskParameterMetadataSection(c *TaskParameterMetadataSectionContext)

	// EnterTaskDeclaration is called when entering the taskDeclaration production.
	EnterTaskDeclaration(c *TaskDeclarationContext)

	// EnterWorkflowInputSection is called when entering the workflowInputSection production.
	EnterWorkflowInputSection(c *WorkflowInputSectionContext)

	// EnterWorkflowOutputSection is called when entering the workflowOutputSection production.
	EnterWorkflowOutputSection(c *WorkflowOutputSectionContext)

	// EnterWorkflowHintsSection is called when entering the workflowHintsSection production.
	EnterWorkflowHintsSection(c *WorkflowHintsSectionContext)

	// EnterWorkflowConditionalStatement is called when entering the workflowConditionalStatement production.
	EnterWorkflowConditionalStatement(c *WorkflowConditionalStatementContext)

	// EnterWorkflowScatterStatement is called when entering the workflowScatterStatement production.
	EnterWorkflowScatterStatement(c *WorkflowScatterStatementContext)

	// EnterWorkflowCallStatement is called when entering the workflowCallStatement production.
	EnterWorkflowCallStatement(c *WorkflowCallStatementContext)

	// EnterWorkflowMetadataSection is called when entering the workflowMetadataSection production.
	EnterWorkflowMetadataSection(c *WorkflowMetadataSectionContext)

	// EnterWorkflowParameterMetadataSection is called when entering the workflowParameterMetadataSection production.
	EnterWorkflowParameterMetadataSection(c *WorkflowParameterMetadataSectionContext)

	// EnterWorkflowDeclaration is called when entering the workflowDeclaration production.
	EnterWorkflowDeclaration(c *WorkflowDeclarationContext)

	// EnterInputSection is called when entering the inputSection production.
	EnterInputSection(c *InputSectionContext)

	// EnterOutputSection is called when entering the outputSection production.
	EnterOutputSection(c *OutputSectionContext)

	// EnterRuntimeSection is called when entering the runtimeSection production.
	EnterRuntimeSection(c *RuntimeSectionContext)

	// EnterRuntimeItem is called when entering the runtimeItem production.
	EnterRuntimeItem(c *RuntimeItemContext)

	// EnterRequirementsSection is called when entering the requirementsSection production.
	EnterRequirementsSection(c *RequirementsSectionContext)

	// EnterRequirementsItem is called when entering the requirementsItem production.
	EnterRequirementsItem(c *RequirementsItemContext)

	// EnterHintsSectionTask is called when entering the hintsSectionTask production.
	EnterHintsSectionTask(c *HintsSectionTaskContext)

	// EnterHintsItemTask is called when entering the hintsItemTask production.
	EnterHintsItemTask(c *HintsItemTaskContext)

	// EnterTaskHintValueExpression is called when entering the taskHintValueExpression production.
	EnterTaskHintValueExpression(c *TaskHintValueExpressionContext)

	// EnterTaskHintValueHintsObject is called when entering the taskHintValueHintsObject production.
	EnterTaskHintValueHintsObject(c *TaskHintValueHintsObjectContext)

	// EnterTaskHintValueInputObject is called when entering the taskHintValueInputObject production.
	EnterTaskHintValueInputObject(c *TaskHintValueInputObjectContext)

	// EnterTaskHintValueOutputObject is called when entering the taskHintValueOutputObject production.
	EnterTaskHintValueOutputObject(c *TaskHintValueOutputObjectContext)

	// EnterTaskHintValueArray is called when entering the taskHintValueArray production.
	EnterTaskHintValueArray(c *TaskHintValueArrayContext)

	// EnterHintsTypedObjectTask is called when entering the hintsTypedObjectTask production.
	EnterHintsTypedObjectTask(c *HintsTypedObjectTaskContext)

	// EnterHintsObjectItemTask is called when entering the hintsObjectItemTask production.
	EnterHintsObjectItemTask(c *HintsObjectItemTaskContext)

	// EnterInputHintsObjectTask is called when entering the inputHintsObjectTask production.
	EnterInputHintsObjectTask(c *InputHintsObjectTaskContext)

	// EnterInputHintsItemTask is called when entering the inputHintsItemTask production.
	EnterInputHintsItemTask(c *InputHintsItemTaskContext)

	// EnterOutputHintsObjectTask is called when entering the outputHintsObjectTask production.
	EnterOutputHintsObjectTask(c *OutputHintsObjectTaskContext)

	// EnterOutputHintsItemTask is called when entering the outputHintsItemTask production.
	EnterOutputHintsItemTask(c *OutputHintsItemTaskContext)

	// EnterTaskHintsArray is called when entering the taskHintsArray production.
	EnterTaskHintsArray(c *TaskHintsArrayContext)

	// EnterHintsSectionWorkflow is called when entering the hintsSectionWorkflow production.
	EnterHintsSectionWorkflow(c *HintsSectionWorkflowContext)

	// EnterHintsItemWorkflow is called when entering the hintsItemWorkflow production.
	EnterHintsItemWorkflow(c *HintsItemWorkflowContext)

	// EnterWorkflowHintValueNumber is called when entering the workflowHintValueNumber production.
	EnterWorkflowHintValueNumber(c *WorkflowHintValueNumberContext)

	// EnterWorkflowHintValueString is called when entering the workflowHintValueString production.
	EnterWorkflowHintValueString(c *WorkflowHintValueStringContext)

	// EnterWorkflowHintValueBoolean is called when entering the workflowHintValueBoolean production.
	EnterWorkflowHintValueBoolean(c *WorkflowHintValueBooleanContext)

	// EnterWorkflowHintValueObject is called when entering the workflowHintValueObject production.
	EnterWorkflowHintValueObject(c *WorkflowHintValueObjectContext)

	// EnterWorkflowHintValueArray is called when entering the workflowHintValueArray production.
	EnterWorkflowHintValueArray(c *WorkflowHintValueArrayContext)

	// EnterHintsObjectWorkflow is called when entering the hintsObjectWorkflow production.
	EnterHintsObjectWorkflow(c *HintsObjectWorkflowContext)

	// EnterHintsObjectItemWorkflow is called when entering the hintsObjectItemWorkflow production.
	EnterHintsObjectItemWorkflow(c *HintsObjectItemWorkflowContext)

	// EnterWorkflowHintsArray is called when entering the workflowHintsArray production.
	EnterWorkflowHintsArray(c *WorkflowHintsArrayContext)

	// EnterMetadataSection is called when entering the metadataSection production.
	EnterMetadataSection(c *MetadataSectionContext)

	// EnterParameterMetadataSection is called when entering the parameterMetadataSection production.
	EnterParameterMetadataSection(c *ParameterMetadataSectionContext)

	// EnterMetadataObject is called when entering the metadataObject production.
	EnterMetadataObject(c *MetadataObjectContext)

	// EnterMetadataObjectItem is called when entering the metadataObjectItem production.
	EnterMetadataObjectItem(c *MetadataObjectItemContext)

	// EnterMetadataArray is called when entering the metadataArray production.
	EnterMetadataArray(c *MetadataArrayContext)

	// EnterMetadataValue is called when entering the metadataValue production.
	EnterMetadataValue(c *MetadataValueContext)

	// EnterCommandSection is called when entering the commandSection production.
	EnterCommandSection(c *CommandSectionContext)

	// EnterMultilineStringCommand is called when entering the multilineStringCommand production.
	EnterMultilineStringCommand(c *MultilineStringCommandContext)

	// EnterBracedCommand is called when entering the bracedCommand production.
	EnterBracedCommand(c *BracedCommandContext)

	// EnterWorkflowStatement is called when entering the workflowStatement production.
	EnterWorkflowStatement(c *WorkflowStatementContext)

	// EnterConditionalStatement is called when entering the conditionalStatement production.
	EnterConditionalStatement(c *ConditionalStatementContext)

	// EnterConditionalElseIfClause is called when entering the conditionalElseIfClause production.
	EnterConditionalElseIfClause(c *ConditionalElseIfClauseContext)

	// EnterConditionalElseClause is called when entering the conditionalElseClause production.
	EnterConditionalElseClause(c *ConditionalElseClauseContext)

	// EnterScatterStatement is called when entering the scatterStatement production.
	EnterScatterStatement(c *ScatterStatementContext)

	// EnterScatterBody is called when entering the scatterBody production.
	EnterScatterBody(c *ScatterBodyContext)

	// EnterCallStatement is called when entering the callStatement production.
	EnterCallStatement(c *CallStatementContext)

	// EnterCallTarget is called when entering the callTarget production.
	EnterCallTarget(c *CallTargetContext)

	// EnterCallAlias is called when entering the callAlias production.
	EnterCallAlias(c *CallAliasContext)

	// EnterCallAfterClause is called when entering the callAfterClause production.
	EnterCallAfterClause(c *CallAfterClauseContext)

	// EnterCallInputBlock is called when entering the callInputBlock production.
	EnterCallInputBlock(c *CallInputBlockContext)

	// EnterCallInputItem is called when entering the callInputItem production.
	EnterCallInputItem(c *CallInputItemContext)

	// EnterExpression is called when entering the expression production.
	EnterExpression(c *ExpressionContext)

	// EnterLogicalOrExprOperation is called when entering the logicalOrExprOperation production.
	EnterLogicalOrExprOperation(c *LogicalOrExprOperationContext)

	// EnterLogicalAndExprOperation is called when entering the logicalAndExprOperation production.
	EnterLogicalAndExprOperation(c *LogicalAndExprOperationContext)

	// EnterEqualityExprOperation is called when entering the equalityExprOperation production.
	EnterEqualityExprOperation(c *EqualityExprOperationContext)

	// EnterComparisonExprOperation is called when entering the comparisonExprOperation production.
	EnterComparisonExprOperation(c *ComparisonExprOperationContext)

	// EnterAdditiveExprOperation is called when entering the additiveExprOperation production.
	EnterAdditiveExprOperation(c *AdditiveExprOperationContext)

	// EnterMultiplicativeExprOperation is called when entering the multiplicativeExprOperation production.
	EnterMultiplicativeExprOperation(c *MultiplicativeExprOperationContext)

	// EnterPowerExprOperation is called when entering the powerExprOperation production.
	EnterPowerExprOperation(c *PowerExprOperationContext)

	// EnterPowerExprNone is called when entering the powerExprNone production.
	EnterPowerExprNone(c *PowerExprNoneContext)

	// EnterUnaryExprOperation is called when entering the unaryExprOperation production.
	EnterUnaryExprOperation(c *UnaryExprOperationContext)

	// EnterUnaryExprNone is called when entering the unaryExprNone production.
	EnterUnaryExprNone(c *UnaryExprNoneContext)

	// EnterPostfixExprField is called when entering the postfixExprField production.
	EnterPostfixExprField(c *PostfixExprFieldContext)

	// EnterPostfixExprArrayIndex is called when entering the postfixExprArrayIndex production.
	EnterPostfixExprArrayIndex(c *PostfixExprArrayIndexContext)

	// EnterPostfixExprNone is called when entering the postfixExprNone production.
	EnterPostfixExprNone(c *PostfixExprNoneContext)

	// EnterPrimaryExpression is called when entering the primaryExpression production.
	EnterPrimaryExpression(c *PrimaryExpressionContext)

	// EnterVariable is called when entering the variable production.
	EnterVariable(c *VariableContext)

	// EnterNullLiteral is called when entering the nullLiteral production.
	EnterNullLiteral(c *NullLiteralContext)

	// EnterNoneLiteral is called when entering the noneLiteral production.
	EnterNoneLiteral(c *NoneLiteralContext)

	// EnterBooleanLiteral is called when entering the booleanLiteral production.
	EnterBooleanLiteral(c *BooleanLiteralContext)

	// EnterNumberLiteralInt is called when entering the numberLiteralInt production.
	EnterNumberLiteralInt(c *NumberLiteralIntContext)

	// EnterNumberLiteralFloat is called when entering the numberLiteralFloat production.
	EnterNumberLiteralFloat(c *NumberLiteralFloatContext)

	// EnterNumberLiteralSigned is called when entering the numberLiteralSigned production.
	EnterNumberLiteralSigned(c *NumberLiteralSignedContext)

	// EnterArrayLiteral is called when entering the arrayLiteral production.
	EnterArrayLiteral(c *ArrayLiteralContext)

	// EnterMapLiteral is called when entering the mapLiteral production.
	EnterMapLiteral(c *MapLiteralContext)

	// EnterMapLiteralItem is called when entering the mapLiteralItem production.
	EnterMapLiteralItem(c *MapLiteralItemContext)

	// EnterObjectLiteral is called when entering the objectLiteral production.
	EnterObjectLiteral(c *ObjectLiteralContext)

	// EnterObjectLiteralItem is called when entering the objectLiteralItem production.
	EnterObjectLiteralItem(c *ObjectLiteralItemContext)

	// EnterStructLiteral is called when entering the structLiteral production.
	EnterStructLiteral(c *StructLiteralContext)

	// EnterStructLiteralItem is called when entering the structLiteralItem production.
	EnterStructLiteralItem(c *StructLiteralItemContext)

	// EnterPairLiteral is called when entering the pairLiteral production.
	EnterPairLiteral(c *PairLiteralContext)

	// EnterGroupedExpression is called when entering the groupedExpression production.
	EnterGroupedExpression(c *GroupedExpressionContext)

	// EnterIfExpression is called when entering the ifExpression production.
	EnterIfExpression(c *IfExpressionContext)

	// EnterCallExpression is called when entering the callExpression production.
	EnterCallExpression(c *CallExpressionContext)

	// EnterStringLiteral is called when entering the stringLiteral production.
	EnterStringLiteral(c *StringLiteralContext)

	// EnterQuotedString is called when entering the quotedString production.
	EnterQuotedString(c *QuotedStringContext)

	// EnterStringElementText is called when entering the stringElementText production.
	EnterStringElementText(c *StringElementTextContext)

	// EnterStringElementEscape is called when entering the stringElementEscape production.
	EnterStringElementEscape(c *StringElementEscapeContext)

	// EnterStringElementDollarSign is called when entering the stringElementDollarSign production.
	EnterStringElementDollarSign(c *StringElementDollarSignContext)

	// EnterStringElementTilde is called when entering the stringElementTilde production.
	EnterStringElementTilde(c *StringElementTildeContext)

	// EnterStringElementPlaceholder is called when entering the stringElementPlaceholder production.
	EnterStringElementPlaceholder(c *StringElementPlaceholderContext)

	// EnterStringPlaceholder is called when entering the stringPlaceholder production.
	EnterStringPlaceholder(c *StringPlaceholderContext)

	// EnterMultilineString is called when entering the multilineString production.
	EnterMultilineString(c *MultilineStringContext)

	// EnterMultilineStringElementText is called when entering the multilineStringElementText production.
	EnterMultilineStringElementText(c *MultilineStringElementTextContext)

	// EnterMultilineStringElementEscape is called when entering the multilineStringElementEscape production.
	EnterMultilineStringElementEscape(c *MultilineStringElementEscapeContext)

	// EnterMultilineStringElementDoubleCloseAngle is called when entering the multilineStringElementDoubleCloseAngle production.
	EnterMultilineStringElementDoubleCloseAngle(c *MultilineStringElementDoubleCloseAngleContext)

	// EnterMultilineStringElementSingleCloseAngle is called when entering the multilineStringElementSingleCloseAngle production.
	EnterMultilineStringElementSingleCloseAngle(c *MultilineStringElementSingleCloseAngleContext)

	// EnterMultilineStringElementDollarSign is called when entering the multilineStringElementDollarSign production.
	EnterMultilineStringElementDollarSign(c *MultilineStringElementDollarSignContext)

	// EnterMultilineStringElementTilde is called when entering the multilineStringElementTilde production.
	EnterMultilineStringElementTilde(c *MultilineStringElementTildeContext)

	// EnterMultilineStringElementPlaceholder is called when entering the multilineStringElementPlaceholder production.
	EnterMultilineStringElementPlaceholder(c *MultilineStringElementPlaceholderContext)

	// EnterMultilineStringPlaceholder is called when entering the multilineStringPlaceholder production.
	EnterMultilineStringPlaceholder(c *MultilineStringPlaceholderContext)

	// EnterStringPlaceholderExpression is called when entering the stringPlaceholderExpression production.
	EnterStringPlaceholderExpression(c *StringPlaceholderExpressionContext)

	// EnterStringPlaceholderOptionSepDefault is called when entering the stringPlaceholderOptionSepDefault production.
	EnterStringPlaceholderOptionSepDefault(c *StringPlaceholderOptionSepDefaultContext)

	// EnterStringPlaceholderOptionTrueFalse is called when entering the stringPlaceholderOptionTrueFalse production.
	EnterStringPlaceholderOptionTrueFalse(c *StringPlaceholderOptionTrueFalseContext)

	// EnterStringPlaceholderOptionFalseTrue is called when entering the stringPlaceholderOptionFalseTrue production.
	EnterStringPlaceholderOptionFalseTrue(c *StringPlaceholderOptionFalseTrueContext)

	// EnterStrictIdentifier is called when entering the strictIdentifier production.
	EnterStrictIdentifier(c *StrictIdentifierContext)

	// EnterDottedIdentifier is called when entering the dottedIdentifier production.
	EnterDottedIdentifier(c *DottedIdentifierContext)

	// ExitDocument is called when exiting the document production.
	ExitDocument(c *DocumentContext)

	// ExitVersionStatement is called when exiting the versionStatement production.
	ExitVersionStatement(c *VersionStatementContext)

	// ExitDocumentElement is called when exiting the documentElement production.
	ExitDocumentElement(c *DocumentElementContext)

	// ExitImportStatementStandard is called when exiting the importStatementStandard production.
	ExitImportStatementStandard(c *ImportStatementStandardContext)

	// ExitImportStatementStar is called when exiting the importStatementStar production.
	ExitImportStatementStar(c *ImportStatementStarContext)

	// ExitImportStatementMembers is called when exiting the importStatementMembers production.
	ExitImportStatementMembers(c *ImportStatementMembersContext)

	// ExitImportMembers is called when exiting the importMembers production.
	ExitImportMembers(c *ImportMembersContext)

	// ExitImportMember is called when exiting the importMember production.
	ExitImportMember(c *ImportMemberContext)

	// ExitImportUriLiteral is called when exiting the importUriLiteral production.
	ExitImportUriLiteral(c *ImportUriLiteralContext)

	// ExitImportUriElement is called when exiting the importUriElement production.
	ExitImportUriElement(c *ImportUriElementContext)

	// ExitImportAlias is called when exiting the importAlias production.
	ExitImportAlias(c *ImportAliasContext)

	// ExitStructDefinition is called when exiting the structDefinition production.
	ExitStructDefinition(c *StructDefinitionContext)

	// ExitStructItemMetadata is called when exiting the structItemMetadata production.
	ExitStructItemMetadata(c *StructItemMetadataContext)

	// ExitStructItemParameterMetadata is called when exiting the structItemParameterMetadata production.
	ExitStructItemParameterMetadata(c *StructItemParameterMetadataContext)

	// ExitStructItemMemberDeclaration is called when exiting the structItemMemberDeclaration production.
	ExitStructItemMemberDeclaration(c *StructItemMemberDeclarationContext)

	// ExitStructDeclaration is called when exiting the structDeclaration production.
	ExitStructDeclaration(c *StructDeclarationContext)

	// ExitEnumDefinition is called when exiting the enumDefinition production.
	ExitEnumDefinition(c *EnumDefinitionContext)

	// ExitEnumTypeParameter is called when exiting the enumTypeParameter production.
	ExitEnumTypeParameter(c *EnumTypeParameterContext)

	// ExitEnumChoice is called when exiting the enumChoice production.
	ExitEnumChoice(c *EnumChoiceContext)

	// ExitEnumLiteralExpression is called when exiting the enumLiteralExpression production.
	ExitEnumLiteralExpression(c *EnumLiteralExpressionContext)

	// ExitEnumStringLiteral is called when exiting the enumStringLiteral production.
	ExitEnumStringLiteral(c *EnumStringLiteralContext)

	// ExitEnumQuotedString is called when exiting the enumQuotedString production.
	ExitEnumQuotedString(c *EnumQuotedStringContext)

	// ExitEnumStringElement is called when exiting the enumStringElement production.
	ExitEnumStringElement(c *EnumStringElementContext)

	// ExitEnumMultilineString is called when exiting the enumMultilineString production.
	ExitEnumMultilineString(c *EnumMultilineStringContext)

	// ExitEnumMultilineStringElement is called when exiting the enumMultilineStringElement production.
	ExitEnumMultilineStringElement(c *EnumMultilineStringElementContext)

	// ExitEnumArrayLiteral is called when exiting the enumArrayLiteral production.
	ExitEnumArrayLiteral(c *EnumArrayLiteralContext)

	// ExitEnumMapLiteral is called when exiting the enumMapLiteral production.
	ExitEnumMapLiteral(c *EnumMapLiteralContext)

	// ExitEnumMapLiteralItem is called when exiting the enumMapLiteralItem production.
	ExitEnumMapLiteralItem(c *EnumMapLiteralItemContext)

	// ExitEnumObjectLiteral is called when exiting the enumObjectLiteral production.
	ExitEnumObjectLiteral(c *EnumObjectLiteralContext)

	// ExitEnumObjectLiteralItem is called when exiting the enumObjectLiteralItem production.
	ExitEnumObjectLiteralItem(c *EnumObjectLiteralItemContext)

	// ExitEnumStructLiteral is called when exiting the enumStructLiteral production.
	ExitEnumStructLiteral(c *EnumStructLiteralContext)

	// ExitEnumStructLiteralItem is called when exiting the enumStructLiteralItem production.
	ExitEnumStructLiteralItem(c *EnumStructLiteralItemContext)

	// ExitEnumPairLiteral is called when exiting the enumPairLiteral production.
	ExitEnumPairLiteral(c *EnumPairLiteralContext)

	// ExitTaskDefinition is called when exiting the taskDefinition production.
	ExitTaskDefinition(c *TaskDefinitionContext)

	// ExitWorkflowDefinition is called when exiting the workflowDefinition production.
	ExitWorkflowDefinition(c *WorkflowDefinitionContext)

	// ExitType is called when exiting the type production.
	ExitType(c *TypeContext)

	// ExitMapType is called when exiting the mapType production.
	ExitMapType(c *MapTypeContext)

	// ExitArrayType is called when exiting the arrayType production.
	ExitArrayType(c *ArrayTypeContext)

	// ExitPairType is called when exiting the pairType production.
	ExitPairType(c *PairTypeContext)

	// ExitObjectType is called when exiting the objectType production.
	ExitObjectType(c *ObjectTypeContext)

	// ExitPrimitiveType is called when exiting the primitiveType production.
	ExitPrimitiveType(c *PrimitiveTypeContext)

	// ExitTypeRefType is called when exiting the typeRefType production.
	ExitTypeRefType(c *TypeRefTypeContext)

	// ExitUnboundDeclaration is called when exiting the unboundDeclaration production.
	ExitUnboundDeclaration(c *UnboundDeclarationContext)

	// ExitBoundDeclaration is called when exiting the boundDeclaration production.
	ExitBoundDeclaration(c *BoundDeclarationContext)

	// ExitDeclaration is called when exiting the declaration production.
	ExitDeclaration(c *DeclarationContext)

	// ExitTaskInputSection is called when exiting the taskInputSection production.
	ExitTaskInputSection(c *TaskInputSectionContext)

	// ExitTaskCommandSection is called when exiting the taskCommandSection production.
	ExitTaskCommandSection(c *TaskCommandSectionContext)

	// ExitTaskOutputSection is called when exiting the taskOutputSection production.
	ExitTaskOutputSection(c *TaskOutputSectionContext)

	// ExitTaskRuntimeSection is called when exiting the taskRuntimeSection production.
	ExitTaskRuntimeSection(c *TaskRuntimeSectionContext)

	// ExitTaskRequirementsSection is called when exiting the taskRequirementsSection production.
	ExitTaskRequirementsSection(c *TaskRequirementsSectionContext)

	// ExitTaskHintsSection is called when exiting the taskHintsSection production.
	ExitTaskHintsSection(c *TaskHintsSectionContext)

	// ExitTaskMetadataSection is called when exiting the taskMetadataSection production.
	ExitTaskMetadataSection(c *TaskMetadataSectionContext)

	// ExitTaskParameterMetadataSection is called when exiting the taskParameterMetadataSection production.
	ExitTaskParameterMetadataSection(c *TaskParameterMetadataSectionContext)

	// ExitTaskDeclaration is called when exiting the taskDeclaration production.
	ExitTaskDeclaration(c *TaskDeclarationContext)

	// ExitWorkflowInputSection is called when exiting the workflowInputSection production.
	ExitWorkflowInputSection(c *WorkflowInputSectionContext)

	// ExitWorkflowOutputSection is called when exiting the workflowOutputSection production.
	ExitWorkflowOutputSection(c *WorkflowOutputSectionContext)

	// ExitWorkflowHintsSection is called when exiting the workflowHintsSection production.
	ExitWorkflowHintsSection(c *WorkflowHintsSectionContext)

	// ExitWorkflowConditionalStatement is called when exiting the workflowConditionalStatement production.
	ExitWorkflowConditionalStatement(c *WorkflowConditionalStatementContext)

	// ExitWorkflowScatterStatement is called when exiting the workflowScatterStatement production.
	ExitWorkflowScatterStatement(c *WorkflowScatterStatementContext)

	// ExitWorkflowCallStatement is called when exiting the workflowCallStatement production.
	ExitWorkflowCallStatement(c *WorkflowCallStatementContext)

	// ExitWorkflowMetadataSection is called when exiting the workflowMetadataSection production.
	ExitWorkflowMetadataSection(c *WorkflowMetadataSectionContext)

	// ExitWorkflowParameterMetadataSection is called when exiting the workflowParameterMetadataSection production.
	ExitWorkflowParameterMetadataSection(c *WorkflowParameterMetadataSectionContext)

	// ExitWorkflowDeclaration is called when exiting the workflowDeclaration production.
	ExitWorkflowDeclaration(c *WorkflowDeclarationContext)

	// ExitInputSection is called when exiting the inputSection production.
	ExitInputSection(c *InputSectionContext)

	// ExitOutputSection is called when exiting the outputSection production.
	ExitOutputSection(c *OutputSectionContext)

	// ExitRuntimeSection is called when exiting the runtimeSection production.
	ExitRuntimeSection(c *RuntimeSectionContext)

	// ExitRuntimeItem is called when exiting the runtimeItem production.
	ExitRuntimeItem(c *RuntimeItemContext)

	// ExitRequirementsSection is called when exiting the requirementsSection production.
	ExitRequirementsSection(c *RequirementsSectionContext)

	// ExitRequirementsItem is called when exiting the requirementsItem production.
	ExitRequirementsItem(c *RequirementsItemContext)

	// ExitHintsSectionTask is called when exiting the hintsSectionTask production.
	ExitHintsSectionTask(c *HintsSectionTaskContext)

	// ExitHintsItemTask is called when exiting the hintsItemTask production.
	ExitHintsItemTask(c *HintsItemTaskContext)

	// ExitTaskHintValueExpression is called when exiting the taskHintValueExpression production.
	ExitTaskHintValueExpression(c *TaskHintValueExpressionContext)

	// ExitTaskHintValueHintsObject is called when exiting the taskHintValueHintsObject production.
	ExitTaskHintValueHintsObject(c *TaskHintValueHintsObjectContext)

	// ExitTaskHintValueInputObject is called when exiting the taskHintValueInputObject production.
	ExitTaskHintValueInputObject(c *TaskHintValueInputObjectContext)

	// ExitTaskHintValueOutputObject is called when exiting the taskHintValueOutputObject production.
	ExitTaskHintValueOutputObject(c *TaskHintValueOutputObjectContext)

	// ExitTaskHintValueArray is called when exiting the taskHintValueArray production.
	ExitTaskHintValueArray(c *TaskHintValueArrayContext)

	// ExitHintsTypedObjectTask is called when exiting the hintsTypedObjectTask production.
	ExitHintsTypedObjectTask(c *HintsTypedObjectTaskContext)

	// ExitHintsObjectItemTask is called when exiting the hintsObjectItemTask production.
	ExitHintsObjectItemTask(c *HintsObjectItemTaskContext)

	// ExitInputHintsObjectTask is called when exiting the inputHintsObjectTask production.
	ExitInputHintsObjectTask(c *InputHintsObjectTaskContext)

	// ExitInputHintsItemTask is called when exiting the inputHintsItemTask production.
	ExitInputHintsItemTask(c *InputHintsItemTaskContext)

	// ExitOutputHintsObjectTask is called when exiting the outputHintsObjectTask production.
	ExitOutputHintsObjectTask(c *OutputHintsObjectTaskContext)

	// ExitOutputHintsItemTask is called when exiting the outputHintsItemTask production.
	ExitOutputHintsItemTask(c *OutputHintsItemTaskContext)

	// ExitTaskHintsArray is called when exiting the taskHintsArray production.
	ExitTaskHintsArray(c *TaskHintsArrayContext)

	// ExitHintsSectionWorkflow is called when exiting the hintsSectionWorkflow production.
	ExitHintsSectionWorkflow(c *HintsSectionWorkflowContext)

	// ExitHintsItemWorkflow is called when exiting the hintsItemWorkflow production.
	ExitHintsItemWorkflow(c *HintsItemWorkflowContext)

	// ExitWorkflowHintValueNumber is called when exiting the workflowHintValueNumber production.
	ExitWorkflowHintValueNumber(c *WorkflowHintValueNumberContext)

	// ExitWorkflowHintValueString is called when exiting the workflowHintValueString production.
	ExitWorkflowHintValueString(c *WorkflowHintValueStringContext)

	// ExitWorkflowHintValueBoolean is called when exiting the workflowHintValueBoolean production.
	ExitWorkflowHintValueBoolean(c *WorkflowHintValueBooleanContext)

	// ExitWorkflowHintValueObject is called when exiting the workflowHintValueObject production.
	ExitWorkflowHintValueObject(c *WorkflowHintValueObjectContext)

	// ExitWorkflowHintValueArray is called when exiting the workflowHintValueArray production.
	ExitWorkflowHintValueArray(c *WorkflowHintValueArrayContext)

	// ExitHintsObjectWorkflow is called when exiting the hintsObjectWorkflow production.
	ExitHintsObjectWorkflow(c *HintsObjectWorkflowContext)

	// ExitHintsObjectItemWorkflow is called when exiting the hintsObjectItemWorkflow production.
	ExitHintsObjectItemWorkflow(c *HintsObjectItemWorkflowContext)

	// ExitWorkflowHintsArray is called when exiting the workflowHintsArray production.
	ExitWorkflowHintsArray(c *WorkflowHintsArrayContext)

	// ExitMetadataSection is called when exiting the metadataSection production.
	ExitMetadataSection(c *MetadataSectionContext)

	// ExitParameterMetadataSection is called when exiting the parameterMetadataSection production.
	ExitParameterMetadataSection(c *ParameterMetadataSectionContext)

	// ExitMetadataObject is called when exiting the metadataObject production.
	ExitMetadataObject(c *MetadataObjectContext)

	// ExitMetadataObjectItem is called when exiting the metadataObjectItem production.
	ExitMetadataObjectItem(c *MetadataObjectItemContext)

	// ExitMetadataArray is called when exiting the metadataArray production.
	ExitMetadataArray(c *MetadataArrayContext)

	// ExitMetadataValue is called when exiting the metadataValue production.
	ExitMetadataValue(c *MetadataValueContext)

	// ExitCommandSection is called when exiting the commandSection production.
	ExitCommandSection(c *CommandSectionContext)

	// ExitMultilineStringCommand is called when exiting the multilineStringCommand production.
	ExitMultilineStringCommand(c *MultilineStringCommandContext)

	// ExitBracedCommand is called when exiting the bracedCommand production.
	ExitBracedCommand(c *BracedCommandContext)

	// ExitWorkflowStatement is called when exiting the workflowStatement production.
	ExitWorkflowStatement(c *WorkflowStatementContext)

	// ExitConditionalStatement is called when exiting the conditionalStatement production.
	ExitConditionalStatement(c *ConditionalStatementContext)

	// ExitConditionalElseIfClause is called when exiting the conditionalElseIfClause production.
	ExitConditionalElseIfClause(c *ConditionalElseIfClauseContext)

	// ExitConditionalElseClause is called when exiting the conditionalElseClause production.
	ExitConditionalElseClause(c *ConditionalElseClauseContext)

	// ExitScatterStatement is called when exiting the scatterStatement production.
	ExitScatterStatement(c *ScatterStatementContext)

	// ExitScatterBody is called when exiting the scatterBody production.
	ExitScatterBody(c *ScatterBodyContext)

	// ExitCallStatement is called when exiting the callStatement production.
	ExitCallStatement(c *CallStatementContext)

	// ExitCallTarget is called when exiting the callTarget production.
	ExitCallTarget(c *CallTargetContext)

	// ExitCallAlias is called when exiting the callAlias production.
	ExitCallAlias(c *CallAliasContext)

	// ExitCallAfterClause is called when exiting the callAfterClause production.
	ExitCallAfterClause(c *CallAfterClauseContext)

	// ExitCallInputBlock is called when exiting the callInputBlock production.
	ExitCallInputBlock(c *CallInputBlockContext)

	// ExitCallInputItem is called when exiting the callInputItem production.
	ExitCallInputItem(c *CallInputItemContext)

	// ExitExpression is called when exiting the expression production.
	ExitExpression(c *ExpressionContext)

	// ExitLogicalOrExprOperation is called when exiting the logicalOrExprOperation production.
	ExitLogicalOrExprOperation(c *LogicalOrExprOperationContext)

	// ExitLogicalAndExprOperation is called when exiting the logicalAndExprOperation production.
	ExitLogicalAndExprOperation(c *LogicalAndExprOperationContext)

	// ExitEqualityExprOperation is called when exiting the equalityExprOperation production.
	ExitEqualityExprOperation(c *EqualityExprOperationContext)

	// ExitComparisonExprOperation is called when exiting the comparisonExprOperation production.
	ExitComparisonExprOperation(c *ComparisonExprOperationContext)

	// ExitAdditiveExprOperation is called when exiting the additiveExprOperation production.
	ExitAdditiveExprOperation(c *AdditiveExprOperationContext)

	// ExitMultiplicativeExprOperation is called when exiting the multiplicativeExprOperation production.
	ExitMultiplicativeExprOperation(c *MultiplicativeExprOperationContext)

	// ExitPowerExprOperation is called when exiting the powerExprOperation production.
	ExitPowerExprOperation(c *PowerExprOperationContext)

	// ExitPowerExprNone is called when exiting the powerExprNone production.
	ExitPowerExprNone(c *PowerExprNoneContext)

	// ExitUnaryExprOperation is called when exiting the unaryExprOperation production.
	ExitUnaryExprOperation(c *UnaryExprOperationContext)

	// ExitUnaryExprNone is called when exiting the unaryExprNone production.
	ExitUnaryExprNone(c *UnaryExprNoneContext)

	// ExitPostfixExprField is called when exiting the postfixExprField production.
	ExitPostfixExprField(c *PostfixExprFieldContext)

	// ExitPostfixExprArrayIndex is called when exiting the postfixExprArrayIndex production.
	ExitPostfixExprArrayIndex(c *PostfixExprArrayIndexContext)

	// ExitPostfixExprNone is called when exiting the postfixExprNone production.
	ExitPostfixExprNone(c *PostfixExprNoneContext)

	// ExitPrimaryExpression is called when exiting the primaryExpression production.
	ExitPrimaryExpression(c *PrimaryExpressionContext)

	// ExitVariable is called when exiting the variable production.
	ExitVariable(c *VariableContext)

	// ExitNullLiteral is called when exiting the nullLiteral production.
	ExitNullLiteral(c *NullLiteralContext)

	// ExitNoneLiteral is called when exiting the noneLiteral production.
	ExitNoneLiteral(c *NoneLiteralContext)

	// ExitBooleanLiteral is called when exiting the booleanLiteral production.
	ExitBooleanLiteral(c *BooleanLiteralContext)

	// ExitNumberLiteralInt is called when exiting the numberLiteralInt production.
	ExitNumberLiteralInt(c *NumberLiteralIntContext)

	// ExitNumberLiteralFloat is called when exiting the numberLiteralFloat production.
	ExitNumberLiteralFloat(c *NumberLiteralFloatContext)

	// ExitNumberLiteralSigned is called when exiting the numberLiteralSigned production.
	ExitNumberLiteralSigned(c *NumberLiteralSignedContext)

	// ExitArrayLiteral is called when exiting the arrayLiteral production.
	ExitArrayLiteral(c *ArrayLiteralContext)

	// ExitMapLiteral is called when exiting the mapLiteral production.
	ExitMapLiteral(c *MapLiteralContext)

	// ExitMapLiteralItem is called when exiting the mapLiteralItem production.
	ExitMapLiteralItem(c *MapLiteralItemContext)

	// ExitObjectLiteral is called when exiting the objectLiteral production.
	ExitObjectLiteral(c *ObjectLiteralContext)

	// ExitObjectLiteralItem is called when exiting the objectLiteralItem production.
	ExitObjectLiteralItem(c *ObjectLiteralItemContext)

	// ExitStructLiteral is called when exiting the structLiteral production.
	ExitStructLiteral(c *StructLiteralContext)

	// ExitStructLiteralItem is called when exiting the structLiteralItem production.
	ExitStructLiteralItem(c *StructLiteralItemContext)

	// ExitPairLiteral is called when exiting the pairLiteral production.
	ExitPairLiteral(c *PairLiteralContext)

	// ExitGroupedExpression is called when exiting the groupedExpression production.
	ExitGroupedExpression(c *GroupedExpressionContext)

	// ExitIfExpression is called when exiting the ifExpression production.
	ExitIfExpression(c *IfExpressionContext)

	// ExitCallExpression is called when exiting the callExpression production.
	ExitCallExpression(c *CallExpressionContext)

	// ExitStringLiteral is called when exiting the stringLiteral production.
	ExitStringLiteral(c *StringLiteralContext)

	// ExitQuotedString is called when exiting the quotedString production.
	ExitQuotedString(c *QuotedStringContext)

	// ExitStringElementText is called when exiting the stringElementText production.
	ExitStringElementText(c *StringElementTextContext)

	// ExitStringElementEscape is called when exiting the stringElementEscape production.
	ExitStringElementEscape(c *StringElementEscapeContext)

	// ExitStringElementDollarSign is called when exiting the stringElementDollarSign production.
	ExitStringElementDollarSign(c *StringElementDollarSignContext)

	// ExitStringElementTilde is called when exiting the stringElementTilde production.
	ExitStringElementTilde(c *StringElementTildeContext)

	// ExitStringElementPlaceholder is called when exiting the stringElementPlaceholder production.
	ExitStringElementPlaceholder(c *StringElementPlaceholderContext)

	// ExitStringPlaceholder is called when exiting the stringPlaceholder production.
	ExitStringPlaceholder(c *StringPlaceholderContext)

	// ExitMultilineString is called when exiting the multilineString production.
	ExitMultilineString(c *MultilineStringContext)

	// ExitMultilineStringElementText is called when exiting the multilineStringElementText production.
	ExitMultilineStringElementText(c *MultilineStringElementTextContext)

	// ExitMultilineStringElementEscape is called when exiting the multilineStringElementEscape production.
	ExitMultilineStringElementEscape(c *MultilineStringElementEscapeContext)

	// ExitMultilineStringElementDoubleCloseAngle is called when exiting the multilineStringElementDoubleCloseAngle production.
	ExitMultilineStringElementDoubleCloseAngle(c *MultilineStringElementDoubleCloseAngleContext)

	// ExitMultilineStringElementSingleCloseAngle is called when exiting the multilineStringElementSingleCloseAngle production.
	ExitMultilineStringElementSingleCloseAngle(c *MultilineStringElementSingleCloseAngleContext)

	// ExitMultilineStringElementDollarSign is called when exiting the multilineStringElementDollarSign production.
	ExitMultilineStringElementDollarSign(c *MultilineStringElementDollarSignContext)

	// ExitMultilineStringElementTilde is called when exiting the multilineStringElementTilde production.
	ExitMultilineStringElementTilde(c *MultilineStringElementTildeContext)

	// ExitMultilineStringElementPlaceholder is called when exiting the multilineStringElementPlaceholder production.
	ExitMultilineStringElementPlaceholder(c *MultilineStringElementPlaceholderContext)

	// ExitMultilineStringPlaceholder is called when exiting the multilineStringPlaceholder production.
	ExitMultilineStringPlaceholder(c *MultilineStringPlaceholderContext)

	// ExitStringPlaceholderExpression is called when exiting the stringPlaceholderExpression production.
	ExitStringPlaceholderExpression(c *StringPlaceholderExpressionContext)

	// ExitStringPlaceholderOptionSepDefault is called when exiting the stringPlaceholderOptionSepDefault production.
	ExitStringPlaceholderOptionSepDefault(c *StringPlaceholderOptionSepDefaultContext)

	// ExitStringPlaceholderOptionTrueFalse is called when exiting the stringPlaceholderOptionTrueFalse production.
	ExitStringPlaceholderOptionTrueFalse(c *StringPlaceholderOptionTrueFalseContext)

	// ExitStringPlaceholderOptionFalseTrue is called when exiting the stringPlaceholderOptionFalseTrue production.
	ExitStringPlaceholderOptionFalseTrue(c *StringPlaceholderOptionFalseTrueContext)

	// ExitStrictIdentifier is called when exiting the strictIdentifier production.
	ExitStrictIdentifier(c *StrictIdentifierContext)

	// ExitDottedIdentifier is called when exiting the dottedIdentifier production.
	ExitDottedIdentifier(c *DottedIdentifierContext)
}
