# Generated from antlr4/v1/WdlV1Parser.g4 by ANTLR 4.13.2
from antlr4 import *
if "." in __name__:
    from .WdlV1Parser import WdlV1Parser
else:
    from WdlV1Parser import WdlV1Parser

# This class defines a complete generic visitor for a parse tree produced by WdlV1Parser.

class WdlV1ParserVisitor(ParseTreeVisitor):

    # Visit a parse tree produced by WdlV1Parser#document.
    def visitDocument(self, ctx:WdlV1Parser.DocumentContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#versionStatement.
    def visitVersionStatement(self, ctx:WdlV1Parser.VersionStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#documentElement.
    def visitDocumentElement(self, ctx:WdlV1Parser.DocumentElementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importStatementStandard.
    def visitImportStatementStandard(self, ctx:WdlV1Parser.ImportStatementStandardContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importStatementStar.
    def visitImportStatementStar(self, ctx:WdlV1Parser.ImportStatementStarContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importStatementMembers.
    def visitImportStatementMembers(self, ctx:WdlV1Parser.ImportStatementMembersContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importMembers.
    def visitImportMembers(self, ctx:WdlV1Parser.ImportMembersContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importMember.
    def visitImportMember(self, ctx:WdlV1Parser.ImportMemberContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importUriLiteral.
    def visitImportUriLiteral(self, ctx:WdlV1Parser.ImportUriLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importUriElement.
    def visitImportUriElement(self, ctx:WdlV1Parser.ImportUriElementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#importAlias.
    def visitImportAlias(self, ctx:WdlV1Parser.ImportAliasContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#structDefinition.
    def visitStructDefinition(self, ctx:WdlV1Parser.StructDefinitionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#structItemMetadata.
    def visitStructItemMetadata(self, ctx:WdlV1Parser.StructItemMetadataContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#structItemParameterMetadata.
    def visitStructItemParameterMetadata(self, ctx:WdlV1Parser.StructItemParameterMetadataContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#structItemMemberDeclaration.
    def visitStructItemMemberDeclaration(self, ctx:WdlV1Parser.StructItemMemberDeclarationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#structDeclaration.
    def visitStructDeclaration(self, ctx:WdlV1Parser.StructDeclarationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumDefinition.
    def visitEnumDefinition(self, ctx:WdlV1Parser.EnumDefinitionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumTypeParameter.
    def visitEnumTypeParameter(self, ctx:WdlV1Parser.EnumTypeParameterContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumChoice.
    def visitEnumChoice(self, ctx:WdlV1Parser.EnumChoiceContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumLiteralExpression.
    def visitEnumLiteralExpression(self, ctx:WdlV1Parser.EnumLiteralExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumStringLiteral.
    def visitEnumStringLiteral(self, ctx:WdlV1Parser.EnumStringLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumQuotedString.
    def visitEnumQuotedString(self, ctx:WdlV1Parser.EnumQuotedStringContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumStringElement.
    def visitEnumStringElement(self, ctx:WdlV1Parser.EnumStringElementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumMultilineString.
    def visitEnumMultilineString(self, ctx:WdlV1Parser.EnumMultilineStringContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumMultilineStringElement.
    def visitEnumMultilineStringElement(self, ctx:WdlV1Parser.EnumMultilineStringElementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumArrayLiteral.
    def visitEnumArrayLiteral(self, ctx:WdlV1Parser.EnumArrayLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumMapLiteral.
    def visitEnumMapLiteral(self, ctx:WdlV1Parser.EnumMapLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumMapLiteralItem.
    def visitEnumMapLiteralItem(self, ctx:WdlV1Parser.EnumMapLiteralItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumObjectLiteral.
    def visitEnumObjectLiteral(self, ctx:WdlV1Parser.EnumObjectLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumObjectLiteralItem.
    def visitEnumObjectLiteralItem(self, ctx:WdlV1Parser.EnumObjectLiteralItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumStructLiteral.
    def visitEnumStructLiteral(self, ctx:WdlV1Parser.EnumStructLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumStructLiteralItem.
    def visitEnumStructLiteralItem(self, ctx:WdlV1Parser.EnumStructLiteralItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#enumPairLiteral.
    def visitEnumPairLiteral(self, ctx:WdlV1Parser.EnumPairLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskDefinition.
    def visitTaskDefinition(self, ctx:WdlV1Parser.TaskDefinitionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowDefinition.
    def visitWorkflowDefinition(self, ctx:WdlV1Parser.WorkflowDefinitionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#type.
    def visitType(self, ctx:WdlV1Parser.TypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#mapType.
    def visitMapType(self, ctx:WdlV1Parser.MapTypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#arrayType.
    def visitArrayType(self, ctx:WdlV1Parser.ArrayTypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#pairType.
    def visitPairType(self, ctx:WdlV1Parser.PairTypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#objectType.
    def visitObjectType(self, ctx:WdlV1Parser.ObjectTypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#primitiveType.
    def visitPrimitiveType(self, ctx:WdlV1Parser.PrimitiveTypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#typeRefType.
    def visitTypeRefType(self, ctx:WdlV1Parser.TypeRefTypeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#unboundDeclaration.
    def visitUnboundDeclaration(self, ctx:WdlV1Parser.UnboundDeclarationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#boundDeclaration.
    def visitBoundDeclaration(self, ctx:WdlV1Parser.BoundDeclarationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#declaration.
    def visitDeclaration(self, ctx:WdlV1Parser.DeclarationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskInputSection.
    def visitTaskInputSection(self, ctx:WdlV1Parser.TaskInputSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskCommandSection.
    def visitTaskCommandSection(self, ctx:WdlV1Parser.TaskCommandSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskOutputSection.
    def visitTaskOutputSection(self, ctx:WdlV1Parser.TaskOutputSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskRuntimeSection.
    def visitTaskRuntimeSection(self, ctx:WdlV1Parser.TaskRuntimeSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskRequirementsSection.
    def visitTaskRequirementsSection(self, ctx:WdlV1Parser.TaskRequirementsSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskHintsSection.
    def visitTaskHintsSection(self, ctx:WdlV1Parser.TaskHintsSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskMetadataSection.
    def visitTaskMetadataSection(self, ctx:WdlV1Parser.TaskMetadataSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskParameterMetadataSection.
    def visitTaskParameterMetadataSection(self, ctx:WdlV1Parser.TaskParameterMetadataSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskDeclaration.
    def visitTaskDeclaration(self, ctx:WdlV1Parser.TaskDeclarationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowInputSection.
    def visitWorkflowInputSection(self, ctx:WdlV1Parser.WorkflowInputSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowOutputSection.
    def visitWorkflowOutputSection(self, ctx:WdlV1Parser.WorkflowOutputSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowHintsSection.
    def visitWorkflowHintsSection(self, ctx:WdlV1Parser.WorkflowHintsSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowConditionalStatement.
    def visitWorkflowConditionalStatement(self, ctx:WdlV1Parser.WorkflowConditionalStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowScatterStatement.
    def visitWorkflowScatterStatement(self, ctx:WdlV1Parser.WorkflowScatterStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowCallStatement.
    def visitWorkflowCallStatement(self, ctx:WdlV1Parser.WorkflowCallStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowMetadataSection.
    def visitWorkflowMetadataSection(self, ctx:WdlV1Parser.WorkflowMetadataSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowParameterMetadataSection.
    def visitWorkflowParameterMetadataSection(self, ctx:WdlV1Parser.WorkflowParameterMetadataSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowDeclaration.
    def visitWorkflowDeclaration(self, ctx:WdlV1Parser.WorkflowDeclarationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#inputSection.
    def visitInputSection(self, ctx:WdlV1Parser.InputSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#outputSection.
    def visitOutputSection(self, ctx:WdlV1Parser.OutputSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#runtimeSection.
    def visitRuntimeSection(self, ctx:WdlV1Parser.RuntimeSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#runtimeItem.
    def visitRuntimeItem(self, ctx:WdlV1Parser.RuntimeItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#requirementsSection.
    def visitRequirementsSection(self, ctx:WdlV1Parser.RequirementsSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#requirementsItem.
    def visitRequirementsItem(self, ctx:WdlV1Parser.RequirementsItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsSectionTask.
    def visitHintsSectionTask(self, ctx:WdlV1Parser.HintsSectionTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsItemTask.
    def visitHintsItemTask(self, ctx:WdlV1Parser.HintsItemTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskHintValueExpression.
    def visitTaskHintValueExpression(self, ctx:WdlV1Parser.TaskHintValueExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskHintValueHintsObject.
    def visitTaskHintValueHintsObject(self, ctx:WdlV1Parser.TaskHintValueHintsObjectContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskHintValueInputObject.
    def visitTaskHintValueInputObject(self, ctx:WdlV1Parser.TaskHintValueInputObjectContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskHintValueOutputObject.
    def visitTaskHintValueOutputObject(self, ctx:WdlV1Parser.TaskHintValueOutputObjectContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskHintValueArray.
    def visitTaskHintValueArray(self, ctx:WdlV1Parser.TaskHintValueArrayContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsTypedObjectTask.
    def visitHintsTypedObjectTask(self, ctx:WdlV1Parser.HintsTypedObjectTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsObjectItemTask.
    def visitHintsObjectItemTask(self, ctx:WdlV1Parser.HintsObjectItemTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#inputHintsObjectTask.
    def visitInputHintsObjectTask(self, ctx:WdlV1Parser.InputHintsObjectTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#inputHintsItemTask.
    def visitInputHintsItemTask(self, ctx:WdlV1Parser.InputHintsItemTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#outputHintsObjectTask.
    def visitOutputHintsObjectTask(self, ctx:WdlV1Parser.OutputHintsObjectTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#outputHintsItemTask.
    def visitOutputHintsItemTask(self, ctx:WdlV1Parser.OutputHintsItemTaskContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#taskHintsArray.
    def visitTaskHintsArray(self, ctx:WdlV1Parser.TaskHintsArrayContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsSectionWorkflow.
    def visitHintsSectionWorkflow(self, ctx:WdlV1Parser.HintsSectionWorkflowContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsItemWorkflow.
    def visitHintsItemWorkflow(self, ctx:WdlV1Parser.HintsItemWorkflowContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowHintValueNumber.
    def visitWorkflowHintValueNumber(self, ctx:WdlV1Parser.WorkflowHintValueNumberContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowHintValueString.
    def visitWorkflowHintValueString(self, ctx:WdlV1Parser.WorkflowHintValueStringContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowHintValueBoolean.
    def visitWorkflowHintValueBoolean(self, ctx:WdlV1Parser.WorkflowHintValueBooleanContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowHintValueObject.
    def visitWorkflowHintValueObject(self, ctx:WdlV1Parser.WorkflowHintValueObjectContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowHintValueArray.
    def visitWorkflowHintValueArray(self, ctx:WdlV1Parser.WorkflowHintValueArrayContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsObjectWorkflow.
    def visitHintsObjectWorkflow(self, ctx:WdlV1Parser.HintsObjectWorkflowContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#hintsObjectItemWorkflow.
    def visitHintsObjectItemWorkflow(self, ctx:WdlV1Parser.HintsObjectItemWorkflowContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowHintsArray.
    def visitWorkflowHintsArray(self, ctx:WdlV1Parser.WorkflowHintsArrayContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#metadataSection.
    def visitMetadataSection(self, ctx:WdlV1Parser.MetadataSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#parameterMetadataSection.
    def visitParameterMetadataSection(self, ctx:WdlV1Parser.ParameterMetadataSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#metadataObject.
    def visitMetadataObject(self, ctx:WdlV1Parser.MetadataObjectContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#metadataObjectItem.
    def visitMetadataObjectItem(self, ctx:WdlV1Parser.MetadataObjectItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#metadataArray.
    def visitMetadataArray(self, ctx:WdlV1Parser.MetadataArrayContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#metadataValue.
    def visitMetadataValue(self, ctx:WdlV1Parser.MetadataValueContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#commandSection.
    def visitCommandSection(self, ctx:WdlV1Parser.CommandSectionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringCommand.
    def visitMultilineStringCommand(self, ctx:WdlV1Parser.MultilineStringCommandContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#bracedCommand.
    def visitBracedCommand(self, ctx:WdlV1Parser.BracedCommandContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#workflowStatement.
    def visitWorkflowStatement(self, ctx:WdlV1Parser.WorkflowStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#conditionalStatement.
    def visitConditionalStatement(self, ctx:WdlV1Parser.ConditionalStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#conditionalElseIfClause.
    def visitConditionalElseIfClause(self, ctx:WdlV1Parser.ConditionalElseIfClauseContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#conditionalElseClause.
    def visitConditionalElseClause(self, ctx:WdlV1Parser.ConditionalElseClauseContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#scatterStatement.
    def visitScatterStatement(self, ctx:WdlV1Parser.ScatterStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#scatterBody.
    def visitScatterBody(self, ctx:WdlV1Parser.ScatterBodyContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#callStatement.
    def visitCallStatement(self, ctx:WdlV1Parser.CallStatementContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#callTarget.
    def visitCallTarget(self, ctx:WdlV1Parser.CallTargetContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#callAlias.
    def visitCallAlias(self, ctx:WdlV1Parser.CallAliasContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#callAfterClause.
    def visitCallAfterClause(self, ctx:WdlV1Parser.CallAfterClauseContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#callInputBlock.
    def visitCallInputBlock(self, ctx:WdlV1Parser.CallInputBlockContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#callInputItem.
    def visitCallInputItem(self, ctx:WdlV1Parser.CallInputItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#expression.
    def visitExpression(self, ctx:WdlV1Parser.ExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#logicalOrExprOperation.
    def visitLogicalOrExprOperation(self, ctx:WdlV1Parser.LogicalOrExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#logicalAndExprOperation.
    def visitLogicalAndExprOperation(self, ctx:WdlV1Parser.LogicalAndExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#equalityExprOperation.
    def visitEqualityExprOperation(self, ctx:WdlV1Parser.EqualityExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#comparisonExprOperation.
    def visitComparisonExprOperation(self, ctx:WdlV1Parser.ComparisonExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#additiveExprOperation.
    def visitAdditiveExprOperation(self, ctx:WdlV1Parser.AdditiveExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multiplicativeExprOperation.
    def visitMultiplicativeExprOperation(self, ctx:WdlV1Parser.MultiplicativeExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#powerExprOperation.
    def visitPowerExprOperation(self, ctx:WdlV1Parser.PowerExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#powerExprNone.
    def visitPowerExprNone(self, ctx:WdlV1Parser.PowerExprNoneContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#unaryExprOperation.
    def visitUnaryExprOperation(self, ctx:WdlV1Parser.UnaryExprOperationContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#unaryExprNone.
    def visitUnaryExprNone(self, ctx:WdlV1Parser.UnaryExprNoneContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#postfixExprField.
    def visitPostfixExprField(self, ctx:WdlV1Parser.PostfixExprFieldContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#postfixExprArrayIndex.
    def visitPostfixExprArrayIndex(self, ctx:WdlV1Parser.PostfixExprArrayIndexContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#postfixExprNone.
    def visitPostfixExprNone(self, ctx:WdlV1Parser.PostfixExprNoneContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#primaryExpression.
    def visitPrimaryExpression(self, ctx:WdlV1Parser.PrimaryExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#variable.
    def visitVariable(self, ctx:WdlV1Parser.VariableContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#nullLiteral.
    def visitNullLiteral(self, ctx:WdlV1Parser.NullLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#noneLiteral.
    def visitNoneLiteral(self, ctx:WdlV1Parser.NoneLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#booleanLiteral.
    def visitBooleanLiteral(self, ctx:WdlV1Parser.BooleanLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#numberLiteralInt.
    def visitNumberLiteralInt(self, ctx:WdlV1Parser.NumberLiteralIntContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#numberLiteralFloat.
    def visitNumberLiteralFloat(self, ctx:WdlV1Parser.NumberLiteralFloatContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#numberLiteralSigned.
    def visitNumberLiteralSigned(self, ctx:WdlV1Parser.NumberLiteralSignedContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#arrayLiteral.
    def visitArrayLiteral(self, ctx:WdlV1Parser.ArrayLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#mapLiteral.
    def visitMapLiteral(self, ctx:WdlV1Parser.MapLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#mapLiteralItem.
    def visitMapLiteralItem(self, ctx:WdlV1Parser.MapLiteralItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#objectLiteral.
    def visitObjectLiteral(self, ctx:WdlV1Parser.ObjectLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#objectLiteralItem.
    def visitObjectLiteralItem(self, ctx:WdlV1Parser.ObjectLiteralItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#structLiteral.
    def visitStructLiteral(self, ctx:WdlV1Parser.StructLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#structLiteralItem.
    def visitStructLiteralItem(self, ctx:WdlV1Parser.StructLiteralItemContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#pairLiteral.
    def visitPairLiteral(self, ctx:WdlV1Parser.PairLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#groupedExpression.
    def visitGroupedExpression(self, ctx:WdlV1Parser.GroupedExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#ifExpression.
    def visitIfExpression(self, ctx:WdlV1Parser.IfExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#callExpression.
    def visitCallExpression(self, ctx:WdlV1Parser.CallExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringLiteral.
    def visitStringLiteral(self, ctx:WdlV1Parser.StringLiteralContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#quotedString.
    def visitQuotedString(self, ctx:WdlV1Parser.QuotedStringContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringElementText.
    def visitStringElementText(self, ctx:WdlV1Parser.StringElementTextContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringElementEscape.
    def visitStringElementEscape(self, ctx:WdlV1Parser.StringElementEscapeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringElementDollarSign.
    def visitStringElementDollarSign(self, ctx:WdlV1Parser.StringElementDollarSignContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringElementTilde.
    def visitStringElementTilde(self, ctx:WdlV1Parser.StringElementTildeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringElementPlaceholder.
    def visitStringElementPlaceholder(self, ctx:WdlV1Parser.StringElementPlaceholderContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringPlaceholder.
    def visitStringPlaceholder(self, ctx:WdlV1Parser.StringPlaceholderContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineString.
    def visitMultilineString(self, ctx:WdlV1Parser.MultilineStringContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringElementText.
    def visitMultilineStringElementText(self, ctx:WdlV1Parser.MultilineStringElementTextContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringElementEscape.
    def visitMultilineStringElementEscape(self, ctx:WdlV1Parser.MultilineStringElementEscapeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringElementDoubleCloseAngle.
    def visitMultilineStringElementDoubleCloseAngle(self, ctx:WdlV1Parser.MultilineStringElementDoubleCloseAngleContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringElementSingleCloseAngle.
    def visitMultilineStringElementSingleCloseAngle(self, ctx:WdlV1Parser.MultilineStringElementSingleCloseAngleContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringElementDollarSign.
    def visitMultilineStringElementDollarSign(self, ctx:WdlV1Parser.MultilineStringElementDollarSignContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringElementTilde.
    def visitMultilineStringElementTilde(self, ctx:WdlV1Parser.MultilineStringElementTildeContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringElementPlaceholder.
    def visitMultilineStringElementPlaceholder(self, ctx:WdlV1Parser.MultilineStringElementPlaceholderContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#multilineStringPlaceholder.
    def visitMultilineStringPlaceholder(self, ctx:WdlV1Parser.MultilineStringPlaceholderContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringPlaceholderExpression.
    def visitStringPlaceholderExpression(self, ctx:WdlV1Parser.StringPlaceholderExpressionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionSepDefault.
    def visitStringPlaceholderOptionSepDefault(self, ctx:WdlV1Parser.StringPlaceholderOptionSepDefaultContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionTrueFalse.
    def visitStringPlaceholderOptionTrueFalse(self, ctx:WdlV1Parser.StringPlaceholderOptionTrueFalseContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#stringPlaceholderOptionFalseTrue.
    def visitStringPlaceholderOptionFalseTrue(self, ctx:WdlV1Parser.StringPlaceholderOptionFalseTrueContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#strictIdentifier.
    def visitStrictIdentifier(self, ctx:WdlV1Parser.StrictIdentifierContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by WdlV1Parser#dottedIdentifier.
    def visitDottedIdentifier(self, ctx:WdlV1Parser.DottedIdentifierContext):
        return self.visitChildren(ctx)



del WdlV1Parser