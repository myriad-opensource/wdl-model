parser grammar WdlV1Parser;

options { tokenVocab = WdlV1Lexer; }

// -----------------------------------------------------------------------------
// Document Structure
// -----------------------------------------------------------------------------

document
    : versionStatement documentElement* EOF
    ;

versionStatement
    : KEYWORD_VERSION FLOAT
    ;

documentElement
    : importStatement
    | structDefinition
    | enumDefinition
    | taskDefinition
    | workflowDefinition
    ;

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------

importStatement
    : KEYWORD_IMPORT importUriLiteral (KEYWORD_AS strictIdentifier)? importAlias* #importStatementStandard
    | KEYWORD_IMPORT ASTERISK KEYWORD_FROM importUriLiteral #importStatementStar
    | KEYWORD_IMPORT importMembers KEYWORD_FROM importUriLiteral #importStatementMembers
    ;

importMembers
    : OPEN_BRACE (importMember (COMMA importMember)* COMMA?)? CLOSE_BRACE
    ;

importMember
    : strictIdentifier (KEYWORD_AS strictIdentifier)?
    ;

importUriLiteral
    : SINGLE_QUOTE importUriElement* SINGLE_QUOTE_END
    | DOUBLE_QUOTE importUriElement* DOUBLE_QUOTE_END
    ;

importUriElement
    : STRING_TEXT
    | STRING_ESCAPE
    ;

importAlias
    : KEYWORD_ALIAS strictIdentifier KEYWORD_AS strictIdentifier
    ;

// -----------------------------------------------------------------------------
// Type and Definition Declarations
// -----------------------------------------------------------------------------

structDefinition
    : KEYWORD_STRUCT strictIdentifier OPEN_BRACE structItem* CLOSE_BRACE
    ;

structItem
    : metadataSection #structItemMetadata
    | parameterMetadataSection #structItemParameterMetadata
    | structDeclaration #structItemMemberDeclaration
    ;

structDeclaration
    : wdlType strictIdentifier
    ;

enumDefinition
    : KEYWORD_ENUM strictIdentifier enumTypeParameter? OPEN_BRACE (enumChoice (COMMA enumChoice)* COMMA?)? CLOSE_BRACE
    ;

enumTypeParameter
    : OPEN_BRACKET wdlType CLOSE_BRACKET
    ;

enumChoice
    : strictIdentifier (ASSIGNMENT enumLiteralExpression)?
    ;

enumLiteralExpression
    : numberLiteralSigned
    | booleanLiteral
    | enumStringLiteral
    | enumArrayLiteral
    | enumMapLiteral
    | enumObjectLiteral
    | enumStructLiteral
    | enumPairLiteral
    ;

enumStringLiteral
    : enumQuotedString
    | enumMultilineString
    ;

enumQuotedString
    : SINGLE_QUOTE enumStringElement* SINGLE_QUOTE_END
    | DOUBLE_QUOTE enumStringElement* DOUBLE_QUOTE_END
    ;

enumStringElement
    : STRING_TEXT
    | STRING_ESCAPE
    | STRING_DOLLAR_SIGN
    | STRING_TILDE
    ;

enumMultilineString
    : OPEN_MULTILINE_STRING enumMultilineStringElement* MULTILINE_STRING_END
    ;

enumMultilineStringElement
    : MULTILINE_STRING_TEXT
    | MULTILINE_STRING_ESCAPE
    | MULTILINE_STRING_DOUBLE_CLOSE_ANGLE
    | MULTILINE_STRING_SINGLE_CLOSE_ANGLE
    | MULTILINE_STRING_DOLLAR_SIGN
    | MULTILINE_STRING_TILDE
    ;

enumArrayLiteral
    : OPEN_BRACKET (enumLiteralExpression (COMMA enumLiteralExpression)* COMMA?)? CLOSE_BRACKET
    ;

enumMapLiteral
    : OPEN_BRACE (enumMapLiteralItem (COMMA enumMapLiteralItem)* COMMA?)? CLOSE_BRACE
    ;

enumMapLiteralItem
    : enumLiteralExpression COLON enumLiteralExpression
    ;

enumObjectLiteral
    : KEYWORD_OBJECT OPEN_BRACE (enumObjectLiteralItem (COMMA enumObjectLiteralItem)* COMMA?)? CLOSE_BRACE
    ;

enumObjectLiteralItem
    : strictIdentifier COLON enumLiteralExpression
    ;

