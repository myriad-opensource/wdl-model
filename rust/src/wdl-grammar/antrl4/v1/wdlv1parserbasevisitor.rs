
// Generated from ../wdl-grammar/antrl4/v1/WdlV1Parser.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::wdlv1parser::*;

// A complete Visitor for a parse tree produced by WdlV1Parser.

pub trait WdlV1ParserBaseVisitor<'input>:
    ParseTreeVisitor<'input, WdlV1ParserContextType> {
	// Visit a parse tree produced by WdlV1Parser#document.
	fn visit_document(&mut self, ctx: &DocumentContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#versionStatement.
	fn visit_versionstatement(&mut self, ctx: &VersionStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#documentElement.
	fn visit_documentelement(&mut self, ctx: &DocumentElementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importStatementStandard.
	fn visit_importstatementstandard(&mut self, ctx: &ImportStatementStandardContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importStatementStar.
	fn visit_importstatementstar(&mut self, ctx: &ImportStatementStarContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importStatementMembers.
	fn visit_importstatementmembers(&mut self, ctx: &ImportStatementMembersContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importMembers.
	fn visit_importmembers(&mut self, ctx: &ImportMembersContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importMember.
	fn visit_importmember(&mut self, ctx: &ImportMemberContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importUriLiteral.
	fn visit_importuriliteral(&mut self, ctx: &ImportUriLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importUriElement.
	fn visit_importurielement(&mut self, ctx: &ImportUriElementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#importAlias.
	fn visit_importalias(&mut self, ctx: &ImportAliasContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#structDefinition.
	fn visit_structdefinition(&mut self, ctx: &StructDefinitionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#structItemMetadata.
	fn visit_structitemmetadata(&mut self, ctx: &StructItemMetadataContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#structItemParameterMetadata.
	fn visit_structitemparametermetadata(&mut self, ctx: &StructItemParameterMetadataContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#structItemMemberDeclaration.
	fn visit_structitemmemberdeclaration(&mut self, ctx: &StructItemMemberDeclarationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#structDeclaration.
	fn visit_structdeclaration(&mut self, ctx: &StructDeclarationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumDefinition.
	fn visit_enumdefinition(&mut self, ctx: &EnumDefinitionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumTypeParameter.
	fn visit_enumtypeparameter(&mut self, ctx: &EnumTypeParameterContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumChoice.
	fn visit_enumchoice(&mut self, ctx: &EnumChoiceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumLiteralExpression.
	fn visit_enumliteralexpression(&mut self, ctx: &EnumLiteralExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumStringLiteral.
	fn visit_enumstringliteral(&mut self, ctx: &EnumStringLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumQuotedString.
	fn visit_enumquotedstring(&mut self, ctx: &EnumQuotedStringContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumStringElement.
	fn visit_enumstringelement(&mut self, ctx: &EnumStringElementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumMultilineString.
	fn visit_enummultilinestring(&mut self, ctx: &EnumMultilineStringContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumMultilineStringElement.
	fn visit_enummultilinestringelement(&mut self, ctx: &EnumMultilineStringElementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumArrayLiteral.
	fn visit_enumarrayliteral(&mut self, ctx: &EnumArrayLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumMapLiteral.
	fn visit_enummapliteral(&mut self, ctx: &EnumMapLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumMapLiteralItem.
	fn visit_enummapliteralitem(&mut self, ctx: &EnumMapLiteralItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumObjectLiteral.
	fn visit_enumobjectliteral(&mut self, ctx: &EnumObjectLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumObjectLiteralItem.
	fn visit_enumobjectliteralitem(&mut self, ctx: &EnumObjectLiteralItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumStructLiteral.
	fn visit_enumstructliteral(&mut self, ctx: &EnumStructLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumStructLiteralItem.
	fn visit_enumstructliteralitem(&mut self, ctx: &EnumStructLiteralItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#enumPairLiteral.
	fn visit_enumpairliteral(&mut self, ctx: &EnumPairLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskDefinition.
	fn visit_taskdefinition(&mut self, ctx: &TaskDefinitionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowDefinition.
	fn visit_workflowdefinition(&mut self, ctx: &WorkflowDefinitionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#type.
	fn visit_type(&mut self, ctx: &TypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#mapType.
	fn visit_maptype(&mut self, ctx: &MapTypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#arrayType.
	fn visit_arraytype(&mut self, ctx: &ArrayTypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#pairType.
	fn visit_pairtype(&mut self, ctx: &PairTypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#objectType.
	fn visit_objecttype(&mut self, ctx: &ObjectTypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#primitiveType.
	fn visit_primitivetype(&mut self, ctx: &PrimitiveTypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#typeRefType.
	fn visit_typereftype(&mut self, ctx: &TypeRefTypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#unboundDeclaration.
	fn visit_unbounddeclaration(&mut self, ctx: &UnboundDeclarationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#boundDeclaration.
	fn visit_bounddeclaration(&mut self, ctx: &BoundDeclarationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#declaration.
	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskInputSection.
	fn visit_taskinputsection(&mut self, ctx: &TaskInputSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskCommandSection.
	fn visit_taskcommandsection(&mut self, ctx: &TaskCommandSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskOutputSection.
	fn visit_taskoutputsection(&mut self, ctx: &TaskOutputSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskRuntimeSection.
	fn visit_taskruntimesection(&mut self, ctx: &TaskRuntimeSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskRequirementsSection.
	fn visit_taskrequirementssection(&mut self, ctx: &TaskRequirementsSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskHintsSection.
	fn visit_taskhintssection(&mut self, ctx: &TaskHintsSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskMetadataSection.
	fn visit_taskmetadatasection(&mut self, ctx: &TaskMetadataSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskParameterMetadataSection.
	fn visit_taskparametermetadatasection(&mut self, ctx: &TaskParameterMetadataSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskDeclaration.
	fn visit_taskdeclaration(&mut self, ctx: &TaskDeclarationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowInputSection.
	fn visit_workflowinputsection(&mut self, ctx: &WorkflowInputSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowOutputSection.
	fn visit_workflowoutputsection(&mut self, ctx: &WorkflowOutputSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowHintsSection.
	fn visit_workflowhintssection(&mut self, ctx: &WorkflowHintsSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowConditionalStatement.
	fn visit_workflowconditionalstatement(&mut self, ctx: &WorkflowConditionalStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowScatterStatement.
	fn visit_workflowscatterstatement(&mut self, ctx: &WorkflowScatterStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowCallStatement.
	fn visit_workflowcallstatement(&mut self, ctx: &WorkflowCallStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowMetadataSection.
	fn visit_workflowmetadatasection(&mut self, ctx: &WorkflowMetadataSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowParameterMetadataSection.
	fn visit_workflowparametermetadatasection(&mut self, ctx: &WorkflowParameterMetadataSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowDeclaration.
	fn visit_workflowdeclaration(&mut self, ctx: &WorkflowDeclarationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#inputSection.
	fn visit_inputsection(&mut self, ctx: &InputSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#outputSection.
	fn visit_outputsection(&mut self, ctx: &OutputSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#runtimeSection.
	fn visit_runtimesection(&mut self, ctx: &RuntimeSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#runtimeItem.
	fn visit_runtimeitem(&mut self, ctx: &RuntimeItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#requirementsSection.
	fn visit_requirementssection(&mut self, ctx: &RequirementsSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#requirementsItem.
	fn visit_requirementsitem(&mut self, ctx: &RequirementsItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsSectionTask.
	fn visit_hintssectiontask(&mut self, ctx: &HintsSectionTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsItemTask.
	fn visit_hintsitemtask(&mut self, ctx: &HintsItemTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskHintValueExpression.
	fn visit_taskhintvalueexpression(&mut self, ctx: &TaskHintValueExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskHintValueHintsObject.
	fn visit_taskhintvaluehintsobject(&mut self, ctx: &TaskHintValueHintsObjectContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskHintValueInputObject.
	fn visit_taskhintvalueinputobject(&mut self, ctx: &TaskHintValueInputObjectContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskHintValueOutputObject.
	fn visit_taskhintvalueoutputobject(&mut self, ctx: &TaskHintValueOutputObjectContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskHintValueArray.
	fn visit_taskhintvaluearray(&mut self, ctx: &TaskHintValueArrayContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsTypedObjectTask.
	fn visit_hintstypedobjecttask(&mut self, ctx: &HintsTypedObjectTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsObjectItemTask.
	fn visit_hintsobjectitemtask(&mut self, ctx: &HintsObjectItemTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#inputHintsObjectTask.
	fn visit_inputhintsobjecttask(&mut self, ctx: &InputHintsObjectTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#inputHintsItemTask.
	fn visit_inputhintsitemtask(&mut self, ctx: &InputHintsItemTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#outputHintsObjectTask.
	fn visit_outputhintsobjecttask(&mut self, ctx: &OutputHintsObjectTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#outputHintsItemTask.
	fn visit_outputhintsitemtask(&mut self, ctx: &OutputHintsItemTaskContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#taskHintsArray.
	fn visit_taskhintsarray(&mut self, ctx: &TaskHintsArrayContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsSectionWorkflow.
	fn visit_hintssectionworkflow(&mut self, ctx: &HintsSectionWorkflowContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsItemWorkflow.
	fn visit_hintsitemworkflow(&mut self, ctx: &HintsItemWorkflowContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueNumber.
	fn visit_workflowhintvaluenumber(&mut self, ctx: &WorkflowHintValueNumberContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueString.
	fn visit_workflowhintvaluestring(&mut self, ctx: &WorkflowHintValueStringContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueBoolean.
	fn visit_workflowhintvalueboolean(&mut self, ctx: &WorkflowHintValueBooleanContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueObject.
	fn visit_workflowhintvalueobject(&mut self, ctx: &WorkflowHintValueObjectContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowHintValueArray.
	fn visit_workflowhintvaluearray(&mut self, ctx: &WorkflowHintValueArrayContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsObjectWorkflow.
	fn visit_hintsobjectworkflow(&mut self, ctx: &HintsObjectWorkflowContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#hintsObjectItemWorkflow.
	fn visit_hintsobjectitemworkflow(&mut self, ctx: &HintsObjectItemWorkflowContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowHintsArray.
	fn visit_workflowhintsarray(&mut self, ctx: &WorkflowHintsArrayContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#metadataSection.
	fn visit_metadatasection(&mut self, ctx: &MetadataSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#parameterMetadataSection.
	fn visit_parametermetadatasection(&mut self, ctx: &ParameterMetadataSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#metadataObject.
	fn visit_metadataobject(&mut self, ctx: &MetadataObjectContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#metadataObjectItem.
	fn visit_metadataobjectitem(&mut self, ctx: &MetadataObjectItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#metadataArray.
	fn visit_metadataarray(&mut self, ctx: &MetadataArrayContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#metadataValue.
	fn visit_metadatavalue(&mut self, ctx: &MetadataValueContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#commandSection.
	fn visit_commandsection(&mut self, ctx: &CommandSectionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringCommand.
	fn visit_multilinestringcommand(&mut self, ctx: &MultilineStringCommandContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#bracedCommand.
	fn visit_bracedcommand(&mut self, ctx: &BracedCommandContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#workflowStatement.
	fn visit_workflowstatement(&mut self, ctx: &WorkflowStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#conditionalStatement.
	fn visit_conditionalstatement(&mut self, ctx: &ConditionalStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#conditionalElseIfClause.
	fn visit_conditionalelseifclause(&mut self, ctx: &ConditionalElseIfClauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#conditionalElseClause.
	fn visit_conditionalelseclause(&mut self, ctx: &ConditionalElseClauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#scatterStatement.
	fn visit_scatterstatement(&mut self, ctx: &ScatterStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#scatterBody.
	fn visit_scatterbody(&mut self, ctx: &ScatterBodyContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#callStatement.
	fn visit_callstatement(&mut self, ctx: &CallStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#callTarget.
	fn visit_calltarget(&mut self, ctx: &CallTargetContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#callAlias.
	fn visit_callalias(&mut self, ctx: &CallAliasContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#callAfterClause.
	fn visit_callafterclause(&mut self, ctx: &CallAfterClauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#callInputBlock.
	fn visit_callinputblock(&mut self, ctx: &CallInputBlockContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#callInputItem.
	fn visit_callinputitem(&mut self, ctx: &CallInputItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#expression.
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#logicalOrExprOperation.
	fn visit_logicalorexproperation(&mut self, ctx: &LogicalOrExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#logicalOrExprNone.
	fn visit_logicalorexprnone(&mut self, ctx: &LogicalOrExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#logicalAndExprOperation.
	fn visit_logicalandexproperation(&mut self, ctx: &LogicalAndExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#logicalAndExprNone.
	fn visit_logicalandexprnone(&mut self, ctx: &LogicalAndExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#equalityExprOperation.
	fn visit_equalityexproperation(&mut self, ctx: &EqualityExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#equalityExprNone.
	fn visit_equalityexprnone(&mut self, ctx: &EqualityExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#comparisonExprOperation.
	fn visit_comparisonexproperation(&mut self, ctx: &ComparisonExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#comparisonExprNone.
	fn visit_comparisonexprnone(&mut self, ctx: &ComparisonExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#additiveExprOperation.
	fn visit_additiveexproperation(&mut self, ctx: &AdditiveExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#additiveExprNone.
	fn visit_additiveexprnone(&mut self, ctx: &AdditiveExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multiplicativeExprOperation.
	fn visit_multiplicativeexproperation(&mut self, ctx: &MultiplicativeExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multiplicativeExprNone.
	fn visit_multiplicativeexprnone(&mut self, ctx: &MultiplicativeExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#powerExprOperation.
	fn visit_powerexproperation(&mut self, ctx: &PowerExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#powerExprNone.
	fn visit_powerexprnone(&mut self, ctx: &PowerExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#unaryExprOperation.
	fn visit_unaryexproperation(&mut self, ctx: &UnaryExprOperationContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#unaryExprNone.
	fn visit_unaryexprnone(&mut self, ctx: &UnaryExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#postfixExprField.
	fn visit_postfixexprfield(&mut self, ctx: &PostfixExprFieldContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#postfixExprArrayIndex.
	fn visit_postfixexprarrayindex(&mut self, ctx: &PostfixExprArrayIndexContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#postfixExprNone.
	fn visit_postfixexprnone(&mut self, ctx: &PostfixExprNoneContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#primaryExpression.
	fn visit_primaryexpression(&mut self, ctx: &PrimaryExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#variable.
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#nullLiteral.
	fn visit_nullliteral(&mut self, ctx: &NullLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#noneLiteral.
	fn visit_noneliteral(&mut self, ctx: &NoneLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#booleanLiteral.
	fn visit_booleanliteral(&mut self, ctx: &BooleanLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#numberLiteralInt.
	fn visit_numberliteralint(&mut self, ctx: &NumberLiteralIntContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#numberLiteralFloat.
	fn visit_numberliteralfloat(&mut self, ctx: &NumberLiteralFloatContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#numberLiteralSigned.
	fn visit_numberliteralsigned(&mut self, ctx: &NumberLiteralSignedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#arrayLiteral.
	fn visit_arrayliteral(&mut self, ctx: &ArrayLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#mapLiteral.
	fn visit_mapliteral(&mut self, ctx: &MapLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#mapLiteralItem.
	fn visit_mapliteralitem(&mut self, ctx: &MapLiteralItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#objectLiteral.
	fn visit_objectliteral(&mut self, ctx: &ObjectLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#objectLiteralItem.
	fn visit_objectliteralitem(&mut self, ctx: &ObjectLiteralItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#structLiteral.
	fn visit_structliteral(&mut self, ctx: &StructLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#structLiteralItem.
	fn visit_structliteralitem(&mut self, ctx: &StructLiteralItemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#pairLiteral.
	fn visit_pairliteral(&mut self, ctx: &PairLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#groupedExpression.
	fn visit_groupedexpression(&mut self, ctx: &GroupedExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#ifExpression.
	fn visit_ifexpression(&mut self, ctx: &IfExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#callExpression.
	fn visit_callexpression(&mut self, ctx: &CallExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringLiteral.
	fn visit_stringliteral(&mut self, ctx: &StringLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#quotedString.
	fn visit_quotedstring(&mut self, ctx: &QuotedStringContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringElementText.
	fn visit_stringelementtext(&mut self, ctx: &StringElementTextContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringElementEscape.
	fn visit_stringelementescape(&mut self, ctx: &StringElementEscapeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringElementDollarSign.
	fn visit_stringelementdollarsign(&mut self, ctx: &StringElementDollarSignContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringElementTilde.
	fn visit_stringelementtilde(&mut self, ctx: &StringElementTildeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringElementPlaceholder.
	fn visit_stringelementplaceholder(&mut self, ctx: &StringElementPlaceholderContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholder.
	fn visit_stringplaceholder(&mut self, ctx: &StringPlaceholderContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineString.
	fn visit_multilinestring(&mut self, ctx: &MultilineStringContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementText.
	fn visit_multilinestringelementtext(&mut self, ctx: &MultilineStringElementTextContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementEscape.
	fn visit_multilinestringelementescape(&mut self, ctx: &MultilineStringElementEscapeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementDoubleCloseAngle.
	fn visit_multilinestringelementdoublecloseangle(&mut self, ctx: &MultilineStringElementDoubleCloseAngleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementSingleCloseAngle.
	fn visit_multilinestringelementsinglecloseangle(&mut self, ctx: &MultilineStringElementSingleCloseAngleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementDollarSign.
	fn visit_multilinestringelementdollarsign(&mut self, ctx: &MultilineStringElementDollarSignContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementTilde.
	fn visit_multilinestringelementtilde(&mut self, ctx: &MultilineStringElementTildeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringElementPlaceholder.
	fn visit_multilinestringelementplaceholder(&mut self, ctx: &MultilineStringElementPlaceholderContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#multilineStringPlaceholder.
	fn visit_multilinestringplaceholder(&mut self, ctx: &MultilineStringPlaceholderContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderExpression.
	fn visit_stringplaceholderexpression(&mut self, ctx: &StringPlaceholderExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionSepDefault.
	fn visit_stringplaceholderoptionsepdefault(&mut self, ctx: &StringPlaceholderOptionSepDefaultContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionTrueFalse.
	fn visit_stringplaceholderoptiontruefalse(&mut self, ctx: &StringPlaceholderOptionTrueFalseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionFalseTrue.
	fn visit_stringplaceholderoptionfalsetrue(&mut self, ctx: &StringPlaceholderOptionFalseTrueContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#strictIdentifier.
	fn visit_strictidentifier(&mut self, ctx: &StrictIdentifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#dottedIdentifier.
	fn visit_dottedidentifier(&mut self, ctx: &DottedIdentifierContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by WdlV1Parser#anyIdentBase.
	fn visit_anyidentbase(&mut self, ctx: &AnyIdentBaseContext<'input>) {
            self.visit_children(ctx)
        }

}