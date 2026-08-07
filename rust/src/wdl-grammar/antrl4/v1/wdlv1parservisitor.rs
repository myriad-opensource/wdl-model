#![allow(nonstandard_style)]
// Generated from ../wdl-grammar/antrl4/v1/WdlV1Parser.g4 by ANTLR 4.13.2
use antlr4rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::wdlv1parser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link WdlV1Parser}.
 */
pub trait WdlV1ParserVisitor<'input>: ParseTreeVisitor<'input,WdlV1ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#document}.
	 * @param ctx the parse tree
	 */
	fn visit_document(&mut self, ctx: &DocumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#versionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_versionStatement(&mut self, ctx: &VersionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#documentElement}.
	 * @param ctx the parse tree
	 */
	fn visit_documentElement(&mut self, ctx: &DocumentElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code importStatementStandard}
	 * labeled alternative in {@link WdlV1Parser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatementStandard(&mut self, ctx: &ImportStatementStandardContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code importStatementStar}
	 * labeled alternative in {@link WdlV1Parser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatementStar(&mut self, ctx: &ImportStatementStarContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code importStatementMembers}
	 * labeled alternative in {@link WdlV1Parser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatementMembers(&mut self, ctx: &ImportStatementMembersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importMembers}.
	 * @param ctx the parse tree
	 */
	fn visit_importMembers(&mut self, ctx: &ImportMembersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importMember}.
	 * @param ctx the parse tree
	 */
	fn visit_importMember(&mut self, ctx: &ImportMemberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importUriLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_importUriLiteral(&mut self, ctx: &ImportUriLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importUriElement}.
	 * @param ctx the parse tree
	 */
	fn visit_importUriElement(&mut self, ctx: &ImportUriElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importAlias}.
	 * @param ctx the parse tree
	 */
	fn visit_importAlias(&mut self, ctx: &ImportAliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_structDefinition(&mut self, ctx: &StructDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code structItemMetadata}
	 * labeled alternative in {@link WdlV1Parser#structItem}.
	 * @param ctx the parse tree
	 */
	fn visit_structItemMetadata(&mut self, ctx: &StructItemMetadataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code structItemParameterMetadata}
	 * labeled alternative in {@link WdlV1Parser#structItem}.
	 * @param ctx the parse tree
	 */
	fn visit_structItemParameterMetadata(&mut self, ctx: &StructItemParameterMetadataContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code structItemMemberDeclaration}
	 * labeled alternative in {@link WdlV1Parser#structItem}.
	 * @param ctx the parse tree
	 */
	fn visit_structItemMemberDeclaration(&mut self, ctx: &StructItemMemberDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_structDeclaration(&mut self, ctx: &StructDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumTypeParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_enumTypeParameter(&mut self, ctx: &EnumTypeParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumChoice}.
	 * @param ctx the parse tree
	 */
	fn visit_enumChoice(&mut self, ctx: &EnumChoiceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumLiteralExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_enumLiteralExpression(&mut self, ctx: &EnumLiteralExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStringLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_enumStringLiteral(&mut self, ctx: &EnumStringLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumQuotedString}.
	 * @param ctx the parse tree
	 */
	fn visit_enumQuotedString(&mut self, ctx: &EnumQuotedStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_enumStringElement(&mut self, ctx: &EnumStringElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMultilineString}.
	 * @param ctx the parse tree
	 */
	fn visit_enumMultilineString(&mut self, ctx: &EnumMultilineStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMultilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_enumMultilineStringElement(&mut self, ctx: &EnumMultilineStringElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumArrayLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_enumArrayLiteral(&mut self, ctx: &EnumArrayLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMapLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_enumMapLiteral(&mut self, ctx: &EnumMapLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMapLiteralItem}.
	 * @param ctx the parse tree
	 */
	fn visit_enumMapLiteralItem(&mut self, ctx: &EnumMapLiteralItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumObjectLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_enumObjectLiteral(&mut self, ctx: &EnumObjectLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumObjectLiteralItem}.
	 * @param ctx the parse tree
	 */
	fn visit_enumObjectLiteralItem(&mut self, ctx: &EnumObjectLiteralItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStructLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_enumStructLiteral(&mut self, ctx: &EnumStructLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStructLiteralItem}.
	 * @param ctx the parse tree
	 */
	fn visit_enumStructLiteralItem(&mut self, ctx: &EnumStructLiteralItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumPairLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_enumPairLiteral(&mut self, ctx: &EnumPairLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#taskDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_taskDefinition(&mut self, ctx: &TaskDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#workflowDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowDefinition(&mut self, ctx: &WorkflowDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#type}.
	 * @param ctx the parse tree
	 */
	fn visit_type(&mut self, ctx: &TypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#mapType}.
	 * @param ctx the parse tree
	 */
	fn visit_mapType(&mut self, ctx: &MapTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#arrayType}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#pairType}.
	 * @param ctx the parse tree
	 */
	fn visit_pairType(&mut self, ctx: &PairTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#objectType}.
	 * @param ctx the parse tree
	 */
	fn visit_objectType(&mut self, ctx: &ObjectTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#primitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#typeRefType}.
	 * @param ctx the parse tree
	 */
	fn visit_typeRefType(&mut self, ctx: &TypeRefTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#unboundDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_unboundDeclaration(&mut self, ctx: &UnboundDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#boundDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_boundDeclaration(&mut self, ctx: &BoundDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#declaration}.
	 * @param ctx the parse tree
	 */
	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskInputSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskInputSection(&mut self, ctx: &TaskInputSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskCommandSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskCommandSection(&mut self, ctx: &TaskCommandSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskOutputSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskOutputSection(&mut self, ctx: &TaskOutputSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskRuntimeSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskRuntimeSection(&mut self, ctx: &TaskRuntimeSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskRequirementsSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskRequirementsSection(&mut self, ctx: &TaskRequirementsSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskHintsSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskHintsSection(&mut self, ctx: &TaskHintsSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskMetadataSection(&mut self, ctx: &TaskMetadataSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskParameterMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskParameterMetadataSection(&mut self, ctx: &TaskParameterMetadataSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskDeclaration}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
	fn visit_taskDeclaration(&mut self, ctx: &TaskDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowInputSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowInputSection(&mut self, ctx: &WorkflowInputSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowOutputSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowOutputSection(&mut self, ctx: &WorkflowOutputSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowHintsSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowHintsSection(&mut self, ctx: &WorkflowHintsSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowConditionalStatement}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowConditionalStatement(&mut self, ctx: &WorkflowConditionalStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowScatterStatement}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowScatterStatement(&mut self, ctx: &WorkflowScatterStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowCallStatement}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowCallStatement(&mut self, ctx: &WorkflowCallStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowMetadataSection(&mut self, ctx: &WorkflowMetadataSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowParameterMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowParameterMetadataSection(&mut self, ctx: &WorkflowParameterMetadataSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowDeclaration}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowDeclaration(&mut self, ctx: &WorkflowDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#inputSection}.
	 * @param ctx the parse tree
	 */
	fn visit_inputSection(&mut self, ctx: &InputSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#outputSection}.
	 * @param ctx the parse tree
	 */
	fn visit_outputSection(&mut self, ctx: &OutputSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#runtimeSection}.
	 * @param ctx the parse tree
	 */
	fn visit_runtimeSection(&mut self, ctx: &RuntimeSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#runtimeItem}.
	 * @param ctx the parse tree
	 */
	fn visit_runtimeItem(&mut self, ctx: &RuntimeItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#requirementsSection}.
	 * @param ctx the parse tree
	 */
	fn visit_requirementsSection(&mut self, ctx: &RequirementsSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#requirementsItem}.
	 * @param ctx the parse tree
	 */
	fn visit_requirementsItem(&mut self, ctx: &RequirementsItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsSectionTask}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsSectionTask(&mut self, ctx: &HintsSectionTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsItemTask}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsItemTask(&mut self, ctx: &HintsItemTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskHintValueExpression}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
	fn visit_taskHintValueExpression(&mut self, ctx: &TaskHintValueExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskHintValueHintsObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
	fn visit_taskHintValueHintsObject(&mut self, ctx: &TaskHintValueHintsObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskHintValueInputObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
	fn visit_taskHintValueInputObject(&mut self, ctx: &TaskHintValueInputObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskHintValueOutputObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
	fn visit_taskHintValueOutputObject(&mut self, ctx: &TaskHintValueOutputObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code taskHintValueArray}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
	fn visit_taskHintValueArray(&mut self, ctx: &TaskHintValueArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsTypedObjectTask}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsTypedObjectTask(&mut self, ctx: &HintsTypedObjectTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsObjectItemTask}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsObjectItemTask(&mut self, ctx: &HintsObjectItemTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#inputHintsObjectTask}.
	 * @param ctx the parse tree
	 */
	fn visit_inputHintsObjectTask(&mut self, ctx: &InputHintsObjectTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#inputHintsItemTask}.
	 * @param ctx the parse tree
	 */
	fn visit_inputHintsItemTask(&mut self, ctx: &InputHintsItemTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#outputHintsObjectTask}.
	 * @param ctx the parse tree
	 */
	fn visit_outputHintsObjectTask(&mut self, ctx: &OutputHintsObjectTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#outputHintsItemTask}.
	 * @param ctx the parse tree
	 */
	fn visit_outputHintsItemTask(&mut self, ctx: &OutputHintsItemTaskContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#taskHintsArray}.
	 * @param ctx the parse tree
	 */
	fn visit_taskHintsArray(&mut self, ctx: &TaskHintsArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsSectionWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsSectionWorkflow(&mut self, ctx: &HintsSectionWorkflowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsItemWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsItemWorkflow(&mut self, ctx: &HintsItemWorkflowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueNumber}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowHintValueNumber(&mut self, ctx: &WorkflowHintValueNumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueString}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowHintValueString(&mut self, ctx: &WorkflowHintValueStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueBoolean}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowHintValueBoolean(&mut self, ctx: &WorkflowHintValueBooleanContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowHintValueObject(&mut self, ctx: &WorkflowHintValueObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueArray}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowHintValueArray(&mut self, ctx: &WorkflowHintValueArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsObjectWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsObjectWorkflow(&mut self, ctx: &HintsObjectWorkflowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsObjectItemWorkflow}.
	 * @param ctx the parse tree
	 */
	fn visit_hintsObjectItemWorkflow(&mut self, ctx: &HintsObjectItemWorkflowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#workflowHintsArray}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowHintsArray(&mut self, ctx: &WorkflowHintsArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataSection}.
	 * @param ctx the parse tree
	 */
	fn visit_metadataSection(&mut self, ctx: &MetadataSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#parameterMetadataSection}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterMetadataSection(&mut self, ctx: &ParameterMetadataSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataObject}.
	 * @param ctx the parse tree
	 */
	fn visit_metadataObject(&mut self, ctx: &MetadataObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataObjectItem}.
	 * @param ctx the parse tree
	 */
	fn visit_metadataObjectItem(&mut self, ctx: &MetadataObjectItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataArray}.
	 * @param ctx the parse tree
	 */
	fn visit_metadataArray(&mut self, ctx: &MetadataArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataValue}.
	 * @param ctx the parse tree
	 */
	fn visit_metadataValue(&mut self, ctx: &MetadataValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#commandSection}.
	 * @param ctx the parse tree
	 */
	fn visit_commandSection(&mut self, ctx: &CommandSectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#multilineStringCommand}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringCommand(&mut self, ctx: &MultilineStringCommandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#bracedCommand}.
	 * @param ctx the parse tree
	 */
	fn visit_bracedCommand(&mut self, ctx: &BracedCommandContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#workflowStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_workflowStatement(&mut self, ctx: &WorkflowStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#conditionalStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalStatement(&mut self, ctx: &ConditionalStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#conditionalElseIfClause}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalElseIfClause(&mut self, ctx: &ConditionalElseIfClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#conditionalElseClause}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalElseClause(&mut self, ctx: &ConditionalElseClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#scatterStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_scatterStatement(&mut self, ctx: &ScatterStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#scatterBody}.
	 * @param ctx the parse tree
	 */
	fn visit_scatterBody(&mut self, ctx: &ScatterBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callTarget}.
	 * @param ctx the parse tree
	 */
	fn visit_callTarget(&mut self, ctx: &CallTargetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callAlias}.
	 * @param ctx the parse tree
	 */
	fn visit_callAlias(&mut self, ctx: &CallAliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callAfterClause}.
	 * @param ctx the parse tree
	 */
	fn visit_callAfterClause(&mut self, ctx: &CallAfterClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callInputBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_callInputBlock(&mut self, ctx: &CallInputBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callInputItem}.
	 * @param ctx the parse tree
	 */
	fn visit_callInputItem(&mut self, ctx: &CallInputItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalOrExprOperation}
	 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOrExprOperation(&mut self, ctx: &LogicalOrExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalOrExprNone}
	 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOrExprNone(&mut self, ctx: &LogicalOrExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalAndExprOperation}
	 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalAndExprOperation(&mut self, ctx: &LogicalAndExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code logicalAndExprNone}
	 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalAndExprNone(&mut self, ctx: &LogicalAndExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code equalityExprOperation}
	 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExprOperation(&mut self, ctx: &EqualityExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code equalityExprNone}
	 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExprNone(&mut self, ctx: &EqualityExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code comparisonExprOperation}
	 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonExprOperation(&mut self, ctx: &ComparisonExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code comparisonExprNone}
	 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_comparisonExprNone(&mut self, ctx: &ComparisonExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code additiveExprOperation}
	 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExprOperation(&mut self, ctx: &AdditiveExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code additiveExprNone}
	 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExprNone(&mut self, ctx: &AdditiveExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExprOperation}
	 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExprOperation(&mut self, ctx: &MultiplicativeExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExprNone}
	 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExprNone(&mut self, ctx: &MultiplicativeExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code powerExprOperation}
	 * labeled alternative in {@link WdlV1Parser#powerExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_powerExprOperation(&mut self, ctx: &PowerExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code powerExprNone}
	 * labeled alternative in {@link WdlV1Parser#powerExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_powerExprNone(&mut self, ctx: &PowerExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unaryExprOperation}
	 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExprOperation(&mut self, ctx: &UnaryExprOperationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unaryExprNone}
	 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExprNone(&mut self, ctx: &UnaryExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code postfixExprField}
	 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExprField(&mut self, ctx: &PostfixExprFieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code postfixExprArrayIndex}
	 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExprArrayIndex(&mut self, ctx: &PostfixExprArrayIndexContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code postfixExprNone}
	 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExprNone(&mut self, ctx: &PostfixExprNoneContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#nullLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#noneLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_noneLiteral(&mut self, ctx: &NoneLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numberLiteralInt}
	 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteralInt(&mut self, ctx: &NumberLiteralIntContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numberLiteralFloat}
	 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteralFloat(&mut self, ctx: &NumberLiteralFloatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#numberLiteralSigned}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteralSigned(&mut self, ctx: &NumberLiteralSignedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#arrayLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#mapLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_mapLiteral(&mut self, ctx: &MapLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#mapLiteralItem}.
	 * @param ctx the parse tree
	 */
	fn visit_mapLiteralItem(&mut self, ctx: &MapLiteralItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#objectLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#objectLiteralItem}.
	 * @param ctx the parse tree
	 */
	fn visit_objectLiteralItem(&mut self, ctx: &ObjectLiteralItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_structLiteral(&mut self, ctx: &StructLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structLiteralItem}.
	 * @param ctx the parse tree
	 */
	fn visit_structLiteralItem(&mut self, ctx: &StructLiteralItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#pairLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_pairLiteral(&mut self, ctx: &PairLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#groupedExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_groupedExpression(&mut self, ctx: &GroupedExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#ifExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_ifExpression(&mut self, ctx: &IfExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#stringLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#quotedString}.
	 * @param ctx the parse tree
	 */
	fn visit_quotedString(&mut self, ctx: &QuotedStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringElementText}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_stringElementText(&mut self, ctx: &StringElementTextContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringElementEscape}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_stringElementEscape(&mut self, ctx: &StringElementEscapeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringElementDollarSign}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_stringElementDollarSign(&mut self, ctx: &StringElementDollarSignContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringElementTilde}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_stringElementTilde(&mut self, ctx: &StringElementTildeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringElementPlaceholder}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_stringElementPlaceholder(&mut self, ctx: &StringElementPlaceholderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#stringPlaceholder}.
	 * @param ctx the parse tree
	 */
	fn visit_stringPlaceholder(&mut self, ctx: &StringPlaceholderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#multilineString}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineString(&mut self, ctx: &MultilineStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementText}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringElementText(&mut self, ctx: &MultilineStringElementTextContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementEscape}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringElementEscape(&mut self, ctx: &MultilineStringElementEscapeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementDoubleCloseAngle}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringElementDoubleCloseAngle(&mut self, ctx: &MultilineStringElementDoubleCloseAngleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementSingleCloseAngle}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringElementSingleCloseAngle(&mut self, ctx: &MultilineStringElementSingleCloseAngleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementDollarSign}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringElementDollarSign(&mut self, ctx: &MultilineStringElementDollarSignContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementTilde}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringElementTilde(&mut self, ctx: &MultilineStringElementTildeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementPlaceholder}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringElementPlaceholder(&mut self, ctx: &MultilineStringElementPlaceholderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#multilineStringPlaceholder}.
	 * @param ctx the parse tree
	 */
	fn visit_multilineStringPlaceholder(&mut self, ctx: &MultilineStringPlaceholderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#stringPlaceholderExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_stringPlaceholderExpression(&mut self, ctx: &StringPlaceholderExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringPlaceholderOptionSepDefault}
	 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
	 * @param ctx the parse tree
	 */
	fn visit_stringPlaceholderOptionSepDefault(&mut self, ctx: &StringPlaceholderOptionSepDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringPlaceholderOptionTrueFalse}
	 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
	 * @param ctx the parse tree
	 */
	fn visit_stringPlaceholderOptionTrueFalse(&mut self, ctx: &StringPlaceholderOptionTrueFalseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringPlaceholderOptionFalseTrue}
	 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
	 * @param ctx the parse tree
	 */
	fn visit_stringPlaceholderOptionFalseTrue(&mut self, ctx: &StringPlaceholderOptionFalseTrueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_strictIdentifier(&mut self, ctx: &StrictIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#dottedIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_dottedIdentifier(&mut self, ctx: &DottedIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#anyIdentBase}.
	 * @param ctx the parse tree
	 */
	fn visit_anyIdentBase(&mut self, ctx: &AnyIdentBaseContext<'input>) { self.visit_children(ctx) }

}

pub trait WdlV1ParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= WdlV1ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#document}.
	 * @param ctx the parse tree
	 */
		fn visit_document(&mut self, ctx: &DocumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#versionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_versionStatement(&mut self, ctx: &VersionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#documentElement}.
	 * @param ctx the parse tree
	 */
		fn visit_documentElement(&mut self, ctx: &DocumentElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code importStatementStandard}
	 * labeled alternative in {@link WdlV1Parser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatementStandard(&mut self, ctx: &ImportStatementStandardContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code importStatementStar}
	 * labeled alternative in {@link WdlV1Parser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatementStar(&mut self, ctx: &ImportStatementStarContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code importStatementMembers}
	 * labeled alternative in {@link WdlV1Parser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatementMembers(&mut self, ctx: &ImportStatementMembersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importMembers}.
	 * @param ctx the parse tree
	 */
		fn visit_importMembers(&mut self, ctx: &ImportMembersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importMember}.
	 * @param ctx the parse tree
	 */
		fn visit_importMember(&mut self, ctx: &ImportMemberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importUriLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_importUriLiteral(&mut self, ctx: &ImportUriLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importUriElement}.
	 * @param ctx the parse tree
	 */
		fn visit_importUriElement(&mut self, ctx: &ImportUriElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#importAlias}.
	 * @param ctx the parse tree
	 */
		fn visit_importAlias(&mut self, ctx: &ImportAliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_structDefinition(&mut self, ctx: &StructDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code structItemMetadata}
	 * labeled alternative in {@link WdlV1Parser#structItem}.
	 * @param ctx the parse tree
	 */
		fn visit_structItemMetadata(&mut self, ctx: &StructItemMetadataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code structItemParameterMetadata}
	 * labeled alternative in {@link WdlV1Parser#structItem}.
	 * @param ctx the parse tree
	 */
		fn visit_structItemParameterMetadata(&mut self, ctx: &StructItemParameterMetadataContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code structItemMemberDeclaration}
	 * labeled alternative in {@link WdlV1Parser#structItem}.
	 * @param ctx the parse tree
	 */
		fn visit_structItemMemberDeclaration(&mut self, ctx: &StructItemMemberDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_structDeclaration(&mut self, ctx: &StructDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumTypeParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_enumTypeParameter(&mut self, ctx: &EnumTypeParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumChoice}.
	 * @param ctx the parse tree
	 */
		fn visit_enumChoice(&mut self, ctx: &EnumChoiceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumLiteralExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_enumLiteralExpression(&mut self, ctx: &EnumLiteralExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStringLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_enumStringLiteral(&mut self, ctx: &EnumStringLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumQuotedString}.
	 * @param ctx the parse tree
	 */
		fn visit_enumQuotedString(&mut self, ctx: &EnumQuotedStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_enumStringElement(&mut self, ctx: &EnumStringElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMultilineString}.
	 * @param ctx the parse tree
	 */
		fn visit_enumMultilineString(&mut self, ctx: &EnumMultilineStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMultilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_enumMultilineStringElement(&mut self, ctx: &EnumMultilineStringElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumArrayLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_enumArrayLiteral(&mut self, ctx: &EnumArrayLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMapLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_enumMapLiteral(&mut self, ctx: &EnumMapLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumMapLiteralItem}.
	 * @param ctx the parse tree
	 */
		fn visit_enumMapLiteralItem(&mut self, ctx: &EnumMapLiteralItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumObjectLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_enumObjectLiteral(&mut self, ctx: &EnumObjectLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumObjectLiteralItem}.
	 * @param ctx the parse tree
	 */
		fn visit_enumObjectLiteralItem(&mut self, ctx: &EnumObjectLiteralItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStructLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_enumStructLiteral(&mut self, ctx: &EnumStructLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumStructLiteralItem}.
	 * @param ctx the parse tree
	 */
		fn visit_enumStructLiteralItem(&mut self, ctx: &EnumStructLiteralItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#enumPairLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_enumPairLiteral(&mut self, ctx: &EnumPairLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#taskDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_taskDefinition(&mut self, ctx: &TaskDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#workflowDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowDefinition(&mut self, ctx: &WorkflowDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#type}.
	 * @param ctx the parse tree
	 */
		fn visit_type(&mut self, ctx: &TypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#mapType}.
	 * @param ctx the parse tree
	 */
		fn visit_mapType(&mut self, ctx: &MapTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#arrayType}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#pairType}.
	 * @param ctx the parse tree
	 */
		fn visit_pairType(&mut self, ctx: &PairTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#objectType}.
	 * @param ctx the parse tree
	 */
		fn visit_objectType(&mut self, ctx: &ObjectTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#primitiveType}.
	 * @param ctx the parse tree
	 */
		fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#typeRefType}.
	 * @param ctx the parse tree
	 */
		fn visit_typeRefType(&mut self, ctx: &TypeRefTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#unboundDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_unboundDeclaration(&mut self, ctx: &UnboundDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#boundDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_boundDeclaration(&mut self, ctx: &BoundDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#declaration}.
	 * @param ctx the parse tree
	 */
		fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskInputSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskInputSection(&mut self, ctx: &TaskInputSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskCommandSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskCommandSection(&mut self, ctx: &TaskCommandSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskOutputSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskOutputSection(&mut self, ctx: &TaskOutputSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskRuntimeSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskRuntimeSection(&mut self, ctx: &TaskRuntimeSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskRequirementsSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskRequirementsSection(&mut self, ctx: &TaskRequirementsSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskHintsSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskHintsSection(&mut self, ctx: &TaskHintsSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskMetadataSection(&mut self, ctx: &TaskMetadataSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskParameterMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskParameterMetadataSection(&mut self, ctx: &TaskParameterMetadataSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskDeclaration}
	 * labeled alternative in {@link WdlV1Parser#taskElement}.
	 * @param ctx the parse tree
	 */
		fn visit_taskDeclaration(&mut self, ctx: &TaskDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowInputSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowInputSection(&mut self, ctx: &WorkflowInputSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowOutputSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowOutputSection(&mut self, ctx: &WorkflowOutputSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowHintsSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowHintsSection(&mut self, ctx: &WorkflowHintsSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowConditionalStatement}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowConditionalStatement(&mut self, ctx: &WorkflowConditionalStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowScatterStatement}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowScatterStatement(&mut self, ctx: &WorkflowScatterStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowCallStatement}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowCallStatement(&mut self, ctx: &WorkflowCallStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowMetadataSection(&mut self, ctx: &WorkflowMetadataSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowParameterMetadataSection}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowParameterMetadataSection(&mut self, ctx: &WorkflowParameterMetadataSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowDeclaration}
	 * labeled alternative in {@link WdlV1Parser#workflowElement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowDeclaration(&mut self, ctx: &WorkflowDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#inputSection}.
	 * @param ctx the parse tree
	 */
		fn visit_inputSection(&mut self, ctx: &InputSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#outputSection}.
	 * @param ctx the parse tree
	 */
		fn visit_outputSection(&mut self, ctx: &OutputSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#runtimeSection}.
	 * @param ctx the parse tree
	 */
		fn visit_runtimeSection(&mut self, ctx: &RuntimeSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#runtimeItem}.
	 * @param ctx the parse tree
	 */
		fn visit_runtimeItem(&mut self, ctx: &RuntimeItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#requirementsSection}.
	 * @param ctx the parse tree
	 */
		fn visit_requirementsSection(&mut self, ctx: &RequirementsSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#requirementsItem}.
	 * @param ctx the parse tree
	 */
		fn visit_requirementsItem(&mut self, ctx: &RequirementsItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsSectionTask}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsSectionTask(&mut self, ctx: &HintsSectionTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsItemTask}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsItemTask(&mut self, ctx: &HintsItemTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskHintValueExpression}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
		fn visit_taskHintValueExpression(&mut self, ctx: &TaskHintValueExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskHintValueHintsObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
		fn visit_taskHintValueHintsObject(&mut self, ctx: &TaskHintValueHintsObjectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskHintValueInputObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
		fn visit_taskHintValueInputObject(&mut self, ctx: &TaskHintValueInputObjectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskHintValueOutputObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
		fn visit_taskHintValueOutputObject(&mut self, ctx: &TaskHintValueOutputObjectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code taskHintValueArray}
	 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
	 * @param ctx the parse tree
	 */
		fn visit_taskHintValueArray(&mut self, ctx: &TaskHintValueArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsTypedObjectTask}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsTypedObjectTask(&mut self, ctx: &HintsTypedObjectTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsObjectItemTask}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsObjectItemTask(&mut self, ctx: &HintsObjectItemTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#inputHintsObjectTask}.
	 * @param ctx the parse tree
	 */
		fn visit_inputHintsObjectTask(&mut self, ctx: &InputHintsObjectTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#inputHintsItemTask}.
	 * @param ctx the parse tree
	 */
		fn visit_inputHintsItemTask(&mut self, ctx: &InputHintsItemTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#outputHintsObjectTask}.
	 * @param ctx the parse tree
	 */
		fn visit_outputHintsObjectTask(&mut self, ctx: &OutputHintsObjectTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#outputHintsItemTask}.
	 * @param ctx the parse tree
	 */
		fn visit_outputHintsItemTask(&mut self, ctx: &OutputHintsItemTaskContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#taskHintsArray}.
	 * @param ctx the parse tree
	 */
		fn visit_taskHintsArray(&mut self, ctx: &TaskHintsArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsSectionWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsSectionWorkflow(&mut self, ctx: &HintsSectionWorkflowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsItemWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsItemWorkflow(&mut self, ctx: &HintsItemWorkflowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueNumber}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowHintValueNumber(&mut self, ctx: &WorkflowHintValueNumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueString}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowHintValueString(&mut self, ctx: &WorkflowHintValueStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueBoolean}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowHintValueBoolean(&mut self, ctx: &WorkflowHintValueBooleanContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueObject}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowHintValueObject(&mut self, ctx: &WorkflowHintValueObjectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code workflowHintValueArray}
	 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowHintValueArray(&mut self, ctx: &WorkflowHintValueArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsObjectWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsObjectWorkflow(&mut self, ctx: &HintsObjectWorkflowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#hintsObjectItemWorkflow}.
	 * @param ctx the parse tree
	 */
		fn visit_hintsObjectItemWorkflow(&mut self, ctx: &HintsObjectItemWorkflowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#workflowHintsArray}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowHintsArray(&mut self, ctx: &WorkflowHintsArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataSection}.
	 * @param ctx the parse tree
	 */
		fn visit_metadataSection(&mut self, ctx: &MetadataSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#parameterMetadataSection}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterMetadataSection(&mut self, ctx: &ParameterMetadataSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataObject}.
	 * @param ctx the parse tree
	 */
		fn visit_metadataObject(&mut self, ctx: &MetadataObjectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataObjectItem}.
	 * @param ctx the parse tree
	 */
		fn visit_metadataObjectItem(&mut self, ctx: &MetadataObjectItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataArray}.
	 * @param ctx the parse tree
	 */
		fn visit_metadataArray(&mut self, ctx: &MetadataArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#metadataValue}.
	 * @param ctx the parse tree
	 */
		fn visit_metadataValue(&mut self, ctx: &MetadataValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#commandSection}.
	 * @param ctx the parse tree
	 */
		fn visit_commandSection(&mut self, ctx: &CommandSectionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#multilineStringCommand}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringCommand(&mut self, ctx: &MultilineStringCommandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#bracedCommand}.
	 * @param ctx the parse tree
	 */
		fn visit_bracedCommand(&mut self, ctx: &BracedCommandContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#workflowStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_workflowStatement(&mut self, ctx: &WorkflowStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#conditionalStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalStatement(&mut self, ctx: &ConditionalStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#conditionalElseIfClause}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalElseIfClause(&mut self, ctx: &ConditionalElseIfClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#conditionalElseClause}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalElseClause(&mut self, ctx: &ConditionalElseClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#scatterStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_scatterStatement(&mut self, ctx: &ScatterStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#scatterBody}.
	 * @param ctx the parse tree
	 */
		fn visit_scatterBody(&mut self, ctx: &ScatterBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callTarget}.
	 * @param ctx the parse tree
	 */
		fn visit_callTarget(&mut self, ctx: &CallTargetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callAlias}.
	 * @param ctx the parse tree
	 */
		fn visit_callAlias(&mut self, ctx: &CallAliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callAfterClause}.
	 * @param ctx the parse tree
	 */
		fn visit_callAfterClause(&mut self, ctx: &CallAfterClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callInputBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_callInputBlock(&mut self, ctx: &CallInputBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callInputItem}.
	 * @param ctx the parse tree
	 */
		fn visit_callInputItem(&mut self, ctx: &CallInputItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalOrExprOperation}
	 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOrExprOperation(&mut self, ctx: &LogicalOrExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalOrExprNone}
	 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOrExprNone(&mut self, ctx: &LogicalOrExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalAndExprOperation}
	 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalAndExprOperation(&mut self, ctx: &LogicalAndExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code logicalAndExprNone}
	 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalAndExprNone(&mut self, ctx: &LogicalAndExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code equalityExprOperation}
	 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityExprOperation(&mut self, ctx: &EqualityExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code equalityExprNone}
	 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityExprNone(&mut self, ctx: &EqualityExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code comparisonExprOperation}
	 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonExprOperation(&mut self, ctx: &ComparisonExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code comparisonExprNone}
	 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_comparisonExprNone(&mut self, ctx: &ComparisonExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code additiveExprOperation}
	 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExprOperation(&mut self, ctx: &AdditiveExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code additiveExprNone}
	 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExprNone(&mut self, ctx: &AdditiveExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExprOperation}
	 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExprOperation(&mut self, ctx: &MultiplicativeExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multiplicativeExprNone}
	 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExprNone(&mut self, ctx: &MultiplicativeExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code powerExprOperation}
	 * labeled alternative in {@link WdlV1Parser#powerExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_powerExprOperation(&mut self, ctx: &PowerExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code powerExprNone}
	 * labeled alternative in {@link WdlV1Parser#powerExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_powerExprNone(&mut self, ctx: &PowerExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unaryExprOperation}
	 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExprOperation(&mut self, ctx: &UnaryExprOperationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unaryExprNone}
	 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExprNone(&mut self, ctx: &UnaryExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code postfixExprField}
	 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_postfixExprField(&mut self, ctx: &PostfixExprFieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code postfixExprArrayIndex}
	 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_postfixExprArrayIndex(&mut self, ctx: &PostfixExprArrayIndexContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code postfixExprNone}
	 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_postfixExprNone(&mut self, ctx: &PostfixExprNoneContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#nullLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#noneLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_noneLiteral(&mut self, ctx: &NoneLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numberLiteralInt}
	 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numberLiteralInt(&mut self, ctx: &NumberLiteralIntContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numberLiteralFloat}
	 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numberLiteralFloat(&mut self, ctx: &NumberLiteralFloatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#numberLiteralSigned}.
	 * @param ctx the parse tree
	 */
		fn visit_numberLiteralSigned(&mut self, ctx: &NumberLiteralSignedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#arrayLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#mapLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_mapLiteral(&mut self, ctx: &MapLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#mapLiteralItem}.
	 * @param ctx the parse tree
	 */
		fn visit_mapLiteralItem(&mut self, ctx: &MapLiteralItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#objectLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#objectLiteralItem}.
	 * @param ctx the parse tree
	 */
		fn visit_objectLiteralItem(&mut self, ctx: &ObjectLiteralItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_structLiteral(&mut self, ctx: &StructLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#structLiteralItem}.
	 * @param ctx the parse tree
	 */
		fn visit_structLiteralItem(&mut self, ctx: &StructLiteralItemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#pairLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_pairLiteral(&mut self, ctx: &PairLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#groupedExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_groupedExpression(&mut self, ctx: &GroupedExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#ifExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_ifExpression(&mut self, ctx: &IfExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#callExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#stringLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#quotedString}.
	 * @param ctx the parse tree
	 */
		fn visit_quotedString(&mut self, ctx: &QuotedStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringElementText}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_stringElementText(&mut self, ctx: &StringElementTextContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringElementEscape}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_stringElementEscape(&mut self, ctx: &StringElementEscapeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringElementDollarSign}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_stringElementDollarSign(&mut self, ctx: &StringElementDollarSignContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringElementTilde}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_stringElementTilde(&mut self, ctx: &StringElementTildeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringElementPlaceholder}
	 * labeled alternative in {@link WdlV1Parser#stringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_stringElementPlaceholder(&mut self, ctx: &StringElementPlaceholderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#stringPlaceholder}.
	 * @param ctx the parse tree
	 */
		fn visit_stringPlaceholder(&mut self, ctx: &StringPlaceholderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#multilineString}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineString(&mut self, ctx: &MultilineStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementText}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringElementText(&mut self, ctx: &MultilineStringElementTextContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementEscape}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringElementEscape(&mut self, ctx: &MultilineStringElementEscapeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementDoubleCloseAngle}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringElementDoubleCloseAngle(&mut self, ctx: &MultilineStringElementDoubleCloseAngleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementSingleCloseAngle}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringElementSingleCloseAngle(&mut self, ctx: &MultilineStringElementSingleCloseAngleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementDollarSign}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringElementDollarSign(&mut self, ctx: &MultilineStringElementDollarSignContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementTilde}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringElementTilde(&mut self, ctx: &MultilineStringElementTildeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multilineStringElementPlaceholder}
	 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringElementPlaceholder(&mut self, ctx: &MultilineStringElementPlaceholderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#multilineStringPlaceholder}.
	 * @param ctx the parse tree
	 */
		fn visit_multilineStringPlaceholder(&mut self, ctx: &MultilineStringPlaceholderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#stringPlaceholderExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_stringPlaceholderExpression(&mut self, ctx: &StringPlaceholderExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringPlaceholderOptionSepDefault}
	 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
	 * @param ctx the parse tree
	 */
		fn visit_stringPlaceholderOptionSepDefault(&mut self, ctx: &StringPlaceholderOptionSepDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringPlaceholderOptionTrueFalse}
	 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
	 * @param ctx the parse tree
	 */
		fn visit_stringPlaceholderOptionTrueFalse(&mut self, ctx: &StringPlaceholderOptionTrueFalseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringPlaceholderOptionFalseTrue}
	 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
	 * @param ctx the parse tree
	 */
		fn visit_stringPlaceholderOptionFalseTrue(&mut self, ctx: &StringPlaceholderOptionFalseTrueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#strictIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_strictIdentifier(&mut self, ctx: &StrictIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#dottedIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_dottedIdentifier(&mut self, ctx: &DottedIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link WdlV1Parser#anyIdentBase}.
	 * @param ctx the parse tree
	 */
		fn visit_anyIdentBase(&mut self, ctx: &AnyIdentBaseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> WdlV1ParserVisitor<'input> for T
where
	T: WdlV1ParserVisitorCompat<'input>
{
	fn visit_document(&mut self, ctx: &DocumentContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_document(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_versionStatement(&mut self, ctx: &VersionStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_versionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_documentElement(&mut self, ctx: &DocumentElementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_documentElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatementStandard(&mut self, ctx: &ImportStatementStandardContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importStatementStandard(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatementStar(&mut self, ctx: &ImportStatementStarContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importStatementStar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatementMembers(&mut self, ctx: &ImportStatementMembersContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importStatementMembers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importMembers(&mut self, ctx: &ImportMembersContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importMembers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importMember(&mut self, ctx: &ImportMemberContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importMember(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importUriLiteral(&mut self, ctx: &ImportUriLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importUriLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importUriElement(&mut self, ctx: &ImportUriElementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importUriElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importAlias(&mut self, ctx: &ImportAliasContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_importAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structDefinition(&mut self, ctx: &StructDefinitionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_structDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structItemMetadata(&mut self, ctx: &StructItemMetadataContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_structItemMetadata(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structItemParameterMetadata(&mut self, ctx: &StructItemParameterMetadataContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_structItemParameterMetadata(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structItemMemberDeclaration(&mut self, ctx: &StructItemMemberDeclarationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_structItemMemberDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structDeclaration(&mut self, ctx: &StructDeclarationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_structDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumTypeParameter(&mut self, ctx: &EnumTypeParameterContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumTypeParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumChoice(&mut self, ctx: &EnumChoiceContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumChoice(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumLiteralExpression(&mut self, ctx: &EnumLiteralExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumLiteralExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumStringLiteral(&mut self, ctx: &EnumStringLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumStringLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumQuotedString(&mut self, ctx: &EnumQuotedStringContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumQuotedString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumStringElement(&mut self, ctx: &EnumStringElementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumStringElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumMultilineString(&mut self, ctx: &EnumMultilineStringContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumMultilineString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumMultilineStringElement(&mut self, ctx: &EnumMultilineStringElementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumMultilineStringElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumArrayLiteral(&mut self, ctx: &EnumArrayLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumArrayLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumMapLiteral(&mut self, ctx: &EnumMapLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumMapLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumMapLiteralItem(&mut self, ctx: &EnumMapLiteralItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumMapLiteralItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumObjectLiteral(&mut self, ctx: &EnumObjectLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumObjectLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumObjectLiteralItem(&mut self, ctx: &EnumObjectLiteralItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumObjectLiteralItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumStructLiteral(&mut self, ctx: &EnumStructLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumStructLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumStructLiteralItem(&mut self, ctx: &EnumStructLiteralItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumStructLiteralItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumPairLiteral(&mut self, ctx: &EnumPairLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_enumPairLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskDefinition(&mut self, ctx: &TaskDefinitionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowDefinition(&mut self, ctx: &WorkflowDefinitionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type(&mut self, ctx: &TypeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mapType(&mut self, ctx: &MapTypeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_mapType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_arrayType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pairType(&mut self, ctx: &PairTypeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_pairType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectType(&mut self, ctx: &ObjectTypeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_objectType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_primitiveType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeRefType(&mut self, ctx: &TypeRefTypeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_typeRefType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unboundDeclaration(&mut self, ctx: &UnboundDeclarationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_unboundDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_boundDeclaration(&mut self, ctx: &BoundDeclarationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_boundDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_declaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskInputSection(&mut self, ctx: &TaskInputSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskInputSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskCommandSection(&mut self, ctx: &TaskCommandSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskCommandSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskOutputSection(&mut self, ctx: &TaskOutputSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskOutputSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskRuntimeSection(&mut self, ctx: &TaskRuntimeSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskRuntimeSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskRequirementsSection(&mut self, ctx: &TaskRequirementsSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskRequirementsSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskHintsSection(&mut self, ctx: &TaskHintsSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskHintsSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskMetadataSection(&mut self, ctx: &TaskMetadataSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskMetadataSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskParameterMetadataSection(&mut self, ctx: &TaskParameterMetadataSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskParameterMetadataSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskDeclaration(&mut self, ctx: &TaskDeclarationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowInputSection(&mut self, ctx: &WorkflowInputSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowInputSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowOutputSection(&mut self, ctx: &WorkflowOutputSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowOutputSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowHintsSection(&mut self, ctx: &WorkflowHintsSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowHintsSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowConditionalStatement(&mut self, ctx: &WorkflowConditionalStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowConditionalStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowScatterStatement(&mut self, ctx: &WorkflowScatterStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowScatterStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowCallStatement(&mut self, ctx: &WorkflowCallStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowCallStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowMetadataSection(&mut self, ctx: &WorkflowMetadataSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowMetadataSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowParameterMetadataSection(&mut self, ctx: &WorkflowParameterMetadataSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowParameterMetadataSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowDeclaration(&mut self, ctx: &WorkflowDeclarationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputSection(&mut self, ctx: &InputSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_inputSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_outputSection(&mut self, ctx: &OutputSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_outputSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_runtimeSection(&mut self, ctx: &RuntimeSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_runtimeSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_runtimeItem(&mut self, ctx: &RuntimeItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_runtimeItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_requirementsSection(&mut self, ctx: &RequirementsSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_requirementsSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_requirementsItem(&mut self, ctx: &RequirementsItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_requirementsItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsSectionTask(&mut self, ctx: &HintsSectionTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsSectionTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsItemTask(&mut self, ctx: &HintsItemTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsItemTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskHintValueExpression(&mut self, ctx: &TaskHintValueExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskHintValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskHintValueHintsObject(&mut self, ctx: &TaskHintValueHintsObjectContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskHintValueHintsObject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskHintValueInputObject(&mut self, ctx: &TaskHintValueInputObjectContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskHintValueInputObject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskHintValueOutputObject(&mut self, ctx: &TaskHintValueOutputObjectContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskHintValueOutputObject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskHintValueArray(&mut self, ctx: &TaskHintValueArrayContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskHintValueArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsTypedObjectTask(&mut self, ctx: &HintsTypedObjectTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsTypedObjectTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsObjectItemTask(&mut self, ctx: &HintsObjectItemTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsObjectItemTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputHintsObjectTask(&mut self, ctx: &InputHintsObjectTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_inputHintsObjectTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inputHintsItemTask(&mut self, ctx: &InputHintsItemTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_inputHintsItemTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_outputHintsObjectTask(&mut self, ctx: &OutputHintsObjectTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_outputHintsObjectTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_outputHintsItemTask(&mut self, ctx: &OutputHintsItemTaskContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_outputHintsItemTask(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_taskHintsArray(&mut self, ctx: &TaskHintsArrayContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_taskHintsArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsSectionWorkflow(&mut self, ctx: &HintsSectionWorkflowContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsSectionWorkflow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsItemWorkflow(&mut self, ctx: &HintsItemWorkflowContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsItemWorkflow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowHintValueNumber(&mut self, ctx: &WorkflowHintValueNumberContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowHintValueNumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowHintValueString(&mut self, ctx: &WorkflowHintValueStringContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowHintValueString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowHintValueBoolean(&mut self, ctx: &WorkflowHintValueBooleanContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowHintValueBoolean(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowHintValueObject(&mut self, ctx: &WorkflowHintValueObjectContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowHintValueObject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowHintValueArray(&mut self, ctx: &WorkflowHintValueArrayContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowHintValueArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsObjectWorkflow(&mut self, ctx: &HintsObjectWorkflowContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsObjectWorkflow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hintsObjectItemWorkflow(&mut self, ctx: &HintsObjectItemWorkflowContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_hintsObjectItemWorkflow(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowHintsArray(&mut self, ctx: &WorkflowHintsArrayContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowHintsArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_metadataSection(&mut self, ctx: &MetadataSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_metadataSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterMetadataSection(&mut self, ctx: &ParameterMetadataSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_parameterMetadataSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_metadataObject(&mut self, ctx: &MetadataObjectContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_metadataObject(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_metadataObjectItem(&mut self, ctx: &MetadataObjectItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_metadataObjectItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_metadataArray(&mut self, ctx: &MetadataArrayContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_metadataArray(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_metadataValue(&mut self, ctx: &MetadataValueContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_metadataValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_commandSection(&mut self, ctx: &CommandSectionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_commandSection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringCommand(&mut self, ctx: &MultilineStringCommandContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bracedCommand(&mut self, ctx: &BracedCommandContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_bracedCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_workflowStatement(&mut self, ctx: &WorkflowStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_workflowStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalStatement(&mut self, ctx: &ConditionalStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_conditionalStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalElseIfClause(&mut self, ctx: &ConditionalElseIfClauseContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_conditionalElseIfClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalElseClause(&mut self, ctx: &ConditionalElseClauseContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_conditionalElseClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scatterStatement(&mut self, ctx: &ScatterStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_scatterStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scatterBody(&mut self, ctx: &ScatterBodyContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_scatterBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_callStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callTarget(&mut self, ctx: &CallTargetContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_callTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callAlias(&mut self, ctx: &CallAliasContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_callAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callAfterClause(&mut self, ctx: &CallAfterClauseContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_callAfterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callInputBlock(&mut self, ctx: &CallInputBlockContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_callInputBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callInputItem(&mut self, ctx: &CallInputItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_callInputItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOrExprOperation(&mut self, ctx: &LogicalOrExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_logicalOrExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOrExprNone(&mut self, ctx: &LogicalOrExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_logicalOrExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalAndExprOperation(&mut self, ctx: &LogicalAndExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_logicalAndExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalAndExprNone(&mut self, ctx: &LogicalAndExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_logicalAndExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityExprOperation(&mut self, ctx: &EqualityExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_equalityExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityExprNone(&mut self, ctx: &EqualityExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_equalityExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonExprOperation(&mut self, ctx: &ComparisonExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_comparisonExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparisonExprNone(&mut self, ctx: &ComparisonExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_comparisonExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExprOperation(&mut self, ctx: &AdditiveExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_additiveExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExprNone(&mut self, ctx: &AdditiveExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_additiveExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExprOperation(&mut self, ctx: &MultiplicativeExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multiplicativeExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExprNone(&mut self, ctx: &MultiplicativeExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multiplicativeExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powerExprOperation(&mut self, ctx: &PowerExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_powerExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powerExprNone(&mut self, ctx: &PowerExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_powerExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExprOperation(&mut self, ctx: &UnaryExprOperationContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_unaryExprOperation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExprNone(&mut self, ctx: &UnaryExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_unaryExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfixExprField(&mut self, ctx: &PostfixExprFieldContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_postfixExprField(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfixExprArrayIndex(&mut self, ctx: &PostfixExprArrayIndexContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_postfixExprArrayIndex(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfixExprNone(&mut self, ctx: &PostfixExprNoneContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_postfixExprNone(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_primaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_nullLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noneLiteral(&mut self, ctx: &NoneLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_noneLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_booleanLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberLiteralInt(&mut self, ctx: &NumberLiteralIntContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_numberLiteralInt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberLiteralFloat(&mut self, ctx: &NumberLiteralFloatContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_numberLiteralFloat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberLiteralSigned(&mut self, ctx: &NumberLiteralSignedContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_numberLiteralSigned(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_arrayLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mapLiteral(&mut self, ctx: &MapLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_mapLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mapLiteralItem(&mut self, ctx: &MapLiteralItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_mapLiteralItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_objectLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_objectLiteralItem(&mut self, ctx: &ObjectLiteralItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_objectLiteralItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structLiteral(&mut self, ctx: &StructLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_structLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_structLiteralItem(&mut self, ctx: &StructLiteralItemContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_structLiteralItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pairLiteral(&mut self, ctx: &PairLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_pairLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_groupedExpression(&mut self, ctx: &GroupedExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_groupedExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ifExpression(&mut self, ctx: &IfExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_ifExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_callExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_quotedString(&mut self, ctx: &QuotedStringContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_quotedString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringElementText(&mut self, ctx: &StringElementTextContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringElementText(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringElementEscape(&mut self, ctx: &StringElementEscapeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringElementEscape(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringElementDollarSign(&mut self, ctx: &StringElementDollarSignContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringElementDollarSign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringElementTilde(&mut self, ctx: &StringElementTildeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringElementTilde(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringElementPlaceholder(&mut self, ctx: &StringElementPlaceholderContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringElementPlaceholder(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringPlaceholder(&mut self, ctx: &StringPlaceholderContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringPlaceholder(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineString(&mut self, ctx: &MultilineStringContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringElementText(&mut self, ctx: &MultilineStringElementTextContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringElementText(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringElementEscape(&mut self, ctx: &MultilineStringElementEscapeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringElementEscape(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringElementDoubleCloseAngle(&mut self, ctx: &MultilineStringElementDoubleCloseAngleContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringElementDoubleCloseAngle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringElementSingleCloseAngle(&mut self, ctx: &MultilineStringElementSingleCloseAngleContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringElementSingleCloseAngle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringElementDollarSign(&mut self, ctx: &MultilineStringElementDollarSignContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringElementDollarSign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringElementTilde(&mut self, ctx: &MultilineStringElementTildeContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringElementTilde(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringElementPlaceholder(&mut self, ctx: &MultilineStringElementPlaceholderContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringElementPlaceholder(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multilineStringPlaceholder(&mut self, ctx: &MultilineStringPlaceholderContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_multilineStringPlaceholder(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringPlaceholderExpression(&mut self, ctx: &StringPlaceholderExpressionContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringPlaceholderExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringPlaceholderOptionSepDefault(&mut self, ctx: &StringPlaceholderOptionSepDefaultContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringPlaceholderOptionSepDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringPlaceholderOptionTrueFalse(&mut self, ctx: &StringPlaceholderOptionTrueFalseContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringPlaceholderOptionTrueFalse(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringPlaceholderOptionFalseTrue(&mut self, ctx: &StringPlaceholderOptionFalseTrueContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_stringPlaceholderOptionFalseTrue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_strictIdentifier(&mut self, ctx: &StrictIdentifierContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_strictIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dottedIdentifier(&mut self, ctx: &DottedIdentifierContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_dottedIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_anyIdentBase(&mut self, ctx: &AnyIdentBaseContext<'input>){
		let result = <Self as WdlV1ParserVisitorCompat>::visit_anyIdentBase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}