enumStructLiteral
    : strictIdentifier OPEN_BRACE (enumStructLiteralItem (COMMA enumStructLiteralItem)* COMMA?)? CLOSE_BRACE
    ;

enumStructLiteralItem
    : strictIdentifier COLON enumLiteralExpression
    ;

enumPairLiteral
    : OPEN_PAREN enumLiteralExpression COMMA enumLiteralExpression CLOSE_PAREN
    ;
taskDefinition
    : KEYWORD_TASK strictIdentifier OPEN_BRACE taskElement* CLOSE_BRACE
    ;

workflowDefinition
    : KEYWORD_WORKFLOW strictIdentifier OPEN_BRACE workflowElement* CLOSE_BRACE
    ;

wdlType
    : mapType
    | arrayType
    | pairType
    | objectType
    | primitiveType
    | typeRefType
    ;

mapType
    : KEYWORD_MAP_TYPE OPEN_BRACKET primitiveType COMMA wdlType CLOSE_BRACKET QUESTION_MARK?
    ;

arrayType
    : KEYWORD_ARRAY_TYPE OPEN_BRACKET wdlType CLOSE_BRACKET PLUS? QUESTION_MARK?
    ;

pairType
    : KEYWORD_PAIR_TYPE OPEN_BRACKET wdlType COMMA wdlType CLOSE_BRACKET QUESTION_MARK?
    ;

objectType
    : KEYWORD_OBJECT_TYPE QUESTION_MARK?
    ;

primitiveType
    : (KEYWORD_BOOLEAN_TYPE
      | KEYWORD_INT_TYPE
      | KEYWORD_FLOAT_TYPE
      | KEYWORD_STRING_TYPE
      | KEYWORD_FILE_TYPE
      | KEYWORD_DIRECTORY_TYPE) QUESTION_MARK?
    ;

typeRefType
    : strictIdentifier QUESTION_MARK?
    ;

unboundDeclaration
    : KEYWORD_ENV? wdlType strictIdentifier
    ;

boundDeclaration
    : KEYWORD_ENV? wdlType strictIdentifier ASSIGNMENT expression
    ;

declaration
    : unboundDeclaration
    | boundDeclaration
    ;

// -----------------------------------------------------------------------------
// Task and Workflow Sections
// -----------------------------------------------------------------------------

taskElement
    : inputSection #taskInputSection
    | commandSection #taskCommandSection
    | outputSection #taskOutputSection
    | runtimeSection #taskRuntimeSection
    | requirementsSection #taskRequirementsSection
    | hintsSectionTask #taskHintsSection 
    | metadataSection #taskMetadataSection
    | parameterMetadataSection #taskParameterMetadataSection
    | boundDeclaration #taskDeclaration
    ;

workflowElement
    : inputSection #workflowInputSection
    | outputSection #workflowOutputSection
    | hintsSectionWorkflow #workflowHintsSection
    | conditionalStatement #workflowConditionalStatement
    | scatterStatement #workflowScatterStatement
    | callStatement #workflowCallStatement
    | metadataSection #workflowMetadataSection
    | parameterMetadataSection #workflowParameterMetadataSection
    | boundDeclaration #workflowDeclaration
    ;

inputSection
    : KEYWORD_INPUT OPEN_BRACE declaration* CLOSE_BRACE
    ;

outputSection
    : KEYWORD_OUTPUT OPEN_BRACE boundDeclaration* CLOSE_BRACE
    ;

runtimeSection
    : KEYWORD_RUNTIME OPEN_BRACE runtimeItem* CLOSE_BRACE
    ;

runtimeItem
    : strictIdentifier COLON expression
    ;

requirementsSection
    : KEYWORD_REQUIREMENTS OPEN_BRACE requirementsItem* CLOSE_BRACE
    ;

requirementsItem
    : strictIdentifier COLON expression
    ;

hintsSectionTask
    : KEYWORD_HINTS OPEN_BRACE hintsItemTask* CLOSE_BRACE
    ;

hintsItemTask
    : strictIdentifier COLON hintsValueTask
    ;

hintsValueTask
    : expression #taskHintValueExpression
    | hintsTypedObjectTask #taskHintValueHintsObject
    | inputHintsObjectTask #taskHintValueInputObject
    | outputHintsObjectTask #taskHintValueOutputObject
    | taskHintsArray #taskHintValueArray
    ;

