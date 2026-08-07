// Generated from ../wdl-grammar/antrl4/v1/WdlV1Parser.g4 by ANTLR 4.13.2

use super::wdlv1parser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by WdlV1Parser.

pub trait WdlV1ParserBaseListener<'input>:
    ParseTreeListener<'input, WdlV1ParserContextType> {

    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_document(&mut self, _ctx: &DocumentContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_document(&mut self, _ctx: &DocumentContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_versionstatement(&mut self, _ctx: &VersionStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_versionstatement(&mut self, _ctx: &VersionStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_documentelement(&mut self, _ctx: &DocumentElementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_documentelement(&mut self, _ctx: &DocumentElementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importstatementstandard(&mut self, _ctx: &ImportStatementStandardContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importstatementstandard(&mut self, _ctx: &ImportStatementStandardContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importstatementstar(&mut self, _ctx: &ImportStatementStarContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importstatementstar(&mut self, _ctx: &ImportStatementStarContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importstatementmembers(&mut self, _ctx: &ImportStatementMembersContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importstatementmembers(&mut self, _ctx: &ImportStatementMembersContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importmembers(&mut self, _ctx: &ImportMembersContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importmembers(&mut self, _ctx: &ImportMembersContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importmember(&mut self, _ctx: &ImportMemberContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importmember(&mut self, _ctx: &ImportMemberContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importuriliteral(&mut self, _ctx: &ImportUriLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importuriliteral(&mut self, _ctx: &ImportUriLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importurielement(&mut self, _ctx: &ImportUriElementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importurielement(&mut self, _ctx: &ImportUriElementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_importalias(&mut self, _ctx: &ImportAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_importalias(&mut self, _ctx: &ImportAliasContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structdefinition(&mut self, _ctx: &StructDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structdefinition(&mut self, _ctx: &StructDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structitemmetadata(&mut self, _ctx: &StructItemMetadataContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structitemmetadata(&mut self, _ctx: &StructItemMetadataContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structitemparametermetadata(&mut self, _ctx: &StructItemParameterMetadataContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structitemparametermetadata(&mut self, _ctx: &StructItemParameterMetadataContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structitemmemberdeclaration(&mut self, _ctx: &StructItemMemberDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structitemmemberdeclaration(&mut self, _ctx: &StructItemMemberDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structdeclaration(&mut self, _ctx: &StructDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structdeclaration(&mut self, _ctx: &StructDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumdefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumdefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumtypeparameter(&mut self, _ctx: &EnumTypeParameterContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumtypeparameter(&mut self, _ctx: &EnumTypeParameterContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumchoice(&mut self, _ctx: &EnumChoiceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumchoice(&mut self, _ctx: &EnumChoiceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumliteralexpression(&mut self, _ctx: &EnumLiteralExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumliteralexpression(&mut self, _ctx: &EnumLiteralExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumstringliteral(&mut self, _ctx: &EnumStringLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumstringliteral(&mut self, _ctx: &EnumStringLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumquotedstring(&mut self, _ctx: &EnumQuotedStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumquotedstring(&mut self, _ctx: &EnumQuotedStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumstringelement(&mut self, _ctx: &EnumStringElementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumstringelement(&mut self, _ctx: &EnumStringElementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enummultilinestring(&mut self, _ctx: &EnumMultilineStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enummultilinestring(&mut self, _ctx: &EnumMultilineStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enummultilinestringelement(&mut self, _ctx: &EnumMultilineStringElementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enummultilinestringelement(&mut self, _ctx: &EnumMultilineStringElementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumarrayliteral(&mut self, _ctx: &EnumArrayLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumarrayliteral(&mut self, _ctx: &EnumArrayLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enummapliteral(&mut self, _ctx: &EnumMapLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enummapliteral(&mut self, _ctx: &EnumMapLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enummapliteralitem(&mut self, _ctx: &EnumMapLiteralItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enummapliteralitem(&mut self, _ctx: &EnumMapLiteralItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumobjectliteral(&mut self, _ctx: &EnumObjectLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumobjectliteral(&mut self, _ctx: &EnumObjectLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumobjectliteralitem(&mut self, _ctx: &EnumObjectLiteralItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumobjectliteralitem(&mut self, _ctx: &EnumObjectLiteralItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumstructliteral(&mut self, _ctx: &EnumStructLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumstructliteral(&mut self, _ctx: &EnumStructLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumstructliteralitem(&mut self, _ctx: &EnumStructLiteralItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumstructliteralitem(&mut self, _ctx: &EnumStructLiteralItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumpairliteral(&mut self, _ctx: &EnumPairLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumpairliteral(&mut self, _ctx: &EnumPairLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskdefinition(&mut self, _ctx: &TaskDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskdefinition(&mut self, _ctx: &TaskDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowdefinition(&mut self, _ctx: &WorkflowDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowdefinition(&mut self, _ctx: &WorkflowDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_type(&mut self, _ctx: &TypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_type(&mut self, _ctx: &TypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_maptype(&mut self, _ctx: &MapTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_maptype(&mut self, _ctx: &MapTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_arraytype(&mut self, _ctx: &ArrayTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_arraytype(&mut self, _ctx: &ArrayTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pairtype(&mut self, _ctx: &PairTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pairtype(&mut self, _ctx: &PairTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_objecttype(&mut self, _ctx: &ObjectTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_objecttype(&mut self, _ctx: &ObjectTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_primitivetype(&mut self, _ctx: &PrimitiveTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_primitivetype(&mut self, _ctx: &PrimitiveTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typereftype(&mut self, _ctx: &TypeRefTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typereftype(&mut self, _ctx: &TypeRefTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unbounddeclaration(&mut self, _ctx: &UnboundDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unbounddeclaration(&mut self, _ctx: &UnboundDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bounddeclaration(&mut self, _ctx: &BoundDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bounddeclaration(&mut self, _ctx: &BoundDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskinputsection(&mut self, _ctx: &TaskInputSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskinputsection(&mut self, _ctx: &TaskInputSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskcommandsection(&mut self, _ctx: &TaskCommandSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskcommandsection(&mut self, _ctx: &TaskCommandSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskoutputsection(&mut self, _ctx: &TaskOutputSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskoutputsection(&mut self, _ctx: &TaskOutputSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskruntimesection(&mut self, _ctx: &TaskRuntimeSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskruntimesection(&mut self, _ctx: &TaskRuntimeSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskrequirementssection(&mut self, _ctx: &TaskRequirementsSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskrequirementssection(&mut self, _ctx: &TaskRequirementsSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskhintssection(&mut self, _ctx: &TaskHintsSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskhintssection(&mut self, _ctx: &TaskHintsSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskmetadatasection(&mut self, _ctx: &TaskMetadataSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskmetadatasection(&mut self, _ctx: &TaskMetadataSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskparametermetadatasection(&mut self, _ctx: &TaskParameterMetadataSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskparametermetadatasection(&mut self, _ctx: &TaskParameterMetadataSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskdeclaration(&mut self, _ctx: &TaskDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskdeclaration(&mut self, _ctx: &TaskDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowinputsection(&mut self, _ctx: &WorkflowInputSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowinputsection(&mut self, _ctx: &WorkflowInputSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowoutputsection(&mut self, _ctx: &WorkflowOutputSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowoutputsection(&mut self, _ctx: &WorkflowOutputSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowhintssection(&mut self, _ctx: &WorkflowHintsSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowhintssection(&mut self, _ctx: &WorkflowHintsSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowconditionalstatement(&mut self, _ctx: &WorkflowConditionalStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowconditionalstatement(&mut self, _ctx: &WorkflowConditionalStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowscatterstatement(&mut self, _ctx: &WorkflowScatterStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowscatterstatement(&mut self, _ctx: &WorkflowScatterStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowcallstatement(&mut self, _ctx: &WorkflowCallStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowcallstatement(&mut self, _ctx: &WorkflowCallStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowmetadatasection(&mut self, _ctx: &WorkflowMetadataSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowmetadatasection(&mut self, _ctx: &WorkflowMetadataSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowparametermetadatasection(&mut self, _ctx: &WorkflowParameterMetadataSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowparametermetadatasection(&mut self, _ctx: &WorkflowParameterMetadataSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowdeclaration(&mut self, _ctx: &WorkflowDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowdeclaration(&mut self, _ctx: &WorkflowDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inputsection(&mut self, _ctx: &InputSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inputsection(&mut self, _ctx: &InputSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_outputsection(&mut self, _ctx: &OutputSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_outputsection(&mut self, _ctx: &OutputSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_runtimesection(&mut self, _ctx: &RuntimeSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_runtimesection(&mut self, _ctx: &RuntimeSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_runtimeitem(&mut self, _ctx: &RuntimeItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_runtimeitem(&mut self, _ctx: &RuntimeItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_requirementssection(&mut self, _ctx: &RequirementsSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_requirementssection(&mut self, _ctx: &RequirementsSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_requirementsitem(&mut self, _ctx: &RequirementsItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_requirementsitem(&mut self, _ctx: &RequirementsItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintssectiontask(&mut self, _ctx: &HintsSectionTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintssectiontask(&mut self, _ctx: &HintsSectionTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintsitemtask(&mut self, _ctx: &HintsItemTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintsitemtask(&mut self, _ctx: &HintsItemTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskhintvalueexpression(&mut self, _ctx: &TaskHintValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskhintvalueexpression(&mut self, _ctx: &TaskHintValueExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskhintvaluehintsobject(&mut self, _ctx: &TaskHintValueHintsObjectContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskhintvaluehintsobject(&mut self, _ctx: &TaskHintValueHintsObjectContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskhintvalueinputobject(&mut self, _ctx: &TaskHintValueInputObjectContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskhintvalueinputobject(&mut self, _ctx: &TaskHintValueInputObjectContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskhintvalueoutputobject(&mut self, _ctx: &TaskHintValueOutputObjectContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskhintvalueoutputobject(&mut self, _ctx: &TaskHintValueOutputObjectContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskhintvaluearray(&mut self, _ctx: &TaskHintValueArrayContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskhintvaluearray(&mut self, _ctx: &TaskHintValueArrayContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintstypedobjecttask(&mut self, _ctx: &HintsTypedObjectTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintstypedobjecttask(&mut self, _ctx: &HintsTypedObjectTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintsobjectitemtask(&mut self, _ctx: &HintsObjectItemTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintsobjectitemtask(&mut self, _ctx: &HintsObjectItemTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inputhintsobjecttask(&mut self, _ctx: &InputHintsObjectTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inputhintsobjecttask(&mut self, _ctx: &InputHintsObjectTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inputhintsitemtask(&mut self, _ctx: &InputHintsItemTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inputhintsitemtask(&mut self, _ctx: &InputHintsItemTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_outputhintsobjecttask(&mut self, _ctx: &OutputHintsObjectTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_outputhintsobjecttask(&mut self, _ctx: &OutputHintsObjectTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_outputhintsitemtask(&mut self, _ctx: &OutputHintsItemTaskContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_outputhintsitemtask(&mut self, _ctx: &OutputHintsItemTaskContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_taskhintsarray(&mut self, _ctx: &TaskHintsArrayContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_taskhintsarray(&mut self, _ctx: &TaskHintsArrayContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintssectionworkflow(&mut self, _ctx: &HintsSectionWorkflowContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintssectionworkflow(&mut self, _ctx: &HintsSectionWorkflowContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintsitemworkflow(&mut self, _ctx: &HintsItemWorkflowContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintsitemworkflow(&mut self, _ctx: &HintsItemWorkflowContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowhintvaluenumber(&mut self, _ctx: &WorkflowHintValueNumberContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowhintvaluenumber(&mut self, _ctx: &WorkflowHintValueNumberContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowhintvaluestring(&mut self, _ctx: &WorkflowHintValueStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowhintvaluestring(&mut self, _ctx: &WorkflowHintValueStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowhintvalueboolean(&mut self, _ctx: &WorkflowHintValueBooleanContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowhintvalueboolean(&mut self, _ctx: &WorkflowHintValueBooleanContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowhintvalueobject(&mut self, _ctx: &WorkflowHintValueObjectContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowhintvalueobject(&mut self, _ctx: &WorkflowHintValueObjectContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowhintvaluearray(&mut self, _ctx: &WorkflowHintValueArrayContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowhintvaluearray(&mut self, _ctx: &WorkflowHintValueArrayContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintsobjectworkflow(&mut self, _ctx: &HintsObjectWorkflowContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintsobjectworkflow(&mut self, _ctx: &HintsObjectWorkflowContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hintsobjectitemworkflow(&mut self, _ctx: &HintsObjectItemWorkflowContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hintsobjectitemworkflow(&mut self, _ctx: &HintsObjectItemWorkflowContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowhintsarray(&mut self, _ctx: &WorkflowHintsArrayContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowhintsarray(&mut self, _ctx: &WorkflowHintsArrayContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_metadatasection(&mut self, _ctx: &MetadataSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_metadatasection(&mut self, _ctx: &MetadataSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parametermetadatasection(&mut self, _ctx: &ParameterMetadataSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parametermetadatasection(&mut self, _ctx: &ParameterMetadataSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_metadataobject(&mut self, _ctx: &MetadataObjectContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_metadataobject(&mut self, _ctx: &MetadataObjectContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_metadataobjectitem(&mut self, _ctx: &MetadataObjectItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_metadataobjectitem(&mut self, _ctx: &MetadataObjectItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_metadataarray(&mut self, _ctx: &MetadataArrayContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_metadataarray(&mut self, _ctx: &MetadataArrayContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_metadatavalue(&mut self, _ctx: &MetadataValueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_metadatavalue(&mut self, _ctx: &MetadataValueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_commandsection(&mut self, _ctx: &CommandSectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_commandsection(&mut self, _ctx: &CommandSectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringcommand(&mut self, _ctx: &MultilineStringCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringcommand(&mut self, _ctx: &MultilineStringCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_bracedcommand(&mut self, _ctx: &BracedCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_bracedcommand(&mut self, _ctx: &BracedCommandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_workflowstatement(&mut self, _ctx: &WorkflowStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_workflowstatement(&mut self, _ctx: &WorkflowStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_conditionalstatement(&mut self, _ctx: &ConditionalStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_conditionalstatement(&mut self, _ctx: &ConditionalStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_conditionalelseifclause(&mut self, _ctx: &ConditionalElseIfClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_conditionalelseifclause(&mut self, _ctx: &ConditionalElseIfClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_conditionalelseclause(&mut self, _ctx: &ConditionalElseClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_conditionalelseclause(&mut self, _ctx: &ConditionalElseClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_scatterstatement(&mut self, _ctx: &ScatterStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_scatterstatement(&mut self, _ctx: &ScatterStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_scatterbody(&mut self, _ctx: &ScatterBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_scatterbody(&mut self, _ctx: &ScatterBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callstatement(&mut self, _ctx: &CallStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callstatement(&mut self, _ctx: &CallStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_calltarget(&mut self, _ctx: &CallTargetContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_calltarget(&mut self, _ctx: &CallTargetContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callalias(&mut self, _ctx: &CallAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callalias(&mut self, _ctx: &CallAliasContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callafterclause(&mut self, _ctx: &CallAfterClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callafterclause(&mut self, _ctx: &CallAfterClauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callinputblock(&mut self, _ctx: &CallInputBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callinputblock(&mut self, _ctx: &CallInputBlockContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callinputitem(&mut self, _ctx: &CallInputItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callinputitem(&mut self, _ctx: &CallInputItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_logicalorexproperation(&mut self, _ctx: &LogicalOrExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_logicalorexproperation(&mut self, _ctx: &LogicalOrExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_logicalorexprnone(&mut self, _ctx: &LogicalOrExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_logicalorexprnone(&mut self, _ctx: &LogicalOrExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_logicalandexproperation(&mut self, _ctx: &LogicalAndExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_logicalandexproperation(&mut self, _ctx: &LogicalAndExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_logicalandexprnone(&mut self, _ctx: &LogicalAndExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_logicalandexprnone(&mut self, _ctx: &LogicalAndExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_equalityexproperation(&mut self, _ctx: &EqualityExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_equalityexproperation(&mut self, _ctx: &EqualityExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_equalityexprnone(&mut self, _ctx: &EqualityExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_equalityexprnone(&mut self, _ctx: &EqualityExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comparisonexproperation(&mut self, _ctx: &ComparisonExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comparisonexproperation(&mut self, _ctx: &ComparisonExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comparisonexprnone(&mut self, _ctx: &ComparisonExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comparisonexprnone(&mut self, _ctx: &ComparisonExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_additiveexproperation(&mut self, _ctx: &AdditiveExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_additiveexproperation(&mut self, _ctx: &AdditiveExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_additiveexprnone(&mut self, _ctx: &AdditiveExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_additiveexprnone(&mut self, _ctx: &AdditiveExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multiplicativeexproperation(&mut self, _ctx: &MultiplicativeExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multiplicativeexproperation(&mut self, _ctx: &MultiplicativeExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multiplicativeexprnone(&mut self, _ctx: &MultiplicativeExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multiplicativeexprnone(&mut self, _ctx: &MultiplicativeExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_powerexproperation(&mut self, _ctx: &PowerExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_powerexproperation(&mut self, _ctx: &PowerExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_powerexprnone(&mut self, _ctx: &PowerExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_powerexprnone(&mut self, _ctx: &PowerExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unaryexproperation(&mut self, _ctx: &UnaryExprOperationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unaryexproperation(&mut self, _ctx: &UnaryExprOperationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unaryexprnone(&mut self, _ctx: &UnaryExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unaryexprnone(&mut self, _ctx: &UnaryExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_postfixexprfield(&mut self, _ctx: &PostfixExprFieldContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_postfixexprfield(&mut self, _ctx: &PostfixExprFieldContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_postfixexprarrayindex(&mut self, _ctx: &PostfixExprArrayIndexContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_postfixexprarrayindex(&mut self, _ctx: &PostfixExprArrayIndexContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_postfixexprnone(&mut self, _ctx: &PostfixExprNoneContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_postfixexprnone(&mut self, _ctx: &PostfixExprNoneContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_primaryexpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_primaryexpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nullliteral(&mut self, _ctx: &NullLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nullliteral(&mut self, _ctx: &NullLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_noneliteral(&mut self, _ctx: &NoneLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_noneliteral(&mut self, _ctx: &NoneLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_booleanliteral(&mut self, _ctx: &BooleanLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_booleanliteral(&mut self, _ctx: &BooleanLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numberliteralint(&mut self, _ctx: &NumberLiteralIntContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numberliteralint(&mut self, _ctx: &NumberLiteralIntContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numberliteralfloat(&mut self, _ctx: &NumberLiteralFloatContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numberliteralfloat(&mut self, _ctx: &NumberLiteralFloatContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numberliteralsigned(&mut self, _ctx: &NumberLiteralSignedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numberliteralsigned(&mut self, _ctx: &NumberLiteralSignedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_arrayliteral(&mut self, _ctx: &ArrayLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_arrayliteral(&mut self, _ctx: &ArrayLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mapliteral(&mut self, _ctx: &MapLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mapliteral(&mut self, _ctx: &MapLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mapliteralitem(&mut self, _ctx: &MapLiteralItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mapliteralitem(&mut self, _ctx: &MapLiteralItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_objectliteral(&mut self, _ctx: &ObjectLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_objectliteral(&mut self, _ctx: &ObjectLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_objectliteralitem(&mut self, _ctx: &ObjectLiteralItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_objectliteralitem(&mut self, _ctx: &ObjectLiteralItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structliteral(&mut self, _ctx: &StructLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structliteral(&mut self, _ctx: &StructLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structliteralitem(&mut self, _ctx: &StructLiteralItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structliteralitem(&mut self, _ctx: &StructLiteralItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pairliteral(&mut self, _ctx: &PairLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pairliteral(&mut self, _ctx: &PairLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_groupedexpression(&mut self, _ctx: &GroupedExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_groupedexpression(&mut self, _ctx: &GroupedExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ifexpression(&mut self, _ctx: &IfExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ifexpression(&mut self, _ctx: &IfExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_callexpression(&mut self, _ctx: &CallExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_callexpression(&mut self, _ctx: &CallExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringliteral(&mut self, _ctx: &StringLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringliteral(&mut self, _ctx: &StringLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_quotedstring(&mut self, _ctx: &QuotedStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_quotedstring(&mut self, _ctx: &QuotedStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringelementtext(&mut self, _ctx: &StringElementTextContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringelementtext(&mut self, _ctx: &StringElementTextContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringelementescape(&mut self, _ctx: &StringElementEscapeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringelementescape(&mut self, _ctx: &StringElementEscapeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringelementdollarsign(&mut self, _ctx: &StringElementDollarSignContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringelementdollarsign(&mut self, _ctx: &StringElementDollarSignContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringelementtilde(&mut self, _ctx: &StringElementTildeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringelementtilde(&mut self, _ctx: &StringElementTildeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringelementplaceholder(&mut self, _ctx: &StringElementPlaceholderContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringelementplaceholder(&mut self, _ctx: &StringElementPlaceholderContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringplaceholder(&mut self, _ctx: &StringPlaceholderContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringplaceholder(&mut self, _ctx: &StringPlaceholderContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestring(&mut self, _ctx: &MultilineStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestring(&mut self, _ctx: &MultilineStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringelementtext(&mut self, _ctx: &MultilineStringElementTextContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringelementtext(&mut self, _ctx: &MultilineStringElementTextContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringelementescape(&mut self, _ctx: &MultilineStringElementEscapeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringelementescape(&mut self, _ctx: &MultilineStringElementEscapeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringelementdoublecloseangle(&mut self, _ctx: &MultilineStringElementDoubleCloseAngleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringelementdoublecloseangle(&mut self, _ctx: &MultilineStringElementDoubleCloseAngleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringelementsinglecloseangle(&mut self, _ctx: &MultilineStringElementSingleCloseAngleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringelementsinglecloseangle(&mut self, _ctx: &MultilineStringElementSingleCloseAngleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringelementdollarsign(&mut self, _ctx: &MultilineStringElementDollarSignContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringelementdollarsign(&mut self, _ctx: &MultilineStringElementDollarSignContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringelementtilde(&mut self, _ctx: &MultilineStringElementTildeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringelementtilde(&mut self, _ctx: &MultilineStringElementTildeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringelementplaceholder(&mut self, _ctx: &MultilineStringElementPlaceholderContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringelementplaceholder(&mut self, _ctx: &MultilineStringElementPlaceholderContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinestringplaceholder(&mut self, _ctx: &MultilineStringPlaceholderContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinestringplaceholder(&mut self, _ctx: &MultilineStringPlaceholderContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringplaceholderexpression(&mut self, _ctx: &StringPlaceholderExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringplaceholderexpression(&mut self, _ctx: &StringPlaceholderExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringplaceholderoptionsepdefault(&mut self, _ctx: &StringPlaceholderOptionSepDefaultContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringplaceholderoptionsepdefault(&mut self, _ctx: &StringPlaceholderOptionSepDefaultContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringplaceholderoptiontruefalse(&mut self, _ctx: &StringPlaceholderOptionTrueFalseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringplaceholderoptiontruefalse(&mut self, _ctx: &StringPlaceholderOptionTrueFalseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringplaceholderoptionfalsetrue(&mut self, _ctx: &StringPlaceholderOptionFalseTrueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringplaceholderoptionfalsetrue(&mut self, _ctx: &StringPlaceholderOptionFalseTrueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_strictidentifier(&mut self, _ctx: &StrictIdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_strictidentifier(&mut self, _ctx: &StrictIdentifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dottedidentifier(&mut self, _ctx: &DottedIdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dottedidentifier(&mut self, _ctx: &DottedIdentifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_anyidentbase(&mut self, _ctx: &AnyIdentBaseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  WdlV1ParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_anyidentbase(&mut self, _ctx: &AnyIdentBaseContext<'input>) {}


}