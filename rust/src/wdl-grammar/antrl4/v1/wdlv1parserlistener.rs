#![allow(nonstandard_style)]
// Generated from ../wdl-grammar/antrl4/v1/WdlV1Parser.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::wdlv1parser::*;

pub trait WdlV1ParserListener<'input> : ParseTreeListener<'input,WdlV1ParserContextType>{
/**
 * Enter a parse tree produced by {@link WdlV1Parser#document}.
 * @param ctx the parse tree
 */
fn enter_document(&mut self, _ctx: &DocumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#document}.
 * @param ctx the parse tree
 */
fn exit_document(&mut self, _ctx: &DocumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#versionStatement}.
 * @param ctx the parse tree
 */
fn enter_versionStatement(&mut self, _ctx: &VersionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#versionStatement}.
 * @param ctx the parse tree
 */
fn exit_versionStatement(&mut self, _ctx: &VersionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#documentElement}.
 * @param ctx the parse tree
 */
fn enter_documentElement(&mut self, _ctx: &DocumentElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#documentElement}.
 * @param ctx the parse tree
 */
fn exit_documentElement(&mut self, _ctx: &DocumentElementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code importStatementStandard}
 * labeled alternative in {@link WdlV1Parser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatementStandard(&mut self, _ctx: &ImportStatementStandardContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code importStatementStandard}
 * labeled alternative in {@link WdlV1Parser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatementStandard(&mut self, _ctx: &ImportStatementStandardContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code importStatementStar}
 * labeled alternative in {@link WdlV1Parser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatementStar(&mut self, _ctx: &ImportStatementStarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code importStatementStar}
 * labeled alternative in {@link WdlV1Parser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatementStar(&mut self, _ctx: &ImportStatementStarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code importStatementMembers}
 * labeled alternative in {@link WdlV1Parser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatementMembers(&mut self, _ctx: &ImportStatementMembersContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code importStatementMembers}
 * labeled alternative in {@link WdlV1Parser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatementMembers(&mut self, _ctx: &ImportStatementMembersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#importMembers}.
 * @param ctx the parse tree
 */
fn enter_importMembers(&mut self, _ctx: &ImportMembersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#importMembers}.
 * @param ctx the parse tree
 */
fn exit_importMembers(&mut self, _ctx: &ImportMembersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#importMember}.
 * @param ctx the parse tree
 */
fn enter_importMember(&mut self, _ctx: &ImportMemberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#importMember}.
 * @param ctx the parse tree
 */
fn exit_importMember(&mut self, _ctx: &ImportMemberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#importUriLiteral}.
 * @param ctx the parse tree
 */
fn enter_importUriLiteral(&mut self, _ctx: &ImportUriLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#importUriLiteral}.
 * @param ctx the parse tree
 */
fn exit_importUriLiteral(&mut self, _ctx: &ImportUriLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#importUriElement}.
 * @param ctx the parse tree
 */
fn enter_importUriElement(&mut self, _ctx: &ImportUriElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#importUriElement}.
 * @param ctx the parse tree
 */
fn exit_importUriElement(&mut self, _ctx: &ImportUriElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#importAlias}.
 * @param ctx the parse tree
 */
fn enter_importAlias(&mut self, _ctx: &ImportAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#importAlias}.
 * @param ctx the parse tree
 */
fn exit_importAlias(&mut self, _ctx: &ImportAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#structDefinition}.
 * @param ctx the parse tree
 */
fn enter_structDefinition(&mut self, _ctx: &StructDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#structDefinition}.
 * @param ctx the parse tree
 */
fn exit_structDefinition(&mut self, _ctx: &StructDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code structItemMetadata}
 * labeled alternative in {@link WdlV1Parser#structItem}.
 * @param ctx the parse tree
 */
fn enter_structItemMetadata(&mut self, _ctx: &StructItemMetadataContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code structItemMetadata}
 * labeled alternative in {@link WdlV1Parser#structItem}.
 * @param ctx the parse tree
 */
fn exit_structItemMetadata(&mut self, _ctx: &StructItemMetadataContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code structItemParameterMetadata}
 * labeled alternative in {@link WdlV1Parser#structItem}.
 * @param ctx the parse tree
 */
fn enter_structItemParameterMetadata(&mut self, _ctx: &StructItemParameterMetadataContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code structItemParameterMetadata}
 * labeled alternative in {@link WdlV1Parser#structItem}.
 * @param ctx the parse tree
 */
fn exit_structItemParameterMetadata(&mut self, _ctx: &StructItemParameterMetadataContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code structItemMemberDeclaration}
 * labeled alternative in {@link WdlV1Parser#structItem}.
 * @param ctx the parse tree
 */
fn enter_structItemMemberDeclaration(&mut self, _ctx: &StructItemMemberDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code structItemMemberDeclaration}
 * labeled alternative in {@link WdlV1Parser#structItem}.
 * @param ctx the parse tree
 */
fn exit_structItemMemberDeclaration(&mut self, _ctx: &StructItemMemberDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#structDeclaration}.
 * @param ctx the parse tree
 */
fn enter_structDeclaration(&mut self, _ctx: &StructDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#structDeclaration}.
 * @param ctx the parse tree
 */
fn exit_structDeclaration(&mut self, _ctx: &StructDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumDefinition}.
 * @param ctx the parse tree
 */
fn enter_enumDefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumDefinition}.
 * @param ctx the parse tree
 */
fn exit_enumDefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumTypeParameter}.
 * @param ctx the parse tree
 */
fn enter_enumTypeParameter(&mut self, _ctx: &EnumTypeParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumTypeParameter}.
 * @param ctx the parse tree
 */
fn exit_enumTypeParameter(&mut self, _ctx: &EnumTypeParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumChoice}.
 * @param ctx the parse tree
 */
fn enter_enumChoice(&mut self, _ctx: &EnumChoiceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumChoice}.
 * @param ctx the parse tree
 */
fn exit_enumChoice(&mut self, _ctx: &EnumChoiceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumLiteralExpression}.
 * @param ctx the parse tree
 */
fn enter_enumLiteralExpression(&mut self, _ctx: &EnumLiteralExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumLiteralExpression}.
 * @param ctx the parse tree
 */
fn exit_enumLiteralExpression(&mut self, _ctx: &EnumLiteralExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumStringLiteral}.
 * @param ctx the parse tree
 */
fn enter_enumStringLiteral(&mut self, _ctx: &EnumStringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumStringLiteral}.
 * @param ctx the parse tree
 */
fn exit_enumStringLiteral(&mut self, _ctx: &EnumStringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumQuotedString}.
 * @param ctx the parse tree
 */
fn enter_enumQuotedString(&mut self, _ctx: &EnumQuotedStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumQuotedString}.
 * @param ctx the parse tree
 */
fn exit_enumQuotedString(&mut self, _ctx: &EnumQuotedStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumStringElement}.
 * @param ctx the parse tree
 */
fn enter_enumStringElement(&mut self, _ctx: &EnumStringElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumStringElement}.
 * @param ctx the parse tree
 */
fn exit_enumStringElement(&mut self, _ctx: &EnumStringElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumMultilineString}.
 * @param ctx the parse tree
 */
fn enter_enumMultilineString(&mut self, _ctx: &EnumMultilineStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumMultilineString}.
 * @param ctx the parse tree
 */
fn exit_enumMultilineString(&mut self, _ctx: &EnumMultilineStringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumMultilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_enumMultilineStringElement(&mut self, _ctx: &EnumMultilineStringElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumMultilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_enumMultilineStringElement(&mut self, _ctx: &EnumMultilineStringElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumArrayLiteral}.
 * @param ctx the parse tree
 */
fn enter_enumArrayLiteral(&mut self, _ctx: &EnumArrayLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumArrayLiteral}.
 * @param ctx the parse tree
 */
fn exit_enumArrayLiteral(&mut self, _ctx: &EnumArrayLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumMapLiteral}.
 * @param ctx the parse tree
 */
fn enter_enumMapLiteral(&mut self, _ctx: &EnumMapLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumMapLiteral}.
 * @param ctx the parse tree
 */
fn exit_enumMapLiteral(&mut self, _ctx: &EnumMapLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumMapLiteralItem}.
 * @param ctx the parse tree
 */
fn enter_enumMapLiteralItem(&mut self, _ctx: &EnumMapLiteralItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumMapLiteralItem}.
 * @param ctx the parse tree
 */
fn exit_enumMapLiteralItem(&mut self, _ctx: &EnumMapLiteralItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumObjectLiteral}.
 * @param ctx the parse tree
 */
fn enter_enumObjectLiteral(&mut self, _ctx: &EnumObjectLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumObjectLiteral}.
 * @param ctx the parse tree
 */
fn exit_enumObjectLiteral(&mut self, _ctx: &EnumObjectLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumObjectLiteralItem}.
 * @param ctx the parse tree
 */
fn enter_enumObjectLiteralItem(&mut self, _ctx: &EnumObjectLiteralItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumObjectLiteralItem}.
 * @param ctx the parse tree
 */
fn exit_enumObjectLiteralItem(&mut self, _ctx: &EnumObjectLiteralItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumStructLiteral}.
 * @param ctx the parse tree
 */
fn enter_enumStructLiteral(&mut self, _ctx: &EnumStructLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumStructLiteral}.
 * @param ctx the parse tree
 */
fn exit_enumStructLiteral(&mut self, _ctx: &EnumStructLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumStructLiteralItem}.
 * @param ctx the parse tree
 */
fn enter_enumStructLiteralItem(&mut self, _ctx: &EnumStructLiteralItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumStructLiteralItem}.
 * @param ctx the parse tree
 */
fn exit_enumStructLiteralItem(&mut self, _ctx: &EnumStructLiteralItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#enumPairLiteral}.
 * @param ctx the parse tree
 */
fn enter_enumPairLiteral(&mut self, _ctx: &EnumPairLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#enumPairLiteral}.
 * @param ctx the parse tree
 */
fn exit_enumPairLiteral(&mut self, _ctx: &EnumPairLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#taskDefinition}.
 * @param ctx the parse tree
 */
fn enter_taskDefinition(&mut self, _ctx: &TaskDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#taskDefinition}.
 * @param ctx the parse tree
 */
fn exit_taskDefinition(&mut self, _ctx: &TaskDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#workflowDefinition}.
 * @param ctx the parse tree
 */
fn enter_workflowDefinition(&mut self, _ctx: &WorkflowDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#workflowDefinition}.
 * @param ctx the parse tree
 */
fn exit_workflowDefinition(&mut self, _ctx: &WorkflowDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#type}.
 * @param ctx the parse tree
 */
fn enter_type(&mut self, _ctx: &TypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#type}.
 * @param ctx the parse tree
 */
fn exit_type(&mut self, _ctx: &TypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#mapType}.
 * @param ctx the parse tree
 */
fn enter_mapType(&mut self, _ctx: &MapTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#mapType}.
 * @param ctx the parse tree
 */
fn exit_mapType(&mut self, _ctx: &MapTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#arrayType}.
 * @param ctx the parse tree
 */
fn enter_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#arrayType}.
 * @param ctx the parse tree
 */
fn exit_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#pairType}.
 * @param ctx the parse tree
 */
fn enter_pairType(&mut self, _ctx: &PairTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#pairType}.
 * @param ctx the parse tree
 */
fn exit_pairType(&mut self, _ctx: &PairTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#objectType}.
 * @param ctx the parse tree
 */
fn enter_objectType(&mut self, _ctx: &ObjectTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#objectType}.
 * @param ctx the parse tree
 */
fn exit_objectType(&mut self, _ctx: &ObjectTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#primitiveType}.
 * @param ctx the parse tree
 */
fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#primitiveType}.
 * @param ctx the parse tree
 */
fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#typeRefType}.
 * @param ctx the parse tree
 */
fn enter_typeRefType(&mut self, _ctx: &TypeRefTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#typeRefType}.
 * @param ctx the parse tree
 */
fn exit_typeRefType(&mut self, _ctx: &TypeRefTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#unboundDeclaration}.
 * @param ctx the parse tree
 */
fn enter_unboundDeclaration(&mut self, _ctx: &UnboundDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#unboundDeclaration}.
 * @param ctx the parse tree
 */
fn exit_unboundDeclaration(&mut self, _ctx: &UnboundDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#boundDeclaration}.
 * @param ctx the parse tree
 */
fn enter_boundDeclaration(&mut self, _ctx: &BoundDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#boundDeclaration}.
 * @param ctx the parse tree
 */
fn exit_boundDeclaration(&mut self, _ctx: &BoundDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#declaration}.
 * @param ctx the parse tree
 */
fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#declaration}.
 * @param ctx the parse tree
 */
fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskInputSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskInputSection(&mut self, _ctx: &TaskInputSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskInputSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskInputSection(&mut self, _ctx: &TaskInputSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskCommandSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskCommandSection(&mut self, _ctx: &TaskCommandSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskCommandSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskCommandSection(&mut self, _ctx: &TaskCommandSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskOutputSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskOutputSection(&mut self, _ctx: &TaskOutputSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskOutputSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskOutputSection(&mut self, _ctx: &TaskOutputSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskRuntimeSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskRuntimeSection(&mut self, _ctx: &TaskRuntimeSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskRuntimeSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskRuntimeSection(&mut self, _ctx: &TaskRuntimeSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskRequirementsSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskRequirementsSection(&mut self, _ctx: &TaskRequirementsSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskRequirementsSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskRequirementsSection(&mut self, _ctx: &TaskRequirementsSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskHintsSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskHintsSection(&mut self, _ctx: &TaskHintsSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskHintsSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskHintsSection(&mut self, _ctx: &TaskHintsSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskMetadataSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskMetadataSection(&mut self, _ctx: &TaskMetadataSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskMetadataSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskMetadataSection(&mut self, _ctx: &TaskMetadataSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskParameterMetadataSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskParameterMetadataSection(&mut self, _ctx: &TaskParameterMetadataSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskParameterMetadataSection}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskParameterMetadataSection(&mut self, _ctx: &TaskParameterMetadataSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskDeclaration}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn enter_taskDeclaration(&mut self, _ctx: &TaskDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskDeclaration}
 * labeled alternative in {@link WdlV1Parser#taskElement}.
 * @param ctx the parse tree
 */
fn exit_taskDeclaration(&mut self, _ctx: &TaskDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowInputSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowInputSection(&mut self, _ctx: &WorkflowInputSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowInputSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowInputSection(&mut self, _ctx: &WorkflowInputSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowOutputSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowOutputSection(&mut self, _ctx: &WorkflowOutputSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowOutputSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowOutputSection(&mut self, _ctx: &WorkflowOutputSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowHintsSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowHintsSection(&mut self, _ctx: &WorkflowHintsSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowHintsSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowHintsSection(&mut self, _ctx: &WorkflowHintsSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowConditionalStatement}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowConditionalStatement(&mut self, _ctx: &WorkflowConditionalStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowConditionalStatement}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowConditionalStatement(&mut self, _ctx: &WorkflowConditionalStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowScatterStatement}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowScatterStatement(&mut self, _ctx: &WorkflowScatterStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowScatterStatement}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowScatterStatement(&mut self, _ctx: &WorkflowScatterStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowCallStatement}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowCallStatement(&mut self, _ctx: &WorkflowCallStatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowCallStatement}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowCallStatement(&mut self, _ctx: &WorkflowCallStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowMetadataSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowMetadataSection(&mut self, _ctx: &WorkflowMetadataSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowMetadataSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowMetadataSection(&mut self, _ctx: &WorkflowMetadataSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowParameterMetadataSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowParameterMetadataSection(&mut self, _ctx: &WorkflowParameterMetadataSectionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowParameterMetadataSection}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowParameterMetadataSection(&mut self, _ctx: &WorkflowParameterMetadataSectionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowDeclaration}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn enter_workflowDeclaration(&mut self, _ctx: &WorkflowDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowDeclaration}
 * labeled alternative in {@link WdlV1Parser#workflowElement}.
 * @param ctx the parse tree
 */
fn exit_workflowDeclaration(&mut self, _ctx: &WorkflowDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#inputSection}.
 * @param ctx the parse tree
 */
fn enter_inputSection(&mut self, _ctx: &InputSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#inputSection}.
 * @param ctx the parse tree
 */
fn exit_inputSection(&mut self, _ctx: &InputSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#outputSection}.
 * @param ctx the parse tree
 */
fn enter_outputSection(&mut self, _ctx: &OutputSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#outputSection}.
 * @param ctx the parse tree
 */
fn exit_outputSection(&mut self, _ctx: &OutputSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#runtimeSection}.
 * @param ctx the parse tree
 */
fn enter_runtimeSection(&mut self, _ctx: &RuntimeSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#runtimeSection}.
 * @param ctx the parse tree
 */
fn exit_runtimeSection(&mut self, _ctx: &RuntimeSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#runtimeItem}.
 * @param ctx the parse tree
 */
fn enter_runtimeItem(&mut self, _ctx: &RuntimeItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#runtimeItem}.
 * @param ctx the parse tree
 */
fn exit_runtimeItem(&mut self, _ctx: &RuntimeItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#requirementsSection}.
 * @param ctx the parse tree
 */
fn enter_requirementsSection(&mut self, _ctx: &RequirementsSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#requirementsSection}.
 * @param ctx the parse tree
 */
fn exit_requirementsSection(&mut self, _ctx: &RequirementsSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#requirementsItem}.
 * @param ctx the parse tree
 */
fn enter_requirementsItem(&mut self, _ctx: &RequirementsItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#requirementsItem}.
 * @param ctx the parse tree
 */
fn exit_requirementsItem(&mut self, _ctx: &RequirementsItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsSectionTask}.
 * @param ctx the parse tree
 */
fn enter_hintsSectionTask(&mut self, _ctx: &HintsSectionTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsSectionTask}.
 * @param ctx the parse tree
 */
fn exit_hintsSectionTask(&mut self, _ctx: &HintsSectionTaskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsItemTask}.
 * @param ctx the parse tree
 */
fn enter_hintsItemTask(&mut self, _ctx: &HintsItemTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsItemTask}.
 * @param ctx the parse tree
 */
fn exit_hintsItemTask(&mut self, _ctx: &HintsItemTaskContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskHintValueExpression}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn enter_taskHintValueExpression(&mut self, _ctx: &TaskHintValueExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskHintValueExpression}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn exit_taskHintValueExpression(&mut self, _ctx: &TaskHintValueExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskHintValueHintsObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn enter_taskHintValueHintsObject(&mut self, _ctx: &TaskHintValueHintsObjectContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskHintValueHintsObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn exit_taskHintValueHintsObject(&mut self, _ctx: &TaskHintValueHintsObjectContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskHintValueInputObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn enter_taskHintValueInputObject(&mut self, _ctx: &TaskHintValueInputObjectContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskHintValueInputObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn exit_taskHintValueInputObject(&mut self, _ctx: &TaskHintValueInputObjectContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskHintValueOutputObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn enter_taskHintValueOutputObject(&mut self, _ctx: &TaskHintValueOutputObjectContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskHintValueOutputObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn exit_taskHintValueOutputObject(&mut self, _ctx: &TaskHintValueOutputObjectContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code taskHintValueArray}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn enter_taskHintValueArray(&mut self, _ctx: &TaskHintValueArrayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code taskHintValueArray}
 * labeled alternative in {@link WdlV1Parser#hintsValueTask}.
 * @param ctx the parse tree
 */
fn exit_taskHintValueArray(&mut self, _ctx: &TaskHintValueArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsTypedObjectTask}.
 * @param ctx the parse tree
 */
fn enter_hintsTypedObjectTask(&mut self, _ctx: &HintsTypedObjectTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsTypedObjectTask}.
 * @param ctx the parse tree
 */
fn exit_hintsTypedObjectTask(&mut self, _ctx: &HintsTypedObjectTaskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsObjectItemTask}.
 * @param ctx the parse tree
 */
fn enter_hintsObjectItemTask(&mut self, _ctx: &HintsObjectItemTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsObjectItemTask}.
 * @param ctx the parse tree
 */
fn exit_hintsObjectItemTask(&mut self, _ctx: &HintsObjectItemTaskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#inputHintsObjectTask}.
 * @param ctx the parse tree
 */
fn enter_inputHintsObjectTask(&mut self, _ctx: &InputHintsObjectTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#inputHintsObjectTask}.
 * @param ctx the parse tree
 */
fn exit_inputHintsObjectTask(&mut self, _ctx: &InputHintsObjectTaskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#inputHintsItemTask}.
 * @param ctx the parse tree
 */
fn enter_inputHintsItemTask(&mut self, _ctx: &InputHintsItemTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#inputHintsItemTask}.
 * @param ctx the parse tree
 */
fn exit_inputHintsItemTask(&mut self, _ctx: &InputHintsItemTaskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#outputHintsObjectTask}.
 * @param ctx the parse tree
 */
fn enter_outputHintsObjectTask(&mut self, _ctx: &OutputHintsObjectTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#outputHintsObjectTask}.
 * @param ctx the parse tree
 */
fn exit_outputHintsObjectTask(&mut self, _ctx: &OutputHintsObjectTaskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#outputHintsItemTask}.
 * @param ctx the parse tree
 */
fn enter_outputHintsItemTask(&mut self, _ctx: &OutputHintsItemTaskContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#outputHintsItemTask}.
 * @param ctx the parse tree
 */
fn exit_outputHintsItemTask(&mut self, _ctx: &OutputHintsItemTaskContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#taskHintsArray}.
 * @param ctx the parse tree
 */
fn enter_taskHintsArray(&mut self, _ctx: &TaskHintsArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#taskHintsArray}.
 * @param ctx the parse tree
 */
fn exit_taskHintsArray(&mut self, _ctx: &TaskHintsArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsSectionWorkflow}.
 * @param ctx the parse tree
 */
fn enter_hintsSectionWorkflow(&mut self, _ctx: &HintsSectionWorkflowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsSectionWorkflow}.
 * @param ctx the parse tree
 */
fn exit_hintsSectionWorkflow(&mut self, _ctx: &HintsSectionWorkflowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsItemWorkflow}.
 * @param ctx the parse tree
 */
fn enter_hintsItemWorkflow(&mut self, _ctx: &HintsItemWorkflowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsItemWorkflow}.
 * @param ctx the parse tree
 */
fn exit_hintsItemWorkflow(&mut self, _ctx: &HintsItemWorkflowContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowHintValueNumber}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn enter_workflowHintValueNumber(&mut self, _ctx: &WorkflowHintValueNumberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowHintValueNumber}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn exit_workflowHintValueNumber(&mut self, _ctx: &WorkflowHintValueNumberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowHintValueString}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn enter_workflowHintValueString(&mut self, _ctx: &WorkflowHintValueStringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowHintValueString}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn exit_workflowHintValueString(&mut self, _ctx: &WorkflowHintValueStringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowHintValueBoolean}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn enter_workflowHintValueBoolean(&mut self, _ctx: &WorkflowHintValueBooleanContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowHintValueBoolean}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn exit_workflowHintValueBoolean(&mut self, _ctx: &WorkflowHintValueBooleanContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowHintValueObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn enter_workflowHintValueObject(&mut self, _ctx: &WorkflowHintValueObjectContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowHintValueObject}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn exit_workflowHintValueObject(&mut self, _ctx: &WorkflowHintValueObjectContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code workflowHintValueArray}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn enter_workflowHintValueArray(&mut self, _ctx: &WorkflowHintValueArrayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code workflowHintValueArray}
 * labeled alternative in {@link WdlV1Parser#hintsValueWorkflow}.
 * @param ctx the parse tree
 */
fn exit_workflowHintValueArray(&mut self, _ctx: &WorkflowHintValueArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsObjectWorkflow}.
 * @param ctx the parse tree
 */
fn enter_hintsObjectWorkflow(&mut self, _ctx: &HintsObjectWorkflowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsObjectWorkflow}.
 * @param ctx the parse tree
 */
fn exit_hintsObjectWorkflow(&mut self, _ctx: &HintsObjectWorkflowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#hintsObjectItemWorkflow}.
 * @param ctx the parse tree
 */
fn enter_hintsObjectItemWorkflow(&mut self, _ctx: &HintsObjectItemWorkflowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#hintsObjectItemWorkflow}.
 * @param ctx the parse tree
 */
fn exit_hintsObjectItemWorkflow(&mut self, _ctx: &HintsObjectItemWorkflowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#workflowHintsArray}.
 * @param ctx the parse tree
 */
fn enter_workflowHintsArray(&mut self, _ctx: &WorkflowHintsArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#workflowHintsArray}.
 * @param ctx the parse tree
 */
fn exit_workflowHintsArray(&mut self, _ctx: &WorkflowHintsArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#metadataSection}.
 * @param ctx the parse tree
 */
fn enter_metadataSection(&mut self, _ctx: &MetadataSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#metadataSection}.
 * @param ctx the parse tree
 */
fn exit_metadataSection(&mut self, _ctx: &MetadataSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#parameterMetadataSection}.
 * @param ctx the parse tree
 */
fn enter_parameterMetadataSection(&mut self, _ctx: &ParameterMetadataSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#parameterMetadataSection}.
 * @param ctx the parse tree
 */
fn exit_parameterMetadataSection(&mut self, _ctx: &ParameterMetadataSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#metadataObject}.
 * @param ctx the parse tree
 */
fn enter_metadataObject(&mut self, _ctx: &MetadataObjectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#metadataObject}.
 * @param ctx the parse tree
 */
fn exit_metadataObject(&mut self, _ctx: &MetadataObjectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#metadataObjectItem}.
 * @param ctx the parse tree
 */
fn enter_metadataObjectItem(&mut self, _ctx: &MetadataObjectItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#metadataObjectItem}.
 * @param ctx the parse tree
 */
fn exit_metadataObjectItem(&mut self, _ctx: &MetadataObjectItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#metadataArray}.
 * @param ctx the parse tree
 */
fn enter_metadataArray(&mut self, _ctx: &MetadataArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#metadataArray}.
 * @param ctx the parse tree
 */
fn exit_metadataArray(&mut self, _ctx: &MetadataArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#metadataValue}.
 * @param ctx the parse tree
 */
fn enter_metadataValue(&mut self, _ctx: &MetadataValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#metadataValue}.
 * @param ctx the parse tree
 */
fn exit_metadataValue(&mut self, _ctx: &MetadataValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#commandSection}.
 * @param ctx the parse tree
 */
fn enter_commandSection(&mut self, _ctx: &CommandSectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#commandSection}.
 * @param ctx the parse tree
 */
fn exit_commandSection(&mut self, _ctx: &CommandSectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#multilineStringCommand}.
 * @param ctx the parse tree
 */
fn enter_multilineStringCommand(&mut self, _ctx: &MultilineStringCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#multilineStringCommand}.
 * @param ctx the parse tree
 */
fn exit_multilineStringCommand(&mut self, _ctx: &MultilineStringCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#bracedCommand}.
 * @param ctx the parse tree
 */
fn enter_bracedCommand(&mut self, _ctx: &BracedCommandContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#bracedCommand}.
 * @param ctx the parse tree
 */
fn exit_bracedCommand(&mut self, _ctx: &BracedCommandContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#workflowStatement}.
 * @param ctx the parse tree
 */
fn enter_workflowStatement(&mut self, _ctx: &WorkflowStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#workflowStatement}.
 * @param ctx the parse tree
 */
fn exit_workflowStatement(&mut self, _ctx: &WorkflowStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#conditionalStatement}.
 * @param ctx the parse tree
 */
fn enter_conditionalStatement(&mut self, _ctx: &ConditionalStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#conditionalStatement}.
 * @param ctx the parse tree
 */
fn exit_conditionalStatement(&mut self, _ctx: &ConditionalStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#conditionalElseIfClause}.
 * @param ctx the parse tree
 */
fn enter_conditionalElseIfClause(&mut self, _ctx: &ConditionalElseIfClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#conditionalElseIfClause}.
 * @param ctx the parse tree
 */
fn exit_conditionalElseIfClause(&mut self, _ctx: &ConditionalElseIfClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#conditionalElseClause}.
 * @param ctx the parse tree
 */
fn enter_conditionalElseClause(&mut self, _ctx: &ConditionalElseClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#conditionalElseClause}.
 * @param ctx the parse tree
 */
fn exit_conditionalElseClause(&mut self, _ctx: &ConditionalElseClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#scatterStatement}.
 * @param ctx the parse tree
 */
fn enter_scatterStatement(&mut self, _ctx: &ScatterStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#scatterStatement}.
 * @param ctx the parse tree
 */
fn exit_scatterStatement(&mut self, _ctx: &ScatterStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#scatterBody}.
 * @param ctx the parse tree
 */
fn enter_scatterBody(&mut self, _ctx: &ScatterBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#scatterBody}.
 * @param ctx the parse tree
 */
fn exit_scatterBody(&mut self, _ctx: &ScatterBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#callStatement}.
 * @param ctx the parse tree
 */
fn enter_callStatement(&mut self, _ctx: &CallStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#callStatement}.
 * @param ctx the parse tree
 */
fn exit_callStatement(&mut self, _ctx: &CallStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#callTarget}.
 * @param ctx the parse tree
 */
fn enter_callTarget(&mut self, _ctx: &CallTargetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#callTarget}.
 * @param ctx the parse tree
 */
fn exit_callTarget(&mut self, _ctx: &CallTargetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#callAlias}.
 * @param ctx the parse tree
 */
fn enter_callAlias(&mut self, _ctx: &CallAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#callAlias}.
 * @param ctx the parse tree
 */
fn exit_callAlias(&mut self, _ctx: &CallAliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#callAfterClause}.
 * @param ctx the parse tree
 */
fn enter_callAfterClause(&mut self, _ctx: &CallAfterClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#callAfterClause}.
 * @param ctx the parse tree
 */
fn exit_callAfterClause(&mut self, _ctx: &CallAfterClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#callInputBlock}.
 * @param ctx the parse tree
 */
fn enter_callInputBlock(&mut self, _ctx: &CallInputBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#callInputBlock}.
 * @param ctx the parse tree
 */
fn exit_callInputBlock(&mut self, _ctx: &CallInputBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#callInputItem}.
 * @param ctx the parse tree
 */
fn enter_callInputItem(&mut self, _ctx: &CallInputItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#callInputItem}.
 * @param ctx the parse tree
 */
fn exit_callInputItem(&mut self, _ctx: &CallInputItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalOrExprOperation}
 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalOrExprOperation(&mut self, _ctx: &LogicalOrExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalOrExprOperation}
 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalOrExprOperation(&mut self, _ctx: &LogicalOrExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalOrExprNone}
 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalOrExprNone(&mut self, _ctx: &LogicalOrExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalOrExprNone}
 * labeled alternative in {@link WdlV1Parser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalOrExprNone(&mut self, _ctx: &LogicalOrExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalAndExprOperation}
 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalAndExprOperation(&mut self, _ctx: &LogicalAndExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalAndExprOperation}
 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalAndExprOperation(&mut self, _ctx: &LogicalAndExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code logicalAndExprNone}
 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalAndExprNone(&mut self, _ctx: &LogicalAndExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code logicalAndExprNone}
 * labeled alternative in {@link WdlV1Parser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalAndExprNone(&mut self, _ctx: &LogicalAndExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code equalityExprOperation}
 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn enter_equalityExprOperation(&mut self, _ctx: &EqualityExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code equalityExprOperation}
 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn exit_equalityExprOperation(&mut self, _ctx: &EqualityExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code equalityExprNone}
 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn enter_equalityExprNone(&mut self, _ctx: &EqualityExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code equalityExprNone}
 * labeled alternative in {@link WdlV1Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn exit_equalityExprNone(&mut self, _ctx: &EqualityExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code comparisonExprOperation}
 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
 * @param ctx the parse tree
 */
fn enter_comparisonExprOperation(&mut self, _ctx: &ComparisonExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code comparisonExprOperation}
 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
 * @param ctx the parse tree
 */
fn exit_comparisonExprOperation(&mut self, _ctx: &ComparisonExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code comparisonExprNone}
 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
 * @param ctx the parse tree
 */
fn enter_comparisonExprNone(&mut self, _ctx: &ComparisonExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code comparisonExprNone}
 * labeled alternative in {@link WdlV1Parser#comparisonExpression}.
 * @param ctx the parse tree
 */
fn exit_comparisonExprNone(&mut self, _ctx: &ComparisonExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code additiveExprOperation}
 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn enter_additiveExprOperation(&mut self, _ctx: &AdditiveExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code additiveExprOperation}
 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn exit_additiveExprOperation(&mut self, _ctx: &AdditiveExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code additiveExprNone}
 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn enter_additiveExprNone(&mut self, _ctx: &AdditiveExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code additiveExprNone}
 * labeled alternative in {@link WdlV1Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn exit_additiveExprNone(&mut self, _ctx: &AdditiveExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multiplicativeExprOperation}
 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExprOperation(&mut self, _ctx: &MultiplicativeExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multiplicativeExprOperation}
 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExprOperation(&mut self, _ctx: &MultiplicativeExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multiplicativeExprNone}
 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExprNone(&mut self, _ctx: &MultiplicativeExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multiplicativeExprNone}
 * labeled alternative in {@link WdlV1Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExprNone(&mut self, _ctx: &MultiplicativeExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code powerExprOperation}
 * labeled alternative in {@link WdlV1Parser#powerExpression}.
 * @param ctx the parse tree
 */
fn enter_powerExprOperation(&mut self, _ctx: &PowerExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code powerExprOperation}
 * labeled alternative in {@link WdlV1Parser#powerExpression}.
 * @param ctx the parse tree
 */
fn exit_powerExprOperation(&mut self, _ctx: &PowerExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code powerExprNone}
 * labeled alternative in {@link WdlV1Parser#powerExpression}.
 * @param ctx the parse tree
 */
fn enter_powerExprNone(&mut self, _ctx: &PowerExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code powerExprNone}
 * labeled alternative in {@link WdlV1Parser#powerExpression}.
 * @param ctx the parse tree
 */
fn exit_powerExprNone(&mut self, _ctx: &PowerExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unaryExprOperation}
 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExprOperation(&mut self, _ctx: &UnaryExprOperationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryExprOperation}
 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExprOperation(&mut self, _ctx: &UnaryExprOperationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unaryExprNone}
 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExprNone(&mut self, _ctx: &UnaryExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryExprNone}
 * labeled alternative in {@link WdlV1Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExprNone(&mut self, _ctx: &UnaryExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code postfixExprField}
 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn enter_postfixExprField(&mut self, _ctx: &PostfixExprFieldContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code postfixExprField}
 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn exit_postfixExprField(&mut self, _ctx: &PostfixExprFieldContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code postfixExprArrayIndex}
 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn enter_postfixExprArrayIndex(&mut self, _ctx: &PostfixExprArrayIndexContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code postfixExprArrayIndex}
 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn exit_postfixExprArrayIndex(&mut self, _ctx: &PostfixExprArrayIndexContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code postfixExprNone}
 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn enter_postfixExprNone(&mut self, _ctx: &PostfixExprNoneContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code postfixExprNone}
 * labeled alternative in {@link WdlV1Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn exit_postfixExprNone(&mut self, _ctx: &PostfixExprNoneContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#nullLiteral}.
 * @param ctx the parse tree
 */
fn enter_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#nullLiteral}.
 * @param ctx the parse tree
 */
fn exit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#noneLiteral}.
 * @param ctx the parse tree
 */
fn enter_noneLiteral(&mut self, _ctx: &NoneLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#noneLiteral}.
 * @param ctx the parse tree
 */
fn exit_noneLiteral(&mut self, _ctx: &NoneLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numberLiteralInt}
 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
 * @param ctx the parse tree
 */
fn enter_numberLiteralInt(&mut self, _ctx: &NumberLiteralIntContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numberLiteralInt}
 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
 * @param ctx the parse tree
 */
fn exit_numberLiteralInt(&mut self, _ctx: &NumberLiteralIntContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numberLiteralFloat}
 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
 * @param ctx the parse tree
 */
fn enter_numberLiteralFloat(&mut self, _ctx: &NumberLiteralFloatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numberLiteralFloat}
 * labeled alternative in {@link WdlV1Parser#numberLiteral}.
 * @param ctx the parse tree
 */
fn exit_numberLiteralFloat(&mut self, _ctx: &NumberLiteralFloatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#numberLiteralSigned}.
 * @param ctx the parse tree
 */
fn enter_numberLiteralSigned(&mut self, _ctx: &NumberLiteralSignedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#numberLiteralSigned}.
 * @param ctx the parse tree
 */
fn exit_numberLiteralSigned(&mut self, _ctx: &NumberLiteralSignedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#arrayLiteral}.
 * @param ctx the parse tree
 */
fn enter_arrayLiteral(&mut self, _ctx: &ArrayLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#arrayLiteral}.
 * @param ctx the parse tree
 */
fn exit_arrayLiteral(&mut self, _ctx: &ArrayLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#mapLiteral}.
 * @param ctx the parse tree
 */
fn enter_mapLiteral(&mut self, _ctx: &MapLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#mapLiteral}.
 * @param ctx the parse tree
 */
fn exit_mapLiteral(&mut self, _ctx: &MapLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#mapLiteralItem}.
 * @param ctx the parse tree
 */
fn enter_mapLiteralItem(&mut self, _ctx: &MapLiteralItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#mapLiteralItem}.
 * @param ctx the parse tree
 */
fn exit_mapLiteralItem(&mut self, _ctx: &MapLiteralItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#objectLiteral}.
 * @param ctx the parse tree
 */
fn enter_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#objectLiteral}.
 * @param ctx the parse tree
 */
fn exit_objectLiteral(&mut self, _ctx: &ObjectLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#objectLiteralItem}.
 * @param ctx the parse tree
 */
fn enter_objectLiteralItem(&mut self, _ctx: &ObjectLiteralItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#objectLiteralItem}.
 * @param ctx the parse tree
 */
fn exit_objectLiteralItem(&mut self, _ctx: &ObjectLiteralItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#structLiteral}.
 * @param ctx the parse tree
 */
fn enter_structLiteral(&mut self, _ctx: &StructLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#structLiteral}.
 * @param ctx the parse tree
 */
fn exit_structLiteral(&mut self, _ctx: &StructLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#structLiteralItem}.
 * @param ctx the parse tree
 */
fn enter_structLiteralItem(&mut self, _ctx: &StructLiteralItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#structLiteralItem}.
 * @param ctx the parse tree
 */
fn exit_structLiteralItem(&mut self, _ctx: &StructLiteralItemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#pairLiteral}.
 * @param ctx the parse tree
 */
fn enter_pairLiteral(&mut self, _ctx: &PairLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#pairLiteral}.
 * @param ctx the parse tree
 */
fn exit_pairLiteral(&mut self, _ctx: &PairLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#groupedExpression}.
 * @param ctx the parse tree
 */
fn enter_groupedExpression(&mut self, _ctx: &GroupedExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#groupedExpression}.
 * @param ctx the parse tree
 */
fn exit_groupedExpression(&mut self, _ctx: &GroupedExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#ifExpression}.
 * @param ctx the parse tree
 */
fn enter_ifExpression(&mut self, _ctx: &IfExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#ifExpression}.
 * @param ctx the parse tree
 */
fn exit_ifExpression(&mut self, _ctx: &IfExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#callExpression}.
 * @param ctx the parse tree
 */
fn enter_callExpression(&mut self, _ctx: &CallExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#callExpression}.
 * @param ctx the parse tree
 */
fn exit_callExpression(&mut self, _ctx: &CallExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#stringLiteral}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#stringLiteral}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#quotedString}.
 * @param ctx the parse tree
 */
fn enter_quotedString(&mut self, _ctx: &QuotedStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#quotedString}.
 * @param ctx the parse tree
 */
fn exit_quotedString(&mut self, _ctx: &QuotedStringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringElementText}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn enter_stringElementText(&mut self, _ctx: &StringElementTextContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringElementText}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn exit_stringElementText(&mut self, _ctx: &StringElementTextContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringElementEscape}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn enter_stringElementEscape(&mut self, _ctx: &StringElementEscapeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringElementEscape}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn exit_stringElementEscape(&mut self, _ctx: &StringElementEscapeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringElementDollarSign}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn enter_stringElementDollarSign(&mut self, _ctx: &StringElementDollarSignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringElementDollarSign}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn exit_stringElementDollarSign(&mut self, _ctx: &StringElementDollarSignContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringElementTilde}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn enter_stringElementTilde(&mut self, _ctx: &StringElementTildeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringElementTilde}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn exit_stringElementTilde(&mut self, _ctx: &StringElementTildeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringElementPlaceholder}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn enter_stringElementPlaceholder(&mut self, _ctx: &StringElementPlaceholderContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringElementPlaceholder}
 * labeled alternative in {@link WdlV1Parser#stringElement}.
 * @param ctx the parse tree
 */
fn exit_stringElementPlaceholder(&mut self, _ctx: &StringElementPlaceholderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#stringPlaceholder}.
 * @param ctx the parse tree
 */
fn enter_stringPlaceholder(&mut self, _ctx: &StringPlaceholderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#stringPlaceholder}.
 * @param ctx the parse tree
 */
fn exit_stringPlaceholder(&mut self, _ctx: &StringPlaceholderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#multilineString}.
 * @param ctx the parse tree
 */
fn enter_multilineString(&mut self, _ctx: &MultilineStringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#multilineString}.
 * @param ctx the parse tree
 */
fn exit_multilineString(&mut self, _ctx: &MultilineStringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multilineStringElementText}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_multilineStringElementText(&mut self, _ctx: &MultilineStringElementTextContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multilineStringElementText}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_multilineStringElementText(&mut self, _ctx: &MultilineStringElementTextContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multilineStringElementEscape}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_multilineStringElementEscape(&mut self, _ctx: &MultilineStringElementEscapeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multilineStringElementEscape}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_multilineStringElementEscape(&mut self, _ctx: &MultilineStringElementEscapeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multilineStringElementDoubleCloseAngle}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_multilineStringElementDoubleCloseAngle(&mut self, _ctx: &MultilineStringElementDoubleCloseAngleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multilineStringElementDoubleCloseAngle}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_multilineStringElementDoubleCloseAngle(&mut self, _ctx: &MultilineStringElementDoubleCloseAngleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multilineStringElementSingleCloseAngle}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_multilineStringElementSingleCloseAngle(&mut self, _ctx: &MultilineStringElementSingleCloseAngleContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multilineStringElementSingleCloseAngle}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_multilineStringElementSingleCloseAngle(&mut self, _ctx: &MultilineStringElementSingleCloseAngleContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multilineStringElementDollarSign}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_multilineStringElementDollarSign(&mut self, _ctx: &MultilineStringElementDollarSignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multilineStringElementDollarSign}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_multilineStringElementDollarSign(&mut self, _ctx: &MultilineStringElementDollarSignContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multilineStringElementTilde}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_multilineStringElementTilde(&mut self, _ctx: &MultilineStringElementTildeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multilineStringElementTilde}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_multilineStringElementTilde(&mut self, _ctx: &MultilineStringElementTildeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multilineStringElementPlaceholder}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn enter_multilineStringElementPlaceholder(&mut self, _ctx: &MultilineStringElementPlaceholderContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multilineStringElementPlaceholder}
 * labeled alternative in {@link WdlV1Parser#multilineStringElement}.
 * @param ctx the parse tree
 */
fn exit_multilineStringElementPlaceholder(&mut self, _ctx: &MultilineStringElementPlaceholderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#multilineStringPlaceholder}.
 * @param ctx the parse tree
 */
fn enter_multilineStringPlaceholder(&mut self, _ctx: &MultilineStringPlaceholderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#multilineStringPlaceholder}.
 * @param ctx the parse tree
 */
fn exit_multilineStringPlaceholder(&mut self, _ctx: &MultilineStringPlaceholderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#stringPlaceholderExpression}.
 * @param ctx the parse tree
 */
fn enter_stringPlaceholderExpression(&mut self, _ctx: &StringPlaceholderExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#stringPlaceholderExpression}.
 * @param ctx the parse tree
 */
fn exit_stringPlaceholderExpression(&mut self, _ctx: &StringPlaceholderExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringPlaceholderOptionSepDefault}
 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
 * @param ctx the parse tree
 */
fn enter_stringPlaceholderOptionSepDefault(&mut self, _ctx: &StringPlaceholderOptionSepDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringPlaceholderOptionSepDefault}
 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
 * @param ctx the parse tree
 */
fn exit_stringPlaceholderOptionSepDefault(&mut self, _ctx: &StringPlaceholderOptionSepDefaultContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringPlaceholderOptionTrueFalse}
 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
 * @param ctx the parse tree
 */
fn enter_stringPlaceholderOptionTrueFalse(&mut self, _ctx: &StringPlaceholderOptionTrueFalseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringPlaceholderOptionTrueFalse}
 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
 * @param ctx the parse tree
 */
fn exit_stringPlaceholderOptionTrueFalse(&mut self, _ctx: &StringPlaceholderOptionTrueFalseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringPlaceholderOptionFalseTrue}
 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
 * @param ctx the parse tree
 */
fn enter_stringPlaceholderOptionFalseTrue(&mut self, _ctx: &StringPlaceholderOptionFalseTrueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringPlaceholderOptionFalseTrue}
 * labeled alternative in {@link WdlV1Parser#stringPlaceholderOption}.
 * @param ctx the parse tree
 */
fn exit_stringPlaceholderOptionFalseTrue(&mut self, _ctx: &StringPlaceholderOptionFalseTrueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn enter_strictIdentifier(&mut self, _ctx: &StrictIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#strictIdentifier}.
 * @param ctx the parse tree
 */
fn exit_strictIdentifier(&mut self, _ctx: &StrictIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#dottedIdentifier}.
 * @param ctx the parse tree
 */
fn enter_dottedIdentifier(&mut self, _ctx: &DottedIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#dottedIdentifier}.
 * @param ctx the parse tree
 */
fn exit_dottedIdentifier(&mut self, _ctx: &DottedIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link WdlV1Parser#anyIdentBase}.
 * @param ctx the parse tree
 */
fn enter_anyIdentBase(&mut self, _ctx: &AnyIdentBaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link WdlV1Parser#anyIdentBase}.
 * @param ctx the parse tree
 */
fn exit_anyIdentBase(&mut self, _ctx: &AnyIdentBaseContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : WdlV1ParserListener<'input> }