hintsTypedObjectTask
    : KEYWORD_HINTS OPEN_BRACE (hintsObjectItemTask (COMMA hintsObjectItemTask)* COMMA?)? CLOSE_BRACE
    ;

hintsObjectItemTask
    : dottedIdentifier COLON hintsValueTask
    ;

inputHintsObjectTask
    : KEYWORD_INPUT OPEN_BRACE (inputHintsItemTask (COMMA inputHintsItemTask)* COMMA?)? CLOSE_BRACE
    ;

inputHintsItemTask
    : dottedIdentifier COLON hintsTypedObjectTask
    ;

outputHintsObjectTask
    : KEYWORD_OUTPUT OPEN_BRACE (outputHintsItemTask (COMMA outputHintsItemTask)* COMMA?)? CLOSE_BRACE
    ;

outputHintsItemTask
    : dottedIdentifier COLON hintsTypedObjectTask
    ;

taskHintsArray
    : OPEN_BRACKET (hintsValueTask (COMMA hintsValueTask)* COMMA?)? CLOSE_BRACKET
    ;
    

hintsSectionWorkflow
    : KEYWORD_HINTS OPEN_BRACE hintsItemWorkflow* CLOSE_BRACE
    ;

hintsItemWorkflow
    : strictIdentifier COLON hintsValueWorkflow
    ;

hintsValueWorkflow
    : numberLiteralSigned #workflowHintValueNumber
    | stringLiteral #workflowHintValueString
    | booleanLiteral #workflowHintValueBoolean
    | hintsObjectWorkflow #workflowHintValueObject
    | workflowHintsArray #workflowHintValueArray
    ;

hintsObjectWorkflow
    : OPEN_BRACE (hintsObjectItemWorkflow (COMMA hintsObjectItemWorkflow)* COMMA?)? CLOSE_BRACE
    ;

hintsObjectItemWorkflow
    : dottedIdentifier COLON hintsValueWorkflow
    ;

workflowHintsArray
    : OPEN_BRACKET (hintsValueWorkflow (COMMA hintsValueWorkflow)* COMMA?)? CLOSE_BRACKET
    ;

metadataSection
    : KEYWORD_META OPEN_BRACE metadataObjectItem* CLOSE_BRACE
    ;

parameterMetadataSection
    : KEYWORD_PARAMETER_META OPEN_BRACE metadataObjectItem* CLOSE_BRACE
    ;

metadataObject
    : OPEN_BRACE (metadataObjectItem (COMMA metadataObjectItem)* COMMA?)? CLOSE_BRACE
    ;

metadataObjectItem
    : dottedIdentifier COLON metadataValue
    ;

metadataArray
    : OPEN_BRACKET (metadataValue (COMMA metadataValue)* COMMA?)? CLOSE_BRACKET
    ;

metadataValue
    : numberLiteralSigned
    | stringLiteral
    | booleanLiteral
    | nullLiteral
    | metadataObject
    | metadataArray
    ;

// -----------------------------------------------------------------------------
// Command Sections
// -----------------------------------------------------------------------------

commandSection
    : KEYWORD_COMMAND (multilineStringCommand | bracedCommand)
    ;

multilineStringCommand
    : OPEN_MULTILINE_STRING multilineStringElement* MULTILINE_STRING_END
    ;

bracedCommand
    : OPEN_BRACE stringElement* CLOSE_BRACE
    ;

workflowStatement
    : conditionalStatement
    | scatterStatement
    | callStatement
    | boundDeclaration
    ;

// -----------------------------------------------------------------------------
// Expressions
// -----------------------------------------------------------------------------

conditionalStatement
    : KEYWORD_IF OPEN_PAREN expression CLOSE_PAREN OPEN_BRACE workflowStatement* CLOSE_BRACE conditionalElseIfClause* conditionalElseClause? 
    ;

conditionalElseIfClause
    : KEYWORD_ELSE KEYWORD_IF OPEN_PAREN expression CLOSE_PAREN OPEN_BRACE workflowStatement* CLOSE_BRACE
    ;
    
conditionalElseClause      
    : KEYWORD_ELSE OPEN_BRACE workflowStatement* CLOSE_BRACE
    ;
    
    

scatterStatement
    : KEYWORD_SCATTER OPEN_PAREN strictIdentifier KEYWORD_IN expression CLOSE_PAREN scatterBody
    ;
    
scatterBody
    : OPEN_BRACE workflowStatement* CLOSE_BRACE
    ;

callStatement
    : KEYWORD_CALL callTarget callAlias? callAfterClause* callInputBlock?
    ;

callTarget
    : strictIdentifier (DOT strictIdentifier)*
    ;

callAlias
    : KEYWORD_AS strictIdentifier
    ;

callAfterClause
    : KEYWORD_AFTER strictIdentifier
    ;

callInputBlock
    : OPEN_BRACE (KEYWORD_INPUT COLON)? (callInputItem (COMMA callInputItem)* COMMA?)? CLOSE_BRACE
    ;

callInputItem
    : strictIdentifier (ASSIGNMENT expression)?
    ;

expression
    : logicalOrExpression
    ;

logicalOrExpression
    : logicalAndExpression LOGICAL_OR logicalOrExpression #logicalOrExprOperation
    | logicalAndExpression #logicalOrExprNone
    ;

logicalAndExpression
    : equalityExpression LOGICAL_AND logicalAndExpression #logicalAndExprOperation
    | equalityExpression #logicalAndExprNone
    ;

equalityExpression
    : comparisonExpression (EQUAL | NOT_EQUAL) equalityExpression #equalityExprOperation
    | comparisonExpression #equalityExprNone
    ;

comparisonExpression
    : additiveExpression (LESS | LESS_EQUAL | GREATER | GREATER_EQUAL) comparisonExpression #comparisonExprOperation
    | additiveExpression #comparisonExprNone
    ;

additiveExpression
    : multiplicativeExpression (PLUS | MINUS) additiveExpression #additiveExprOperation
    | multiplicativeExpression #additiveExprNone
    ;

multiplicativeExpression
    : powerExpression (ASTERISK | SLASH | PERCENT) multiplicativeExpression #multiplicativeExprOperation
    | powerExpression #multiplicativeExprNone
    ;

powerExpression
    : unaryExpression EXPONENTIATION powerExpression #powerExprOperation
    | unaryExpression #powerExprNone
    ;

unaryExpression
    : (EXCLAMATION | MINUS) unaryExpression #unaryExprOperation
    | postfixExpression #unaryExprNone
    ;

postfixExpression
    : postfixExpression OPEN_BRACKET expression CLOSE_BRACKET #postfixExprArrayIndex
    | postfixExpression DOT strictIdentifier #postfixExprField
    | primaryExpression #postfixExprNone
    ;

primaryExpression
    : variable
    | noneLiteral
    | booleanLiteral
    | numberLiteral
    | stringLiteral
    | arrayLiteral
    | mapLiteral
    | objectLiteral
    | structLiteral
    | pairLiteral
    | groupedExpression
    | ifExpression
    | callExpression
    ;

variable
    : strictIdentifier
    ;

nullLiteral
    : KEYWORD_NULL
    ;

noneLiteral
    : KEYWORD_NONE
    ;

booleanLiteral
    : KEYWORD_TRUE
    | KEYWORD_FALSE
    ;

numberLiteral
    : INTEGER #numberLiteralInt
    | FLOAT #numberLiteralFloat
    ;

numberLiteralSigned
    : MINUS? numberLiteral
    ;

arrayLiteral
    : OPEN_BRACKET (expression (COMMA expression)* COMMA?)? CLOSE_BRACKET
    ;

mapLiteral
    : OPEN_BRACE (mapLiteralItem (COMMA mapLiteralItem)* COMMA?)? CLOSE_BRACE
    ;

mapLiteralItem
    : expression COLON expression
    ;

objectLiteral
    : KEYWORD_OBJECT OPEN_BRACE (objectLiteralItem (COMMA objectLiteralItem)* COMMA?)? CLOSE_BRACE
    ;

objectLiteralItem
    : strictIdentifier COLON expression
    ;

structLiteral
    : strictIdentifier OPEN_BRACE (structLiteralItem (COMMA structLiteralItem)* COMMA?)? CLOSE_BRACE
    ;

structLiteralItem
    : strictIdentifier COLON expression
    ;

pairLiteral
    : OPEN_PAREN expression COMMA expression CLOSE_PAREN
    ;

groupedExpression
    : OPEN_PAREN expression CLOSE_PAREN
    ;

ifExpression
    : KEYWORD_IF expression KEYWORD_THEN expression KEYWORD_ELSE expression
    ;

callExpression
    : strictIdentifier OPEN_PAREN (expression (COMMA expression)* COMMA?)? CLOSE_PAREN
    ;

// -----------------------------------------------------------------------------
// Strings and Multiline Strings
// -----------------------------------------------------------------------------

stringLiteral
    : quotedString
    | multilineString
    ;

quotedString
    : SINGLE_QUOTE stringElement* SINGLE_QUOTE_END
    | DOUBLE_QUOTE stringElement* DOUBLE_QUOTE_END
    ;

stringElement
    : STRING_TEXT #stringElementText
    | STRING_ESCAPE #stringElementEscape
    | STRING_DOLLAR_SIGN #stringElementDollarSign
    | STRING_TILDE #stringElementTilde
    | stringPlaceholder #stringElementPlaceholder
    ;

stringPlaceholder
    : STRING_PLACEHOLDER_START stringPlaceholderExpression CLOSE_BRACE
    ;

multilineString
    : OPEN_MULTILINE_STRING multilineStringElement* MULTILINE_STRING_END
    ;

multilineStringElement
    : MULTILINE_STRING_TEXT #multilineStringElementText
    | MULTILINE_STRING_ESCAPE #multilineStringElementEscape
    | MULTILINE_STRING_DOUBLE_CLOSE_ANGLE #multilineStringElementDoubleCloseAngle
    | MULTILINE_STRING_SINGLE_CLOSE_ANGLE #multilineStringElementSingleCloseAngle
    | MULTILINE_STRING_DOLLAR_SIGN #multilineStringElementDollarSign
    | MULTILINE_STRING_TILDE #multilineStringElementTilde
    | multilineStringPlaceholder #multilineStringElementPlaceholder
    ;

multilineStringPlaceholder
    : MULTILINE_STRING_TILDE_PLACEHOLDER_START stringPlaceholderExpression CLOSE_BRACE
    | MULTILINE_STRING_DOLLAR_PLACEHOLDER_START stringPlaceholderExpression CLOSE_BRACE
    ;

stringPlaceholderExpression
    : stringPlaceholderOption* expression
    ;

stringPlaceholderOption
    : IDENTIFIER ASSIGNMENT stringLiteral #stringPlaceholderOptionSepDefault
    | KEYWORD_TRUE ASSIGNMENT stringLiteral KEYWORD_FALSE ASSIGNMENT stringLiteral #stringPlaceholderOptionTrueFalse
    | KEYWORD_FALSE ASSIGNMENT stringLiteral KEYWORD_TRUE ASSIGNMENT stringLiteral #stringPlaceholderOptionFalseTrue
    ;

// -----------------------------------------------------------------------------
// Identifier Compatibility Set
// -----------------------------------------------------------------------------

strictIdentifier
    : anyIdentBase
    ;

dottedIdentifier
    : strictIdentifier (DOT strictIdentifier)*
    ;

anyIdentBase
    : IDENTIFIER
    | KEYWORD_ARRAY_TYPE
    | KEYWORD_BOOLEAN_TYPE
    | KEYWORD_DIRECTORY_TYPE
    | KEYWORD_FILE_TYPE
    | KEYWORD_FLOAT_TYPE
    | KEYWORD_INT_TYPE
    | KEYWORD_MAP_TYPE
    | KEYWORD_OBJECT_TYPE
    | KEYWORD_PAIR_TYPE
    | KEYWORD_STRING_TYPE
    | KEYWORD_AFTER
    | KEYWORD_ALIAS
    | KEYWORD_AS
    | KEYWORD_CALL
    | KEYWORD_COMMAND
    | KEYWORD_ELSE
    | KEYWORD_ENV
    | KEYWORD_FALSE
    | KEYWORD_FROM
    | KEYWORD_HINTS
    | KEYWORD_IF
    | KEYWORD_IN
    | KEYWORD_IMPORT
    | KEYWORD_INPUT
    | KEYWORD_META
    | KEYWORD_NONE
    | KEYWORD_NULL
    | KEYWORD_OBJECT
    | KEYWORD_OUTPUT
    | KEYWORD_PARAMETER_META
    | KEYWORD_REQUIREMENTS
    | KEYWORD_RUNTIME
    | KEYWORD_SCATTER
    | KEYWORD_STRUCT
    | KEYWORD_ENUM
    | KEYWORD_TASK
    | KEYWORD_THEN
    | KEYWORD_TRUE
    | KEYWORD_VERSION
    | KEYWORD_WORKFLOW
    ;
