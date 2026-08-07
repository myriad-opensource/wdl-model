// Generated from ../wdl-grammar/antrl4/v1/WdlV1Parser.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::wdlv1parserlistener::*;
use super::wdlv1parservisitor::*;

use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const WdlV1Parser_STRING_TEXT:i32=1; 
		pub const WdlV1Parser_STRING_ESCAPE:i32=2; 
		pub const WdlV1Parser_STRING_DOLLAR_SIGN:i32=3; 
		pub const WdlV1Parser_STRING_TILDE:i32=4; 
		pub const WdlV1Parser_STRING_PLACEHOLDER_START:i32=5; 
		pub const WdlV1Parser_FLOAT:i32=6; 
		pub const WdlV1Parser_INTEGER:i32=7; 
		pub const WdlV1Parser_OPEN_MULTILINE_STRING:i32=8; 
		pub const WdlV1Parser_CLOSE_MULTILINE_STRING:i32=9; 
		pub const WdlV1Parser_SINGLE_QUOTE:i32=10; 
		pub const WdlV1Parser_DOUBLE_QUOTE:i32=11; 
		pub const WdlV1Parser_KEYWORD_ARRAY_TYPE:i32=12; 
		pub const WdlV1Parser_KEYWORD_BOOLEAN_TYPE:i32=13; 
		pub const WdlV1Parser_KEYWORD_DIRECTORY_TYPE:i32=14; 
		pub const WdlV1Parser_KEYWORD_FILE_TYPE:i32=15; 
		pub const WdlV1Parser_KEYWORD_FLOAT_TYPE:i32=16; 
		pub const WdlV1Parser_KEYWORD_INT_TYPE:i32=17; 
		pub const WdlV1Parser_KEYWORD_MAP_TYPE:i32=18; 
		pub const WdlV1Parser_KEYWORD_OBJECT_TYPE:i32=19; 
		pub const WdlV1Parser_KEYWORD_PAIR_TYPE:i32=20; 
		pub const WdlV1Parser_KEYWORD_STRING_TYPE:i32=21; 
		pub const WdlV1Parser_KEYWORD_AFTER:i32=22; 
		pub const WdlV1Parser_KEYWORD_ALIAS:i32=23; 
		pub const WdlV1Parser_KEYWORD_AS:i32=24; 
		pub const WdlV1Parser_KEYWORD_CALL:i32=25; 
		pub const WdlV1Parser_KEYWORD_COMMAND:i32=26; 
		pub const WdlV1Parser_KEYWORD_ELSE:i32=27; 
		pub const WdlV1Parser_KEYWORD_ENV:i32=28; 
		pub const WdlV1Parser_KEYWORD_FALSE:i32=29; 
		pub const WdlV1Parser_KEYWORD_FROM:i32=30; 
		pub const WdlV1Parser_KEYWORD_HINTS:i32=31; 
		pub const WdlV1Parser_KEYWORD_IF:i32=32; 
		pub const WdlV1Parser_KEYWORD_IN:i32=33; 
		pub const WdlV1Parser_KEYWORD_IMPORT:i32=34; 
		pub const WdlV1Parser_KEYWORD_INPUT:i32=35; 
		pub const WdlV1Parser_KEYWORD_META:i32=36; 
		pub const WdlV1Parser_KEYWORD_NONE:i32=37; 
		pub const WdlV1Parser_KEYWORD_NULL:i32=38; 
		pub const WdlV1Parser_KEYWORD_OBJECT:i32=39; 
		pub const WdlV1Parser_KEYWORD_OUTPUT:i32=40; 
		pub const WdlV1Parser_KEYWORD_PARAMETER_META:i32=41; 
		pub const WdlV1Parser_KEYWORD_REQUIREMENTS:i32=42; 
		pub const WdlV1Parser_KEYWORD_RUNTIME:i32=43; 
		pub const WdlV1Parser_KEYWORD_SCATTER:i32=44; 
		pub const WdlV1Parser_KEYWORD_STRUCT:i32=45; 
		pub const WdlV1Parser_KEYWORD_ENUM:i32=46; 
		pub const WdlV1Parser_KEYWORD_TASK:i32=47; 
		pub const WdlV1Parser_KEYWORD_THEN:i32=48; 
		pub const WdlV1Parser_KEYWORD_TRUE:i32=49; 
		pub const WdlV1Parser_KEYWORD_VERSION:i32=50; 
		pub const WdlV1Parser_KEYWORD_WORKFLOW:i32=51; 
		pub const WdlV1Parser_IDENTIFIER:i32=52; 
		pub const WdlV1Parser_EXPONENTIATION:i32=53; 
		pub const WdlV1Parser_LOGICAL_OR:i32=54; 
		pub const WdlV1Parser_LOGICAL_AND:i32=55; 
		pub const WdlV1Parser_EQUAL:i32=56; 
		pub const WdlV1Parser_NOT_EQUAL:i32=57; 
		pub const WdlV1Parser_LESS_EQUAL:i32=58; 
		pub const WdlV1Parser_GREATER_EQUAL:i32=59; 
		pub const WdlV1Parser_OPEN_BRACE:i32=60; 
		pub const WdlV1Parser_CLOSE_BRACE:i32=61; 
		pub const WdlV1Parser_OPEN_BRACKET:i32=62; 
		pub const WdlV1Parser_CLOSE_BRACKET:i32=63; 
		pub const WdlV1Parser_ASSIGNMENT:i32=64; 
		pub const WdlV1Parser_COLON:i32=65; 
		pub const WdlV1Parser_COMMA:i32=66; 
		pub const WdlV1Parser_OPEN_PAREN:i32=67; 
		pub const WdlV1Parser_CLOSE_PAREN:i32=68; 
		pub const WdlV1Parser_QUESTION_MARK:i32=69; 
		pub const WdlV1Parser_EXCLAMATION:i32=70; 
		pub const WdlV1Parser_PLUS:i32=71; 
		pub const WdlV1Parser_MINUS:i32=72; 
		pub const WdlV1Parser_ASTERISK:i32=73; 
		pub const WdlV1Parser_SLASH:i32=74; 
		pub const WdlV1Parser_PERCENT:i32=75; 
		pub const WdlV1Parser_LESS:i32=76; 
		pub const WdlV1Parser_GREATER:i32=77; 
		pub const WdlV1Parser_DOT:i32=78; 
		pub const WdlV1Parser_COMMENT:i32=79; 
		pub const WdlV1Parser_WHITESPACE:i32=80; 
		pub const WdlV1Parser_UNEXPECTED_CHAR:i32=81; 
		pub const WdlV1Parser_SINGLE_QUOTE_END:i32=82; 
		pub const WdlV1Parser_DOUBLE_QUOTE_END:i32=83; 
		pub const WdlV1Parser_MULTILINE_STRING_DOLLAR_PLACEHOLDER_START:i32=84; 
		pub const WdlV1Parser_MULTILINE_STRING_TILDE_PLACEHOLDER_START:i32=85; 
		pub const WdlV1Parser_MULTILINE_STRING_ESCAPE:i32=86; 
		pub const WdlV1Parser_MULTILINE_STRING_END:i32=87; 
		pub const WdlV1Parser_MULTILINE_STRING_DOUBLE_CLOSE_ANGLE:i32=88; 
		pub const WdlV1Parser_MULTILINE_STRING_SINGLE_CLOSE_ANGLE:i32=89; 
		pub const WdlV1Parser_MULTILINE_STRING_TEXT:i32=90; 
		pub const WdlV1Parser_MULTILINE_STRING_DOLLAR_SIGN:i32=91; 
		pub const WdlV1Parser_MULTILINE_STRING_TILDE:i32=92; 
		pub const WdlV1Parser_ESC_VALID:i32=93; 
		pub const WdlV1Parser_ESC_CONTINUATION:i32=94; 
		pub const WdlV1Parser_ESC_VALID_OCTAL:i32=95; 
		pub const WdlV1Parser_ESC_INVALID_OCTAL:i32=96; 
		pub const WdlV1Parser_ESC_VALID_HEX:i32=97; 
		pub const WdlV1Parser_ESC_INVALID_HEX:i32=98; 
		pub const WdlV1Parser_ESC_VALID_UNICODE:i32=99; 
		pub const WdlV1Parser_ESC_INVALID_SHORT_UNICODE:i32=100; 
		pub const WdlV1Parser_ESC_INVALID_UNICODE:i32=101; 
		pub const WdlV1Parser_ESC_NEWLINE:i32=102; 
		pub const WdlV1Parser_ESC_TAB:i32=103; 
		pub const WdlV1Parser_ESC_UNKNOWN:i32=104; 
		pub const WdlV1Parser_ESC_TEXT:i32=105; 
		pub const WdlV1Parser_VERSION_DECLARATION_WHITESPACE:i32=106; 
		pub const WdlV1Parser_VERSION_NUMBER:i32=107; 
		pub const WdlV1Parser_SINGLE_QUOTE_DOLLAR_SIGN:i32=108; 
		pub const WdlV1Parser_SINGLE_QUOTE_TILDE:i32=109;
	pub const WdlV1Parser_EOF:i32=EOF;
	pub const RULE_document:usize = 0; 
	pub const RULE_versionStatement:usize = 1; 
	pub const RULE_documentElement:usize = 2; 
	pub const RULE_importStatement:usize = 3; 
	pub const RULE_importMembers:usize = 4; 
	pub const RULE_importMember:usize = 5; 
	pub const RULE_importUriLiteral:usize = 6; 
	pub const RULE_importUriElement:usize = 7; 
	pub const RULE_importAlias:usize = 8; 
	pub const RULE_structDefinition:usize = 9; 
	pub const RULE_structItem:usize = 10; 
	pub const RULE_structDeclaration:usize = 11; 
	pub const RULE_enumDefinition:usize = 12; 
	pub const RULE_enumTypeParameter:usize = 13; 
	pub const RULE_enumChoice:usize = 14; 
	pub const RULE_enumLiteralExpression:usize = 15; 
	pub const RULE_enumStringLiteral:usize = 16; 
	pub const RULE_enumQuotedString:usize = 17; 
	pub const RULE_enumStringElement:usize = 18; 
	pub const RULE_enumMultilineString:usize = 19; 
	pub const RULE_enumMultilineStringElement:usize = 20; 
	pub const RULE_enumArrayLiteral:usize = 21; 
	pub const RULE_enumMapLiteral:usize = 22; 
	pub const RULE_enumMapLiteralItem:usize = 23; 
	pub const RULE_enumObjectLiteral:usize = 24; 
	pub const RULE_enumObjectLiteralItem:usize = 25; 
	pub const RULE_enumStructLiteral:usize = 26; 
	pub const RULE_enumStructLiteralItem:usize = 27; 
	pub const RULE_enumPairLiteral:usize = 28; 
	pub const RULE_taskDefinition:usize = 29; 
	pub const RULE_workflowDefinition:usize = 30; 
	pub const RULE_type:usize = 31; 
	pub const RULE_mapType:usize = 32; 
	pub const RULE_arrayType:usize = 33; 
	pub const RULE_pairType:usize = 34; 
	pub const RULE_objectType:usize = 35; 
	pub const RULE_primitiveType:usize = 36; 
	pub const RULE_typeRefType:usize = 37; 
	pub const RULE_unboundDeclaration:usize = 38; 
	pub const RULE_boundDeclaration:usize = 39; 
	pub const RULE_declaration:usize = 40; 
	pub const RULE_taskElement:usize = 41; 
	pub const RULE_workflowElement:usize = 42; 
	pub const RULE_inputSection:usize = 43; 
	pub const RULE_outputSection:usize = 44; 
	pub const RULE_runtimeSection:usize = 45; 
	pub const RULE_runtimeItem:usize = 46; 
	pub const RULE_requirementsSection:usize = 47; 
	pub const RULE_requirementsItem:usize = 48; 
	pub const RULE_hintsSectionTask:usize = 49; 
	pub const RULE_hintsItemTask:usize = 50; 
	pub const RULE_hintsValueTask:usize = 51; 
	pub const RULE_hintsTypedObjectTask:usize = 52; 
	pub const RULE_hintsObjectItemTask:usize = 53; 
	pub const RULE_inputHintsObjectTask:usize = 54; 
	pub const RULE_inputHintsItemTask:usize = 55; 
	pub const RULE_outputHintsObjectTask:usize = 56; 
	pub const RULE_outputHintsItemTask:usize = 57; 
	pub const RULE_taskHintsArray:usize = 58; 
	pub const RULE_hintsSectionWorkflow:usize = 59; 
	pub const RULE_hintsItemWorkflow:usize = 60; 
	pub const RULE_hintsValueWorkflow:usize = 61; 
	pub const RULE_hintsObjectWorkflow:usize = 62; 
	pub const RULE_hintsObjectItemWorkflow:usize = 63; 
	pub const RULE_workflowHintsArray:usize = 64; 
	pub const RULE_metadataSection:usize = 65; 
	pub const RULE_parameterMetadataSection:usize = 66; 
	pub const RULE_metadataObject:usize = 67; 
	pub const RULE_metadataObjectItem:usize = 68; 
	pub const RULE_metadataArray:usize = 69; 
	pub const RULE_metadataValue:usize = 70; 
	pub const RULE_commandSection:usize = 71; 
	pub const RULE_multilineStringCommand:usize = 72; 
	pub const RULE_bracedCommand:usize = 73; 
	pub const RULE_workflowStatement:usize = 74; 
	pub const RULE_conditionalStatement:usize = 75; 
	pub const RULE_conditionalElseIfClause:usize = 76; 
	pub const RULE_conditionalElseClause:usize = 77; 
	pub const RULE_scatterStatement:usize = 78; 
	pub const RULE_scatterBody:usize = 79; 
	pub const RULE_callStatement:usize = 80; 
	pub const RULE_callTarget:usize = 81; 
	pub const RULE_callAlias:usize = 82; 
	pub const RULE_callAfterClause:usize = 83; 
	pub const RULE_callInputBlock:usize = 84; 
	pub const RULE_callInputItem:usize = 85; 
	pub const RULE_expression:usize = 86; 
	pub const RULE_logicalOrExpression:usize = 87; 
	pub const RULE_logicalAndExpression:usize = 88; 
	pub const RULE_equalityExpression:usize = 89; 
	pub const RULE_comparisonExpression:usize = 90; 
	pub const RULE_additiveExpression:usize = 91; 
	pub const RULE_multiplicativeExpression:usize = 92; 
	pub const RULE_powerExpression:usize = 93; 
	pub const RULE_unaryExpression:usize = 94; 
	pub const RULE_postfixExpression:usize = 95; 
	pub const RULE_primaryExpression:usize = 96; 
	pub const RULE_variable:usize = 97; 
	pub const RULE_nullLiteral:usize = 98; 
	pub const RULE_noneLiteral:usize = 99; 
	pub const RULE_booleanLiteral:usize = 100; 
	pub const RULE_numberLiteral:usize = 101; 
	pub const RULE_numberLiteralSigned:usize = 102; 
	pub const RULE_arrayLiteral:usize = 103; 
	pub const RULE_mapLiteral:usize = 104; 
	pub const RULE_mapLiteralItem:usize = 105; 
	pub const RULE_objectLiteral:usize = 106; 
	pub const RULE_objectLiteralItem:usize = 107; 
	pub const RULE_structLiteral:usize = 108; 
	pub const RULE_structLiteralItem:usize = 109; 
	pub const RULE_pairLiteral:usize = 110; 
	pub const RULE_groupedExpression:usize = 111; 
	pub const RULE_ifExpression:usize = 112; 
	pub const RULE_callExpression:usize = 113; 
	pub const RULE_stringLiteral:usize = 114; 
	pub const RULE_quotedString:usize = 115; 
	pub const RULE_stringElement:usize = 116; 
	pub const RULE_stringPlaceholder:usize = 117; 
	pub const RULE_multilineString:usize = 118; 
	pub const RULE_multilineStringElement:usize = 119; 
	pub const RULE_multilineStringPlaceholder:usize = 120; 
	pub const RULE_stringPlaceholderExpression:usize = 121; 
	pub const RULE_stringPlaceholderOption:usize = 122; 
	pub const RULE_strictIdentifier:usize = 123; 
	pub const RULE_dottedIdentifier:usize = 124; 
	pub const RULE_anyIdentBase:usize = 125;
	pub const ruleNames: [&'static str; 126] =  [
		"document", "versionStatement", "documentElement", "importStatement", 
		"importMembers", "importMember", "importUriLiteral", "importUriElement", 
		"importAlias", "structDefinition", "structItem", "structDeclaration", 
		"enumDefinition", "enumTypeParameter", "enumChoice", "enumLiteralExpression", 
		"enumStringLiteral", "enumQuotedString", "enumStringElement", "enumMultilineString", 
		"enumMultilineStringElement", "enumArrayLiteral", "enumMapLiteral", "enumMapLiteralItem", 
		"enumObjectLiteral", "enumObjectLiteralItem", "enumStructLiteral", "enumStructLiteralItem", 
		"enumPairLiteral", "taskDefinition", "workflowDefinition", "type", "mapType", 
		"arrayType", "pairType", "objectType", "primitiveType", "typeRefType", 
		"unboundDeclaration", "boundDeclaration", "declaration", "taskElement", 
		"workflowElement", "inputSection", "outputSection", "runtimeSection", 
		"runtimeItem", "requirementsSection", "requirementsItem", "hintsSectionTask", 
		"hintsItemTask", "hintsValueTask", "hintsTypedObjectTask", "hintsObjectItemTask", 
		"inputHintsObjectTask", "inputHintsItemTask", "outputHintsObjectTask", 
		"outputHintsItemTask", "taskHintsArray", "hintsSectionWorkflow", "hintsItemWorkflow", 
		"hintsValueWorkflow", "hintsObjectWorkflow", "hintsObjectItemWorkflow", 
		"workflowHintsArray", "metadataSection", "parameterMetadataSection", "metadataObject", 
		"metadataObjectItem", "metadataArray", "metadataValue", "commandSection", 
		"multilineStringCommand", "bracedCommand", "workflowStatement", "conditionalStatement", 
		"conditionalElseIfClause", "conditionalElseClause", "scatterStatement", 
		"scatterBody", "callStatement", "callTarget", "callAlias", "callAfterClause", 
		"callInputBlock", "callInputItem", "expression", "logicalOrExpression", 
		"logicalAndExpression", "equalityExpression", "comparisonExpression", 
		"additiveExpression", "multiplicativeExpression", "powerExpression", "unaryExpression", 
		"postfixExpression", "primaryExpression", "variable", "nullLiteral", "noneLiteral", 
		"booleanLiteral", "numberLiteral", "numberLiteralSigned", "arrayLiteral", 
		"mapLiteral", "mapLiteralItem", "objectLiteral", "objectLiteralItem", 
		"structLiteral", "structLiteralItem", "pairLiteral", "groupedExpression", 
		"ifExpression", "callExpression", "stringLiteral", "quotedString", "stringElement", 
		"stringPlaceholder", "multilineString", "multilineStringElement", "multilineStringPlaceholder", 
		"stringPlaceholderExpression", "stringPlaceholderOption", "strictIdentifier", 
		"dottedIdentifier", "anyIdentBase"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;104] = [
		None, None, None, None, None, None, None, None, Some("'<<<'"), None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, Some("'{'"), None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, Some("'${'"), Some("'~{'"), None, None, Some("'>>'"), None, 
		None, None, None, None, None, None, None, None, Some("'\\x'"), None, Some("'\\u'"), 
		Some("'\\U'"), Some("'\\n'"), Some("'\\t'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;110]  = [
		None, Some("STRING_TEXT"), Some("STRING_ESCAPE"), Some("STRING_DOLLAR_SIGN"), 
		Some("STRING_TILDE"), Some("STRING_PLACEHOLDER_START"), Some("FLOAT"), 
		Some("INTEGER"), Some("OPEN_MULTILINE_STRING"), Some("CLOSE_MULTILINE_STRING"), 
		Some("SINGLE_QUOTE"), Some("DOUBLE_QUOTE"), Some("KEYWORD_ARRAY_TYPE"), 
		Some("KEYWORD_BOOLEAN_TYPE"), Some("KEYWORD_DIRECTORY_TYPE"), Some("KEYWORD_FILE_TYPE"), 
		Some("KEYWORD_FLOAT_TYPE"), Some("KEYWORD_INT_TYPE"), Some("KEYWORD_MAP_TYPE"), 
		Some("KEYWORD_OBJECT_TYPE"), Some("KEYWORD_PAIR_TYPE"), Some("KEYWORD_STRING_TYPE"), 
		Some("KEYWORD_AFTER"), Some("KEYWORD_ALIAS"), Some("KEYWORD_AS"), Some("KEYWORD_CALL"), 
		Some("KEYWORD_COMMAND"), Some("KEYWORD_ELSE"), Some("KEYWORD_ENV"), Some("KEYWORD_FALSE"), 
		Some("KEYWORD_FROM"), Some("KEYWORD_HINTS"), Some("KEYWORD_IF"), Some("KEYWORD_IN"), 
		Some("KEYWORD_IMPORT"), Some("KEYWORD_INPUT"), Some("KEYWORD_META"), Some("KEYWORD_NONE"), 
		Some("KEYWORD_NULL"), Some("KEYWORD_OBJECT"), Some("KEYWORD_OUTPUT"), 
		Some("KEYWORD_PARAMETER_META"), Some("KEYWORD_REQUIREMENTS"), Some("KEYWORD_RUNTIME"), 
		Some("KEYWORD_SCATTER"), Some("KEYWORD_STRUCT"), Some("KEYWORD_ENUM"), 
		Some("KEYWORD_TASK"), Some("KEYWORD_THEN"), Some("KEYWORD_TRUE"), Some("KEYWORD_VERSION"), 
		Some("KEYWORD_WORKFLOW"), Some("IDENTIFIER"), Some("EXPONENTIATION"), 
		Some("LOGICAL_OR"), Some("LOGICAL_AND"), Some("EQUAL"), Some("NOT_EQUAL"), 
		Some("LESS_EQUAL"), Some("GREATER_EQUAL"), Some("OPEN_BRACE"), Some("CLOSE_BRACE"), 
		Some("OPEN_BRACKET"), Some("CLOSE_BRACKET"), Some("ASSIGNMENT"), Some("COLON"), 
		Some("COMMA"), Some("OPEN_PAREN"), Some("CLOSE_PAREN"), Some("QUESTION_MARK"), 
		Some("EXCLAMATION"), Some("PLUS"), Some("MINUS"), Some("ASTERISK"), Some("SLASH"), 
		Some("PERCENT"), Some("LESS"), Some("GREATER"), Some("DOT"), Some("COMMENT"), 
		Some("WHITESPACE"), Some("UNEXPECTED_CHAR"), Some("SINGLE_QUOTE_END"), 
		Some("DOUBLE_QUOTE_END"), Some("MULTILINE_STRING_DOLLAR_PLACEHOLDER_START"), 
		Some("MULTILINE_STRING_TILDE_PLACEHOLDER_START"), Some("MULTILINE_STRING_ESCAPE"), 
		Some("MULTILINE_STRING_END"), Some("MULTILINE_STRING_DOUBLE_CLOSE_ANGLE"), 
		Some("MULTILINE_STRING_SINGLE_CLOSE_ANGLE"), Some("MULTILINE_STRING_TEXT"), 
		Some("MULTILINE_STRING_DOLLAR_SIGN"), Some("MULTILINE_STRING_TILDE"), 
		Some("ESC_VALID"), Some("ESC_CONTINUATION"), Some("ESC_VALID_OCTAL"), 
		Some("ESC_INVALID_OCTAL"), Some("ESC_VALID_HEX"), Some("ESC_INVALID_HEX"), 
		Some("ESC_VALID_UNICODE"), Some("ESC_INVALID_SHORT_UNICODE"), Some("ESC_INVALID_UNICODE"), 
		Some("ESC_NEWLINE"), Some("ESC_TAB"), Some("ESC_UNKNOWN"), Some("ESC_TEXT"), 
		Some("VERSION_DECLARATION_WHITESPACE"), Some("VERSION_NUMBER"), Some("SINGLE_QUOTE_DOLLAR_SIGN"), 
		Some("SINGLE_QUOTE_TILDE")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,WdlV1ParserExt<'input>, I, WdlV1ParserContextType , dyn WdlV1ParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type WdlV1ParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, WdlV1ParserContextType , dyn WdlV1ParserListener<'input> + 'a>;

/// Parser for WdlV1Parser grammar
pub struct WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				WdlV1ParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for WdlV1Parser
pub trait WdlV1ParserContext<'input>:
	for<'x> Listenable<dyn WdlV1ParserListener<'input> + 'x > + 
	for<'x> Visitable<dyn WdlV1ParserVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=WdlV1ParserContextType>
{}

antlr4rust::coerce_from!{ 'input : WdlV1ParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn WdlV1ParserContext<'input> + 'input
where
    T: WdlV1ParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn WdlV1ParserVisitor<'input> + 'x))
    }
}

impl<'input> WdlV1ParserContext<'input> for TerminalNode<'input,WdlV1ParserContextType> {}
impl<'input> WdlV1ParserContext<'input> for ErrorNode<'input,WdlV1ParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn WdlV1ParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn WdlV1ParserListener<'input> + 'input }

pub struct WdlV1ParserContextType;
antlr4rust::tid!{WdlV1ParserContextType}

impl<'input> ParserNodeType<'input> for WdlV1ParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn WdlV1ParserContext<'input> + 'input;
}

impl<'input, I> Deref for WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct WdlV1ParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> WdlV1ParserExt<'input>{
}
antlr4rust::tid! { WdlV1ParserExt<'a> }

impl<'input> TokenAware<'input> for WdlV1ParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for WdlV1ParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for WdlV1ParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "WdlV1Parser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn WdlV1ParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					95 => WdlV1Parser::<'input,I>::postfixExpression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn postfixExpression_sempred(_localctx: Option<&PostfixExpressionContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 3)
				}
				1=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- document ----------------
pub type DocumentContextAll<'input> = DocumentContext<'input>;


pub type DocumentContext<'input> = BaseParserRuleContext<'input,DocumentContextExt<'input>>;

#[derive(Clone)]
pub struct DocumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for DocumentContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for DocumentContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_document(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_document(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for DocumentContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_document(self);
	}
}

impl<'input> CustomRuleContext<'input> for DocumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_document }
	//fn type_rule_index() -> usize where Self: Sized { RULE_document }
}
antlr4rust::tid!{DocumentContextExt<'a>}

impl<'input> DocumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DocumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DocumentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DocumentContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<DocumentContextExt<'input>>{

fn versionStatement(&self) -> Option<Rc<VersionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_EOF, 0)
}
fn documentElement_all(&self) ->  Vec<Rc<DocumentElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn documentElement(&self, i: usize) -> Option<Rc<DocumentElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DocumentContextAttrs<'input> for DocumentContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn document(&mut self,)
	-> Result<Rc<DocumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DocumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_document);
        let mut _localctx: Rc<DocumentContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule versionStatement*/
			recog.base.set_state(252);
			recog.versionStatement()?;

			recog.base.set_state(256);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & 145409) != 0) {
				{
				{
				/*InvokeRule documentElement*/
				recog.base.set_state(253);
				recog.documentElement()?;

				}
				}
				recog.base.set_state(258);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(259);
			recog.base.match_token(WdlV1Parser_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- versionStatement ----------------
pub type VersionStatementContextAll<'input> = VersionStatementContext<'input>;


pub type VersionStatementContext<'input> = BaseParserRuleContext<'input,VersionStatementContextExt<'input>>;

#[derive(Clone)]
pub struct VersionStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for VersionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for VersionStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_versionStatement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_versionStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for VersionStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_versionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_versionStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_versionStatement }
}
antlr4rust::tid!{VersionStatementContextExt<'a>}

impl<'input> VersionStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VersionStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VersionStatementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<VersionStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_VERSION
/// Returns `None` if there is no child corresponding to token KEYWORD_VERSION
fn KEYWORD_VERSION(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_VERSION, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_FLOAT, 0)
}

}

impl<'input> VersionStatementContextAttrs<'input> for VersionStatementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn versionStatement(&mut self,)
	-> Result<Rc<VersionStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_versionStatement);
        let mut _localctx: Rc<VersionStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(261);
			recog.base.match_token(WdlV1Parser_KEYWORD_VERSION,&mut recog.err_handler)?;

			recog.base.set_state(262);
			recog.base.match_token(WdlV1Parser_FLOAT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- documentElement ----------------
pub type DocumentElementContextAll<'input> = DocumentElementContext<'input>;


pub type DocumentElementContext<'input> = BaseParserRuleContext<'input,DocumentElementContextExt<'input>>;

#[derive(Clone)]
pub struct DocumentElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for DocumentElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for DocumentElementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_documentElement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_documentElement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for DocumentElementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_documentElement(self);
	}
}

impl<'input> CustomRuleContext<'input> for DocumentElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_documentElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_documentElement }
}
antlr4rust::tid!{DocumentElementContextExt<'a>}

impl<'input> DocumentElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DocumentElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DocumentElementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DocumentElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<DocumentElementContextExt<'input>>{

fn importStatement(&self) -> Option<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn structDefinition(&self) -> Option<Rc<StructDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumDefinition(&self) -> Option<Rc<EnumDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn taskDefinition(&self) -> Option<Rc<TaskDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn workflowDefinition(&self) -> Option<Rc<WorkflowDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DocumentElementContextAttrs<'input> for DocumentElementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn documentElement(&mut self,)
	-> Result<Rc<DocumentElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DocumentElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_documentElement);
        let mut _localctx: Rc<DocumentElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(269);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_KEYWORD_IMPORT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule importStatement*/
					recog.base.set_state(264);
					recog.importStatement()?;

					}
				}

			WdlV1Parser_KEYWORD_STRUCT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule structDefinition*/
					recog.base.set_state(265);
					recog.structDefinition()?;

					}
				}

			WdlV1Parser_KEYWORD_ENUM 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule enumDefinition*/
					recog.base.set_state(266);
					recog.enumDefinition()?;

					}
				}

			WdlV1Parser_KEYWORD_TASK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule taskDefinition*/
					recog.base.set_state(267);
					recog.taskDefinition()?;

					}
				}

			WdlV1Parser_KEYWORD_WORKFLOW 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule workflowDefinition*/
					recog.base.set_state(268);
					recog.workflowDefinition()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- importStatement ----------------
#[derive(Debug)]
pub enum ImportStatementContextAll<'input>{
	ImportStatementMembersContext(ImportStatementMembersContext<'input>),
	ImportStatementStandardContext(ImportStatementStandardContext<'input>),
	ImportStatementStarContext(ImportStatementStarContext<'input>),
Error(ImportStatementContext<'input>)
}
antlr4rust::tid!{ImportStatementContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ImportStatementContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for ImportStatementContextAll<'input>{}

impl<'input> Deref for ImportStatementContextAll<'input>{
	type Target = dyn ImportStatementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ImportStatementContextAll::*;
		match self{
			ImportStatementMembersContext(inner) => inner,
			ImportStatementStandardContext(inner) => inner,
			ImportStatementStarContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportStatementContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportStatementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ImportStatementContext<'input> = BaseParserRuleContext<'input,ImportStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ImportStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportStatementContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportStatementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr4rust::tid!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
		ImportStatementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{


}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

pub type ImportStatementMembersContext<'input> = BaseParserRuleContext<'input,ImportStatementMembersContextExt<'input>>;

pub trait ImportStatementMembersContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_IMPORT
	/// Returns `None` if there is no child corresponding to token KEYWORD_IMPORT
	fn KEYWORD_IMPORT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_IMPORT, 0)
	}
	fn importMembers(&self) -> Option<Rc<ImportMembersContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token KEYWORD_FROM
	/// Returns `None` if there is no child corresponding to token KEYWORD_FROM
	fn KEYWORD_FROM(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_FROM, 0)
	}
	fn importUriLiteral(&self) -> Option<Rc<ImportUriLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ImportStatementMembersContextAttrs<'input> for ImportStatementMembersContext<'input>{}

pub struct ImportStatementMembersContextExt<'input>{
	base:ImportStatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ImportStatementMembersContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for ImportStatementMembersContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportStatementMembersContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_importStatementMembers(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_importStatementMembers(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportStatementMembersContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importStatementMembers(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementMembersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}

impl<'input> Borrow<ImportStatementContextExt<'input>> for ImportStatementMembersContext<'input>{
	fn borrow(&self) -> &ImportStatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ImportStatementContextExt<'input>> for ImportStatementMembersContext<'input>{
	fn borrow_mut(&mut self) -> &mut ImportStatementContextExt<'input> { &mut self.base }
}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementMembersContext<'input> {}

impl<'input> ImportStatementMembersContextExt<'input>{
	fn new(ctx: &dyn ImportStatementContextAttrs<'input>) -> Rc<ImportStatementContextAll<'input>>  {
		Rc::new(
			ImportStatementContextAll::ImportStatementMembersContext(
				BaseParserRuleContext::copy_from(ctx,ImportStatementMembersContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ImportStatementStandardContext<'input> = BaseParserRuleContext<'input,ImportStatementStandardContextExt<'input>>;

pub trait ImportStatementStandardContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_IMPORT
	/// Returns `None` if there is no child corresponding to token KEYWORD_IMPORT
	fn KEYWORD_IMPORT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_IMPORT, 0)
	}
	fn importUriLiteral(&self) -> Option<Rc<ImportUriLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token KEYWORD_AS
	/// Returns `None` if there is no child corresponding to token KEYWORD_AS
	fn KEYWORD_AS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_AS, 0)
	}
	fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn importAlias_all(&self) ->  Vec<Rc<ImportAliasContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn importAlias(&self, i: usize) -> Option<Rc<ImportAliasContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ImportStatementStandardContextAttrs<'input> for ImportStatementStandardContext<'input>{}

pub struct ImportStatementStandardContextExt<'input>{
	base:ImportStatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ImportStatementStandardContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for ImportStatementStandardContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportStatementStandardContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_importStatementStandard(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_importStatementStandard(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportStatementStandardContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importStatementStandard(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementStandardContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}

impl<'input> Borrow<ImportStatementContextExt<'input>> for ImportStatementStandardContext<'input>{
	fn borrow(&self) -> &ImportStatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ImportStatementContextExt<'input>> for ImportStatementStandardContext<'input>{
	fn borrow_mut(&mut self) -> &mut ImportStatementContextExt<'input> { &mut self.base }
}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementStandardContext<'input> {}

impl<'input> ImportStatementStandardContextExt<'input>{
	fn new(ctx: &dyn ImportStatementContextAttrs<'input>) -> Rc<ImportStatementContextAll<'input>>  {
		Rc::new(
			ImportStatementContextAll::ImportStatementStandardContext(
				BaseParserRuleContext::copy_from(ctx,ImportStatementStandardContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ImportStatementStarContext<'input> = BaseParserRuleContext<'input,ImportStatementStarContextExt<'input>>;

pub trait ImportStatementStarContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_IMPORT
	/// Returns `None` if there is no child corresponding to token KEYWORD_IMPORT
	fn KEYWORD_IMPORT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_IMPORT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ASTERISK
	/// Returns `None` if there is no child corresponding to token ASTERISK
	fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_ASTERISK, 0)
	}
	/// Retrieves first TerminalNode corresponding to token KEYWORD_FROM
	/// Returns `None` if there is no child corresponding to token KEYWORD_FROM
	fn KEYWORD_FROM(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_FROM, 0)
	}
	fn importUriLiteral(&self) -> Option<Rc<ImportUriLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ImportStatementStarContextAttrs<'input> for ImportStatementStarContext<'input>{}

pub struct ImportStatementStarContextExt<'input>{
	base:ImportStatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ImportStatementStarContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for ImportStatementStarContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportStatementStarContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_importStatementStar(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_importStatementStar(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportStatementStarContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importStatementStar(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementStarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}

impl<'input> Borrow<ImportStatementContextExt<'input>> for ImportStatementStarContext<'input>{
	fn borrow(&self) -> &ImportStatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ImportStatementContextExt<'input>> for ImportStatementStarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ImportStatementContextExt<'input> { &mut self.base }
}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementStarContext<'input> {}

impl<'input> ImportStatementStarContextExt<'input>{
	fn new(ctx: &dyn ImportStatementContextAttrs<'input>) -> Rc<ImportStatementContextAll<'input>>  {
		Rc::new(
			ImportStatementContextAll::ImportStatementStarContext(
				BaseParserRuleContext::copy_from(ctx,ImportStatementStarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn importStatement(&mut self,)
	-> Result<Rc<ImportStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_importStatement);
        let mut _localctx: Rc<ImportStatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(292);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					let tmp = ImportStatementStandardContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(271);
					recog.base.match_token(WdlV1Parser_KEYWORD_IMPORT,&mut recog.err_handler)?;

					/*InvokeRule importUriLiteral*/
					recog.base.set_state(272);
					recog.importUriLiteral()?;

					recog.base.set_state(275);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==WdlV1Parser_KEYWORD_AS {
						{
						recog.base.set_state(273);
						recog.base.match_token(WdlV1Parser_KEYWORD_AS,&mut recog.err_handler)?;

						/*InvokeRule strictIdentifier*/
						recog.base.set_state(274);
						recog.strictIdentifier()?;

						}
					}

					recog.base.set_state(280);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==WdlV1Parser_KEYWORD_ALIAS {
						{
						{
						/*InvokeRule importAlias*/
						recog.base.set_state(277);
						recog.importAlias()?;

						}
						}
						recog.base.set_state(282);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				2 =>{
					let tmp = ImportStatementStarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(283);
					recog.base.match_token(WdlV1Parser_KEYWORD_IMPORT,&mut recog.err_handler)?;

					recog.base.set_state(284);
					recog.base.match_token(WdlV1Parser_ASTERISK,&mut recog.err_handler)?;

					recog.base.set_state(285);
					recog.base.match_token(WdlV1Parser_KEYWORD_FROM,&mut recog.err_handler)?;

					/*InvokeRule importUriLiteral*/
					recog.base.set_state(286);
					recog.importUriLiteral()?;

					}
				}
			,
				3 =>{
					let tmp = ImportStatementMembersContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(287);
					recog.base.match_token(WdlV1Parser_KEYWORD_IMPORT,&mut recog.err_handler)?;

					/*InvokeRule importMembers*/
					recog.base.set_state(288);
					recog.importMembers()?;

					recog.base.set_state(289);
					recog.base.match_token(WdlV1Parser_KEYWORD_FROM,&mut recog.err_handler)?;

					/*InvokeRule importUriLiteral*/
					recog.base.set_state(290);
					recog.importUriLiteral()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- importMembers ----------------
pub type ImportMembersContextAll<'input> = ImportMembersContext<'input>;


pub type ImportMembersContext<'input> = BaseParserRuleContext<'input,ImportMembersContextExt<'input>>;

#[derive(Clone)]
pub struct ImportMembersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ImportMembersContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportMembersContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_importMembers(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_importMembers(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportMembersContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importMembers(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportMembersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importMembers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importMembers }
}
antlr4rust::tid!{ImportMembersContextExt<'a>}

impl<'input> ImportMembersContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ImportMembersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportMembersContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ImportMembersContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ImportMembersContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn importMember_all(&self) ->  Vec<Rc<ImportMemberContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importMember(&self, i: usize) -> Option<Rc<ImportMemberContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> ImportMembersContextAttrs<'input> for ImportMembersContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn importMembers(&mut self,)
	-> Result<Rc<ImportMembersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportMembersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_importMembers);
        let mut _localctx: Rc<ImportMembersContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(294);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(306);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule importMember*/
				recog.base.set_state(295);
				recog.importMember()?;

				recog.base.set_state(300);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(296);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule importMember*/
						recog.base.set_state(297);
						recog.importMember()?;

						}
						} 
					}
					recog.base.set_state(302);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
				}
				recog.base.set_state(304);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(303);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(308);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- importMember ----------------
pub type ImportMemberContextAll<'input> = ImportMemberContext<'input>;


pub type ImportMemberContext<'input> = BaseParserRuleContext<'input,ImportMemberContextExt<'input>>;

#[derive(Clone)]
pub struct ImportMemberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ImportMemberContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportMemberContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_importMember(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_importMember(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportMemberContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importMember(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportMemberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importMember }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importMember }
}
antlr4rust::tid!{ImportMemberContextExt<'a>}

impl<'input> ImportMemberContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ImportMemberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportMemberContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ImportMemberContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ImportMemberContextExt<'input>>{

fn strictIdentifier_all(&self) ->  Vec<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn strictIdentifier(&self, i: usize) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_AS
/// Returns `None` if there is no child corresponding to token KEYWORD_AS
fn KEYWORD_AS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_AS, 0)
}

}

impl<'input> ImportMemberContextAttrs<'input> for ImportMemberContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn importMember(&mut self,)
	-> Result<Rc<ImportMemberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportMemberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_importMember);
        let mut _localctx: Rc<ImportMemberContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(310);
			recog.strictIdentifier()?;

			recog.base.set_state(313);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_KEYWORD_AS {
				{
				recog.base.set_state(311);
				recog.base.match_token(WdlV1Parser_KEYWORD_AS,&mut recog.err_handler)?;

				/*InvokeRule strictIdentifier*/
				recog.base.set_state(312);
				recog.strictIdentifier()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- importUriLiteral ----------------
pub type ImportUriLiteralContextAll<'input> = ImportUriLiteralContext<'input>;


pub type ImportUriLiteralContext<'input> = BaseParserRuleContext<'input,ImportUriLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ImportUriLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ImportUriLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportUriLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_importUriLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_importUriLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportUriLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importUriLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportUriLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importUriLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importUriLiteral }
}
antlr4rust::tid!{ImportUriLiteralContextExt<'a>}

impl<'input> ImportUriLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ImportUriLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportUriLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ImportUriLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ImportUriLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTE
/// Returns `None` if there is no child corresponding to token SINGLE_QUOTE
fn SINGLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_SINGLE_QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTE_END
/// Returns `None` if there is no child corresponding to token SINGLE_QUOTE_END
fn SINGLE_QUOTE_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_SINGLE_QUOTE_END, 0)
}
fn importUriElement_all(&self) ->  Vec<Rc<ImportUriElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importUriElement(&self, i: usize) -> Option<Rc<ImportUriElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_QUOTE
/// Returns `None` if there is no child corresponding to token DOUBLE_QUOTE
fn DOUBLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOUBLE_QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_QUOTE_END
/// Returns `None` if there is no child corresponding to token DOUBLE_QUOTE_END
fn DOUBLE_QUOTE_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOUBLE_QUOTE_END, 0)
}

}

impl<'input> ImportUriLiteralContextAttrs<'input> for ImportUriLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn importUriLiteral(&mut self,)
	-> Result<Rc<ImportUriLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportUriLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_importUriLiteral);
        let mut _localctx: Rc<ImportUriLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(331);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_SINGLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(315);
					recog.base.match_token(WdlV1Parser_SINGLE_QUOTE,&mut recog.err_handler)?;

					recog.base.set_state(319);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==WdlV1Parser_STRING_TEXT || _la==WdlV1Parser_STRING_ESCAPE {
						{
						{
						/*InvokeRule importUriElement*/
						recog.base.set_state(316);
						recog.importUriElement()?;

						}
						}
						recog.base.set_state(321);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(322);
					recog.base.match_token(WdlV1Parser_SINGLE_QUOTE_END,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_DOUBLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(323);
					recog.base.match_token(WdlV1Parser_DOUBLE_QUOTE,&mut recog.err_handler)?;

					recog.base.set_state(327);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==WdlV1Parser_STRING_TEXT || _la==WdlV1Parser_STRING_ESCAPE {
						{
						{
						/*InvokeRule importUriElement*/
						recog.base.set_state(324);
						recog.importUriElement()?;

						}
						}
						recog.base.set_state(329);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(330);
					recog.base.match_token(WdlV1Parser_DOUBLE_QUOTE_END,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- importUriElement ----------------
pub type ImportUriElementContextAll<'input> = ImportUriElementContext<'input>;


pub type ImportUriElementContext<'input> = BaseParserRuleContext<'input,ImportUriElementContextExt<'input>>;

#[derive(Clone)]
pub struct ImportUriElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ImportUriElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportUriElementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_importUriElement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_importUriElement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportUriElementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importUriElement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportUriElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importUriElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importUriElement }
}
antlr4rust::tid!{ImportUriElementContextExt<'a>}

impl<'input> ImportUriElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ImportUriElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportUriElementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ImportUriElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ImportUriElementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING_TEXT
/// Returns `None` if there is no child corresponding to token STRING_TEXT
fn STRING_TEXT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_STRING_TEXT, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_ESCAPE
/// Returns `None` if there is no child corresponding to token STRING_ESCAPE
fn STRING_ESCAPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_STRING_ESCAPE, 0)
}

}

impl<'input> ImportUriElementContextAttrs<'input> for ImportUriElementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn importUriElement(&mut self,)
	-> Result<Rc<ImportUriElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportUriElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_importUriElement);
        let mut _localctx: Rc<ImportUriElementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(333);
			_la = recog.base.input.la(1);
			if { !(_la==WdlV1Parser_STRING_TEXT || _la==WdlV1Parser_STRING_ESCAPE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- importAlias ----------------
pub type ImportAliasContextAll<'input> = ImportAliasContext<'input>;


pub type ImportAliasContext<'input> = BaseParserRuleContext<'input,ImportAliasContextExt<'input>>;

#[derive(Clone)]
pub struct ImportAliasContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ImportAliasContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ImportAliasContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_importAlias(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_importAlias(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ImportAliasContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_importAlias(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportAliasContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importAlias }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importAlias }
}
antlr4rust::tid!{ImportAliasContextExt<'a>}

impl<'input> ImportAliasContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ImportAliasContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportAliasContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ImportAliasContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ImportAliasContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_ALIAS
/// Returns `None` if there is no child corresponding to token KEYWORD_ALIAS
fn KEYWORD_ALIAS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ALIAS, 0)
}
fn strictIdentifier_all(&self) ->  Vec<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn strictIdentifier(&self, i: usize) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_AS
/// Returns `None` if there is no child corresponding to token KEYWORD_AS
fn KEYWORD_AS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_AS, 0)
}

}

impl<'input> ImportAliasContextAttrs<'input> for ImportAliasContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn importAlias(&mut self,)
	-> Result<Rc<ImportAliasContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportAliasContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_importAlias);
        let mut _localctx: Rc<ImportAliasContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(335);
			recog.base.match_token(WdlV1Parser_KEYWORD_ALIAS,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(336);
			recog.strictIdentifier()?;

			recog.base.set_state(337);
			recog.base.match_token(WdlV1Parser_KEYWORD_AS,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(338);
			recog.strictIdentifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structDefinition ----------------
pub type StructDefinitionContextAll<'input> = StructDefinitionContext<'input>;


pub type StructDefinitionContext<'input> = BaseParserRuleContext<'input,StructDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct StructDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StructDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structDefinition(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structDefinition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_structDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDefinition }
}
antlr4rust::tid!{StructDefinitionContextExt<'a>}

impl<'input> StructDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDefinitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructDefinitionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StructDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_STRUCT
/// Returns `None` if there is no child corresponding to token KEYWORD_STRUCT
fn KEYWORD_STRUCT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_STRUCT, 0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn structItem_all(&self) ->  Vec<Rc<StructItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn structItem(&self, i: usize) -> Option<Rc<StructItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StructDefinitionContextAttrs<'input> for StructDefinitionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structDefinition(&mut self,)
	-> Result<Rc<StructDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_structDefinition);
        let mut _localctx: Rc<StructDefinitionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(340);
			recog.base.match_token(WdlV1Parser_KEYWORD_STRUCT,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(341);
			recog.strictIdentifier()?;

			recog.base.set_state(342);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(346);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule structItem*/
				recog.base.set_state(343);
				recog.structItem()?;

				}
				}
				recog.base.set_state(348);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(349);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structItem ----------------
#[derive(Debug)]
pub enum StructItemContextAll<'input>{
	StructItemParameterMetadataContext(StructItemParameterMetadataContext<'input>),
	StructItemMetadataContext(StructItemMetadataContext<'input>),
	StructItemMemberDeclarationContext(StructItemMemberDeclarationContext<'input>),
Error(StructItemContext<'input>)
}
antlr4rust::tid!{StructItemContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for StructItemContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for StructItemContextAll<'input>{}

impl<'input> Deref for StructItemContextAll<'input>{
	type Target = dyn StructItemContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StructItemContextAll::*;
		match self{
			StructItemParameterMetadataContext(inner) => inner,
			StructItemMetadataContext(inner) => inner,
			StructItemMemberDeclarationContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructItemContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructItemContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type StructItemContext<'input> = BaseParserRuleContext<'input,StructItemContextExt<'input>>;

#[derive(Clone)]
pub struct StructItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StructItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructItemContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructItemContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StructItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structItem }
}
antlr4rust::tid!{StructItemContextExt<'a>}

impl<'input> StructItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructItemContextAll<'input>> {
		Rc::new(
		StructItemContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructItemContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StructItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StructItemContextExt<'input>>{


}

impl<'input> StructItemContextAttrs<'input> for StructItemContext<'input>{}

pub type StructItemParameterMetadataContext<'input> = BaseParserRuleContext<'input,StructItemParameterMetadataContextExt<'input>>;

pub trait StructItemParameterMetadataContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn parameterMetadataSection(&self) -> Option<Rc<ParameterMetadataSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StructItemParameterMetadataContextAttrs<'input> for StructItemParameterMetadataContext<'input>{}

pub struct StructItemParameterMetadataContextExt<'input>{
	base:StructItemContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StructItemParameterMetadataContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StructItemParameterMetadataContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructItemParameterMetadataContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_structItemParameterMetadata(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_structItemParameterMetadata(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructItemParameterMetadataContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_structItemParameterMetadata(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructItemParameterMetadataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structItem }
}

impl<'input> Borrow<StructItemContextExt<'input>> for StructItemParameterMetadataContext<'input>{
	fn borrow(&self) -> &StructItemContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StructItemContextExt<'input>> for StructItemParameterMetadataContext<'input>{
	fn borrow_mut(&mut self) -> &mut StructItemContextExt<'input> { &mut self.base }
}

impl<'input> StructItemContextAttrs<'input> for StructItemParameterMetadataContext<'input> {}

impl<'input> StructItemParameterMetadataContextExt<'input>{
	fn new(ctx: &dyn StructItemContextAttrs<'input>) -> Rc<StructItemContextAll<'input>>  {
		Rc::new(
			StructItemContextAll::StructItemParameterMetadataContext(
				BaseParserRuleContext::copy_from(ctx,StructItemParameterMetadataContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructItemMetadataContext<'input> = BaseParserRuleContext<'input,StructItemMetadataContextExt<'input>>;

pub trait StructItemMetadataContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn metadataSection(&self) -> Option<Rc<MetadataSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StructItemMetadataContextAttrs<'input> for StructItemMetadataContext<'input>{}

pub struct StructItemMetadataContextExt<'input>{
	base:StructItemContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StructItemMetadataContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StructItemMetadataContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructItemMetadataContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_structItemMetadata(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_structItemMetadata(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructItemMetadataContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_structItemMetadata(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructItemMetadataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structItem }
}

impl<'input> Borrow<StructItemContextExt<'input>> for StructItemMetadataContext<'input>{
	fn borrow(&self) -> &StructItemContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StructItemContextExt<'input>> for StructItemMetadataContext<'input>{
	fn borrow_mut(&mut self) -> &mut StructItemContextExt<'input> { &mut self.base }
}

impl<'input> StructItemContextAttrs<'input> for StructItemMetadataContext<'input> {}

impl<'input> StructItemMetadataContextExt<'input>{
	fn new(ctx: &dyn StructItemContextAttrs<'input>) -> Rc<StructItemContextAll<'input>>  {
		Rc::new(
			StructItemContextAll::StructItemMetadataContext(
				BaseParserRuleContext::copy_from(ctx,StructItemMetadataContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructItemMemberDeclarationContext<'input> = BaseParserRuleContext<'input,StructItemMemberDeclarationContextExt<'input>>;

pub trait StructItemMemberDeclarationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn structDeclaration(&self) -> Option<Rc<StructDeclarationContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StructItemMemberDeclarationContextAttrs<'input> for StructItemMemberDeclarationContext<'input>{}

pub struct StructItemMemberDeclarationContextExt<'input>{
	base:StructItemContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StructItemMemberDeclarationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StructItemMemberDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructItemMemberDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_structItemMemberDeclaration(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_structItemMemberDeclaration(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructItemMemberDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_structItemMemberDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructItemMemberDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structItem }
}

impl<'input> Borrow<StructItemContextExt<'input>> for StructItemMemberDeclarationContext<'input>{
	fn borrow(&self) -> &StructItemContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StructItemContextExt<'input>> for StructItemMemberDeclarationContext<'input>{
	fn borrow_mut(&mut self) -> &mut StructItemContextExt<'input> { &mut self.base }
}

impl<'input> StructItemContextAttrs<'input> for StructItemMemberDeclarationContext<'input> {}

impl<'input> StructItemMemberDeclarationContextExt<'input>{
	fn new(ctx: &dyn StructItemContextAttrs<'input>) -> Rc<StructItemContextAll<'input>>  {
		Rc::new(
			StructItemContextAll::StructItemMemberDeclarationContext(
				BaseParserRuleContext::copy_from(ctx,StructItemMemberDeclarationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structItem(&mut self,)
	-> Result<Rc<StructItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_structItem);
        let mut _localctx: Rc<StructItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(354);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
				1 =>{
					let tmp = StructItemMetadataContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule metadataSection*/
					recog.base.set_state(351);
					recog.metadataSection()?;

					}
				}
			,
				2 =>{
					let tmp = StructItemParameterMetadataContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule parameterMetadataSection*/
					recog.base.set_state(352);
					recog.parameterMetadataSection()?;

					}
				}
			,
				3 =>{
					let tmp = StructItemMemberDeclarationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule structDeclaration*/
					recog.base.set_state(353);
					recog.structDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structDeclaration ----------------
pub type StructDeclarationContextAll<'input> = StructDeclarationContext<'input>;


pub type StructDeclarationContext<'input> = BaseParserRuleContext<'input,StructDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct StructDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StructDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structDeclaration(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_structDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDeclaration }
}
antlr4rust::tid!{StructDeclarationContextExt<'a>}

impl<'input> StructDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructDeclarationContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StructDeclarationContextExt<'input>>{

fn type_(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StructDeclarationContextAttrs<'input> for StructDeclarationContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structDeclaration(&mut self,)
	-> Result<Rc<StructDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_structDeclaration);
        let mut _localctx: Rc<StructDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule type_*/
			recog.base.set_state(356);
			recog.type_()?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(357);
			recog.strictIdentifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumDefinition ----------------
pub type EnumDefinitionContextAll<'input> = EnumDefinitionContext<'input>;


pub type EnumDefinitionContext<'input> = BaseParserRuleContext<'input,EnumDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct EnumDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumDefinition(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumDefinition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumDefinition }
}
antlr4rust::tid!{EnumDefinitionContextExt<'a>}

impl<'input> EnumDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumDefinitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumDefinitionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_ENUM
/// Returns `None` if there is no child corresponding to token KEYWORD_ENUM
fn KEYWORD_ENUM(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ENUM, 0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn enumTypeParameter(&self) -> Option<Rc<EnumTypeParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumChoice_all(&self) ->  Vec<Rc<EnumChoiceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumChoice(&self, i: usize) -> Option<Rc<EnumChoiceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> EnumDefinitionContextAttrs<'input> for EnumDefinitionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumDefinition(&mut self,)
	-> Result<Rc<EnumDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_enumDefinition);
        let mut _localctx: Rc<EnumDefinitionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(359);
			recog.base.match_token(WdlV1Parser_KEYWORD_ENUM,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(360);
			recog.strictIdentifier()?;

			recog.base.set_state(362);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_OPEN_BRACKET {
				{
				/*InvokeRule enumTypeParameter*/
				recog.base.set_state(361);
				recog.enumTypeParameter()?;

				}
			}

			recog.base.set_state(364);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(376);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule enumChoice*/
				recog.base.set_state(365);
				recog.enumChoice()?;

				recog.base.set_state(370);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(366);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule enumChoice*/
						recog.base.set_state(367);
						recog.enumChoice()?;

						}
						} 
					}
					recog.base.set_state(372);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
				}
				recog.base.set_state(374);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(373);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(378);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumTypeParameter ----------------
pub type EnumTypeParameterContextAll<'input> = EnumTypeParameterContext<'input>;


pub type EnumTypeParameterContext<'input> = BaseParserRuleContext<'input,EnumTypeParameterContextExt<'input>>;

#[derive(Clone)]
pub struct EnumTypeParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumTypeParameterContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumTypeParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumTypeParameter(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumTypeParameter(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumTypeParameterContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumTypeParameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumTypeParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumTypeParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumTypeParameter }
}
antlr4rust::tid!{EnumTypeParameterContextExt<'a>}

impl<'input> EnumTypeParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumTypeParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumTypeParameterContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumTypeParameterContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumTypeParameterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
fn type_(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}

}

impl<'input> EnumTypeParameterContextAttrs<'input> for EnumTypeParameterContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumTypeParameter(&mut self,)
	-> Result<Rc<EnumTypeParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumTypeParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_enumTypeParameter);
        let mut _localctx: Rc<EnumTypeParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(380);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			/*InvokeRule type_*/
			recog.base.set_state(381);
			recog.type_()?;

			recog.base.set_state(382);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumChoice ----------------
pub type EnumChoiceContextAll<'input> = EnumChoiceContext<'input>;


pub type EnumChoiceContext<'input> = BaseParserRuleContext<'input,EnumChoiceContextExt<'input>>;

#[derive(Clone)]
pub struct EnumChoiceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumChoiceContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumChoiceContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumChoice(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumChoice(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumChoiceContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumChoice(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumChoiceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumChoice }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumChoice }
}
antlr4rust::tid!{EnumChoiceContextExt<'a>}

impl<'input> EnumChoiceContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumChoiceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumChoiceContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumChoiceContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumChoiceContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
/// Returns `None` if there is no child corresponding to token ASSIGNMENT
fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_ASSIGNMENT, 0)
}
fn enumLiteralExpression(&self) -> Option<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumChoiceContextAttrs<'input> for EnumChoiceContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumChoice(&mut self,)
	-> Result<Rc<EnumChoiceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumChoiceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_enumChoice);
        let mut _localctx: Rc<EnumChoiceContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(384);
			recog.strictIdentifier()?;

			recog.base.set_state(387);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_ASSIGNMENT {
				{
				recog.base.set_state(385);
				recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

				/*InvokeRule enumLiteralExpression*/
				recog.base.set_state(386);
				recog.enumLiteralExpression()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumLiteralExpression ----------------
pub type EnumLiteralExpressionContextAll<'input> = EnumLiteralExpressionContext<'input>;


pub type EnumLiteralExpressionContext<'input> = BaseParserRuleContext<'input,EnumLiteralExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct EnumLiteralExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumLiteralExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumLiteralExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumLiteralExpression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumLiteralExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumLiteralExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumLiteralExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumLiteralExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumLiteralExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumLiteralExpression }
}
antlr4rust::tid!{EnumLiteralExpressionContextExt<'a>}

impl<'input> EnumLiteralExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumLiteralExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumLiteralExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumLiteralExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumLiteralExpressionContextExt<'input>>{

fn numberLiteralSigned(&self) -> Option<Rc<NumberLiteralSignedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn booleanLiteral(&self) -> Option<Rc<BooleanLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumStringLiteral(&self) -> Option<Rc<EnumStringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumArrayLiteral(&self) -> Option<Rc<EnumArrayLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumMapLiteral(&self) -> Option<Rc<EnumMapLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumObjectLiteral(&self) -> Option<Rc<EnumObjectLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumStructLiteral(&self) -> Option<Rc<EnumStructLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumPairLiteral(&self) -> Option<Rc<EnumPairLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumLiteralExpressionContextAttrs<'input> for EnumLiteralExpressionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumLiteralExpression(&mut self,)
	-> Result<Rc<EnumLiteralExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumLiteralExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_enumLiteralExpression);
        let mut _localctx: Rc<EnumLiteralExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(397);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule numberLiteralSigned*/
					recog.base.set_state(389);
					recog.numberLiteralSigned()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule booleanLiteral*/
					recog.base.set_state(390);
					recog.booleanLiteral()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule enumStringLiteral*/
					recog.base.set_state(391);
					recog.enumStringLiteral()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule enumArrayLiteral*/
					recog.base.set_state(392);
					recog.enumArrayLiteral()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule enumMapLiteral*/
					recog.base.set_state(393);
					recog.enumMapLiteral()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule enumObjectLiteral*/
					recog.base.set_state(394);
					recog.enumObjectLiteral()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					/*InvokeRule enumStructLiteral*/
					recog.base.set_state(395);
					recog.enumStructLiteral()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					/*InvokeRule enumPairLiteral*/
					recog.base.set_state(396);
					recog.enumPairLiteral()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumStringLiteral ----------------
pub type EnumStringLiteralContextAll<'input> = EnumStringLiteralContext<'input>;


pub type EnumStringLiteralContext<'input> = BaseParserRuleContext<'input,EnumStringLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct EnumStringLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumStringLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumStringLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumStringLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumStringLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumStringLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumStringLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumStringLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumStringLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumStringLiteral }
}
antlr4rust::tid!{EnumStringLiteralContextExt<'a>}

impl<'input> EnumStringLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumStringLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumStringLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumStringLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumStringLiteralContextExt<'input>>{

fn enumQuotedString(&self) -> Option<Rc<EnumQuotedStringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumMultilineString(&self) -> Option<Rc<EnumMultilineStringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumStringLiteralContextAttrs<'input> for EnumStringLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumStringLiteral(&mut self,)
	-> Result<Rc<EnumStringLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumStringLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_enumStringLiteral);
        let mut _localctx: Rc<EnumStringLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(401);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_SINGLE_QUOTE |WdlV1Parser_DOUBLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule enumQuotedString*/
					recog.base.set_state(399);
					recog.enumQuotedString()?;

					}
				}

			WdlV1Parser_OPEN_MULTILINE_STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule enumMultilineString*/
					recog.base.set_state(400);
					recog.enumMultilineString()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumQuotedString ----------------
pub type EnumQuotedStringContextAll<'input> = EnumQuotedStringContext<'input>;


pub type EnumQuotedStringContext<'input> = BaseParserRuleContext<'input,EnumQuotedStringContextExt<'input>>;

#[derive(Clone)]
pub struct EnumQuotedStringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumQuotedStringContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumQuotedStringContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumQuotedString(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumQuotedString(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumQuotedStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumQuotedString(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumQuotedStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumQuotedString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumQuotedString }
}
antlr4rust::tid!{EnumQuotedStringContextExt<'a>}

impl<'input> EnumQuotedStringContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumQuotedStringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumQuotedStringContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumQuotedStringContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumQuotedStringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTE
/// Returns `None` if there is no child corresponding to token SINGLE_QUOTE
fn SINGLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_SINGLE_QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTE_END
/// Returns `None` if there is no child corresponding to token SINGLE_QUOTE_END
fn SINGLE_QUOTE_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_SINGLE_QUOTE_END, 0)
}
fn enumStringElement_all(&self) ->  Vec<Rc<EnumStringElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumStringElement(&self, i: usize) -> Option<Rc<EnumStringElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_QUOTE
/// Returns `None` if there is no child corresponding to token DOUBLE_QUOTE
fn DOUBLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOUBLE_QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_QUOTE_END
/// Returns `None` if there is no child corresponding to token DOUBLE_QUOTE_END
fn DOUBLE_QUOTE_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOUBLE_QUOTE_END, 0)
}

}

impl<'input> EnumQuotedStringContextAttrs<'input> for EnumQuotedStringContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumQuotedString(&mut self,)
	-> Result<Rc<EnumQuotedStringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumQuotedStringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_enumQuotedString);
        let mut _localctx: Rc<EnumQuotedStringContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(419);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_SINGLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(403);
					recog.base.match_token(WdlV1Parser_SINGLE_QUOTE,&mut recog.err_handler)?;

					recog.base.set_state(407);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & 30) != 0) {
						{
						{
						/*InvokeRule enumStringElement*/
						recog.base.set_state(404);
						recog.enumStringElement()?;

						}
						}
						recog.base.set_state(409);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(410);
					recog.base.match_token(WdlV1Parser_SINGLE_QUOTE_END,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_DOUBLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(411);
					recog.base.match_token(WdlV1Parser_DOUBLE_QUOTE,&mut recog.err_handler)?;

					recog.base.set_state(415);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & 30) != 0) {
						{
						{
						/*InvokeRule enumStringElement*/
						recog.base.set_state(412);
						recog.enumStringElement()?;

						}
						}
						recog.base.set_state(417);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(418);
					recog.base.match_token(WdlV1Parser_DOUBLE_QUOTE_END,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumStringElement ----------------
pub type EnumStringElementContextAll<'input> = EnumStringElementContext<'input>;


pub type EnumStringElementContext<'input> = BaseParserRuleContext<'input,EnumStringElementContextExt<'input>>;

#[derive(Clone)]
pub struct EnumStringElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumStringElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumStringElementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumStringElement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumStringElement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumStringElementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumStringElement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumStringElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumStringElement }
}
antlr4rust::tid!{EnumStringElementContextExt<'a>}

impl<'input> EnumStringElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumStringElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumStringElementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumStringElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumStringElementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING_TEXT
/// Returns `None` if there is no child corresponding to token STRING_TEXT
fn STRING_TEXT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_STRING_TEXT, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_ESCAPE
/// Returns `None` if there is no child corresponding to token STRING_ESCAPE
fn STRING_ESCAPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_STRING_ESCAPE, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_DOLLAR_SIGN
/// Returns `None` if there is no child corresponding to token STRING_DOLLAR_SIGN
fn STRING_DOLLAR_SIGN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_STRING_DOLLAR_SIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_TILDE
/// Returns `None` if there is no child corresponding to token STRING_TILDE
fn STRING_TILDE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_STRING_TILDE, 0)
}

}

impl<'input> EnumStringElementContextAttrs<'input> for EnumStringElementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumStringElement(&mut self,)
	-> Result<Rc<EnumStringElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumStringElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_enumStringElement);
        let mut _localctx: Rc<EnumStringElementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(421);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 30) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumMultilineString ----------------
pub type EnumMultilineStringContextAll<'input> = EnumMultilineStringContext<'input>;


pub type EnumMultilineStringContext<'input> = BaseParserRuleContext<'input,EnumMultilineStringContextExt<'input>>;

#[derive(Clone)]
pub struct EnumMultilineStringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumMultilineStringContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumMultilineStringContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumMultilineString(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumMultilineString(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumMultilineStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumMultilineString(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumMultilineStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumMultilineString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumMultilineString }
}
antlr4rust::tid!{EnumMultilineStringContextExt<'a>}

impl<'input> EnumMultilineStringContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumMultilineStringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumMultilineStringContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumMultilineStringContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumMultilineStringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_MULTILINE_STRING
/// Returns `None` if there is no child corresponding to token OPEN_MULTILINE_STRING
fn OPEN_MULTILINE_STRING(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_MULTILINE_STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_END
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_END
fn MULTILINE_STRING_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_END, 0)
}
fn enumMultilineStringElement_all(&self) ->  Vec<Rc<EnumMultilineStringElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumMultilineStringElement(&self, i: usize) -> Option<Rc<EnumMultilineStringElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EnumMultilineStringContextAttrs<'input> for EnumMultilineStringContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumMultilineString(&mut self,)
	-> Result<Rc<EnumMultilineStringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumMultilineStringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_enumMultilineString);
        let mut _localctx: Rc<EnumMultilineStringContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(423);
			recog.base.match_token(WdlV1Parser_OPEN_MULTILINE_STRING,&mut recog.err_handler)?;

			recog.base.set_state(427);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 86)) & !0x3f) == 0 && ((1usize << (_la - 86)) & 125) != 0) {
				{
				{
				/*InvokeRule enumMultilineStringElement*/
				recog.base.set_state(424);
				recog.enumMultilineStringElement()?;

				}
				}
				recog.base.set_state(429);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(430);
			recog.base.match_token(WdlV1Parser_MULTILINE_STRING_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumMultilineStringElement ----------------
pub type EnumMultilineStringElementContextAll<'input> = EnumMultilineStringElementContext<'input>;


pub type EnumMultilineStringElementContext<'input> = BaseParserRuleContext<'input,EnumMultilineStringElementContextExt<'input>>;

#[derive(Clone)]
pub struct EnumMultilineStringElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumMultilineStringElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumMultilineStringElementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumMultilineStringElement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumMultilineStringElement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumMultilineStringElementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumMultilineStringElement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumMultilineStringElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumMultilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumMultilineStringElement }
}
antlr4rust::tid!{EnumMultilineStringElementContextExt<'a>}

impl<'input> EnumMultilineStringElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumMultilineStringElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumMultilineStringElementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumMultilineStringElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumMultilineStringElementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_TEXT
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_TEXT
fn MULTILINE_STRING_TEXT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_TEXT, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_ESCAPE
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_ESCAPE
fn MULTILINE_STRING_ESCAPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_ESCAPE, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_DOUBLE_CLOSE_ANGLE
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_DOUBLE_CLOSE_ANGLE
fn MULTILINE_STRING_DOUBLE_CLOSE_ANGLE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_DOUBLE_CLOSE_ANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_SINGLE_CLOSE_ANGLE
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_SINGLE_CLOSE_ANGLE
fn MULTILINE_STRING_SINGLE_CLOSE_ANGLE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_SINGLE_CLOSE_ANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_DOLLAR_SIGN
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_DOLLAR_SIGN
fn MULTILINE_STRING_DOLLAR_SIGN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_DOLLAR_SIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_TILDE
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_TILDE
fn MULTILINE_STRING_TILDE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_TILDE, 0)
}

}

impl<'input> EnumMultilineStringElementContextAttrs<'input> for EnumMultilineStringElementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumMultilineStringElement(&mut self,)
	-> Result<Rc<EnumMultilineStringElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumMultilineStringElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_enumMultilineStringElement);
        let mut _localctx: Rc<EnumMultilineStringElementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(432);
			_la = recog.base.input.la(1);
			if { !(((((_la - 86)) & !0x3f) == 0 && ((1usize << (_la - 86)) & 125) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumArrayLiteral ----------------
pub type EnumArrayLiteralContextAll<'input> = EnumArrayLiteralContext<'input>;


pub type EnumArrayLiteralContext<'input> = BaseParserRuleContext<'input,EnumArrayLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct EnumArrayLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumArrayLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumArrayLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumArrayLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumArrayLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumArrayLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumArrayLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumArrayLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumArrayLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumArrayLiteral }
}
antlr4rust::tid!{EnumArrayLiteralContextExt<'a>}

impl<'input> EnumArrayLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumArrayLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumArrayLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumArrayLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumArrayLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
fn enumLiteralExpression_all(&self) ->  Vec<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumLiteralExpression(&self, i: usize) -> Option<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> EnumArrayLiteralContextAttrs<'input> for EnumArrayLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumArrayLiteral(&mut self,)
	-> Result<Rc<EnumArrayLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumArrayLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_enumArrayLiteral);
        let mut _localctx: Rc<EnumArrayLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(434);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(446);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294966720) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 1344274431) != 0) || _la==WdlV1Parser_OPEN_PAREN || _la==WdlV1Parser_MINUS {
				{
				/*InvokeRule enumLiteralExpression*/
				recog.base.set_state(435);
				recog.enumLiteralExpression()?;

				recog.base.set_state(440);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(436);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule enumLiteralExpression*/
						recog.base.set_state(437);
						recog.enumLiteralExpression()?;

						}
						} 
					}
					recog.base.set_state(442);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
				}
				recog.base.set_state(444);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(443);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(448);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumMapLiteral ----------------
pub type EnumMapLiteralContextAll<'input> = EnumMapLiteralContext<'input>;


pub type EnumMapLiteralContext<'input> = BaseParserRuleContext<'input,EnumMapLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct EnumMapLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumMapLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumMapLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumMapLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumMapLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumMapLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumMapLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumMapLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumMapLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumMapLiteral }
}
antlr4rust::tid!{EnumMapLiteralContextExt<'a>}

impl<'input> EnumMapLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumMapLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumMapLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumMapLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumMapLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn enumMapLiteralItem_all(&self) ->  Vec<Rc<EnumMapLiteralItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumMapLiteralItem(&self, i: usize) -> Option<Rc<EnumMapLiteralItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> EnumMapLiteralContextAttrs<'input> for EnumMapLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumMapLiteral(&mut self,)
	-> Result<Rc<EnumMapLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumMapLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_enumMapLiteral);
        let mut _localctx: Rc<EnumMapLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(450);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(462);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294966720) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 1344274431) != 0) || _la==WdlV1Parser_OPEN_PAREN || _la==WdlV1Parser_MINUS {
				{
				/*InvokeRule enumMapLiteralItem*/
				recog.base.set_state(451);
				recog.enumMapLiteralItem()?;

				recog.base.set_state(456);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(28,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(452);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule enumMapLiteralItem*/
						recog.base.set_state(453);
						recog.enumMapLiteralItem()?;

						}
						} 
					}
					recog.base.set_state(458);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(28,&mut recog.base)?;
				}
				recog.base.set_state(460);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(459);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(464);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumMapLiteralItem ----------------
pub type EnumMapLiteralItemContextAll<'input> = EnumMapLiteralItemContext<'input>;


pub type EnumMapLiteralItemContext<'input> = BaseParserRuleContext<'input,EnumMapLiteralItemContextExt<'input>>;

#[derive(Clone)]
pub struct EnumMapLiteralItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumMapLiteralItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumMapLiteralItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumMapLiteralItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumMapLiteralItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumMapLiteralItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumMapLiteralItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumMapLiteralItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumMapLiteralItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumMapLiteralItem }
}
antlr4rust::tid!{EnumMapLiteralItemContextExt<'a>}

impl<'input> EnumMapLiteralItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumMapLiteralItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumMapLiteralItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumMapLiteralItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumMapLiteralItemContextExt<'input>>{

fn enumLiteralExpression_all(&self) ->  Vec<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumLiteralExpression(&self, i: usize) -> Option<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}

}

impl<'input> EnumMapLiteralItemContextAttrs<'input> for EnumMapLiteralItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumMapLiteralItem(&mut self,)
	-> Result<Rc<EnumMapLiteralItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumMapLiteralItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_enumMapLiteralItem);
        let mut _localctx: Rc<EnumMapLiteralItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule enumLiteralExpression*/
			recog.base.set_state(466);
			recog.enumLiteralExpression()?;

			recog.base.set_state(467);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule enumLiteralExpression*/
			recog.base.set_state(468);
			recog.enumLiteralExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumObjectLiteral ----------------
pub type EnumObjectLiteralContextAll<'input> = EnumObjectLiteralContext<'input>;


pub type EnumObjectLiteralContext<'input> = BaseParserRuleContext<'input,EnumObjectLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct EnumObjectLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumObjectLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumObjectLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumObjectLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumObjectLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumObjectLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumObjectLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumObjectLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumObjectLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumObjectLiteral }
}
antlr4rust::tid!{EnumObjectLiteralContextExt<'a>}

impl<'input> EnumObjectLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumObjectLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumObjectLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumObjectLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumObjectLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_OBJECT
/// Returns `None` if there is no child corresponding to token KEYWORD_OBJECT
fn KEYWORD_OBJECT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OBJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn enumObjectLiteralItem_all(&self) ->  Vec<Rc<EnumObjectLiteralItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumObjectLiteralItem(&self, i: usize) -> Option<Rc<EnumObjectLiteralItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> EnumObjectLiteralContextAttrs<'input> for EnumObjectLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumObjectLiteral(&mut self,)
	-> Result<Rc<EnumObjectLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumObjectLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_enumObjectLiteral);
        let mut _localctx: Rc<EnumObjectLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(470);
			recog.base.match_token(WdlV1Parser_KEYWORD_OBJECT,&mut recog.err_handler)?;

			recog.base.set_state(471);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(483);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule enumObjectLiteralItem*/
				recog.base.set_state(472);
				recog.enumObjectLiteralItem()?;

				recog.base.set_state(477);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(31,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(473);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule enumObjectLiteralItem*/
						recog.base.set_state(474);
						recog.enumObjectLiteralItem()?;

						}
						} 
					}
					recog.base.set_state(479);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(31,&mut recog.base)?;
				}
				recog.base.set_state(481);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(480);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(485);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumObjectLiteralItem ----------------
pub type EnumObjectLiteralItemContextAll<'input> = EnumObjectLiteralItemContext<'input>;


pub type EnumObjectLiteralItemContext<'input> = BaseParserRuleContext<'input,EnumObjectLiteralItemContextExt<'input>>;

#[derive(Clone)]
pub struct EnumObjectLiteralItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumObjectLiteralItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumObjectLiteralItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumObjectLiteralItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumObjectLiteralItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumObjectLiteralItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumObjectLiteralItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumObjectLiteralItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumObjectLiteralItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumObjectLiteralItem }
}
antlr4rust::tid!{EnumObjectLiteralItemContextExt<'a>}

impl<'input> EnumObjectLiteralItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumObjectLiteralItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumObjectLiteralItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumObjectLiteralItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumObjectLiteralItemContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn enumLiteralExpression(&self) -> Option<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumObjectLiteralItemContextAttrs<'input> for EnumObjectLiteralItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumObjectLiteralItem(&mut self,)
	-> Result<Rc<EnumObjectLiteralItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumObjectLiteralItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_enumObjectLiteralItem);
        let mut _localctx: Rc<EnumObjectLiteralItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(487);
			recog.strictIdentifier()?;

			recog.base.set_state(488);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule enumLiteralExpression*/
			recog.base.set_state(489);
			recog.enumLiteralExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumStructLiteral ----------------
pub type EnumStructLiteralContextAll<'input> = EnumStructLiteralContext<'input>;


pub type EnumStructLiteralContext<'input> = BaseParserRuleContext<'input,EnumStructLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct EnumStructLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumStructLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumStructLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumStructLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumStructLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumStructLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumStructLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumStructLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumStructLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumStructLiteral }
}
antlr4rust::tid!{EnumStructLiteralContextExt<'a>}

impl<'input> EnumStructLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumStructLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumStructLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumStructLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumStructLiteralContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn enumStructLiteralItem_all(&self) ->  Vec<Rc<EnumStructLiteralItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumStructLiteralItem(&self, i: usize) -> Option<Rc<EnumStructLiteralItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> EnumStructLiteralContextAttrs<'input> for EnumStructLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumStructLiteral(&mut self,)
	-> Result<Rc<EnumStructLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumStructLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_enumStructLiteral);
        let mut _localctx: Rc<EnumStructLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(491);
			recog.strictIdentifier()?;

			recog.base.set_state(492);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(504);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule enumStructLiteralItem*/
				recog.base.set_state(493);
				recog.enumStructLiteralItem()?;

				recog.base.set_state(498);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(494);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule enumStructLiteralItem*/
						recog.base.set_state(495);
						recog.enumStructLiteralItem()?;

						}
						} 
					}
					recog.base.set_state(500);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
				}
				recog.base.set_state(502);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(501);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(506);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumStructLiteralItem ----------------
pub type EnumStructLiteralItemContextAll<'input> = EnumStructLiteralItemContext<'input>;


pub type EnumStructLiteralItemContext<'input> = BaseParserRuleContext<'input,EnumStructLiteralItemContextExt<'input>>;

#[derive(Clone)]
pub struct EnumStructLiteralItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumStructLiteralItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumStructLiteralItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumStructLiteralItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumStructLiteralItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumStructLiteralItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumStructLiteralItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumStructLiteralItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumStructLiteralItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumStructLiteralItem }
}
antlr4rust::tid!{EnumStructLiteralItemContextExt<'a>}

impl<'input> EnumStructLiteralItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumStructLiteralItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumStructLiteralItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumStructLiteralItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumStructLiteralItemContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn enumLiteralExpression(&self) -> Option<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumStructLiteralItemContextAttrs<'input> for EnumStructLiteralItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumStructLiteralItem(&mut self,)
	-> Result<Rc<EnumStructLiteralItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumStructLiteralItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_enumStructLiteralItem);
        let mut _localctx: Rc<EnumStructLiteralItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(508);
			recog.strictIdentifier()?;

			recog.base.set_state(509);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule enumLiteralExpression*/
			recog.base.set_state(510);
			recog.enumLiteralExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumPairLiteral ----------------
pub type EnumPairLiteralContextAll<'input> = EnumPairLiteralContext<'input>;


pub type EnumPairLiteralContext<'input> = BaseParserRuleContext<'input,EnumPairLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct EnumPairLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EnumPairLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EnumPairLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumPairLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumPairLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EnumPairLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_enumPairLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumPairLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumPairLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumPairLiteral }
}
antlr4rust::tid!{EnumPairLiteralContextExt<'a>}

impl<'input> EnumPairLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumPairLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumPairLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumPairLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EnumPairLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_PAREN
/// Returns `None` if there is no child corresponding to token OPEN_PAREN
fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_PAREN, 0)
}
fn enumLiteralExpression_all(&self) ->  Vec<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumLiteralExpression(&self, i: usize) -> Option<Rc<EnumLiteralExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
/// Returns `None` if there is no child corresponding to token CLOSE_PAREN
fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_PAREN, 0)
}

}

impl<'input> EnumPairLiteralContextAttrs<'input> for EnumPairLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumPairLiteral(&mut self,)
	-> Result<Rc<EnumPairLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumPairLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_enumPairLiteral);
        let mut _localctx: Rc<EnumPairLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(512);
			recog.base.match_token(WdlV1Parser_OPEN_PAREN,&mut recog.err_handler)?;

			/*InvokeRule enumLiteralExpression*/
			recog.base.set_state(513);
			recog.enumLiteralExpression()?;

			recog.base.set_state(514);
			recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

			/*InvokeRule enumLiteralExpression*/
			recog.base.set_state(515);
			recog.enumLiteralExpression()?;

			recog.base.set_state(516);
			recog.base.match_token(WdlV1Parser_CLOSE_PAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- taskDefinition ----------------
pub type TaskDefinitionContextAll<'input> = TaskDefinitionContext<'input>;


pub type TaskDefinitionContext<'input> = BaseParserRuleContext<'input,TaskDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct TaskDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for TaskDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_taskDefinition(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_taskDefinition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskDefinition }
}
antlr4rust::tid!{TaskDefinitionContextExt<'a>}

impl<'input> TaskDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TaskDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TaskDefinitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TaskDefinitionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<TaskDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_TASK
/// Returns `None` if there is no child corresponding to token KEYWORD_TASK
fn KEYWORD_TASK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_TASK, 0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn taskElement_all(&self) ->  Vec<Rc<TaskElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn taskElement(&self, i: usize) -> Option<Rc<TaskElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TaskDefinitionContextAttrs<'input> for TaskDefinitionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn taskDefinition(&mut self,)
	-> Result<Rc<TaskDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TaskDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_taskDefinition);
        let mut _localctx: Rc<TaskDefinitionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(518);
			recog.base.match_token(WdlV1Parser_KEYWORD_TASK,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(519);
			recog.strictIdentifier()?;

			recog.base.set_state(520);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(524);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule taskElement*/
				recog.base.set_state(521);
				recog.taskElement()?;

				}
				}
				recog.base.set_state(526);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(527);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- workflowDefinition ----------------
pub type WorkflowDefinitionContextAll<'input> = WorkflowDefinitionContext<'input>;


pub type WorkflowDefinitionContext<'input> = BaseParserRuleContext<'input,WorkflowDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct WorkflowDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for WorkflowDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_workflowDefinition(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_workflowDefinition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowDefinition }
}
antlr4rust::tid!{WorkflowDefinitionContextExt<'a>}

impl<'input> WorkflowDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<WorkflowDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WorkflowDefinitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait WorkflowDefinitionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<WorkflowDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_WORKFLOW
/// Returns `None` if there is no child corresponding to token KEYWORD_WORKFLOW
fn KEYWORD_WORKFLOW(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_WORKFLOW, 0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn workflowElement_all(&self) ->  Vec<Rc<WorkflowElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn workflowElement(&self, i: usize) -> Option<Rc<WorkflowElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> WorkflowDefinitionContextAttrs<'input> for WorkflowDefinitionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn workflowDefinition(&mut self,)
	-> Result<Rc<WorkflowDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WorkflowDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_workflowDefinition);
        let mut _localctx: Rc<WorkflowDefinitionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(529);
			recog.base.match_token(WdlV1Parser_KEYWORD_WORKFLOW,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(530);
			recog.strictIdentifier()?;

			recog.base.set_state(531);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(535);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule workflowElement*/
				recog.base.set_state(532);
				recog.workflowElement()?;

				}
				}
				recog.base.set_state(537);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(538);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- type ----------------
pub type TypeContextAll<'input> = TypeContext<'input>;


pub type TypeContext<'input> = BaseParserRuleContext<'input,TypeContextExt<'input>>;

#[derive(Clone)]
pub struct TypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for TypeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TypeContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_type(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_type(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_type(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type }
}
antlr4rust::tid!{TypeContextExt<'a>}

impl<'input> TypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<TypeContextExt<'input>>{

fn mapType(&self) -> Option<Rc<MapTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arrayType(&self) -> Option<Rc<ArrayTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pairType(&self) -> Option<Rc<PairTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn objectType(&self) -> Option<Rc<ObjectTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn primitiveType(&self) -> Option<Rc<PrimitiveTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeRefType(&self) -> Option<Rc<TypeRefTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeContextAttrs<'input> for TypeContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn type_(&mut self,)
	-> Result<Rc<TypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_type);
        let mut _localctx: Rc<TypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(546);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule mapType*/
					recog.base.set_state(540);
					recog.mapType()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule arrayType*/
					recog.base.set_state(541);
					recog.arrayType()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule pairType*/
					recog.base.set_state(542);
					recog.pairType()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule objectType*/
					recog.base.set_state(543);
					recog.objectType()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule primitiveType*/
					recog.base.set_state(544);
					recog.primitiveType()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule typeRefType*/
					recog.base.set_state(545);
					recog.typeRefType()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- mapType ----------------
pub type MapTypeContextAll<'input> = MapTypeContext<'input>;


pub type MapTypeContext<'input> = BaseParserRuleContext<'input,MapTypeContextExt<'input>>;

#[derive(Clone)]
pub struct MapTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MapTypeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MapTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mapType(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mapType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MapTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_mapType(self);
	}
}

impl<'input> CustomRuleContext<'input> for MapTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapType }
}
antlr4rust::tid!{MapTypeContextExt<'a>}

impl<'input> MapTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MapTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MapTypeContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MapTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_MAP_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_MAP_TYPE
fn KEYWORD_MAP_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_MAP_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
fn primitiveType(&self) -> Option<Rc<PrimitiveTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, 0)
}
fn type_(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION_MARK
/// Returns `None` if there is no child corresponding to token QUESTION_MARK
fn QUESTION_MARK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_QUESTION_MARK, 0)
}

}

impl<'input> MapTypeContextAttrs<'input> for MapTypeContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn mapType(&mut self,)
	-> Result<Rc<MapTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_mapType);
        let mut _localctx: Rc<MapTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(548);
			recog.base.match_token(WdlV1Parser_KEYWORD_MAP_TYPE,&mut recog.err_handler)?;

			recog.base.set_state(549);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			/*InvokeRule primitiveType*/
			recog.base.set_state(550);
			recog.primitiveType()?;

			recog.base.set_state(551);
			recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

			/*InvokeRule type_*/
			recog.base.set_state(552);
			recog.type_()?;

			recog.base.set_state(553);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(555);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_QUESTION_MARK {
				{
				recog.base.set_state(554);
				recog.base.match_token(WdlV1Parser_QUESTION_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- arrayType ----------------
pub type ArrayTypeContextAll<'input> = ArrayTypeContext<'input>;


pub type ArrayTypeContext<'input> = BaseParserRuleContext<'input,ArrayTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ArrayTypeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ArrayTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_arrayType(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_arrayType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ArrayTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_arrayType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayType }
}
antlr4rust::tid!{ArrayTypeContextExt<'a>}

impl<'input> ArrayTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ArrayTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayTypeContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ArrayTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_ARRAY_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_ARRAY_TYPE
fn KEYWORD_ARRAY_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ARRAY_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
fn type_(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION_MARK
/// Returns `None` if there is no child corresponding to token QUESTION_MARK
fn QUESTION_MARK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_QUESTION_MARK, 0)
}

}

impl<'input> ArrayTypeContextAttrs<'input> for ArrayTypeContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn arrayType(&mut self,)
	-> Result<Rc<ArrayTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_arrayType);
        let mut _localctx: Rc<ArrayTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(557);
			recog.base.match_token(WdlV1Parser_KEYWORD_ARRAY_TYPE,&mut recog.err_handler)?;

			recog.base.set_state(558);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			/*InvokeRule type_*/
			recog.base.set_state(559);
			recog.type_()?;

			recog.base.set_state(560);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(562);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_PLUS {
				{
				recog.base.set_state(561);
				recog.base.match_token(WdlV1Parser_PLUS,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(565);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_QUESTION_MARK {
				{
				recog.base.set_state(564);
				recog.base.match_token(WdlV1Parser_QUESTION_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- pairType ----------------
pub type PairTypeContextAll<'input> = PairTypeContext<'input>;


pub type PairTypeContext<'input> = BaseParserRuleContext<'input,PairTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PairTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for PairTypeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PairTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_pairType(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_pairType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PairTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_pairType(self);
	}
}

impl<'input> CustomRuleContext<'input> for PairTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pairType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pairType }
}
antlr4rust::tid!{PairTypeContextExt<'a>}

impl<'input> PairTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PairTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PairTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PairTypeContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<PairTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_PAIR_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_PAIR_TYPE
fn KEYWORD_PAIR_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_PAIR_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
fn type_all(&self) ->  Vec<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn type_(&self, i: usize) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION_MARK
/// Returns `None` if there is no child corresponding to token QUESTION_MARK
fn QUESTION_MARK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_QUESTION_MARK, 0)
}

}

impl<'input> PairTypeContextAttrs<'input> for PairTypeContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn pairType(&mut self,)
	-> Result<Rc<PairTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PairTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_pairType);
        let mut _localctx: Rc<PairTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(567);
			recog.base.match_token(WdlV1Parser_KEYWORD_PAIR_TYPE,&mut recog.err_handler)?;

			recog.base.set_state(568);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			/*InvokeRule type_*/
			recog.base.set_state(569);
			recog.type_()?;

			recog.base.set_state(570);
			recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

			/*InvokeRule type_*/
			recog.base.set_state(571);
			recog.type_()?;

			recog.base.set_state(572);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(574);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_QUESTION_MARK {
				{
				recog.base.set_state(573);
				recog.base.match_token(WdlV1Parser_QUESTION_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- objectType ----------------
pub type ObjectTypeContextAll<'input> = ObjectTypeContext<'input>;


pub type ObjectTypeContext<'input> = BaseParserRuleContext<'input,ObjectTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ObjectTypeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ObjectTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_objectType(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_objectType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ObjectTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_objectType(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectType }
}
antlr4rust::tid!{ObjectTypeContextExt<'a>}

impl<'input> ObjectTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ObjectTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectTypeContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ObjectTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_OBJECT_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_OBJECT_TYPE
fn KEYWORD_OBJECT_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OBJECT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION_MARK
/// Returns `None` if there is no child corresponding to token QUESTION_MARK
fn QUESTION_MARK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_QUESTION_MARK, 0)
}

}

impl<'input> ObjectTypeContextAttrs<'input> for ObjectTypeContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn objectType(&mut self,)
	-> Result<Rc<ObjectTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_objectType);
        let mut _localctx: Rc<ObjectTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(576);
			recog.base.match_token(WdlV1Parser_KEYWORD_OBJECT_TYPE,&mut recog.err_handler)?;

			recog.base.set_state(578);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_QUESTION_MARK {
				{
				recog.base.set_state(577);
				recog.base.match_token(WdlV1Parser_QUESTION_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- primitiveType ----------------
pub type PrimitiveTypeContextAll<'input> = PrimitiveTypeContext<'input>;


pub type PrimitiveTypeContext<'input> = BaseParserRuleContext<'input,PrimitiveTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrimitiveTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for PrimitiveTypeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PrimitiveTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_primitiveType(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_primitiveType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PrimitiveTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_primitiveType(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimitiveTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primitiveType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primitiveType }
}
antlr4rust::tid!{PrimitiveTypeContextExt<'a>}

impl<'input> PrimitiveTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrimitiveTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimitiveTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PrimitiveTypeContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<PrimitiveTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_BOOLEAN_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_BOOLEAN_TYPE
fn KEYWORD_BOOLEAN_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_BOOLEAN_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_INT_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_INT_TYPE
fn KEYWORD_INT_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_INT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_FLOAT_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_FLOAT_TYPE
fn KEYWORD_FLOAT_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_FLOAT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_STRING_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_STRING_TYPE
fn KEYWORD_STRING_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_STRING_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_FILE_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_FILE_TYPE
fn KEYWORD_FILE_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_FILE_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_DIRECTORY_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_DIRECTORY_TYPE
fn KEYWORD_DIRECTORY_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_DIRECTORY_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION_MARK
/// Returns `None` if there is no child corresponding to token QUESTION_MARK
fn QUESTION_MARK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_QUESTION_MARK, 0)
}

}

impl<'input> PrimitiveTypeContextAttrs<'input> for PrimitiveTypeContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn primitiveType(&mut self,)
	-> Result<Rc<PrimitiveTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimitiveTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_primitiveType);
        let mut _localctx: Rc<PrimitiveTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(580);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 2351104) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(582);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_QUESTION_MARK {
				{
				recog.base.set_state(581);
				recog.base.match_token(WdlV1Parser_QUESTION_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- typeRefType ----------------
pub type TypeRefTypeContextAll<'input> = TypeRefTypeContext<'input>;


pub type TypeRefTypeContext<'input> = BaseParserRuleContext<'input,TypeRefTypeContextExt<'input>>;

#[derive(Clone)]
pub struct TypeRefTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for TypeRefTypeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TypeRefTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeRefType(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeRefType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TypeRefTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_typeRefType(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeRefTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeRefType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeRefType }
}
antlr4rust::tid!{TypeRefTypeContextExt<'a>}

impl<'input> TypeRefTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeRefTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeRefTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeRefTypeContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<TypeRefTypeContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION_MARK
/// Returns `None` if there is no child corresponding to token QUESTION_MARK
fn QUESTION_MARK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_QUESTION_MARK, 0)
}

}

impl<'input> TypeRefTypeContextAttrs<'input> for TypeRefTypeContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeRefType(&mut self,)
	-> Result<Rc<TypeRefTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeRefTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_typeRefType);
        let mut _localctx: Rc<TypeRefTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(584);
			recog.strictIdentifier()?;

			recog.base.set_state(586);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_QUESTION_MARK {
				{
				recog.base.set_state(585);
				recog.base.match_token(WdlV1Parser_QUESTION_MARK,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- unboundDeclaration ----------------
pub type UnboundDeclarationContextAll<'input> = UnboundDeclarationContext<'input>;


pub type UnboundDeclarationContext<'input> = BaseParserRuleContext<'input,UnboundDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct UnboundDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for UnboundDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for UnboundDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_unboundDeclaration(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_unboundDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for UnboundDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_unboundDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnboundDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unboundDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unboundDeclaration }
}
antlr4rust::tid!{UnboundDeclarationContextExt<'a>}

impl<'input> UnboundDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<UnboundDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnboundDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait UnboundDeclarationContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<UnboundDeclarationContextExt<'input>>{

fn type_(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ENV
/// Returns `None` if there is no child corresponding to token KEYWORD_ENV
fn KEYWORD_ENV(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ENV, 0)
}

}

impl<'input> UnboundDeclarationContextAttrs<'input> for UnboundDeclarationContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn unboundDeclaration(&mut self,)
	-> Result<Rc<UnboundDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnboundDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_unboundDeclaration);
        let mut _localctx: Rc<UnboundDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(589);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(47,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(588);
					recog.base.match_token(WdlV1Parser_KEYWORD_ENV,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule type_*/
			recog.base.set_state(591);
			recog.type_()?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(592);
			recog.strictIdentifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- boundDeclaration ----------------
pub type BoundDeclarationContextAll<'input> = BoundDeclarationContext<'input>;


pub type BoundDeclarationContext<'input> = BaseParserRuleContext<'input,BoundDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct BoundDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for BoundDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for BoundDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_boundDeclaration(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_boundDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for BoundDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_boundDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for BoundDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_boundDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_boundDeclaration }
}
antlr4rust::tid!{BoundDeclarationContextExt<'a>}

impl<'input> BoundDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BoundDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BoundDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait BoundDeclarationContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<BoundDeclarationContextExt<'input>>{

fn type_(&self) -> Option<Rc<TypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
/// Returns `None` if there is no child corresponding to token ASSIGNMENT
fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_ASSIGNMENT, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ENV
/// Returns `None` if there is no child corresponding to token KEYWORD_ENV
fn KEYWORD_ENV(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ENV, 0)
}

}

impl<'input> BoundDeclarationContextAttrs<'input> for BoundDeclarationContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn boundDeclaration(&mut self,)
	-> Result<Rc<BoundDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BoundDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_boundDeclaration);
        let mut _localctx: Rc<BoundDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(595);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(48,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(594);
					recog.base.match_token(WdlV1Parser_KEYWORD_ENV,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule type_*/
			recog.base.set_state(597);
			recog.type_()?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(598);
			recog.strictIdentifier()?;

			recog.base.set_state(599);
			recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(600);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declaration ----------------
pub type DeclarationContextAll<'input> = DeclarationContext<'input>;


pub type DeclarationContext<'input> = BaseParserRuleContext<'input,DeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for DeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for DeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declaration(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for DeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_declaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declaration }
}
antlr4rust::tid!{DeclarationContextExt<'a>}

impl<'input> DeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<DeclarationContextExt<'input>>{

fn unboundDeclaration(&self) -> Option<Rc<UnboundDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn boundDeclaration(&self) -> Option<Rc<BoundDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationContextAttrs<'input> for DeclarationContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declaration(&mut self,)
	-> Result<Rc<DeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_declaration);
        let mut _localctx: Rc<DeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(604);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(49,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule unboundDeclaration*/
					recog.base.set_state(602);
					recog.unboundDeclaration()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule boundDeclaration*/
					recog.base.set_state(603);
					recog.boundDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- taskElement ----------------
#[derive(Debug)]
pub enum TaskElementContextAll<'input>{
	TaskParameterMetadataSectionContext(TaskParameterMetadataSectionContext<'input>),
	TaskCommandSectionContext(TaskCommandSectionContext<'input>),
	TaskMetadataSectionContext(TaskMetadataSectionContext<'input>),
	TaskOutputSectionContext(TaskOutputSectionContext<'input>),
	TaskRuntimeSectionContext(TaskRuntimeSectionContext<'input>),
	TaskInputSectionContext(TaskInputSectionContext<'input>),
	TaskDeclarationContext(TaskDeclarationContext<'input>),
	TaskRequirementsSectionContext(TaskRequirementsSectionContext<'input>),
	TaskHintsSectionContext(TaskHintsSectionContext<'input>),
Error(TaskElementContext<'input>)
}
antlr4rust::tid!{TaskElementContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for TaskElementContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for TaskElementContextAll<'input>{}

impl<'input> Deref for TaskElementContextAll<'input>{
	type Target = dyn TaskElementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TaskElementContextAll::*;
		match self{
			TaskParameterMetadataSectionContext(inner) => inner,
			TaskCommandSectionContext(inner) => inner,
			TaskMetadataSectionContext(inner) => inner,
			TaskOutputSectionContext(inner) => inner,
			TaskRuntimeSectionContext(inner) => inner,
			TaskInputSectionContext(inner) => inner,
			TaskDeclarationContext(inner) => inner,
			TaskRequirementsSectionContext(inner) => inner,
			TaskHintsSectionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskElementContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskElementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type TaskElementContext<'input> = BaseParserRuleContext<'input,TaskElementContextExt<'input>>;

#[derive(Clone)]
pub struct TaskElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for TaskElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskElementContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskElementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TaskElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}
antlr4rust::tid!{TaskElementContextExt<'a>}

impl<'input> TaskElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TaskElementContextAll<'input>> {
		Rc::new(
		TaskElementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TaskElementContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TaskElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<TaskElementContextExt<'input>>{


}

impl<'input> TaskElementContextAttrs<'input> for TaskElementContext<'input>{}

pub type TaskParameterMetadataSectionContext<'input> = BaseParserRuleContext<'input,TaskParameterMetadataSectionContextExt<'input>>;

pub trait TaskParameterMetadataSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn parameterMetadataSection(&self) -> Option<Rc<ParameterMetadataSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskParameterMetadataSectionContextAttrs<'input> for TaskParameterMetadataSectionContext<'input>{}

pub struct TaskParameterMetadataSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskParameterMetadataSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskParameterMetadataSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskParameterMetadataSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskParameterMetadataSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskParameterMetadataSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskParameterMetadataSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskParameterMetadataSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskParameterMetadataSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskParameterMetadataSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskParameterMetadataSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskParameterMetadataSectionContext<'input> {}

impl<'input> TaskParameterMetadataSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskParameterMetadataSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskParameterMetadataSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskCommandSectionContext<'input> = BaseParserRuleContext<'input,TaskCommandSectionContextExt<'input>>;

pub trait TaskCommandSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn commandSection(&self) -> Option<Rc<CommandSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskCommandSectionContextAttrs<'input> for TaskCommandSectionContext<'input>{}

pub struct TaskCommandSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskCommandSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskCommandSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskCommandSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskCommandSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskCommandSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskCommandSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskCommandSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskCommandSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskCommandSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskCommandSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskCommandSectionContext<'input> {}

impl<'input> TaskCommandSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskCommandSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskCommandSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskMetadataSectionContext<'input> = BaseParserRuleContext<'input,TaskMetadataSectionContextExt<'input>>;

pub trait TaskMetadataSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn metadataSection(&self) -> Option<Rc<MetadataSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskMetadataSectionContextAttrs<'input> for TaskMetadataSectionContext<'input>{}

pub struct TaskMetadataSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskMetadataSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskMetadataSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskMetadataSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskMetadataSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskMetadataSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskMetadataSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskMetadataSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskMetadataSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskMetadataSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskMetadataSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskMetadataSectionContext<'input> {}

impl<'input> TaskMetadataSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskMetadataSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskMetadataSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskOutputSectionContext<'input> = BaseParserRuleContext<'input,TaskOutputSectionContextExt<'input>>;

pub trait TaskOutputSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn outputSection(&self) -> Option<Rc<OutputSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskOutputSectionContextAttrs<'input> for TaskOutputSectionContext<'input>{}

pub struct TaskOutputSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskOutputSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskOutputSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskOutputSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskOutputSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskOutputSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskOutputSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskOutputSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskOutputSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskOutputSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskOutputSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskOutputSectionContext<'input> {}

impl<'input> TaskOutputSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskOutputSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskOutputSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskRuntimeSectionContext<'input> = BaseParserRuleContext<'input,TaskRuntimeSectionContextExt<'input>>;

pub trait TaskRuntimeSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn runtimeSection(&self) -> Option<Rc<RuntimeSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskRuntimeSectionContextAttrs<'input> for TaskRuntimeSectionContext<'input>{}

pub struct TaskRuntimeSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskRuntimeSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskRuntimeSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskRuntimeSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskRuntimeSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskRuntimeSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskRuntimeSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskRuntimeSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskRuntimeSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskRuntimeSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskRuntimeSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskRuntimeSectionContext<'input> {}

impl<'input> TaskRuntimeSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskRuntimeSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskRuntimeSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskInputSectionContext<'input> = BaseParserRuleContext<'input,TaskInputSectionContextExt<'input>>;

pub trait TaskInputSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn inputSection(&self) -> Option<Rc<InputSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskInputSectionContextAttrs<'input> for TaskInputSectionContext<'input>{}

pub struct TaskInputSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskInputSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskInputSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskInputSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskInputSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskInputSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskInputSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskInputSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskInputSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskInputSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskInputSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskInputSectionContext<'input> {}

impl<'input> TaskInputSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskInputSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskInputSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskDeclarationContext<'input> = BaseParserRuleContext<'input,TaskDeclarationContextExt<'input>>;

pub trait TaskDeclarationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn boundDeclaration(&self) -> Option<Rc<BoundDeclarationContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskDeclarationContextAttrs<'input> for TaskDeclarationContext<'input>{}

pub struct TaskDeclarationContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskDeclarationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskDeclaration(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskDeclaration(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskDeclarationContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskDeclarationContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskDeclarationContext<'input> {}

impl<'input> TaskDeclarationContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskDeclarationContext(
				BaseParserRuleContext::copy_from(ctx,TaskDeclarationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskRequirementsSectionContext<'input> = BaseParserRuleContext<'input,TaskRequirementsSectionContextExt<'input>>;

pub trait TaskRequirementsSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn requirementsSection(&self) -> Option<Rc<RequirementsSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskRequirementsSectionContextAttrs<'input> for TaskRequirementsSectionContext<'input>{}

pub struct TaskRequirementsSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskRequirementsSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskRequirementsSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskRequirementsSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskRequirementsSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskRequirementsSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskRequirementsSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskRequirementsSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskRequirementsSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskRequirementsSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskRequirementsSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskRequirementsSectionContext<'input> {}

impl<'input> TaskRequirementsSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskRequirementsSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskRequirementsSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskHintsSectionContext<'input> = BaseParserRuleContext<'input,TaskHintsSectionContextExt<'input>>;

pub trait TaskHintsSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn hintsSectionTask(&self) -> Option<Rc<HintsSectionTaskContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskHintsSectionContextAttrs<'input> for TaskHintsSectionContext<'input>{}

pub struct TaskHintsSectionContextExt<'input>{
	base:TaskElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskHintsSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskHintsSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskHintsSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskHintsSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskHintsSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskHintsSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskHintsSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskHintsSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskElement }
}

impl<'input> Borrow<TaskElementContextExt<'input>> for TaskHintsSectionContext<'input>{
	fn borrow(&self) -> &TaskElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TaskElementContextExt<'input>> for TaskHintsSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TaskElementContextExt<'input> { &mut self.base }
}

impl<'input> TaskElementContextAttrs<'input> for TaskHintsSectionContext<'input> {}

impl<'input> TaskHintsSectionContextExt<'input>{
	fn new(ctx: &dyn TaskElementContextAttrs<'input>) -> Rc<TaskElementContextAll<'input>>  {
		Rc::new(
			TaskElementContextAll::TaskHintsSectionContext(
				BaseParserRuleContext::copy_from(ctx,TaskHintsSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn taskElement(&mut self,)
	-> Result<Rc<TaskElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TaskElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_taskElement);
        let mut _localctx: Rc<TaskElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(615);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(50,&mut recog.base)? {
				1 =>{
					let tmp = TaskInputSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule inputSection*/
					recog.base.set_state(606);
					recog.inputSection()?;

					}
				}
			,
				2 =>{
					let tmp = TaskCommandSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule commandSection*/
					recog.base.set_state(607);
					recog.commandSection()?;

					}
				}
			,
				3 =>{
					let tmp = TaskOutputSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule outputSection*/
					recog.base.set_state(608);
					recog.outputSection()?;

					}
				}
			,
				4 =>{
					let tmp = TaskRuntimeSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					/*InvokeRule runtimeSection*/
					recog.base.set_state(609);
					recog.runtimeSection()?;

					}
				}
			,
				5 =>{
					let tmp = TaskRequirementsSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					/*InvokeRule requirementsSection*/
					recog.base.set_state(610);
					recog.requirementsSection()?;

					}
				}
			,
				6 =>{
					let tmp = TaskHintsSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					/*InvokeRule hintsSectionTask*/
					recog.base.set_state(611);
					recog.hintsSectionTask()?;

					}
				}
			,
				7 =>{
					let tmp = TaskMetadataSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					/*InvokeRule metadataSection*/
					recog.base.set_state(612);
					recog.metadataSection()?;

					}
				}
			,
				8 =>{
					let tmp = TaskParameterMetadataSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8)?;
					_localctx = tmp;
					{
					/*InvokeRule parameterMetadataSection*/
					recog.base.set_state(613);
					recog.parameterMetadataSection()?;

					}
				}
			,
				9 =>{
					let tmp = TaskDeclarationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9)?;
					_localctx = tmp;
					{
					/*InvokeRule boundDeclaration*/
					recog.base.set_state(614);
					recog.boundDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- workflowElement ----------------
#[derive(Debug)]
pub enum WorkflowElementContextAll<'input>{
	WorkflowConditionalStatementContext(WorkflowConditionalStatementContext<'input>),
	WorkflowInputSectionContext(WorkflowInputSectionContext<'input>),
	WorkflowMetadataSectionContext(WorkflowMetadataSectionContext<'input>),
	WorkflowDeclarationContext(WorkflowDeclarationContext<'input>),
	WorkflowOutputSectionContext(WorkflowOutputSectionContext<'input>),
	WorkflowScatterStatementContext(WorkflowScatterStatementContext<'input>),
	WorkflowHintsSectionContext(WorkflowHintsSectionContext<'input>),
	WorkflowCallStatementContext(WorkflowCallStatementContext<'input>),
	WorkflowParameterMetadataSectionContext(WorkflowParameterMetadataSectionContext<'input>),
Error(WorkflowElementContext<'input>)
}
antlr4rust::tid!{WorkflowElementContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for WorkflowElementContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for WorkflowElementContextAll<'input>{}

impl<'input> Deref for WorkflowElementContextAll<'input>{
	type Target = dyn WorkflowElementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use WorkflowElementContextAll::*;
		match self{
			WorkflowConditionalStatementContext(inner) => inner,
			WorkflowInputSectionContext(inner) => inner,
			WorkflowMetadataSectionContext(inner) => inner,
			WorkflowDeclarationContext(inner) => inner,
			WorkflowOutputSectionContext(inner) => inner,
			WorkflowScatterStatementContext(inner) => inner,
			WorkflowHintsSectionContext(inner) => inner,
			WorkflowCallStatementContext(inner) => inner,
			WorkflowParameterMetadataSectionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowElementContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowElementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type WorkflowElementContext<'input> = BaseParserRuleContext<'input,WorkflowElementContextExt<'input>>;

#[derive(Clone)]
pub struct WorkflowElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for WorkflowElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowElementContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowElementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for WorkflowElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}
antlr4rust::tid!{WorkflowElementContextExt<'a>}

impl<'input> WorkflowElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<WorkflowElementContextAll<'input>> {
		Rc::new(
		WorkflowElementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WorkflowElementContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait WorkflowElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<WorkflowElementContextExt<'input>>{


}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowElementContext<'input>{}

pub type WorkflowConditionalStatementContext<'input> = BaseParserRuleContext<'input,WorkflowConditionalStatementContextExt<'input>>;

pub trait WorkflowConditionalStatementContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn conditionalStatement(&self) -> Option<Rc<ConditionalStatementContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowConditionalStatementContextAttrs<'input> for WorkflowConditionalStatementContext<'input>{}

pub struct WorkflowConditionalStatementContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowConditionalStatementContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowConditionalStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowConditionalStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowConditionalStatement(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowConditionalStatement(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowConditionalStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowConditionalStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowConditionalStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowConditionalStatementContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowConditionalStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowConditionalStatementContext<'input> {}

impl<'input> WorkflowConditionalStatementContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowConditionalStatementContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowConditionalStatementContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowInputSectionContext<'input> = BaseParserRuleContext<'input,WorkflowInputSectionContextExt<'input>>;

pub trait WorkflowInputSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn inputSection(&self) -> Option<Rc<InputSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowInputSectionContextAttrs<'input> for WorkflowInputSectionContext<'input>{}

pub struct WorkflowInputSectionContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowInputSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowInputSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowInputSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowInputSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowInputSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowInputSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowInputSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowInputSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowInputSectionContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowInputSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowInputSectionContext<'input> {}

impl<'input> WorkflowInputSectionContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowInputSectionContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowInputSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowMetadataSectionContext<'input> = BaseParserRuleContext<'input,WorkflowMetadataSectionContextExt<'input>>;

pub trait WorkflowMetadataSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn metadataSection(&self) -> Option<Rc<MetadataSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowMetadataSectionContextAttrs<'input> for WorkflowMetadataSectionContext<'input>{}

pub struct WorkflowMetadataSectionContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowMetadataSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowMetadataSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowMetadataSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowMetadataSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowMetadataSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowMetadataSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowMetadataSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowMetadataSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowMetadataSectionContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowMetadataSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowMetadataSectionContext<'input> {}

impl<'input> WorkflowMetadataSectionContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowMetadataSectionContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowMetadataSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowDeclarationContext<'input> = BaseParserRuleContext<'input,WorkflowDeclarationContextExt<'input>>;

pub trait WorkflowDeclarationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn boundDeclaration(&self) -> Option<Rc<BoundDeclarationContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowDeclarationContextAttrs<'input> for WorkflowDeclarationContext<'input>{}

pub struct WorkflowDeclarationContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowDeclarationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowDeclaration(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowDeclaration(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowDeclarationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowDeclarationContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowDeclarationContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowDeclarationContext<'input> {}

impl<'input> WorkflowDeclarationContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowDeclarationContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowDeclarationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowOutputSectionContext<'input> = BaseParserRuleContext<'input,WorkflowOutputSectionContextExt<'input>>;

pub trait WorkflowOutputSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn outputSection(&self) -> Option<Rc<OutputSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowOutputSectionContextAttrs<'input> for WorkflowOutputSectionContext<'input>{}

pub struct WorkflowOutputSectionContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowOutputSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowOutputSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowOutputSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowOutputSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowOutputSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowOutputSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowOutputSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowOutputSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowOutputSectionContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowOutputSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowOutputSectionContext<'input> {}

impl<'input> WorkflowOutputSectionContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowOutputSectionContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowOutputSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowScatterStatementContext<'input> = BaseParserRuleContext<'input,WorkflowScatterStatementContextExt<'input>>;

pub trait WorkflowScatterStatementContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn scatterStatement(&self) -> Option<Rc<ScatterStatementContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowScatterStatementContextAttrs<'input> for WorkflowScatterStatementContext<'input>{}

pub struct WorkflowScatterStatementContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowScatterStatementContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowScatterStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowScatterStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowScatterStatement(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowScatterStatement(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowScatterStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowScatterStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowScatterStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowScatterStatementContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowScatterStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowScatterStatementContext<'input> {}

impl<'input> WorkflowScatterStatementContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowScatterStatementContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowScatterStatementContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowHintsSectionContext<'input> = BaseParserRuleContext<'input,WorkflowHintsSectionContextExt<'input>>;

pub trait WorkflowHintsSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn hintsSectionWorkflow(&self) -> Option<Rc<HintsSectionWorkflowContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowHintsSectionContextAttrs<'input> for WorkflowHintsSectionContext<'input>{}

pub struct WorkflowHintsSectionContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowHintsSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowHintsSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowHintsSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowHintsSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowHintsSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowHintsSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowHintsSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowHintsSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowHintsSectionContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowHintsSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowHintsSectionContext<'input> {}

impl<'input> WorkflowHintsSectionContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowHintsSectionContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowHintsSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowCallStatementContext<'input> = BaseParserRuleContext<'input,WorkflowCallStatementContextExt<'input>>;

pub trait WorkflowCallStatementContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn callStatement(&self) -> Option<Rc<CallStatementContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowCallStatementContextAttrs<'input> for WorkflowCallStatementContext<'input>{}

pub struct WorkflowCallStatementContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowCallStatementContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowCallStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowCallStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowCallStatement(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowCallStatement(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowCallStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowCallStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowCallStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowCallStatementContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowCallStatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowCallStatementContext<'input> {}

impl<'input> WorkflowCallStatementContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowCallStatementContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowCallStatementContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowParameterMetadataSectionContext<'input> = BaseParserRuleContext<'input,WorkflowParameterMetadataSectionContextExt<'input>>;

pub trait WorkflowParameterMetadataSectionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn parameterMetadataSection(&self) -> Option<Rc<ParameterMetadataSectionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowParameterMetadataSectionContextAttrs<'input> for WorkflowParameterMetadataSectionContext<'input>{}

pub struct WorkflowParameterMetadataSectionContextExt<'input>{
	base:WorkflowElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowParameterMetadataSectionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowParameterMetadataSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowParameterMetadataSectionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowParameterMetadataSection(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowParameterMetadataSection(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowParameterMetadataSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowParameterMetadataSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowParameterMetadataSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowElement }
}

impl<'input> Borrow<WorkflowElementContextExt<'input>> for WorkflowParameterMetadataSectionContext<'input>{
	fn borrow(&self) -> &WorkflowElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<WorkflowElementContextExt<'input>> for WorkflowParameterMetadataSectionContext<'input>{
	fn borrow_mut(&mut self) -> &mut WorkflowElementContextExt<'input> { &mut self.base }
}

impl<'input> WorkflowElementContextAttrs<'input> for WorkflowParameterMetadataSectionContext<'input> {}

impl<'input> WorkflowParameterMetadataSectionContextExt<'input>{
	fn new(ctx: &dyn WorkflowElementContextAttrs<'input>) -> Rc<WorkflowElementContextAll<'input>>  {
		Rc::new(
			WorkflowElementContextAll::WorkflowParameterMetadataSectionContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowParameterMetadataSectionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn workflowElement(&mut self,)
	-> Result<Rc<WorkflowElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WorkflowElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_workflowElement);
        let mut _localctx: Rc<WorkflowElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(626);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(51,&mut recog.base)? {
				1 =>{
					let tmp = WorkflowInputSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule inputSection*/
					recog.base.set_state(617);
					recog.inputSection()?;

					}
				}
			,
				2 =>{
					let tmp = WorkflowOutputSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule outputSection*/
					recog.base.set_state(618);
					recog.outputSection()?;

					}
				}
			,
				3 =>{
					let tmp = WorkflowHintsSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule hintsSectionWorkflow*/
					recog.base.set_state(619);
					recog.hintsSectionWorkflow()?;

					}
				}
			,
				4 =>{
					let tmp = WorkflowConditionalStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					/*InvokeRule conditionalStatement*/
					recog.base.set_state(620);
					recog.conditionalStatement()?;

					}
				}
			,
				5 =>{
					let tmp = WorkflowScatterStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					/*InvokeRule scatterStatement*/
					recog.base.set_state(621);
					recog.scatterStatement()?;

					}
				}
			,
				6 =>{
					let tmp = WorkflowCallStatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					/*InvokeRule callStatement*/
					recog.base.set_state(622);
					recog.callStatement()?;

					}
				}
			,
				7 =>{
					let tmp = WorkflowMetadataSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					/*InvokeRule metadataSection*/
					recog.base.set_state(623);
					recog.metadataSection()?;

					}
				}
			,
				8 =>{
					let tmp = WorkflowParameterMetadataSectionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8)?;
					_localctx = tmp;
					{
					/*InvokeRule parameterMetadataSection*/
					recog.base.set_state(624);
					recog.parameterMetadataSection()?;

					}
				}
			,
				9 =>{
					let tmp = WorkflowDeclarationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9)?;
					_localctx = tmp;
					{
					/*InvokeRule boundDeclaration*/
					recog.base.set_state(625);
					recog.boundDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- inputSection ----------------
pub type InputSectionContextAll<'input> = InputSectionContext<'input>;


pub type InputSectionContext<'input> = BaseParserRuleContext<'input,InputSectionContextExt<'input>>;

#[derive(Clone)]
pub struct InputSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for InputSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for InputSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_inputSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_inputSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for InputSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_inputSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for InputSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inputSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inputSection }
}
antlr4rust::tid!{InputSectionContextExt<'a>}

impl<'input> InputSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InputSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InputSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InputSectionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<InputSectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_INPUT
/// Returns `None` if there is no child corresponding to token KEYWORD_INPUT
fn KEYWORD_INPUT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_INPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn declaration_all(&self) ->  Vec<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declaration(&self, i: usize) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InputSectionContextAttrs<'input> for InputSectionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn inputSection(&mut self,)
	-> Result<Rc<InputSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InputSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_inputSection);
        let mut _localctx: Rc<InputSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(628);
			recog.base.match_token(WdlV1Parser_KEYWORD_INPUT,&mut recog.err_handler)?;

			recog.base.set_state(629);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(633);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule declaration*/
				recog.base.set_state(630);
				recog.declaration()?;

				}
				}
				recog.base.set_state(635);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(636);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- outputSection ----------------
pub type OutputSectionContextAll<'input> = OutputSectionContext<'input>;


pub type OutputSectionContext<'input> = BaseParserRuleContext<'input,OutputSectionContextExt<'input>>;

#[derive(Clone)]
pub struct OutputSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for OutputSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for OutputSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_outputSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_outputSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for OutputSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_outputSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for OutputSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_outputSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_outputSection }
}
antlr4rust::tid!{OutputSectionContextExt<'a>}

impl<'input> OutputSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OutputSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OutputSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OutputSectionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<OutputSectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_OUTPUT
/// Returns `None` if there is no child corresponding to token KEYWORD_OUTPUT
fn KEYWORD_OUTPUT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OUTPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn boundDeclaration_all(&self) ->  Vec<Rc<BoundDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn boundDeclaration(&self, i: usize) -> Option<Rc<BoundDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OutputSectionContextAttrs<'input> for OutputSectionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn outputSection(&mut self,)
	-> Result<Rc<OutputSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OutputSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_outputSection);
        let mut _localctx: Rc<OutputSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(638);
			recog.base.match_token(WdlV1Parser_KEYWORD_OUTPUT,&mut recog.err_handler)?;

			recog.base.set_state(639);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(643);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule boundDeclaration*/
				recog.base.set_state(640);
				recog.boundDeclaration()?;

				}
				}
				recog.base.set_state(645);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(646);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- runtimeSection ----------------
pub type RuntimeSectionContextAll<'input> = RuntimeSectionContext<'input>;


pub type RuntimeSectionContext<'input> = BaseParserRuleContext<'input,RuntimeSectionContextExt<'input>>;

#[derive(Clone)]
pub struct RuntimeSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for RuntimeSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for RuntimeSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_runtimeSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_runtimeSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for RuntimeSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_runtimeSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for RuntimeSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_runtimeSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_runtimeSection }
}
antlr4rust::tid!{RuntimeSectionContextExt<'a>}

impl<'input> RuntimeSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RuntimeSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RuntimeSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RuntimeSectionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<RuntimeSectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_RUNTIME
/// Returns `None` if there is no child corresponding to token KEYWORD_RUNTIME
fn KEYWORD_RUNTIME(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_RUNTIME, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn runtimeItem_all(&self) ->  Vec<Rc<RuntimeItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn runtimeItem(&self, i: usize) -> Option<Rc<RuntimeItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RuntimeSectionContextAttrs<'input> for RuntimeSectionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn runtimeSection(&mut self,)
	-> Result<Rc<RuntimeSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RuntimeSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_runtimeSection);
        let mut _localctx: Rc<RuntimeSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(648);
			recog.base.match_token(WdlV1Parser_KEYWORD_RUNTIME,&mut recog.err_handler)?;

			recog.base.set_state(649);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(653);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule runtimeItem*/
				recog.base.set_state(650);
				recog.runtimeItem()?;

				}
				}
				recog.base.set_state(655);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(656);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- runtimeItem ----------------
pub type RuntimeItemContextAll<'input> = RuntimeItemContext<'input>;


pub type RuntimeItemContext<'input> = BaseParserRuleContext<'input,RuntimeItemContextExt<'input>>;

#[derive(Clone)]
pub struct RuntimeItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for RuntimeItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for RuntimeItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_runtimeItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_runtimeItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for RuntimeItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_runtimeItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for RuntimeItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_runtimeItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_runtimeItem }
}
antlr4rust::tid!{RuntimeItemContextExt<'a>}

impl<'input> RuntimeItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RuntimeItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RuntimeItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RuntimeItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<RuntimeItemContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RuntimeItemContextAttrs<'input> for RuntimeItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn runtimeItem(&mut self,)
	-> Result<Rc<RuntimeItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RuntimeItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_runtimeItem);
        let mut _localctx: Rc<RuntimeItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(658);
			recog.strictIdentifier()?;

			recog.base.set_state(659);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(660);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- requirementsSection ----------------
pub type RequirementsSectionContextAll<'input> = RequirementsSectionContext<'input>;


pub type RequirementsSectionContext<'input> = BaseParserRuleContext<'input,RequirementsSectionContextExt<'input>>;

#[derive(Clone)]
pub struct RequirementsSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for RequirementsSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for RequirementsSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_requirementsSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_requirementsSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for RequirementsSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_requirementsSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for RequirementsSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_requirementsSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_requirementsSection }
}
antlr4rust::tid!{RequirementsSectionContextExt<'a>}

impl<'input> RequirementsSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RequirementsSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RequirementsSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RequirementsSectionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<RequirementsSectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_REQUIREMENTS
/// Returns `None` if there is no child corresponding to token KEYWORD_REQUIREMENTS
fn KEYWORD_REQUIREMENTS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_REQUIREMENTS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn requirementsItem_all(&self) ->  Vec<Rc<RequirementsItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn requirementsItem(&self, i: usize) -> Option<Rc<RequirementsItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RequirementsSectionContextAttrs<'input> for RequirementsSectionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn requirementsSection(&mut self,)
	-> Result<Rc<RequirementsSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RequirementsSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_requirementsSection);
        let mut _localctx: Rc<RequirementsSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(662);
			recog.base.match_token(WdlV1Parser_KEYWORD_REQUIREMENTS,&mut recog.err_handler)?;

			recog.base.set_state(663);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(667);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule requirementsItem*/
				recog.base.set_state(664);
				recog.requirementsItem()?;

				}
				}
				recog.base.set_state(669);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(670);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- requirementsItem ----------------
pub type RequirementsItemContextAll<'input> = RequirementsItemContext<'input>;


pub type RequirementsItemContext<'input> = BaseParserRuleContext<'input,RequirementsItemContextExt<'input>>;

#[derive(Clone)]
pub struct RequirementsItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for RequirementsItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for RequirementsItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_requirementsItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_requirementsItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for RequirementsItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_requirementsItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for RequirementsItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_requirementsItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_requirementsItem }
}
antlr4rust::tid!{RequirementsItemContextExt<'a>}

impl<'input> RequirementsItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RequirementsItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RequirementsItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RequirementsItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<RequirementsItemContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RequirementsItemContextAttrs<'input> for RequirementsItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn requirementsItem(&mut self,)
	-> Result<Rc<RequirementsItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RequirementsItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_requirementsItem);
        let mut _localctx: Rc<RequirementsItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(672);
			recog.strictIdentifier()?;

			recog.base.set_state(673);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(674);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsSectionTask ----------------
pub type HintsSectionTaskContextAll<'input> = HintsSectionTaskContext<'input>;


pub type HintsSectionTaskContext<'input> = BaseParserRuleContext<'input,HintsSectionTaskContextExt<'input>>;

#[derive(Clone)]
pub struct HintsSectionTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsSectionTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsSectionTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsSectionTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsSectionTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsSectionTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsSectionTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsSectionTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsSectionTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsSectionTask }
}
antlr4rust::tid!{HintsSectionTaskContextExt<'a>}

impl<'input> HintsSectionTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsSectionTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsSectionTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsSectionTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsSectionTaskContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_HINTS
/// Returns `None` if there is no child corresponding to token KEYWORD_HINTS
fn KEYWORD_HINTS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_HINTS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn hintsItemTask_all(&self) ->  Vec<Rc<HintsItemTaskContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hintsItemTask(&self, i: usize) -> Option<Rc<HintsItemTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> HintsSectionTaskContextAttrs<'input> for HintsSectionTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsSectionTask(&mut self,)
	-> Result<Rc<HintsSectionTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsSectionTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_hintsSectionTask);
        let mut _localctx: Rc<HintsSectionTaskContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(676);
			recog.base.match_token(WdlV1Parser_KEYWORD_HINTS,&mut recog.err_handler)?;

			recog.base.set_state(677);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(681);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule hintsItemTask*/
				recog.base.set_state(678);
				recog.hintsItemTask()?;

				}
				}
				recog.base.set_state(683);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(684);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsItemTask ----------------
pub type HintsItemTaskContextAll<'input> = HintsItemTaskContext<'input>;


pub type HintsItemTaskContext<'input> = BaseParserRuleContext<'input,HintsItemTaskContextExt<'input>>;

#[derive(Clone)]
pub struct HintsItemTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsItemTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsItemTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsItemTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsItemTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsItemTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsItemTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsItemTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsItemTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsItemTask }
}
antlr4rust::tid!{HintsItemTaskContextExt<'a>}

impl<'input> HintsItemTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsItemTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsItemTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsItemTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsItemTaskContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn hintsValueTask(&self) -> Option<Rc<HintsValueTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HintsItemTaskContextAttrs<'input> for HintsItemTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsItemTask(&mut self,)
	-> Result<Rc<HintsItemTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsItemTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_hintsItemTask);
        let mut _localctx: Rc<HintsItemTaskContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(686);
			recog.strictIdentifier()?;

			recog.base.set_state(687);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule hintsValueTask*/
			recog.base.set_state(688);
			recog.hintsValueTask()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsValueTask ----------------
#[derive(Debug)]
pub enum HintsValueTaskContextAll<'input>{
	TaskHintValueInputObjectContext(TaskHintValueInputObjectContext<'input>),
	TaskHintValueHintsObjectContext(TaskHintValueHintsObjectContext<'input>),
	TaskHintValueOutputObjectContext(TaskHintValueOutputObjectContext<'input>),
	TaskHintValueExpressionContext(TaskHintValueExpressionContext<'input>),
	TaskHintValueArrayContext(TaskHintValueArrayContext<'input>),
Error(HintsValueTaskContext<'input>)
}
antlr4rust::tid!{HintsValueTaskContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for HintsValueTaskContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for HintsValueTaskContextAll<'input>{}

impl<'input> Deref for HintsValueTaskContextAll<'input>{
	type Target = dyn HintsValueTaskContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use HintsValueTaskContextAll::*;
		match self{
			TaskHintValueInputObjectContext(inner) => inner,
			TaskHintValueHintsObjectContext(inner) => inner,
			TaskHintValueOutputObjectContext(inner) => inner,
			TaskHintValueExpressionContext(inner) => inner,
			TaskHintValueArrayContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsValueTaskContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsValueTaskContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type HintsValueTaskContext<'input> = BaseParserRuleContext<'input,HintsValueTaskContextExt<'input>>;

#[derive(Clone)]
pub struct HintsValueTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsValueTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsValueTaskContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsValueTaskContext<'input>{
}

impl<'input> CustomRuleContext<'input> for HintsValueTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueTask }
}
antlr4rust::tid!{HintsValueTaskContextExt<'a>}

impl<'input> HintsValueTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsValueTaskContextAll<'input>> {
		Rc::new(
		HintsValueTaskContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsValueTaskContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait HintsValueTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsValueTaskContextExt<'input>>{


}

impl<'input> HintsValueTaskContextAttrs<'input> for HintsValueTaskContext<'input>{}

pub type TaskHintValueInputObjectContext<'input> = BaseParserRuleContext<'input,TaskHintValueInputObjectContextExt<'input>>;

pub trait TaskHintValueInputObjectContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn inputHintsObjectTask(&self) -> Option<Rc<InputHintsObjectTaskContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskHintValueInputObjectContextAttrs<'input> for TaskHintValueInputObjectContext<'input>{}

pub struct TaskHintValueInputObjectContextExt<'input>{
	base:HintsValueTaskContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskHintValueInputObjectContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskHintValueInputObjectContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskHintValueInputObjectContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskHintValueInputObject(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskHintValueInputObject(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskHintValueInputObjectContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskHintValueInputObject(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskHintValueInputObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueTask }
}

impl<'input> Borrow<HintsValueTaskContextExt<'input>> for TaskHintValueInputObjectContext<'input>{
	fn borrow(&self) -> &HintsValueTaskContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueTaskContextExt<'input>> for TaskHintValueInputObjectContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueTaskContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueTaskContextAttrs<'input> for TaskHintValueInputObjectContext<'input> {}

impl<'input> TaskHintValueInputObjectContextExt<'input>{
	fn new(ctx: &dyn HintsValueTaskContextAttrs<'input>) -> Rc<HintsValueTaskContextAll<'input>>  {
		Rc::new(
			HintsValueTaskContextAll::TaskHintValueInputObjectContext(
				BaseParserRuleContext::copy_from(ctx,TaskHintValueInputObjectContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskHintValueHintsObjectContext<'input> = BaseParserRuleContext<'input,TaskHintValueHintsObjectContextExt<'input>>;

pub trait TaskHintValueHintsObjectContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn hintsTypedObjectTask(&self) -> Option<Rc<HintsTypedObjectTaskContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskHintValueHintsObjectContextAttrs<'input> for TaskHintValueHintsObjectContext<'input>{}

pub struct TaskHintValueHintsObjectContextExt<'input>{
	base:HintsValueTaskContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskHintValueHintsObjectContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskHintValueHintsObjectContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskHintValueHintsObjectContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskHintValueHintsObject(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskHintValueHintsObject(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskHintValueHintsObjectContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskHintValueHintsObject(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskHintValueHintsObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueTask }
}

impl<'input> Borrow<HintsValueTaskContextExt<'input>> for TaskHintValueHintsObjectContext<'input>{
	fn borrow(&self) -> &HintsValueTaskContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueTaskContextExt<'input>> for TaskHintValueHintsObjectContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueTaskContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueTaskContextAttrs<'input> for TaskHintValueHintsObjectContext<'input> {}

impl<'input> TaskHintValueHintsObjectContextExt<'input>{
	fn new(ctx: &dyn HintsValueTaskContextAttrs<'input>) -> Rc<HintsValueTaskContextAll<'input>>  {
		Rc::new(
			HintsValueTaskContextAll::TaskHintValueHintsObjectContext(
				BaseParserRuleContext::copy_from(ctx,TaskHintValueHintsObjectContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskHintValueOutputObjectContext<'input> = BaseParserRuleContext<'input,TaskHintValueOutputObjectContextExt<'input>>;

pub trait TaskHintValueOutputObjectContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn outputHintsObjectTask(&self) -> Option<Rc<OutputHintsObjectTaskContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskHintValueOutputObjectContextAttrs<'input> for TaskHintValueOutputObjectContext<'input>{}

pub struct TaskHintValueOutputObjectContextExt<'input>{
	base:HintsValueTaskContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskHintValueOutputObjectContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskHintValueOutputObjectContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskHintValueOutputObjectContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskHintValueOutputObject(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskHintValueOutputObject(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskHintValueOutputObjectContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskHintValueOutputObject(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskHintValueOutputObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueTask }
}

impl<'input> Borrow<HintsValueTaskContextExt<'input>> for TaskHintValueOutputObjectContext<'input>{
	fn borrow(&self) -> &HintsValueTaskContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueTaskContextExt<'input>> for TaskHintValueOutputObjectContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueTaskContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueTaskContextAttrs<'input> for TaskHintValueOutputObjectContext<'input> {}

impl<'input> TaskHintValueOutputObjectContextExt<'input>{
	fn new(ctx: &dyn HintsValueTaskContextAttrs<'input>) -> Rc<HintsValueTaskContextAll<'input>>  {
		Rc::new(
			HintsValueTaskContextAll::TaskHintValueOutputObjectContext(
				BaseParserRuleContext::copy_from(ctx,TaskHintValueOutputObjectContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskHintValueExpressionContext<'input> = BaseParserRuleContext<'input,TaskHintValueExpressionContextExt<'input>>;

pub trait TaskHintValueExpressionContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskHintValueExpressionContextAttrs<'input> for TaskHintValueExpressionContext<'input>{}

pub struct TaskHintValueExpressionContextExt<'input>{
	base:HintsValueTaskContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskHintValueExpressionContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskHintValueExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskHintValueExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskHintValueExpression(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskHintValueExpression(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskHintValueExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskHintValueExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskHintValueExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueTask }
}

impl<'input> Borrow<HintsValueTaskContextExt<'input>> for TaskHintValueExpressionContext<'input>{
	fn borrow(&self) -> &HintsValueTaskContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueTaskContextExt<'input>> for TaskHintValueExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueTaskContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueTaskContextAttrs<'input> for TaskHintValueExpressionContext<'input> {}

impl<'input> TaskHintValueExpressionContextExt<'input>{
	fn new(ctx: &dyn HintsValueTaskContextAttrs<'input>) -> Rc<HintsValueTaskContextAll<'input>>  {
		Rc::new(
			HintsValueTaskContextAll::TaskHintValueExpressionContext(
				BaseParserRuleContext::copy_from(ctx,TaskHintValueExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TaskHintValueArrayContext<'input> = BaseParserRuleContext<'input,TaskHintValueArrayContextExt<'input>>;

pub trait TaskHintValueArrayContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn taskHintsArray(&self) -> Option<Rc<TaskHintsArrayContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TaskHintValueArrayContextAttrs<'input> for TaskHintValueArrayContext<'input>{}

pub struct TaskHintValueArrayContextExt<'input>{
	base:HintsValueTaskContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TaskHintValueArrayContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for TaskHintValueArrayContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskHintValueArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_taskHintValueArray(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_taskHintValueArray(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskHintValueArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskHintValueArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskHintValueArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueTask }
}

impl<'input> Borrow<HintsValueTaskContextExt<'input>> for TaskHintValueArrayContext<'input>{
	fn borrow(&self) -> &HintsValueTaskContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueTaskContextExt<'input>> for TaskHintValueArrayContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueTaskContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueTaskContextAttrs<'input> for TaskHintValueArrayContext<'input> {}

impl<'input> TaskHintValueArrayContextExt<'input>{
	fn new(ctx: &dyn HintsValueTaskContextAttrs<'input>) -> Rc<HintsValueTaskContextAll<'input>>  {
		Rc::new(
			HintsValueTaskContextAll::TaskHintValueArrayContext(
				BaseParserRuleContext::copy_from(ctx,TaskHintValueArrayContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsValueTask(&mut self,)
	-> Result<Rc<HintsValueTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsValueTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_hintsValueTask);
        let mut _localctx: Rc<HintsValueTaskContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(695);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
				1 =>{
					let tmp = TaskHintValueExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule expression*/
					recog.base.set_state(690);
					recog.expression()?;

					}
				}
			,
				2 =>{
					let tmp = TaskHintValueHintsObjectContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule hintsTypedObjectTask*/
					recog.base.set_state(691);
					recog.hintsTypedObjectTask()?;

					}
				}
			,
				3 =>{
					let tmp = TaskHintValueInputObjectContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule inputHintsObjectTask*/
					recog.base.set_state(692);
					recog.inputHintsObjectTask()?;

					}
				}
			,
				4 =>{
					let tmp = TaskHintValueOutputObjectContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					/*InvokeRule outputHintsObjectTask*/
					recog.base.set_state(693);
					recog.outputHintsObjectTask()?;

					}
				}
			,
				5 =>{
					let tmp = TaskHintValueArrayContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					/*InvokeRule taskHintsArray*/
					recog.base.set_state(694);
					recog.taskHintsArray()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsTypedObjectTask ----------------
pub type HintsTypedObjectTaskContextAll<'input> = HintsTypedObjectTaskContext<'input>;


pub type HintsTypedObjectTaskContext<'input> = BaseParserRuleContext<'input,HintsTypedObjectTaskContextExt<'input>>;

#[derive(Clone)]
pub struct HintsTypedObjectTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsTypedObjectTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsTypedObjectTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsTypedObjectTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsTypedObjectTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsTypedObjectTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsTypedObjectTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsTypedObjectTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsTypedObjectTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsTypedObjectTask }
}
antlr4rust::tid!{HintsTypedObjectTaskContextExt<'a>}

impl<'input> HintsTypedObjectTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsTypedObjectTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsTypedObjectTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsTypedObjectTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsTypedObjectTaskContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_HINTS
/// Returns `None` if there is no child corresponding to token KEYWORD_HINTS
fn KEYWORD_HINTS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_HINTS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn hintsObjectItemTask_all(&self) ->  Vec<Rc<HintsObjectItemTaskContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hintsObjectItemTask(&self, i: usize) -> Option<Rc<HintsObjectItemTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> HintsTypedObjectTaskContextAttrs<'input> for HintsTypedObjectTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsTypedObjectTask(&mut self,)
	-> Result<Rc<HintsTypedObjectTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsTypedObjectTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_hintsTypedObjectTask);
        let mut _localctx: Rc<HintsTypedObjectTaskContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(697);
			recog.base.match_token(WdlV1Parser_KEYWORD_HINTS,&mut recog.err_handler)?;

			recog.base.set_state(698);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(710);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule hintsObjectItemTask*/
				recog.base.set_state(699);
				recog.hintsObjectItemTask()?;

				recog.base.set_state(704);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(58,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(700);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule hintsObjectItemTask*/
						recog.base.set_state(701);
						recog.hintsObjectItemTask()?;

						}
						} 
					}
					recog.base.set_state(706);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(58,&mut recog.base)?;
				}
				recog.base.set_state(708);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(707);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(712);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsObjectItemTask ----------------
pub type HintsObjectItemTaskContextAll<'input> = HintsObjectItemTaskContext<'input>;


pub type HintsObjectItemTaskContext<'input> = BaseParserRuleContext<'input,HintsObjectItemTaskContextExt<'input>>;

#[derive(Clone)]
pub struct HintsObjectItemTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsObjectItemTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsObjectItemTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsObjectItemTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsObjectItemTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsObjectItemTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsObjectItemTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsObjectItemTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsObjectItemTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsObjectItemTask }
}
antlr4rust::tid!{HintsObjectItemTaskContextExt<'a>}

impl<'input> HintsObjectItemTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsObjectItemTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsObjectItemTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsObjectItemTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsObjectItemTaskContextExt<'input>>{

fn dottedIdentifier(&self) -> Option<Rc<DottedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn hintsValueTask(&self) -> Option<Rc<HintsValueTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HintsObjectItemTaskContextAttrs<'input> for HintsObjectItemTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsObjectItemTask(&mut self,)
	-> Result<Rc<HintsObjectItemTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsObjectItemTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_hintsObjectItemTask);
        let mut _localctx: Rc<HintsObjectItemTaskContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule dottedIdentifier*/
			recog.base.set_state(714);
			recog.dottedIdentifier()?;

			recog.base.set_state(715);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule hintsValueTask*/
			recog.base.set_state(716);
			recog.hintsValueTask()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- inputHintsObjectTask ----------------
pub type InputHintsObjectTaskContextAll<'input> = InputHintsObjectTaskContext<'input>;


pub type InputHintsObjectTaskContext<'input> = BaseParserRuleContext<'input,InputHintsObjectTaskContextExt<'input>>;

#[derive(Clone)]
pub struct InputHintsObjectTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for InputHintsObjectTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for InputHintsObjectTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_inputHintsObjectTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_inputHintsObjectTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for InputHintsObjectTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_inputHintsObjectTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for InputHintsObjectTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inputHintsObjectTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inputHintsObjectTask }
}
antlr4rust::tid!{InputHintsObjectTaskContextExt<'a>}

impl<'input> InputHintsObjectTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InputHintsObjectTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InputHintsObjectTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InputHintsObjectTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<InputHintsObjectTaskContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_INPUT
/// Returns `None` if there is no child corresponding to token KEYWORD_INPUT
fn KEYWORD_INPUT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_INPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn inputHintsItemTask_all(&self) ->  Vec<Rc<InputHintsItemTaskContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inputHintsItemTask(&self, i: usize) -> Option<Rc<InputHintsItemTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> InputHintsObjectTaskContextAttrs<'input> for InputHintsObjectTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn inputHintsObjectTask(&mut self,)
	-> Result<Rc<InputHintsObjectTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InputHintsObjectTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_inputHintsObjectTask);
        let mut _localctx: Rc<InputHintsObjectTaskContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(718);
			recog.base.match_token(WdlV1Parser_KEYWORD_INPUT,&mut recog.err_handler)?;

			recog.base.set_state(719);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(731);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule inputHintsItemTask*/
				recog.base.set_state(720);
				recog.inputHintsItemTask()?;

				recog.base.set_state(725);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(61,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(721);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule inputHintsItemTask*/
						recog.base.set_state(722);
						recog.inputHintsItemTask()?;

						}
						} 
					}
					recog.base.set_state(727);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(61,&mut recog.base)?;
				}
				recog.base.set_state(729);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(728);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(733);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- inputHintsItemTask ----------------
pub type InputHintsItemTaskContextAll<'input> = InputHintsItemTaskContext<'input>;


pub type InputHintsItemTaskContext<'input> = BaseParserRuleContext<'input,InputHintsItemTaskContextExt<'input>>;

#[derive(Clone)]
pub struct InputHintsItemTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for InputHintsItemTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for InputHintsItemTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_inputHintsItemTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_inputHintsItemTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for InputHintsItemTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_inputHintsItemTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for InputHintsItemTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inputHintsItemTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inputHintsItemTask }
}
antlr4rust::tid!{InputHintsItemTaskContextExt<'a>}

impl<'input> InputHintsItemTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InputHintsItemTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InputHintsItemTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InputHintsItemTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<InputHintsItemTaskContextExt<'input>>{

fn dottedIdentifier(&self) -> Option<Rc<DottedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn hintsTypedObjectTask(&self) -> Option<Rc<HintsTypedObjectTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InputHintsItemTaskContextAttrs<'input> for InputHintsItemTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn inputHintsItemTask(&mut self,)
	-> Result<Rc<InputHintsItemTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InputHintsItemTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_inputHintsItemTask);
        let mut _localctx: Rc<InputHintsItemTaskContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule dottedIdentifier*/
			recog.base.set_state(735);
			recog.dottedIdentifier()?;

			recog.base.set_state(736);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule hintsTypedObjectTask*/
			recog.base.set_state(737);
			recog.hintsTypedObjectTask()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- outputHintsObjectTask ----------------
pub type OutputHintsObjectTaskContextAll<'input> = OutputHintsObjectTaskContext<'input>;


pub type OutputHintsObjectTaskContext<'input> = BaseParserRuleContext<'input,OutputHintsObjectTaskContextExt<'input>>;

#[derive(Clone)]
pub struct OutputHintsObjectTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for OutputHintsObjectTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for OutputHintsObjectTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_outputHintsObjectTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_outputHintsObjectTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for OutputHintsObjectTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_outputHintsObjectTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for OutputHintsObjectTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_outputHintsObjectTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_outputHintsObjectTask }
}
antlr4rust::tid!{OutputHintsObjectTaskContextExt<'a>}

impl<'input> OutputHintsObjectTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OutputHintsObjectTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OutputHintsObjectTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OutputHintsObjectTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<OutputHintsObjectTaskContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_OUTPUT
/// Returns `None` if there is no child corresponding to token KEYWORD_OUTPUT
fn KEYWORD_OUTPUT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OUTPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn outputHintsItemTask_all(&self) ->  Vec<Rc<OutputHintsItemTaskContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn outputHintsItemTask(&self, i: usize) -> Option<Rc<OutputHintsItemTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> OutputHintsObjectTaskContextAttrs<'input> for OutputHintsObjectTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn outputHintsObjectTask(&mut self,)
	-> Result<Rc<OutputHintsObjectTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OutputHintsObjectTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_outputHintsObjectTask);
        let mut _localctx: Rc<OutputHintsObjectTaskContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(739);
			recog.base.match_token(WdlV1Parser_KEYWORD_OUTPUT,&mut recog.err_handler)?;

			recog.base.set_state(740);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(752);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule outputHintsItemTask*/
				recog.base.set_state(741);
				recog.outputHintsItemTask()?;

				recog.base.set_state(746);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(64,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(742);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule outputHintsItemTask*/
						recog.base.set_state(743);
						recog.outputHintsItemTask()?;

						}
						} 
					}
					recog.base.set_state(748);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(64,&mut recog.base)?;
				}
				recog.base.set_state(750);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(749);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(754);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- outputHintsItemTask ----------------
pub type OutputHintsItemTaskContextAll<'input> = OutputHintsItemTaskContext<'input>;


pub type OutputHintsItemTaskContext<'input> = BaseParserRuleContext<'input,OutputHintsItemTaskContextExt<'input>>;

#[derive(Clone)]
pub struct OutputHintsItemTaskContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for OutputHintsItemTaskContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for OutputHintsItemTaskContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_outputHintsItemTask(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_outputHintsItemTask(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for OutputHintsItemTaskContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_outputHintsItemTask(self);
	}
}

impl<'input> CustomRuleContext<'input> for OutputHintsItemTaskContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_outputHintsItemTask }
	//fn type_rule_index() -> usize where Self: Sized { RULE_outputHintsItemTask }
}
antlr4rust::tid!{OutputHintsItemTaskContextExt<'a>}

impl<'input> OutputHintsItemTaskContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OutputHintsItemTaskContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OutputHintsItemTaskContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OutputHintsItemTaskContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<OutputHintsItemTaskContextExt<'input>>{

fn dottedIdentifier(&self) -> Option<Rc<DottedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn hintsTypedObjectTask(&self) -> Option<Rc<HintsTypedObjectTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OutputHintsItemTaskContextAttrs<'input> for OutputHintsItemTaskContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn outputHintsItemTask(&mut self,)
	-> Result<Rc<OutputHintsItemTaskContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OutputHintsItemTaskContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_outputHintsItemTask);
        let mut _localctx: Rc<OutputHintsItemTaskContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule dottedIdentifier*/
			recog.base.set_state(756);
			recog.dottedIdentifier()?;

			recog.base.set_state(757);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule hintsTypedObjectTask*/
			recog.base.set_state(758);
			recog.hintsTypedObjectTask()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- taskHintsArray ----------------
pub type TaskHintsArrayContextAll<'input> = TaskHintsArrayContext<'input>;


pub type TaskHintsArrayContext<'input> = BaseParserRuleContext<'input,TaskHintsArrayContextExt<'input>>;

#[derive(Clone)]
pub struct TaskHintsArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for TaskHintsArrayContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for TaskHintsArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_taskHintsArray(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_taskHintsArray(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for TaskHintsArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_taskHintsArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for TaskHintsArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_taskHintsArray }
	//fn type_rule_index() -> usize where Self: Sized { RULE_taskHintsArray }
}
antlr4rust::tid!{TaskHintsArrayContextExt<'a>}

impl<'input> TaskHintsArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TaskHintsArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TaskHintsArrayContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TaskHintsArrayContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<TaskHintsArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
fn hintsValueTask_all(&self) ->  Vec<Rc<HintsValueTaskContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hintsValueTask(&self, i: usize) -> Option<Rc<HintsValueTaskContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> TaskHintsArrayContextAttrs<'input> for TaskHintsArrayContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn taskHintsArray(&mut self,)
	-> Result<Rc<TaskHintsArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TaskHintsArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_taskHintsArray);
        let mut _localctx: Rc<TaskHintsArrayContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(760);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(772);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294966720) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 1344274431) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & 41) != 0) {
				{
				/*InvokeRule hintsValueTask*/
				recog.base.set_state(761);
				recog.hintsValueTask()?;

				recog.base.set_state(766);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(67,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(762);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule hintsValueTask*/
						recog.base.set_state(763);
						recog.hintsValueTask()?;

						}
						} 
					}
					recog.base.set_state(768);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(67,&mut recog.base)?;
				}
				recog.base.set_state(770);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(769);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(774);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsSectionWorkflow ----------------
pub type HintsSectionWorkflowContextAll<'input> = HintsSectionWorkflowContext<'input>;


pub type HintsSectionWorkflowContext<'input> = BaseParserRuleContext<'input,HintsSectionWorkflowContextExt<'input>>;

#[derive(Clone)]
pub struct HintsSectionWorkflowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsSectionWorkflowContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsSectionWorkflowContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsSectionWorkflow(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsSectionWorkflow(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsSectionWorkflowContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsSectionWorkflow(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsSectionWorkflowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsSectionWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsSectionWorkflow }
}
antlr4rust::tid!{HintsSectionWorkflowContextExt<'a>}

impl<'input> HintsSectionWorkflowContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsSectionWorkflowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsSectionWorkflowContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsSectionWorkflowContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsSectionWorkflowContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_HINTS
/// Returns `None` if there is no child corresponding to token KEYWORD_HINTS
fn KEYWORD_HINTS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_HINTS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn hintsItemWorkflow_all(&self) ->  Vec<Rc<HintsItemWorkflowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hintsItemWorkflow(&self, i: usize) -> Option<Rc<HintsItemWorkflowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> HintsSectionWorkflowContextAttrs<'input> for HintsSectionWorkflowContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsSectionWorkflow(&mut self,)
	-> Result<Rc<HintsSectionWorkflowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsSectionWorkflowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_hintsSectionWorkflow);
        let mut _localctx: Rc<HintsSectionWorkflowContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(776);
			recog.base.match_token(WdlV1Parser_KEYWORD_HINTS,&mut recog.err_handler)?;

			recog.base.set_state(777);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(781);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule hintsItemWorkflow*/
				recog.base.set_state(778);
				recog.hintsItemWorkflow()?;

				}
				}
				recog.base.set_state(783);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(784);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsItemWorkflow ----------------
pub type HintsItemWorkflowContextAll<'input> = HintsItemWorkflowContext<'input>;


pub type HintsItemWorkflowContext<'input> = BaseParserRuleContext<'input,HintsItemWorkflowContextExt<'input>>;

#[derive(Clone)]
pub struct HintsItemWorkflowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsItemWorkflowContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsItemWorkflowContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsItemWorkflow(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsItemWorkflow(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsItemWorkflowContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsItemWorkflow(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsItemWorkflowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsItemWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsItemWorkflow }
}
antlr4rust::tid!{HintsItemWorkflowContextExt<'a>}

impl<'input> HintsItemWorkflowContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsItemWorkflowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsItemWorkflowContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsItemWorkflowContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsItemWorkflowContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn hintsValueWorkflow(&self) -> Option<Rc<HintsValueWorkflowContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HintsItemWorkflowContextAttrs<'input> for HintsItemWorkflowContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsItemWorkflow(&mut self,)
	-> Result<Rc<HintsItemWorkflowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsItemWorkflowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_hintsItemWorkflow);
        let mut _localctx: Rc<HintsItemWorkflowContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(786);
			recog.strictIdentifier()?;

			recog.base.set_state(787);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule hintsValueWorkflow*/
			recog.base.set_state(788);
			recog.hintsValueWorkflow()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsValueWorkflow ----------------
#[derive(Debug)]
pub enum HintsValueWorkflowContextAll<'input>{
	WorkflowHintValueBooleanContext(WorkflowHintValueBooleanContext<'input>),
	WorkflowHintValueStringContext(WorkflowHintValueStringContext<'input>),
	WorkflowHintValueNumberContext(WorkflowHintValueNumberContext<'input>),
	WorkflowHintValueArrayContext(WorkflowHintValueArrayContext<'input>),
	WorkflowHintValueObjectContext(WorkflowHintValueObjectContext<'input>),
Error(HintsValueWorkflowContext<'input>)
}
antlr4rust::tid!{HintsValueWorkflowContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for HintsValueWorkflowContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for HintsValueWorkflowContextAll<'input>{}

impl<'input> Deref for HintsValueWorkflowContextAll<'input>{
	type Target = dyn HintsValueWorkflowContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use HintsValueWorkflowContextAll::*;
		match self{
			WorkflowHintValueBooleanContext(inner) => inner,
			WorkflowHintValueStringContext(inner) => inner,
			WorkflowHintValueNumberContext(inner) => inner,
			WorkflowHintValueArrayContext(inner) => inner,
			WorkflowHintValueObjectContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsValueWorkflowContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsValueWorkflowContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type HintsValueWorkflowContext<'input> = BaseParserRuleContext<'input,HintsValueWorkflowContextExt<'input>>;

#[derive(Clone)]
pub struct HintsValueWorkflowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsValueWorkflowContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsValueWorkflowContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsValueWorkflowContext<'input>{
}

impl<'input> CustomRuleContext<'input> for HintsValueWorkflowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueWorkflow }
}
antlr4rust::tid!{HintsValueWorkflowContextExt<'a>}

impl<'input> HintsValueWorkflowContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsValueWorkflowContextAll<'input>> {
		Rc::new(
		HintsValueWorkflowContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsValueWorkflowContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait HintsValueWorkflowContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsValueWorkflowContextExt<'input>>{


}

impl<'input> HintsValueWorkflowContextAttrs<'input> for HintsValueWorkflowContext<'input>{}

pub type WorkflowHintValueBooleanContext<'input> = BaseParserRuleContext<'input,WorkflowHintValueBooleanContextExt<'input>>;

pub trait WorkflowHintValueBooleanContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn booleanLiteral(&self) -> Option<Rc<BooleanLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowHintValueBooleanContextAttrs<'input> for WorkflowHintValueBooleanContext<'input>{}

pub struct WorkflowHintValueBooleanContextExt<'input>{
	base:HintsValueWorkflowContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowHintValueBooleanContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowHintValueBooleanContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowHintValueBooleanContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowHintValueBoolean(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowHintValueBoolean(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowHintValueBooleanContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowHintValueBoolean(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowHintValueBooleanContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueWorkflow }
}

impl<'input> Borrow<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueBooleanContext<'input>{
	fn borrow(&self) -> &HintsValueWorkflowContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueBooleanContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueWorkflowContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueWorkflowContextAttrs<'input> for WorkflowHintValueBooleanContext<'input> {}

impl<'input> WorkflowHintValueBooleanContextExt<'input>{
	fn new(ctx: &dyn HintsValueWorkflowContextAttrs<'input>) -> Rc<HintsValueWorkflowContextAll<'input>>  {
		Rc::new(
			HintsValueWorkflowContextAll::WorkflowHintValueBooleanContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowHintValueBooleanContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowHintValueStringContext<'input> = BaseParserRuleContext<'input,WorkflowHintValueStringContextExt<'input>>;

pub trait WorkflowHintValueStringContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowHintValueStringContextAttrs<'input> for WorkflowHintValueStringContext<'input>{}

pub struct WorkflowHintValueStringContextExt<'input>{
	base:HintsValueWorkflowContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowHintValueStringContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowHintValueStringContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowHintValueStringContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowHintValueString(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowHintValueString(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowHintValueStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowHintValueString(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowHintValueStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueWorkflow }
}

impl<'input> Borrow<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueStringContext<'input>{
	fn borrow(&self) -> &HintsValueWorkflowContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueStringContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueWorkflowContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueWorkflowContextAttrs<'input> for WorkflowHintValueStringContext<'input> {}

impl<'input> WorkflowHintValueStringContextExt<'input>{
	fn new(ctx: &dyn HintsValueWorkflowContextAttrs<'input>) -> Rc<HintsValueWorkflowContextAll<'input>>  {
		Rc::new(
			HintsValueWorkflowContextAll::WorkflowHintValueStringContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowHintValueStringContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowHintValueNumberContext<'input> = BaseParserRuleContext<'input,WorkflowHintValueNumberContextExt<'input>>;

pub trait WorkflowHintValueNumberContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn numberLiteralSigned(&self) -> Option<Rc<NumberLiteralSignedContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowHintValueNumberContextAttrs<'input> for WorkflowHintValueNumberContext<'input>{}

pub struct WorkflowHintValueNumberContextExt<'input>{
	base:HintsValueWorkflowContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowHintValueNumberContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowHintValueNumberContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowHintValueNumberContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowHintValueNumber(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowHintValueNumber(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowHintValueNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowHintValueNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowHintValueNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueWorkflow }
}

impl<'input> Borrow<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueNumberContext<'input>{
	fn borrow(&self) -> &HintsValueWorkflowContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueNumberContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueWorkflowContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueWorkflowContextAttrs<'input> for WorkflowHintValueNumberContext<'input> {}

impl<'input> WorkflowHintValueNumberContextExt<'input>{
	fn new(ctx: &dyn HintsValueWorkflowContextAttrs<'input>) -> Rc<HintsValueWorkflowContextAll<'input>>  {
		Rc::new(
			HintsValueWorkflowContextAll::WorkflowHintValueNumberContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowHintValueNumberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowHintValueArrayContext<'input> = BaseParserRuleContext<'input,WorkflowHintValueArrayContextExt<'input>>;

pub trait WorkflowHintValueArrayContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn workflowHintsArray(&self) -> Option<Rc<WorkflowHintsArrayContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowHintValueArrayContextAttrs<'input> for WorkflowHintValueArrayContext<'input>{}

pub struct WorkflowHintValueArrayContextExt<'input>{
	base:HintsValueWorkflowContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowHintValueArrayContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowHintValueArrayContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowHintValueArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowHintValueArray(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowHintValueArray(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowHintValueArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowHintValueArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowHintValueArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueWorkflow }
}

impl<'input> Borrow<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueArrayContext<'input>{
	fn borrow(&self) -> &HintsValueWorkflowContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueArrayContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueWorkflowContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueWorkflowContextAttrs<'input> for WorkflowHintValueArrayContext<'input> {}

impl<'input> WorkflowHintValueArrayContextExt<'input>{
	fn new(ctx: &dyn HintsValueWorkflowContextAttrs<'input>) -> Rc<HintsValueWorkflowContextAll<'input>>  {
		Rc::new(
			HintsValueWorkflowContextAll::WorkflowHintValueArrayContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowHintValueArrayContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WorkflowHintValueObjectContext<'input> = BaseParserRuleContext<'input,WorkflowHintValueObjectContextExt<'input>>;

pub trait WorkflowHintValueObjectContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn hintsObjectWorkflow(&self) -> Option<Rc<HintsObjectWorkflowContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WorkflowHintValueObjectContextAttrs<'input> for WorkflowHintValueObjectContext<'input>{}

pub struct WorkflowHintValueObjectContextExt<'input>{
	base:HintsValueWorkflowContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{WorkflowHintValueObjectContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for WorkflowHintValueObjectContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowHintValueObjectContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_workflowHintValueObject(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_workflowHintValueObject(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowHintValueObjectContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowHintValueObject(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowHintValueObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsValueWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsValueWorkflow }
}

impl<'input> Borrow<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueObjectContext<'input>{
	fn borrow(&self) -> &HintsValueWorkflowContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<HintsValueWorkflowContextExt<'input>> for WorkflowHintValueObjectContext<'input>{
	fn borrow_mut(&mut self) -> &mut HintsValueWorkflowContextExt<'input> { &mut self.base }
}

impl<'input> HintsValueWorkflowContextAttrs<'input> for WorkflowHintValueObjectContext<'input> {}

impl<'input> WorkflowHintValueObjectContextExt<'input>{
	fn new(ctx: &dyn HintsValueWorkflowContextAttrs<'input>) -> Rc<HintsValueWorkflowContextAll<'input>>  {
		Rc::new(
			HintsValueWorkflowContextAll::WorkflowHintValueObjectContext(
				BaseParserRuleContext::copy_from(ctx,WorkflowHintValueObjectContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsValueWorkflow(&mut self,)
	-> Result<Rc<HintsValueWorkflowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsValueWorkflowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_hintsValueWorkflow);
        let mut _localctx: Rc<HintsValueWorkflowContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(795);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_FLOAT |WdlV1Parser_INTEGER |WdlV1Parser_MINUS 
				=> {
					let tmp = WorkflowHintValueNumberContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule numberLiteralSigned*/
					recog.base.set_state(790);
					recog.numberLiteralSigned()?;

					}
				}

			WdlV1Parser_OPEN_MULTILINE_STRING |WdlV1Parser_SINGLE_QUOTE |WdlV1Parser_DOUBLE_QUOTE 
				=> {
					let tmp = WorkflowHintValueStringContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(791);
					recog.stringLiteral()?;

					}
				}

			WdlV1Parser_KEYWORD_FALSE |WdlV1Parser_KEYWORD_TRUE 
				=> {
					let tmp = WorkflowHintValueBooleanContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule booleanLiteral*/
					recog.base.set_state(792);
					recog.booleanLiteral()?;

					}
				}

			WdlV1Parser_OPEN_BRACE 
				=> {
					let tmp = WorkflowHintValueObjectContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					/*InvokeRule hintsObjectWorkflow*/
					recog.base.set_state(793);
					recog.hintsObjectWorkflow()?;

					}
				}

			WdlV1Parser_OPEN_BRACKET 
				=> {
					let tmp = WorkflowHintValueArrayContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					/*InvokeRule workflowHintsArray*/
					recog.base.set_state(794);
					recog.workflowHintsArray()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsObjectWorkflow ----------------
pub type HintsObjectWorkflowContextAll<'input> = HintsObjectWorkflowContext<'input>;


pub type HintsObjectWorkflowContext<'input> = BaseParserRuleContext<'input,HintsObjectWorkflowContextExt<'input>>;

#[derive(Clone)]
pub struct HintsObjectWorkflowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsObjectWorkflowContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsObjectWorkflowContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsObjectWorkflow(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsObjectWorkflow(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsObjectWorkflowContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsObjectWorkflow(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsObjectWorkflowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsObjectWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsObjectWorkflow }
}
antlr4rust::tid!{HintsObjectWorkflowContextExt<'a>}

impl<'input> HintsObjectWorkflowContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsObjectWorkflowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsObjectWorkflowContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsObjectWorkflowContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsObjectWorkflowContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn hintsObjectItemWorkflow_all(&self) ->  Vec<Rc<HintsObjectItemWorkflowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hintsObjectItemWorkflow(&self, i: usize) -> Option<Rc<HintsObjectItemWorkflowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> HintsObjectWorkflowContextAttrs<'input> for HintsObjectWorkflowContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsObjectWorkflow(&mut self,)
	-> Result<Rc<HintsObjectWorkflowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsObjectWorkflowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_hintsObjectWorkflow);
        let mut _localctx: Rc<HintsObjectWorkflowContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(797);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(809);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule hintsObjectItemWorkflow*/
				recog.base.set_state(798);
				recog.hintsObjectItemWorkflow()?;

				recog.base.set_state(803);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(72,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(799);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule hintsObjectItemWorkflow*/
						recog.base.set_state(800);
						recog.hintsObjectItemWorkflow()?;

						}
						} 
					}
					recog.base.set_state(805);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(72,&mut recog.base)?;
				}
				recog.base.set_state(807);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(806);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(811);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hintsObjectItemWorkflow ----------------
pub type HintsObjectItemWorkflowContextAll<'input> = HintsObjectItemWorkflowContext<'input>;


pub type HintsObjectItemWorkflowContext<'input> = BaseParserRuleContext<'input,HintsObjectItemWorkflowContextExt<'input>>;

#[derive(Clone)]
pub struct HintsObjectItemWorkflowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for HintsObjectItemWorkflowContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for HintsObjectItemWorkflowContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hintsObjectItemWorkflow(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hintsObjectItemWorkflow(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for HintsObjectItemWorkflowContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_hintsObjectItemWorkflow(self);
	}
}

impl<'input> CustomRuleContext<'input> for HintsObjectItemWorkflowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hintsObjectItemWorkflow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hintsObjectItemWorkflow }
}
antlr4rust::tid!{HintsObjectItemWorkflowContextExt<'a>}

impl<'input> HintsObjectItemWorkflowContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HintsObjectItemWorkflowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HintsObjectItemWorkflowContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HintsObjectItemWorkflowContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<HintsObjectItemWorkflowContextExt<'input>>{

fn dottedIdentifier(&self) -> Option<Rc<DottedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn hintsValueWorkflow(&self) -> Option<Rc<HintsValueWorkflowContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> HintsObjectItemWorkflowContextAttrs<'input> for HintsObjectItemWorkflowContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hintsObjectItemWorkflow(&mut self,)
	-> Result<Rc<HintsObjectItemWorkflowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HintsObjectItemWorkflowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_hintsObjectItemWorkflow);
        let mut _localctx: Rc<HintsObjectItemWorkflowContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule dottedIdentifier*/
			recog.base.set_state(813);
			recog.dottedIdentifier()?;

			recog.base.set_state(814);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule hintsValueWorkflow*/
			recog.base.set_state(815);
			recog.hintsValueWorkflow()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- workflowHintsArray ----------------
pub type WorkflowHintsArrayContextAll<'input> = WorkflowHintsArrayContext<'input>;


pub type WorkflowHintsArrayContext<'input> = BaseParserRuleContext<'input,WorkflowHintsArrayContextExt<'input>>;

#[derive(Clone)]
pub struct WorkflowHintsArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for WorkflowHintsArrayContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowHintsArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_workflowHintsArray(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_workflowHintsArray(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowHintsArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowHintsArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowHintsArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowHintsArray }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowHintsArray }
}
antlr4rust::tid!{WorkflowHintsArrayContextExt<'a>}

impl<'input> WorkflowHintsArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<WorkflowHintsArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WorkflowHintsArrayContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait WorkflowHintsArrayContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<WorkflowHintsArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
fn hintsValueWorkflow_all(&self) ->  Vec<Rc<HintsValueWorkflowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hintsValueWorkflow(&self, i: usize) -> Option<Rc<HintsValueWorkflowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> WorkflowHintsArrayContextAttrs<'input> for WorkflowHintsArrayContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn workflowHintsArray(&mut self,)
	-> Result<Rc<WorkflowHintsArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WorkflowHintsArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_workflowHintsArray);
        let mut _localctx: Rc<WorkflowHintsArrayContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(817);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(829);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 536874432) != 0) || ((((_la - 49)) & !0x3f) == 0 && ((1usize << (_la - 49)) & 8398849) != 0) {
				{
				/*InvokeRule hintsValueWorkflow*/
				recog.base.set_state(818);
				recog.hintsValueWorkflow()?;

				recog.base.set_state(823);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(75,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(819);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule hintsValueWorkflow*/
						recog.base.set_state(820);
						recog.hintsValueWorkflow()?;

						}
						} 
					}
					recog.base.set_state(825);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(75,&mut recog.base)?;
				}
				recog.base.set_state(827);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(826);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(831);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- metadataSection ----------------
pub type MetadataSectionContextAll<'input> = MetadataSectionContext<'input>;


pub type MetadataSectionContext<'input> = BaseParserRuleContext<'input,MetadataSectionContextExt<'input>>;

#[derive(Clone)]
pub struct MetadataSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MetadataSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MetadataSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_metadataSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_metadataSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MetadataSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_metadataSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for MetadataSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_metadataSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_metadataSection }
}
antlr4rust::tid!{MetadataSectionContextExt<'a>}

impl<'input> MetadataSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MetadataSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MetadataSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MetadataSectionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MetadataSectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_META
/// Returns `None` if there is no child corresponding to token KEYWORD_META
fn KEYWORD_META(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_META, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn metadataObjectItem_all(&self) ->  Vec<Rc<MetadataObjectItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn metadataObjectItem(&self, i: usize) -> Option<Rc<MetadataObjectItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MetadataSectionContextAttrs<'input> for MetadataSectionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn metadataSection(&mut self,)
	-> Result<Rc<MetadataSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MetadataSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_metadataSection);
        let mut _localctx: Rc<MetadataSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(833);
			recog.base.match_token(WdlV1Parser_KEYWORD_META,&mut recog.err_handler)?;

			recog.base.set_state(834);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(838);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule metadataObjectItem*/
				recog.base.set_state(835);
				recog.metadataObjectItem()?;

				}
				}
				recog.base.set_state(840);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(841);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- parameterMetadataSection ----------------
pub type ParameterMetadataSectionContextAll<'input> = ParameterMetadataSectionContext<'input>;


pub type ParameterMetadataSectionContext<'input> = BaseParserRuleContext<'input,ParameterMetadataSectionContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterMetadataSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ParameterMetadataSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ParameterMetadataSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parameterMetadataSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parameterMetadataSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ParameterMetadataSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_parameterMetadataSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterMetadataSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterMetadataSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterMetadataSection }
}
antlr4rust::tid!{ParameterMetadataSectionContextExt<'a>}

impl<'input> ParameterMetadataSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterMetadataSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterMetadataSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterMetadataSectionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ParameterMetadataSectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_PARAMETER_META
/// Returns `None` if there is no child corresponding to token KEYWORD_PARAMETER_META
fn KEYWORD_PARAMETER_META(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_PARAMETER_META, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn metadataObjectItem_all(&self) ->  Vec<Rc<MetadataObjectItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn metadataObjectItem(&self, i: usize) -> Option<Rc<MetadataObjectItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParameterMetadataSectionContextAttrs<'input> for ParameterMetadataSectionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameterMetadataSection(&mut self,)
	-> Result<Rc<ParameterMetadataSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterMetadataSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_parameterMetadataSection);
        let mut _localctx: Rc<ParameterMetadataSectionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(843);
			recog.base.match_token(WdlV1Parser_KEYWORD_PARAMETER_META,&mut recog.err_handler)?;

			recog.base.set_state(844);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(848);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule metadataObjectItem*/
				recog.base.set_state(845);
				recog.metadataObjectItem()?;

				}
				}
				recog.base.set_state(850);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(851);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- metadataObject ----------------
pub type MetadataObjectContextAll<'input> = MetadataObjectContext<'input>;


pub type MetadataObjectContext<'input> = BaseParserRuleContext<'input,MetadataObjectContextExt<'input>>;

#[derive(Clone)]
pub struct MetadataObjectContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MetadataObjectContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MetadataObjectContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_metadataObject(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_metadataObject(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MetadataObjectContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_metadataObject(self);
	}
}

impl<'input> CustomRuleContext<'input> for MetadataObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_metadataObject }
	//fn type_rule_index() -> usize where Self: Sized { RULE_metadataObject }
}
antlr4rust::tid!{MetadataObjectContextExt<'a>}

impl<'input> MetadataObjectContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MetadataObjectContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MetadataObjectContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MetadataObjectContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MetadataObjectContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn metadataObjectItem_all(&self) ->  Vec<Rc<MetadataObjectItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn metadataObjectItem(&self, i: usize) -> Option<Rc<MetadataObjectItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> MetadataObjectContextAttrs<'input> for MetadataObjectContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn metadataObject(&mut self,)
	-> Result<Rc<MetadataObjectContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MetadataObjectContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_metadataObject);
        let mut _localctx: Rc<MetadataObjectContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(853);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(865);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule metadataObjectItem*/
				recog.base.set_state(854);
				recog.metadataObjectItem()?;

				recog.base.set_state(859);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(80,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(855);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule metadataObjectItem*/
						recog.base.set_state(856);
						recog.metadataObjectItem()?;

						}
						} 
					}
					recog.base.set_state(861);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(80,&mut recog.base)?;
				}
				recog.base.set_state(863);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(862);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(867);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- metadataObjectItem ----------------
pub type MetadataObjectItemContextAll<'input> = MetadataObjectItemContext<'input>;


pub type MetadataObjectItemContext<'input> = BaseParserRuleContext<'input,MetadataObjectItemContextExt<'input>>;

#[derive(Clone)]
pub struct MetadataObjectItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MetadataObjectItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MetadataObjectItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_metadataObjectItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_metadataObjectItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MetadataObjectItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_metadataObjectItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for MetadataObjectItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_metadataObjectItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_metadataObjectItem }
}
antlr4rust::tid!{MetadataObjectItemContextExt<'a>}

impl<'input> MetadataObjectItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MetadataObjectItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MetadataObjectItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MetadataObjectItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MetadataObjectItemContextExt<'input>>{

fn dottedIdentifier(&self) -> Option<Rc<DottedIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn metadataValue(&self) -> Option<Rc<MetadataValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MetadataObjectItemContextAttrs<'input> for MetadataObjectItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn metadataObjectItem(&mut self,)
	-> Result<Rc<MetadataObjectItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MetadataObjectItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_metadataObjectItem);
        let mut _localctx: Rc<MetadataObjectItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule dottedIdentifier*/
			recog.base.set_state(869);
			recog.dottedIdentifier()?;

			recog.base.set_state(870);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule metadataValue*/
			recog.base.set_state(871);
			recog.metadataValue()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- metadataArray ----------------
pub type MetadataArrayContextAll<'input> = MetadataArrayContext<'input>;


pub type MetadataArrayContext<'input> = BaseParserRuleContext<'input,MetadataArrayContextExt<'input>>;

#[derive(Clone)]
pub struct MetadataArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MetadataArrayContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MetadataArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_metadataArray(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_metadataArray(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MetadataArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_metadataArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for MetadataArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_metadataArray }
	//fn type_rule_index() -> usize where Self: Sized { RULE_metadataArray }
}
antlr4rust::tid!{MetadataArrayContextExt<'a>}

impl<'input> MetadataArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MetadataArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MetadataArrayContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MetadataArrayContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MetadataArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
fn metadataValue_all(&self) ->  Vec<Rc<MetadataValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn metadataValue(&self, i: usize) -> Option<Rc<MetadataValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> MetadataArrayContextAttrs<'input> for MetadataArrayContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn metadataArray(&mut self,)
	-> Result<Rc<MetadataArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MetadataArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_metadataArray);
        let mut _localctx: Rc<MetadataArrayContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(873);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(885);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 536874432) != 0) || ((((_la - 38)) & !0x3f) == 0 && ((1usize << (_la - 38)) & 20973569) != 0) || _la==WdlV1Parser_MINUS {
				{
				/*InvokeRule metadataValue*/
				recog.base.set_state(874);
				recog.metadataValue()?;

				recog.base.set_state(879);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(875);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule metadataValue*/
						recog.base.set_state(876);
						recog.metadataValue()?;

						}
						} 
					}
					recog.base.set_state(881);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
				}
				recog.base.set_state(883);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(882);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(887);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- metadataValue ----------------
pub type MetadataValueContextAll<'input> = MetadataValueContext<'input>;


pub type MetadataValueContext<'input> = BaseParserRuleContext<'input,MetadataValueContextExt<'input>>;

#[derive(Clone)]
pub struct MetadataValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MetadataValueContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MetadataValueContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_metadataValue(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_metadataValue(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MetadataValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_metadataValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for MetadataValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_metadataValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_metadataValue }
}
antlr4rust::tid!{MetadataValueContextExt<'a>}

impl<'input> MetadataValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MetadataValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MetadataValueContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MetadataValueContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MetadataValueContextExt<'input>>{

fn numberLiteralSigned(&self) -> Option<Rc<NumberLiteralSignedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn booleanLiteral(&self) -> Option<Rc<BooleanLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nullLiteral(&self) -> Option<Rc<NullLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn metadataObject(&self) -> Option<Rc<MetadataObjectContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn metadataArray(&self) -> Option<Rc<MetadataArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MetadataValueContextAttrs<'input> for MetadataValueContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn metadataValue(&mut self,)
	-> Result<Rc<MetadataValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MetadataValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_metadataValue);
        let mut _localctx: Rc<MetadataValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(895);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_FLOAT |WdlV1Parser_INTEGER |WdlV1Parser_MINUS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule numberLiteralSigned*/
					recog.base.set_state(889);
					recog.numberLiteralSigned()?;

					}
				}

			WdlV1Parser_OPEN_MULTILINE_STRING |WdlV1Parser_SINGLE_QUOTE |WdlV1Parser_DOUBLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(890);
					recog.stringLiteral()?;

					}
				}

			WdlV1Parser_KEYWORD_FALSE |WdlV1Parser_KEYWORD_TRUE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule booleanLiteral*/
					recog.base.set_state(891);
					recog.booleanLiteral()?;

					}
				}

			WdlV1Parser_KEYWORD_NULL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule nullLiteral*/
					recog.base.set_state(892);
					recog.nullLiteral()?;

					}
				}

			WdlV1Parser_OPEN_BRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule metadataObject*/
					recog.base.set_state(893);
					recog.metadataObject()?;

					}
				}

			WdlV1Parser_OPEN_BRACKET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule metadataArray*/
					recog.base.set_state(894);
					recog.metadataArray()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- commandSection ----------------
pub type CommandSectionContextAll<'input> = CommandSectionContext<'input>;


pub type CommandSectionContext<'input> = BaseParserRuleContext<'input,CommandSectionContextExt<'input>>;

#[derive(Clone)]
pub struct CommandSectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CommandSectionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CommandSectionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_commandSection(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_commandSection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CommandSectionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_commandSection(self);
	}
}

impl<'input> CustomRuleContext<'input> for CommandSectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_commandSection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_commandSection }
}
antlr4rust::tid!{CommandSectionContextExt<'a>}

impl<'input> CommandSectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CommandSectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommandSectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CommandSectionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CommandSectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_COMMAND
/// Returns `None` if there is no child corresponding to token KEYWORD_COMMAND
fn KEYWORD_COMMAND(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_COMMAND, 0)
}
fn multilineStringCommand(&self) -> Option<Rc<MultilineStringCommandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn bracedCommand(&self) -> Option<Rc<BracedCommandContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CommandSectionContextAttrs<'input> for CommandSectionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn commandSection(&mut self,)
	-> Result<Rc<CommandSectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommandSectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_commandSection);
        let mut _localctx: Rc<CommandSectionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(897);
			recog.base.match_token(WdlV1Parser_KEYWORD_COMMAND,&mut recog.err_handler)?;

			recog.base.set_state(900);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_OPEN_MULTILINE_STRING 
				=> {
					{
					/*InvokeRule multilineStringCommand*/
					recog.base.set_state(898);
					recog.multilineStringCommand()?;

					}
				}

			WdlV1Parser_OPEN_BRACE 
				=> {
					{
					/*InvokeRule bracedCommand*/
					recog.base.set_state(899);
					recog.bracedCommand()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- multilineStringCommand ----------------
pub type MultilineStringCommandContextAll<'input> = MultilineStringCommandContext<'input>;


pub type MultilineStringCommandContext<'input> = BaseParserRuleContext<'input,MultilineStringCommandContextExt<'input>>;

#[derive(Clone)]
pub struct MultilineStringCommandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MultilineStringCommandContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringCommandContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_multilineStringCommand(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_multilineStringCommand(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringCommandContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringCommand(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringCommandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringCommand }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringCommand }
}
antlr4rust::tid!{MultilineStringCommandContextExt<'a>}

impl<'input> MultilineStringCommandContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MultilineStringCommandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultilineStringCommandContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MultilineStringCommandContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MultilineStringCommandContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_MULTILINE_STRING
/// Returns `None` if there is no child corresponding to token OPEN_MULTILINE_STRING
fn OPEN_MULTILINE_STRING(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_MULTILINE_STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_END
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_END
fn MULTILINE_STRING_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_END, 0)
}
fn multilineStringElement_all(&self) ->  Vec<Rc<MultilineStringElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multilineStringElement(&self, i: usize) -> Option<Rc<MultilineStringElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MultilineStringCommandContextAttrs<'input> for MultilineStringCommandContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn multilineStringCommand(&mut self,)
	-> Result<Rc<MultilineStringCommandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultilineStringCommandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_multilineStringCommand);
        let mut _localctx: Rc<MultilineStringCommandContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(902);
			recog.base.match_token(WdlV1Parser_OPEN_MULTILINE_STRING,&mut recog.err_handler)?;

			recog.base.set_state(906);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 84)) & !0x3f) == 0 && ((1usize << (_la - 84)) & 503) != 0) {
				{
				{
				/*InvokeRule multilineStringElement*/
				recog.base.set_state(903);
				recog.multilineStringElement()?;

				}
				}
				recog.base.set_state(908);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(909);
			recog.base.match_token(WdlV1Parser_MULTILINE_STRING_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- bracedCommand ----------------
pub type BracedCommandContextAll<'input> = BracedCommandContext<'input>;


pub type BracedCommandContext<'input> = BaseParserRuleContext<'input,BracedCommandContextExt<'input>>;

#[derive(Clone)]
pub struct BracedCommandContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for BracedCommandContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for BracedCommandContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_bracedCommand(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_bracedCommand(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for BracedCommandContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_bracedCommand(self);
	}
}

impl<'input> CustomRuleContext<'input> for BracedCommandContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bracedCommand }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bracedCommand }
}
antlr4rust::tid!{BracedCommandContextExt<'a>}

impl<'input> BracedCommandContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BracedCommandContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BracedCommandContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait BracedCommandContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<BracedCommandContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn stringElement_all(&self) ->  Vec<Rc<StringElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stringElement(&self, i: usize) -> Option<Rc<StringElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BracedCommandContextAttrs<'input> for BracedCommandContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn bracedCommand(&mut self,)
	-> Result<Rc<BracedCommandContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BracedCommandContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_bracedCommand);
        let mut _localctx: Rc<BracedCommandContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(911);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(915);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 62) != 0) {
				{
				{
				/*InvokeRule stringElement*/
				recog.base.set_state(912);
				recog.stringElement()?;

				}
				}
				recog.base.set_state(917);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(918);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- workflowStatement ----------------
pub type WorkflowStatementContextAll<'input> = WorkflowStatementContext<'input>;


pub type WorkflowStatementContext<'input> = BaseParserRuleContext<'input,WorkflowStatementContextExt<'input>>;

#[derive(Clone)]
pub struct WorkflowStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for WorkflowStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for WorkflowStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_workflowStatement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_workflowStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for WorkflowStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_workflowStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for WorkflowStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_workflowStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_workflowStatement }
}
antlr4rust::tid!{WorkflowStatementContextExt<'a>}

impl<'input> WorkflowStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<WorkflowStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WorkflowStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait WorkflowStatementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<WorkflowStatementContextExt<'input>>{

fn conditionalStatement(&self) -> Option<Rc<ConditionalStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scatterStatement(&self) -> Option<Rc<ScatterStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn callStatement(&self) -> Option<Rc<CallStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn boundDeclaration(&self) -> Option<Rc<BoundDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WorkflowStatementContextAttrs<'input> for WorkflowStatementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn workflowStatement(&mut self,)
	-> Result<Rc<WorkflowStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WorkflowStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_workflowStatement);
        let mut _localctx: Rc<WorkflowStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(924);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(90,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule conditionalStatement*/
					recog.base.set_state(920);
					recog.conditionalStatement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule scatterStatement*/
					recog.base.set_state(921);
					recog.scatterStatement()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule callStatement*/
					recog.base.set_state(922);
					recog.callStatement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule boundDeclaration*/
					recog.base.set_state(923);
					recog.boundDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- conditionalStatement ----------------
pub type ConditionalStatementContextAll<'input> = ConditionalStatementContext<'input>;


pub type ConditionalStatementContext<'input> = BaseParserRuleContext<'input,ConditionalStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ConditionalStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ConditionalStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_conditionalStatement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_conditionalStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ConditionalStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_conditionalStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalStatement }
}
antlr4rust::tid!{ConditionalStatementContextExt<'a>}

impl<'input> ConditionalStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConditionalStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalStatementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ConditionalStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_IF
/// Returns `None` if there is no child corresponding to token KEYWORD_IF
fn KEYWORD_IF(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_IF, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_PAREN
/// Returns `None` if there is no child corresponding to token OPEN_PAREN
fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_PAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
/// Returns `None` if there is no child corresponding to token CLOSE_PAREN
fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn workflowStatement_all(&self) ->  Vec<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn workflowStatement(&self, i: usize) -> Option<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn conditionalElseIfClause_all(&self) ->  Vec<Rc<ConditionalElseIfClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn conditionalElseIfClause(&self, i: usize) -> Option<Rc<ConditionalElseIfClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn conditionalElseClause(&self) -> Option<Rc<ConditionalElseClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConditionalStatementContextAttrs<'input> for ConditionalStatementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn conditionalStatement(&mut self,)
	-> Result<Rc<ConditionalStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_conditionalStatement);
        let mut _localctx: Rc<ConditionalStatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(926);
			recog.base.match_token(WdlV1Parser_KEYWORD_IF,&mut recog.err_handler)?;

			recog.base.set_state(927);
			recog.base.match_token(WdlV1Parser_OPEN_PAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(928);
			recog.expression()?;

			recog.base.set_state(929);
			recog.base.match_token(WdlV1Parser_CLOSE_PAREN,&mut recog.err_handler)?;

			recog.base.set_state(930);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(934);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule workflowStatement*/
				recog.base.set_state(931);
				recog.workflowStatement()?;

				}
				}
				recog.base.set_state(936);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(937);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(941);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(92,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule conditionalElseIfClause*/
					recog.base.set_state(938);
					recog.conditionalElseIfClause()?;

					}
					} 
				}
				recog.base.set_state(943);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(92,&mut recog.base)?;
			}
			recog.base.set_state(945);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(93,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule conditionalElseClause*/
					recog.base.set_state(944);
					recog.conditionalElseClause()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- conditionalElseIfClause ----------------
pub type ConditionalElseIfClauseContextAll<'input> = ConditionalElseIfClauseContext<'input>;


pub type ConditionalElseIfClauseContext<'input> = BaseParserRuleContext<'input,ConditionalElseIfClauseContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalElseIfClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ConditionalElseIfClauseContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ConditionalElseIfClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_conditionalElseIfClause(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_conditionalElseIfClause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ConditionalElseIfClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_conditionalElseIfClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalElseIfClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalElseIfClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalElseIfClause }
}
antlr4rust::tid!{ConditionalElseIfClauseContextExt<'a>}

impl<'input> ConditionalElseIfClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConditionalElseIfClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalElseIfClauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalElseIfClauseContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ConditionalElseIfClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_ELSE
/// Returns `None` if there is no child corresponding to token KEYWORD_ELSE
fn KEYWORD_ELSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_IF
/// Returns `None` if there is no child corresponding to token KEYWORD_IF
fn KEYWORD_IF(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_IF, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_PAREN
/// Returns `None` if there is no child corresponding to token OPEN_PAREN
fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_PAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
/// Returns `None` if there is no child corresponding to token CLOSE_PAREN
fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn workflowStatement_all(&self) ->  Vec<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn workflowStatement(&self, i: usize) -> Option<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ConditionalElseIfClauseContextAttrs<'input> for ConditionalElseIfClauseContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn conditionalElseIfClause(&mut self,)
	-> Result<Rc<ConditionalElseIfClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalElseIfClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_conditionalElseIfClause);
        let mut _localctx: Rc<ConditionalElseIfClauseContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(947);
			recog.base.match_token(WdlV1Parser_KEYWORD_ELSE,&mut recog.err_handler)?;

			recog.base.set_state(948);
			recog.base.match_token(WdlV1Parser_KEYWORD_IF,&mut recog.err_handler)?;

			recog.base.set_state(949);
			recog.base.match_token(WdlV1Parser_OPEN_PAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(950);
			recog.expression()?;

			recog.base.set_state(951);
			recog.base.match_token(WdlV1Parser_CLOSE_PAREN,&mut recog.err_handler)?;

			recog.base.set_state(952);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(956);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule workflowStatement*/
				recog.base.set_state(953);
				recog.workflowStatement()?;

				}
				}
				recog.base.set_state(958);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(959);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- conditionalElseClause ----------------
pub type ConditionalElseClauseContextAll<'input> = ConditionalElseClauseContext<'input>;


pub type ConditionalElseClauseContext<'input> = BaseParserRuleContext<'input,ConditionalElseClauseContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalElseClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ConditionalElseClauseContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ConditionalElseClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_conditionalElseClause(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_conditionalElseClause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ConditionalElseClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_conditionalElseClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConditionalElseClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalElseClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalElseClause }
}
antlr4rust::tid!{ConditionalElseClauseContextExt<'a>}

impl<'input> ConditionalElseClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConditionalElseClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalElseClauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalElseClauseContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ConditionalElseClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_ELSE
/// Returns `None` if there is no child corresponding to token KEYWORD_ELSE
fn KEYWORD_ELSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn workflowStatement_all(&self) ->  Vec<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn workflowStatement(&self, i: usize) -> Option<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ConditionalElseClauseContextAttrs<'input> for ConditionalElseClauseContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn conditionalElseClause(&mut self,)
	-> Result<Rc<ConditionalElseClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalElseClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_conditionalElseClause);
        let mut _localctx: Rc<ConditionalElseClauseContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(961);
			recog.base.match_token(WdlV1Parser_KEYWORD_ELSE,&mut recog.err_handler)?;

			recog.base.set_state(962);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(966);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule workflowStatement*/
				recog.base.set_state(963);
				recog.workflowStatement()?;

				}
				}
				recog.base.set_state(968);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(969);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- scatterStatement ----------------
pub type ScatterStatementContextAll<'input> = ScatterStatementContext<'input>;


pub type ScatterStatementContext<'input> = BaseParserRuleContext<'input,ScatterStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ScatterStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ScatterStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ScatterStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_scatterStatement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_scatterStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ScatterStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_scatterStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScatterStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scatterStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scatterStatement }
}
antlr4rust::tid!{ScatterStatementContextExt<'a>}

impl<'input> ScatterStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ScatterStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScatterStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ScatterStatementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ScatterStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_SCATTER
/// Returns `None` if there is no child corresponding to token KEYWORD_SCATTER
fn KEYWORD_SCATTER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_SCATTER, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_PAREN
/// Returns `None` if there is no child corresponding to token OPEN_PAREN
fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_PAREN, 0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_IN
/// Returns `None` if there is no child corresponding to token KEYWORD_IN
fn KEYWORD_IN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_IN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
/// Returns `None` if there is no child corresponding to token CLOSE_PAREN
fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_PAREN, 0)
}
fn scatterBody(&self) -> Option<Rc<ScatterBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ScatterStatementContextAttrs<'input> for ScatterStatementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn scatterStatement(&mut self,)
	-> Result<Rc<ScatterStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScatterStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_scatterStatement);
        let mut _localctx: Rc<ScatterStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(971);
			recog.base.match_token(WdlV1Parser_KEYWORD_SCATTER,&mut recog.err_handler)?;

			recog.base.set_state(972);
			recog.base.match_token(WdlV1Parser_OPEN_PAREN,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(973);
			recog.strictIdentifier()?;

			recog.base.set_state(974);
			recog.base.match_token(WdlV1Parser_KEYWORD_IN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(975);
			recog.expression()?;

			recog.base.set_state(976);
			recog.base.match_token(WdlV1Parser_CLOSE_PAREN,&mut recog.err_handler)?;

			/*InvokeRule scatterBody*/
			recog.base.set_state(977);
			recog.scatterBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- scatterBody ----------------
pub type ScatterBodyContextAll<'input> = ScatterBodyContext<'input>;


pub type ScatterBodyContext<'input> = BaseParserRuleContext<'input,ScatterBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ScatterBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ScatterBodyContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ScatterBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_scatterBody(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_scatterBody(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ScatterBodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_scatterBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScatterBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scatterBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scatterBody }
}
antlr4rust::tid!{ScatterBodyContextExt<'a>}

impl<'input> ScatterBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ScatterBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScatterBodyContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ScatterBodyContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ScatterBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn workflowStatement_all(&self) ->  Vec<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn workflowStatement(&self, i: usize) -> Option<Rc<WorkflowStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ScatterBodyContextAttrs<'input> for ScatterBodyContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn scatterBody(&mut self,)
	-> Result<Rc<ScatterBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScatterBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_scatterBody);
        let mut _localctx: Rc<ScatterBodyContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(979);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(983);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				{
				/*InvokeRule workflowStatement*/
				recog.base.set_state(980);
				recog.workflowStatement()?;

				}
				}
				recog.base.set_state(985);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(986);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- callStatement ----------------
pub type CallStatementContextAll<'input> = CallStatementContext<'input>;


pub type CallStatementContext<'input> = BaseParserRuleContext<'input,CallStatementContextExt<'input>>;

#[derive(Clone)]
pub struct CallStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CallStatementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CallStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_callStatement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_callStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CallStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_callStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callStatement }
}
antlr4rust::tid!{CallStatementContextExt<'a>}

impl<'input> CallStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CallStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CallStatementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CallStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_CALL
/// Returns `None` if there is no child corresponding to token KEYWORD_CALL
fn KEYWORD_CALL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_CALL, 0)
}
fn callTarget(&self) -> Option<Rc<CallTargetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn callAlias(&self) -> Option<Rc<CallAliasContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn callAfterClause_all(&self) ->  Vec<Rc<CallAfterClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn callAfterClause(&self, i: usize) -> Option<Rc<CallAfterClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn callInputBlock(&self) -> Option<Rc<CallInputBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CallStatementContextAttrs<'input> for CallStatementContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn callStatement(&mut self,)
	-> Result<Rc<CallStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_callStatement);
        let mut _localctx: Rc<CallStatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(988);
			recog.base.match_token(WdlV1Parser_KEYWORD_CALL,&mut recog.err_handler)?;

			/*InvokeRule callTarget*/
			recog.base.set_state(989);
			recog.callTarget()?;

			recog.base.set_state(991);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(97,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule callAlias*/
					recog.base.set_state(990);
					recog.callAlias()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(996);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(98,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule callAfterClause*/
					recog.base.set_state(993);
					recog.callAfterClause()?;

					}
					} 
				}
				recog.base.set_state(998);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(98,&mut recog.base)?;
			}
			recog.base.set_state(1000);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_OPEN_BRACE {
				{
				/*InvokeRule callInputBlock*/
				recog.base.set_state(999);
				recog.callInputBlock()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- callTarget ----------------
pub type CallTargetContextAll<'input> = CallTargetContext<'input>;


pub type CallTargetContext<'input> = BaseParserRuleContext<'input,CallTargetContextExt<'input>>;

#[derive(Clone)]
pub struct CallTargetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CallTargetContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CallTargetContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_callTarget(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_callTarget(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CallTargetContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_callTarget(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallTargetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callTarget }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callTarget }
}
antlr4rust::tid!{CallTargetContextExt<'a>}

impl<'input> CallTargetContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CallTargetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallTargetContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CallTargetContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CallTargetContextExt<'input>>{

fn strictIdentifier_all(&self) ->  Vec<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn strictIdentifier(&self, i: usize) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOT, i)
}

}

impl<'input> CallTargetContextAttrs<'input> for CallTargetContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn callTarget(&mut self,)
	-> Result<Rc<CallTargetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallTargetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_callTarget);
        let mut _localctx: Rc<CallTargetContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1002);
			recog.strictIdentifier()?;

			recog.base.set_state(1007);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==WdlV1Parser_DOT {
				{
				{
				recog.base.set_state(1003);
				recog.base.match_token(WdlV1Parser_DOT,&mut recog.err_handler)?;

				/*InvokeRule strictIdentifier*/
				recog.base.set_state(1004);
				recog.strictIdentifier()?;

				}
				}
				recog.base.set_state(1009);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- callAlias ----------------
pub type CallAliasContextAll<'input> = CallAliasContext<'input>;


pub type CallAliasContext<'input> = BaseParserRuleContext<'input,CallAliasContextExt<'input>>;

#[derive(Clone)]
pub struct CallAliasContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CallAliasContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CallAliasContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_callAlias(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_callAlias(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CallAliasContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_callAlias(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallAliasContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callAlias }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callAlias }
}
antlr4rust::tid!{CallAliasContextExt<'a>}

impl<'input> CallAliasContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CallAliasContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallAliasContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CallAliasContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CallAliasContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_AS
/// Returns `None` if there is no child corresponding to token KEYWORD_AS
fn KEYWORD_AS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_AS, 0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CallAliasContextAttrs<'input> for CallAliasContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn callAlias(&mut self,)
	-> Result<Rc<CallAliasContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallAliasContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_callAlias);
        let mut _localctx: Rc<CallAliasContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1010);
			recog.base.match_token(WdlV1Parser_KEYWORD_AS,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1011);
			recog.strictIdentifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- callAfterClause ----------------
pub type CallAfterClauseContextAll<'input> = CallAfterClauseContext<'input>;


pub type CallAfterClauseContext<'input> = BaseParserRuleContext<'input,CallAfterClauseContextExt<'input>>;

#[derive(Clone)]
pub struct CallAfterClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CallAfterClauseContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CallAfterClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_callAfterClause(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_callAfterClause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CallAfterClauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_callAfterClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallAfterClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callAfterClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callAfterClause }
}
antlr4rust::tid!{CallAfterClauseContextExt<'a>}

impl<'input> CallAfterClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CallAfterClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallAfterClauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CallAfterClauseContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CallAfterClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_AFTER
/// Returns `None` if there is no child corresponding to token KEYWORD_AFTER
fn KEYWORD_AFTER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_AFTER, 0)
}
fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CallAfterClauseContextAttrs<'input> for CallAfterClauseContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn callAfterClause(&mut self,)
	-> Result<Rc<CallAfterClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallAfterClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_callAfterClause);
        let mut _localctx: Rc<CallAfterClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1013);
			recog.base.match_token(WdlV1Parser_KEYWORD_AFTER,&mut recog.err_handler)?;

			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1014);
			recog.strictIdentifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- callInputBlock ----------------
pub type CallInputBlockContextAll<'input> = CallInputBlockContext<'input>;


pub type CallInputBlockContext<'input> = BaseParserRuleContext<'input,CallInputBlockContextExt<'input>>;

#[derive(Clone)]
pub struct CallInputBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CallInputBlockContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CallInputBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_callInputBlock(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_callInputBlock(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CallInputBlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_callInputBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallInputBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callInputBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callInputBlock }
}
antlr4rust::tid!{CallInputBlockContextExt<'a>}

impl<'input> CallInputBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CallInputBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallInputBlockContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CallInputBlockContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CallInputBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_INPUT
/// Returns `None` if there is no child corresponding to token KEYWORD_INPUT
fn KEYWORD_INPUT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_INPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn callInputItem_all(&self) ->  Vec<Rc<CallInputItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn callInputItem(&self, i: usize) -> Option<Rc<CallInputItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> CallInputBlockContextAttrs<'input> for CallInputBlockContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn callInputBlock(&mut self,)
	-> Result<Rc<CallInputBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallInputBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_callInputBlock);
        let mut _localctx: Rc<CallInputBlockContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1016);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(1019);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(101,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1017);
					recog.base.match_token(WdlV1Parser_KEYWORD_INPUT,&mut recog.err_handler)?;

					recog.base.set_state(1018);
					recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(1032);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule callInputItem*/
				recog.base.set_state(1021);
				recog.callInputItem()?;

				recog.base.set_state(1026);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(102,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(1022);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule callInputItem*/
						recog.base.set_state(1023);
						recog.callInputItem()?;

						}
						} 
					}
					recog.base.set_state(1028);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(102,&mut recog.base)?;
				}
				recog.base.set_state(1030);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(1029);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1034);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- callInputItem ----------------
pub type CallInputItemContextAll<'input> = CallInputItemContext<'input>;


pub type CallInputItemContext<'input> = BaseParserRuleContext<'input,CallInputItemContextExt<'input>>;

#[derive(Clone)]
pub struct CallInputItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CallInputItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CallInputItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_callInputItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_callInputItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CallInputItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_callInputItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallInputItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callInputItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callInputItem }
}
antlr4rust::tid!{CallInputItemContextExt<'a>}

impl<'input> CallInputItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CallInputItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallInputItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CallInputItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CallInputItemContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
/// Returns `None` if there is no child corresponding to token ASSIGNMENT
fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_ASSIGNMENT, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CallInputItemContextAttrs<'input> for CallInputItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn callInputItem(&mut self,)
	-> Result<Rc<CallInputItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallInputItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_callInputItem);
        let mut _localctx: Rc<CallInputItemContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1036);
			recog.strictIdentifier()?;

			recog.base.set_state(1039);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_ASSIGNMENT {
				{
				recog.base.set_state(1037);
				recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(1038);
				recog.expression()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_expression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_expression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr4rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn logicalOrExpression(&self) -> Option<Rc<LogicalOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule logicalOrExpression*/
			recog.base.set_state(1041);
			recog.logicalOrExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- logicalOrExpression ----------------
#[derive(Debug)]
pub enum LogicalOrExpressionContextAll<'input>{
	LogicalOrExprNoneContext(LogicalOrExprNoneContext<'input>),
	LogicalOrExprOperationContext(LogicalOrExprOperationContext<'input>),
Error(LogicalOrExpressionContext<'input>)
}
antlr4rust::tid!{LogicalOrExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for LogicalOrExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for LogicalOrExpressionContextAll<'input>{}

impl<'input> Deref for LogicalOrExpressionContextAll<'input>{
	type Target = dyn LogicalOrExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LogicalOrExpressionContextAll::*;
		match self{
			LogicalOrExprNoneContext(inner) => inner,
			LogicalOrExprOperationContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalOrExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalOrExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type LogicalOrExpressionContext<'input> = BaseParserRuleContext<'input,LogicalOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for LogicalOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalOrExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalOrExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LogicalOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalOrExpression }
}
antlr4rust::tid!{LogicalOrExpressionContextExt<'a>}

impl<'input> LogicalOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LogicalOrExpressionContextAll<'input>> {
		Rc::new(
		LogicalOrExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalOrExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LogicalOrExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<LogicalOrExpressionContextExt<'input>>{


}

impl<'input> LogicalOrExpressionContextAttrs<'input> for LogicalOrExpressionContext<'input>{}

pub type LogicalOrExprNoneContext<'input> = BaseParserRuleContext<'input,LogicalOrExprNoneContextExt<'input>>;

pub trait LogicalOrExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn logicalAndExpression(&self) -> Option<Rc<LogicalAndExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LogicalOrExprNoneContextAttrs<'input> for LogicalOrExprNoneContext<'input>{}

pub struct LogicalOrExprNoneContextExt<'input>{
	base:LogicalOrExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{LogicalOrExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for LogicalOrExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalOrExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_logicalOrExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_logicalOrExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalOrExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_logicalOrExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalOrExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalOrExpression }
}

impl<'input> Borrow<LogicalOrExpressionContextExt<'input>> for LogicalOrExprNoneContext<'input>{
	fn borrow(&self) -> &LogicalOrExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicalOrExpressionContextExt<'input>> for LogicalOrExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicalOrExpressionContextExt<'input> { &mut self.base }
}

impl<'input> LogicalOrExpressionContextAttrs<'input> for LogicalOrExprNoneContext<'input> {}

impl<'input> LogicalOrExprNoneContextExt<'input>{
	fn new(ctx: &dyn LogicalOrExpressionContextAttrs<'input>) -> Rc<LogicalOrExpressionContextAll<'input>>  {
		Rc::new(
			LogicalOrExpressionContextAll::LogicalOrExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,LogicalOrExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicalOrExprOperationContext<'input> = BaseParserRuleContext<'input,LogicalOrExprOperationContextExt<'input>>;

pub trait LogicalOrExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn logicalAndExpression(&self) -> Option<Rc<LogicalAndExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LOGICAL_OR
	/// Returns `None` if there is no child corresponding to token LOGICAL_OR
	fn LOGICAL_OR(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_LOGICAL_OR, 0)
	}
	fn logicalOrExpression(&self) -> Option<Rc<LogicalOrExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LogicalOrExprOperationContextAttrs<'input> for LogicalOrExprOperationContext<'input>{}

pub struct LogicalOrExprOperationContextExt<'input>{
	base:LogicalOrExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{LogicalOrExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for LogicalOrExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalOrExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_logicalOrExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_logicalOrExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalOrExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_logicalOrExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalOrExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalOrExpression }
}

impl<'input> Borrow<LogicalOrExpressionContextExt<'input>> for LogicalOrExprOperationContext<'input>{
	fn borrow(&self) -> &LogicalOrExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicalOrExpressionContextExt<'input>> for LogicalOrExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicalOrExpressionContextExt<'input> { &mut self.base }
}

impl<'input> LogicalOrExpressionContextAttrs<'input> for LogicalOrExprOperationContext<'input> {}

impl<'input> LogicalOrExprOperationContextExt<'input>{
	fn new(ctx: &dyn LogicalOrExpressionContextAttrs<'input>) -> Rc<LogicalOrExpressionContextAll<'input>>  {
		Rc::new(
			LogicalOrExpressionContextAll::LogicalOrExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,LogicalOrExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn logicalOrExpression(&mut self,)
	-> Result<Rc<LogicalOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_logicalOrExpression);
        let mut _localctx: Rc<LogicalOrExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1048);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(106,&mut recog.base)? {
				1 =>{
					let tmp = LogicalOrExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule logicalAndExpression*/
					recog.base.set_state(1043);
					recog.logicalAndExpression()?;

					recog.base.set_state(1044);
					recog.base.match_token(WdlV1Parser_LOGICAL_OR,&mut recog.err_handler)?;

					/*InvokeRule logicalOrExpression*/
					recog.base.set_state(1045);
					recog.logicalOrExpression()?;

					}
				}
			,
				2 =>{
					let tmp = LogicalOrExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule logicalAndExpression*/
					recog.base.set_state(1047);
					recog.logicalAndExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- logicalAndExpression ----------------
#[derive(Debug)]
pub enum LogicalAndExpressionContextAll<'input>{
	LogicalAndExprOperationContext(LogicalAndExprOperationContext<'input>),
	LogicalAndExprNoneContext(LogicalAndExprNoneContext<'input>),
Error(LogicalAndExpressionContext<'input>)
}
antlr4rust::tid!{LogicalAndExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for LogicalAndExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for LogicalAndExpressionContextAll<'input>{}

impl<'input> Deref for LogicalAndExpressionContextAll<'input>{
	type Target = dyn LogicalAndExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LogicalAndExpressionContextAll::*;
		match self{
			LogicalAndExprOperationContext(inner) => inner,
			LogicalAndExprNoneContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalAndExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalAndExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type LogicalAndExpressionContext<'input> = BaseParserRuleContext<'input,LogicalAndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalAndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for LogicalAndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalAndExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalAndExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LogicalAndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalAndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalAndExpression }
}
antlr4rust::tid!{LogicalAndExpressionContextExt<'a>}

impl<'input> LogicalAndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LogicalAndExpressionContextAll<'input>> {
		Rc::new(
		LogicalAndExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalAndExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LogicalAndExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<LogicalAndExpressionContextExt<'input>>{


}

impl<'input> LogicalAndExpressionContextAttrs<'input> for LogicalAndExpressionContext<'input>{}

pub type LogicalAndExprOperationContext<'input> = BaseParserRuleContext<'input,LogicalAndExprOperationContextExt<'input>>;

pub trait LogicalAndExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn equalityExpression(&self) -> Option<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LOGICAL_AND
	/// Returns `None` if there is no child corresponding to token LOGICAL_AND
	fn LOGICAL_AND(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_LOGICAL_AND, 0)
	}
	fn logicalAndExpression(&self) -> Option<Rc<LogicalAndExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LogicalAndExprOperationContextAttrs<'input> for LogicalAndExprOperationContext<'input>{}

pub struct LogicalAndExprOperationContextExt<'input>{
	base:LogicalAndExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{LogicalAndExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for LogicalAndExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalAndExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_logicalAndExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_logicalAndExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalAndExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_logicalAndExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalAndExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalAndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalAndExpression }
}

impl<'input> Borrow<LogicalAndExpressionContextExt<'input>> for LogicalAndExprOperationContext<'input>{
	fn borrow(&self) -> &LogicalAndExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicalAndExpressionContextExt<'input>> for LogicalAndExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicalAndExpressionContextExt<'input> { &mut self.base }
}

impl<'input> LogicalAndExpressionContextAttrs<'input> for LogicalAndExprOperationContext<'input> {}

impl<'input> LogicalAndExprOperationContextExt<'input>{
	fn new(ctx: &dyn LogicalAndExpressionContextAttrs<'input>) -> Rc<LogicalAndExpressionContextAll<'input>>  {
		Rc::new(
			LogicalAndExpressionContextAll::LogicalAndExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,LogicalAndExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicalAndExprNoneContext<'input> = BaseParserRuleContext<'input,LogicalAndExprNoneContextExt<'input>>;

pub trait LogicalAndExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn equalityExpression(&self) -> Option<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> LogicalAndExprNoneContextAttrs<'input> for LogicalAndExprNoneContext<'input>{}

pub struct LogicalAndExprNoneContextExt<'input>{
	base:LogicalAndExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{LogicalAndExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for LogicalAndExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for LogicalAndExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_logicalAndExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_logicalAndExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for LogicalAndExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_logicalAndExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicalAndExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalAndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalAndExpression }
}

impl<'input> Borrow<LogicalAndExpressionContextExt<'input>> for LogicalAndExprNoneContext<'input>{
	fn borrow(&self) -> &LogicalAndExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicalAndExpressionContextExt<'input>> for LogicalAndExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicalAndExpressionContextExt<'input> { &mut self.base }
}

impl<'input> LogicalAndExpressionContextAttrs<'input> for LogicalAndExprNoneContext<'input> {}

impl<'input> LogicalAndExprNoneContextExt<'input>{
	fn new(ctx: &dyn LogicalAndExpressionContextAttrs<'input>) -> Rc<LogicalAndExpressionContextAll<'input>>  {
		Rc::new(
			LogicalAndExpressionContextAll::LogicalAndExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,LogicalAndExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn logicalAndExpression(&mut self,)
	-> Result<Rc<LogicalAndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalAndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_logicalAndExpression);
        let mut _localctx: Rc<LogicalAndExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1055);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(107,&mut recog.base)? {
				1 =>{
					let tmp = LogicalAndExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule equalityExpression*/
					recog.base.set_state(1050);
					recog.equalityExpression()?;

					recog.base.set_state(1051);
					recog.base.match_token(WdlV1Parser_LOGICAL_AND,&mut recog.err_handler)?;

					/*InvokeRule logicalAndExpression*/
					recog.base.set_state(1052);
					recog.logicalAndExpression()?;

					}
				}
			,
				2 =>{
					let tmp = LogicalAndExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule equalityExpression*/
					recog.base.set_state(1054);
					recog.equalityExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- equalityExpression ----------------
#[derive(Debug)]
pub enum EqualityExpressionContextAll<'input>{
	EqualityExprNoneContext(EqualityExprNoneContext<'input>),
	EqualityExprOperationContext(EqualityExprOperationContext<'input>),
Error(EqualityExpressionContext<'input>)
}
antlr4rust::tid!{EqualityExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for EqualityExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for EqualityExpressionContextAll<'input>{}

impl<'input> Deref for EqualityExpressionContextAll<'input>{
	type Target = dyn EqualityExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use EqualityExpressionContextAll::*;
		match self{
			EqualityExprNoneContext(inner) => inner,
			EqualityExprOperationContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EqualityExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EqualityExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type EqualityExpressionContext<'input> = BaseParserRuleContext<'input,EqualityExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for EqualityExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EqualityExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EqualityExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for EqualityExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equalityExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equalityExpression }
}
antlr4rust::tid!{EqualityExpressionContextExt<'a>}

impl<'input> EqualityExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EqualityExpressionContextAll<'input>> {
		Rc::new(
		EqualityExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait EqualityExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<EqualityExpressionContextExt<'input>>{


}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExpressionContext<'input>{}

pub type EqualityExprNoneContext<'input> = BaseParserRuleContext<'input,EqualityExprNoneContextExt<'input>>;

pub trait EqualityExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn comparisonExpression(&self) -> Option<Rc<ComparisonExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> EqualityExprNoneContextAttrs<'input> for EqualityExprNoneContext<'input>{}

pub struct EqualityExprNoneContextExt<'input>{
	base:EqualityExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{EqualityExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for EqualityExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EqualityExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_equalityExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_equalityExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EqualityExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_equalityExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equalityExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equalityExpression }
}

impl<'input> Borrow<EqualityExpressionContextExt<'input>> for EqualityExprNoneContext<'input>{
	fn borrow(&self) -> &EqualityExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EqualityExpressionContextExt<'input>> for EqualityExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut EqualityExpressionContextExt<'input> { &mut self.base }
}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExprNoneContext<'input> {}

impl<'input> EqualityExprNoneContextExt<'input>{
	fn new(ctx: &dyn EqualityExpressionContextAttrs<'input>) -> Rc<EqualityExpressionContextAll<'input>>  {
		Rc::new(
			EqualityExpressionContextAll::EqualityExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,EqualityExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqualityExprOperationContext<'input> = BaseParserRuleContext<'input,EqualityExprOperationContextExt<'input>>;

pub trait EqualityExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn comparisonExpression(&self) -> Option<Rc<ComparisonExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn equalityExpression(&self) -> Option<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token EQUAL
	/// Returns `None` if there is no child corresponding to token EQUAL
	fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_EQUAL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token NOT_EQUAL
	/// Returns `None` if there is no child corresponding to token NOT_EQUAL
	fn NOT_EQUAL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_NOT_EQUAL, 0)
	}
}

impl<'input> EqualityExprOperationContextAttrs<'input> for EqualityExprOperationContext<'input>{}

pub struct EqualityExprOperationContextExt<'input>{
	base:EqualityExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{EqualityExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for EqualityExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for EqualityExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_equalityExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_equalityExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for EqualityExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_equalityExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equalityExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equalityExpression }
}

impl<'input> Borrow<EqualityExpressionContextExt<'input>> for EqualityExprOperationContext<'input>{
	fn borrow(&self) -> &EqualityExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EqualityExpressionContextExt<'input>> for EqualityExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut EqualityExpressionContextExt<'input> { &mut self.base }
}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExprOperationContext<'input> {}

impl<'input> EqualityExprOperationContextExt<'input>{
	fn new(ctx: &dyn EqualityExpressionContextAttrs<'input>) -> Rc<EqualityExpressionContextAll<'input>>  {
		Rc::new(
			EqualityExpressionContextAll::EqualityExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,EqualityExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn equalityExpression(&mut self,)
	-> Result<Rc<EqualityExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_equalityExpression);
        let mut _localctx: Rc<EqualityExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1062);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(108,&mut recog.base)? {
				1 =>{
					let tmp = EqualityExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule comparisonExpression*/
					recog.base.set_state(1057);
					recog.comparisonExpression()?;

					recog.base.set_state(1058);
					_la = recog.base.input.la(1);
					if { !(_la==WdlV1Parser_EQUAL || _la==WdlV1Parser_NOT_EQUAL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule equalityExpression*/
					recog.base.set_state(1059);
					recog.equalityExpression()?;

					}
				}
			,
				2 =>{
					let tmp = EqualityExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule comparisonExpression*/
					recog.base.set_state(1061);
					recog.comparisonExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- comparisonExpression ----------------
#[derive(Debug)]
pub enum ComparisonExpressionContextAll<'input>{
	ComparisonExprNoneContext(ComparisonExprNoneContext<'input>),
	ComparisonExprOperationContext(ComparisonExprOperationContext<'input>),
Error(ComparisonExpressionContext<'input>)
}
antlr4rust::tid!{ComparisonExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ComparisonExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for ComparisonExpressionContextAll<'input>{}

impl<'input> Deref for ComparisonExpressionContextAll<'input>{
	type Target = dyn ComparisonExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ComparisonExpressionContextAll::*;
		match self{
			ComparisonExprNoneContext(inner) => inner,
			ComparisonExprOperationContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ComparisonExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ComparisonExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ComparisonExpressionContext<'input> = BaseParserRuleContext<'input,ComparisonExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ComparisonExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ComparisonExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ComparisonExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ComparisonExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonExpression }
}
antlr4rust::tid!{ComparisonExpressionContextExt<'a>}

impl<'input> ComparisonExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ComparisonExpressionContextAll<'input>> {
		Rc::new(
		ComparisonExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ComparisonExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ComparisonExpressionContextExt<'input>>{


}

impl<'input> ComparisonExpressionContextAttrs<'input> for ComparisonExpressionContext<'input>{}

pub type ComparisonExprNoneContext<'input> = BaseParserRuleContext<'input,ComparisonExprNoneContextExt<'input>>;

pub trait ComparisonExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn additiveExpression(&self) -> Option<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ComparisonExprNoneContextAttrs<'input> for ComparisonExprNoneContext<'input>{}

pub struct ComparisonExprNoneContextExt<'input>{
	base:ComparisonExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ComparisonExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for ComparisonExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ComparisonExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_comparisonExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_comparisonExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ComparisonExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_comparisonExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonExpression }
}

impl<'input> Borrow<ComparisonExpressionContextExt<'input>> for ComparisonExprNoneContext<'input>{
	fn borrow(&self) -> &ComparisonExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ComparisonExpressionContextExt<'input>> for ComparisonExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut ComparisonExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ComparisonExpressionContextAttrs<'input> for ComparisonExprNoneContext<'input> {}

impl<'input> ComparisonExprNoneContextExt<'input>{
	fn new(ctx: &dyn ComparisonExpressionContextAttrs<'input>) -> Rc<ComparisonExpressionContextAll<'input>>  {
		Rc::new(
			ComparisonExpressionContextAll::ComparisonExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,ComparisonExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ComparisonExprOperationContext<'input> = BaseParserRuleContext<'input,ComparisonExprOperationContextExt<'input>>;

pub trait ComparisonExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn additiveExpression(&self) -> Option<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn comparisonExpression(&self) -> Option<Rc<ComparisonExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LESS
	/// Returns `None` if there is no child corresponding to token LESS
	fn LESS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_LESS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LESS_EQUAL
	/// Returns `None` if there is no child corresponding to token LESS_EQUAL
	fn LESS_EQUAL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_LESS_EQUAL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token GREATER
	/// Returns `None` if there is no child corresponding to token GREATER
	fn GREATER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_GREATER, 0)
	}
	/// Retrieves first TerminalNode corresponding to token GREATER_EQUAL
	/// Returns `None` if there is no child corresponding to token GREATER_EQUAL
	fn GREATER_EQUAL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_GREATER_EQUAL, 0)
	}
}

impl<'input> ComparisonExprOperationContextAttrs<'input> for ComparisonExprOperationContext<'input>{}

pub struct ComparisonExprOperationContextExt<'input>{
	base:ComparisonExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ComparisonExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for ComparisonExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ComparisonExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_comparisonExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_comparisonExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ComparisonExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_comparisonExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparisonExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparisonExpression }
}

impl<'input> Borrow<ComparisonExpressionContextExt<'input>> for ComparisonExprOperationContext<'input>{
	fn borrow(&self) -> &ComparisonExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ComparisonExpressionContextExt<'input>> for ComparisonExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut ComparisonExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ComparisonExpressionContextAttrs<'input> for ComparisonExprOperationContext<'input> {}

impl<'input> ComparisonExprOperationContextExt<'input>{
	fn new(ctx: &dyn ComparisonExpressionContextAttrs<'input>) -> Rc<ComparisonExpressionContextAll<'input>>  {
		Rc::new(
			ComparisonExpressionContextAll::ComparisonExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,ComparisonExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn comparisonExpression(&mut self,)
	-> Result<Rc<ComparisonExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_comparisonExpression);
        let mut _localctx: Rc<ComparisonExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1069);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(109,&mut recog.base)? {
				1 =>{
					let tmp = ComparisonExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule additiveExpression*/
					recog.base.set_state(1064);
					recog.additiveExpression()?;

					recog.base.set_state(1065);
					_la = recog.base.input.la(1);
					if { !(((((_la - 58)) & !0x3f) == 0 && ((1usize << (_la - 58)) & 786435) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule comparisonExpression*/
					recog.base.set_state(1066);
					recog.comparisonExpression()?;

					}
				}
			,
				2 =>{
					let tmp = ComparisonExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule additiveExpression*/
					recog.base.set_state(1068);
					recog.additiveExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- additiveExpression ----------------
#[derive(Debug)]
pub enum AdditiveExpressionContextAll<'input>{
	AdditiveExprOperationContext(AdditiveExprOperationContext<'input>),
	AdditiveExprNoneContext(AdditiveExprNoneContext<'input>),
Error(AdditiveExpressionContext<'input>)
}
antlr4rust::tid!{AdditiveExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for AdditiveExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for AdditiveExpressionContextAll<'input>{}

impl<'input> Deref for AdditiveExpressionContextAll<'input>{
	type Target = dyn AdditiveExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AdditiveExpressionContextAll::*;
		match self{
			AdditiveExprOperationContext(inner) => inner,
			AdditiveExprNoneContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for AdditiveExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for AdditiveExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type AdditiveExpressionContext<'input> = BaseParserRuleContext<'input,AdditiveExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AdditiveExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for AdditiveExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for AdditiveExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for AdditiveExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AdditiveExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additiveExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additiveExpression }
}
antlr4rust::tid!{AdditiveExpressionContextExt<'a>}

impl<'input> AdditiveExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AdditiveExpressionContextAll<'input>> {
		Rc::new(
		AdditiveExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditiveExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AdditiveExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<AdditiveExpressionContextExt<'input>>{


}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExpressionContext<'input>{}

pub type AdditiveExprOperationContext<'input> = BaseParserRuleContext<'input,AdditiveExprOperationContextExt<'input>>;

pub trait AdditiveExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn multiplicativeExpression(&self) -> Option<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn additiveExpression(&self) -> Option<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token PLUS
	/// Returns `None` if there is no child corresponding to token PLUS
	fn PLUS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_PLUS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MINUS, 0)
	}
}

impl<'input> AdditiveExprOperationContextAttrs<'input> for AdditiveExprOperationContext<'input>{}

pub struct AdditiveExprOperationContextExt<'input>{
	base:AdditiveExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AdditiveExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for AdditiveExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for AdditiveExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_additiveExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_additiveExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for AdditiveExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_additiveExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditiveExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additiveExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additiveExpression }
}

impl<'input> Borrow<AdditiveExpressionContextExt<'input>> for AdditiveExprOperationContext<'input>{
	fn borrow(&self) -> &AdditiveExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AdditiveExpressionContextExt<'input>> for AdditiveExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut AdditiveExpressionContextExt<'input> { &mut self.base }
}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExprOperationContext<'input> {}

impl<'input> AdditiveExprOperationContextExt<'input>{
	fn new(ctx: &dyn AdditiveExpressionContextAttrs<'input>) -> Rc<AdditiveExpressionContextAll<'input>>  {
		Rc::new(
			AdditiveExpressionContextAll::AdditiveExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,AdditiveExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AdditiveExprNoneContext<'input> = BaseParserRuleContext<'input,AdditiveExprNoneContextExt<'input>>;

pub trait AdditiveExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn multiplicativeExpression(&self) -> Option<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AdditiveExprNoneContextAttrs<'input> for AdditiveExprNoneContext<'input>{}

pub struct AdditiveExprNoneContextExt<'input>{
	base:AdditiveExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AdditiveExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for AdditiveExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for AdditiveExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_additiveExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_additiveExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for AdditiveExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_additiveExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditiveExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additiveExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additiveExpression }
}

impl<'input> Borrow<AdditiveExpressionContextExt<'input>> for AdditiveExprNoneContext<'input>{
	fn borrow(&self) -> &AdditiveExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AdditiveExpressionContextExt<'input>> for AdditiveExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut AdditiveExpressionContextExt<'input> { &mut self.base }
}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExprNoneContext<'input> {}

impl<'input> AdditiveExprNoneContextExt<'input>{
	fn new(ctx: &dyn AdditiveExpressionContextAttrs<'input>) -> Rc<AdditiveExpressionContextAll<'input>>  {
		Rc::new(
			AdditiveExpressionContextAll::AdditiveExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,AdditiveExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn additiveExpression(&mut self,)
	-> Result<Rc<AdditiveExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AdditiveExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_additiveExpression);
        let mut _localctx: Rc<AdditiveExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1076);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(110,&mut recog.base)? {
				1 =>{
					let tmp = AdditiveExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule multiplicativeExpression*/
					recog.base.set_state(1071);
					recog.multiplicativeExpression()?;

					recog.base.set_state(1072);
					_la = recog.base.input.la(1);
					if { !(_la==WdlV1Parser_PLUS || _la==WdlV1Parser_MINUS) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule additiveExpression*/
					recog.base.set_state(1073);
					recog.additiveExpression()?;

					}
				}
			,
				2 =>{
					let tmp = AdditiveExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule multiplicativeExpression*/
					recog.base.set_state(1075);
					recog.multiplicativeExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- multiplicativeExpression ----------------
#[derive(Debug)]
pub enum MultiplicativeExpressionContextAll<'input>{
	MultiplicativeExprOperationContext(MultiplicativeExprOperationContext<'input>),
	MultiplicativeExprNoneContext(MultiplicativeExprNoneContext<'input>),
Error(MultiplicativeExpressionContext<'input>)
}
antlr4rust::tid!{MultiplicativeExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for MultiplicativeExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for MultiplicativeExpressionContextAll<'input>{}

impl<'input> Deref for MultiplicativeExpressionContextAll<'input>{
	type Target = dyn MultiplicativeExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use MultiplicativeExpressionContextAll::*;
		match self{
			MultiplicativeExprOperationContext(inner) => inner,
			MultiplicativeExprNoneContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultiplicativeExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultiplicativeExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type MultiplicativeExpressionContext<'input> = BaseParserRuleContext<'input,MultiplicativeExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplicativeExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultiplicativeExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultiplicativeExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicativeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicativeExpression }
}
antlr4rust::tid!{MultiplicativeExpressionContextExt<'a>}

impl<'input> MultiplicativeExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MultiplicativeExpressionContextAll<'input>> {
		Rc::new(
		MultiplicativeExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplicativeExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait MultiplicativeExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MultiplicativeExpressionContextExt<'input>>{


}

impl<'input> MultiplicativeExpressionContextAttrs<'input> for MultiplicativeExpressionContext<'input>{}

pub type MultiplicativeExprOperationContext<'input> = BaseParserRuleContext<'input,MultiplicativeExprOperationContextExt<'input>>;

pub trait MultiplicativeExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn powerExpression(&self) -> Option<Rc<PowerExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn multiplicativeExpression(&self) -> Option<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ASTERISK
	/// Returns `None` if there is no child corresponding to token ASTERISK
	fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_ASTERISK, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SLASH
	/// Returns `None` if there is no child corresponding to token SLASH
	fn SLASH(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_SLASH, 0)
	}
	/// Retrieves first TerminalNode corresponding to token PERCENT
	/// Returns `None` if there is no child corresponding to token PERCENT
	fn PERCENT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_PERCENT, 0)
	}
}

impl<'input> MultiplicativeExprOperationContextAttrs<'input> for MultiplicativeExprOperationContext<'input>{}

pub struct MultiplicativeExprOperationContextExt<'input>{
	base:MultiplicativeExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultiplicativeExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultiplicativeExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultiplicativeExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multiplicativeExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multiplicativeExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultiplicativeExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multiplicativeExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicativeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicativeExpression }
}

impl<'input> Borrow<MultiplicativeExpressionContextExt<'input>> for MultiplicativeExprOperationContext<'input>{
	fn borrow(&self) -> &MultiplicativeExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultiplicativeExpressionContextExt<'input>> for MultiplicativeExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultiplicativeExpressionContextExt<'input> { &mut self.base }
}

impl<'input> MultiplicativeExpressionContextAttrs<'input> for MultiplicativeExprOperationContext<'input> {}

impl<'input> MultiplicativeExprOperationContextExt<'input>{
	fn new(ctx: &dyn MultiplicativeExpressionContextAttrs<'input>) -> Rc<MultiplicativeExpressionContextAll<'input>>  {
		Rc::new(
			MultiplicativeExpressionContextAll::MultiplicativeExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,MultiplicativeExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultiplicativeExprNoneContext<'input> = BaseParserRuleContext<'input,MultiplicativeExprNoneContextExt<'input>>;

pub trait MultiplicativeExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn powerExpression(&self) -> Option<Rc<PowerExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MultiplicativeExprNoneContextAttrs<'input> for MultiplicativeExprNoneContext<'input>{}

pub struct MultiplicativeExprNoneContextExt<'input>{
	base:MultiplicativeExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultiplicativeExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultiplicativeExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultiplicativeExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multiplicativeExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multiplicativeExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultiplicativeExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multiplicativeExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicativeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicativeExpression }
}

impl<'input> Borrow<MultiplicativeExpressionContextExt<'input>> for MultiplicativeExprNoneContext<'input>{
	fn borrow(&self) -> &MultiplicativeExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultiplicativeExpressionContextExt<'input>> for MultiplicativeExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultiplicativeExpressionContextExt<'input> { &mut self.base }
}

impl<'input> MultiplicativeExpressionContextAttrs<'input> for MultiplicativeExprNoneContext<'input> {}

impl<'input> MultiplicativeExprNoneContextExt<'input>{
	fn new(ctx: &dyn MultiplicativeExpressionContextAttrs<'input>) -> Rc<MultiplicativeExpressionContextAll<'input>>  {
		Rc::new(
			MultiplicativeExpressionContextAll::MultiplicativeExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,MultiplicativeExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn multiplicativeExpression(&mut self,)
	-> Result<Rc<MultiplicativeExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplicativeExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_multiplicativeExpression);
        let mut _localctx: Rc<MultiplicativeExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1083);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(111,&mut recog.base)? {
				1 =>{
					let tmp = MultiplicativeExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule powerExpression*/
					recog.base.set_state(1078);
					recog.powerExpression()?;

					recog.base.set_state(1079);
					_la = recog.base.input.la(1);
					if { !(((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & 7) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule multiplicativeExpression*/
					recog.base.set_state(1080);
					recog.multiplicativeExpression()?;

					}
				}
			,
				2 =>{
					let tmp = MultiplicativeExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule powerExpression*/
					recog.base.set_state(1082);
					recog.powerExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- powerExpression ----------------
#[derive(Debug)]
pub enum PowerExpressionContextAll<'input>{
	PowerExprNoneContext(PowerExprNoneContext<'input>),
	PowerExprOperationContext(PowerExprOperationContext<'input>),
Error(PowerExpressionContext<'input>)
}
antlr4rust::tid!{PowerExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for PowerExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for PowerExpressionContextAll<'input>{}

impl<'input> Deref for PowerExpressionContextAll<'input>{
	type Target = dyn PowerExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use PowerExpressionContextAll::*;
		match self{
			PowerExprNoneContext(inner) => inner,
			PowerExprOperationContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PowerExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PowerExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type PowerExpressionContext<'input> = BaseParserRuleContext<'input,PowerExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PowerExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for PowerExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PowerExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PowerExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for PowerExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_powerExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_powerExpression }
}
antlr4rust::tid!{PowerExpressionContextExt<'a>}

impl<'input> PowerExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PowerExpressionContextAll<'input>> {
		Rc::new(
		PowerExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PowerExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait PowerExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<PowerExpressionContextExt<'input>>{


}

impl<'input> PowerExpressionContextAttrs<'input> for PowerExpressionContext<'input>{}

pub type PowerExprNoneContext<'input> = BaseParserRuleContext<'input,PowerExprNoneContextExt<'input>>;

pub trait PowerExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PowerExprNoneContextAttrs<'input> for PowerExprNoneContext<'input>{}

pub struct PowerExprNoneContextExt<'input>{
	base:PowerExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PowerExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for PowerExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PowerExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_powerExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_powerExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PowerExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_powerExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowerExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_powerExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_powerExpression }
}

impl<'input> Borrow<PowerExpressionContextExt<'input>> for PowerExprNoneContext<'input>{
	fn borrow(&self) -> &PowerExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PowerExpressionContextExt<'input>> for PowerExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut PowerExpressionContextExt<'input> { &mut self.base }
}

impl<'input> PowerExpressionContextAttrs<'input> for PowerExprNoneContext<'input> {}

impl<'input> PowerExprNoneContextExt<'input>{
	fn new(ctx: &dyn PowerExpressionContextAttrs<'input>) -> Rc<PowerExpressionContextAll<'input>>  {
		Rc::new(
			PowerExpressionContextAll::PowerExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,PowerExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PowerExprOperationContext<'input> = BaseParserRuleContext<'input,PowerExprOperationContextExt<'input>>;

pub trait PowerExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token EXPONENTIATION
	/// Returns `None` if there is no child corresponding to token EXPONENTIATION
	fn EXPONENTIATION(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_EXPONENTIATION, 0)
	}
	fn powerExpression(&self) -> Option<Rc<PowerExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PowerExprOperationContextAttrs<'input> for PowerExprOperationContext<'input>{}

pub struct PowerExprOperationContextExt<'input>{
	base:PowerExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PowerExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for PowerExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PowerExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_powerExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_powerExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PowerExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_powerExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowerExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_powerExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_powerExpression }
}

impl<'input> Borrow<PowerExpressionContextExt<'input>> for PowerExprOperationContext<'input>{
	fn borrow(&self) -> &PowerExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PowerExpressionContextExt<'input>> for PowerExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut PowerExpressionContextExt<'input> { &mut self.base }
}

impl<'input> PowerExpressionContextAttrs<'input> for PowerExprOperationContext<'input> {}

impl<'input> PowerExprOperationContextExt<'input>{
	fn new(ctx: &dyn PowerExpressionContextAttrs<'input>) -> Rc<PowerExpressionContextAll<'input>>  {
		Rc::new(
			PowerExpressionContextAll::PowerExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,PowerExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn powerExpression(&mut self,)
	-> Result<Rc<PowerExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PowerExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_powerExpression);
        let mut _localctx: Rc<PowerExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1090);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(112,&mut recog.base)? {
				1 =>{
					let tmp = PowerExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule unaryExpression*/
					recog.base.set_state(1085);
					recog.unaryExpression()?;

					recog.base.set_state(1086);
					recog.base.match_token(WdlV1Parser_EXPONENTIATION,&mut recog.err_handler)?;

					/*InvokeRule powerExpression*/
					recog.base.set_state(1087);
					recog.powerExpression()?;

					}
				}
			,
				2 =>{
					let tmp = PowerExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule unaryExpression*/
					recog.base.set_state(1089);
					recog.unaryExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- unaryExpression ----------------
#[derive(Debug)]
pub enum UnaryExpressionContextAll<'input>{
	UnaryExprOperationContext(UnaryExprOperationContext<'input>),
	UnaryExprNoneContext(UnaryExprNoneContext<'input>),
Error(UnaryExpressionContext<'input>)
}
antlr4rust::tid!{UnaryExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for UnaryExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for UnaryExpressionContextAll<'input>{}

impl<'input> Deref for UnaryExpressionContextAll<'input>{
	type Target = dyn UnaryExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use UnaryExpressionContextAll::*;
		match self{
			UnaryExprOperationContext(inner) => inner,
			UnaryExprNoneContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for UnaryExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for UnaryExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type UnaryExpressionContext<'input> = BaseParserRuleContext<'input,UnaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for UnaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for UnaryExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for UnaryExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpression }
}
antlr4rust::tid!{UnaryExpressionContextExt<'a>}

impl<'input> UnaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<UnaryExpressionContextAll<'input>> {
		Rc::new(
		UnaryExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait UnaryExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<UnaryExpressionContextExt<'input>>{


}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input>{}

pub type UnaryExprOperationContext<'input> = BaseParserRuleContext<'input,UnaryExprOperationContextExt<'input>>;

pub trait UnaryExprOperationContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token EXCLAMATION
	/// Returns `None` if there is no child corresponding to token EXCLAMATION
	fn EXCLAMATION(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_EXCLAMATION, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MINUS, 0)
	}
}

impl<'input> UnaryExprOperationContextAttrs<'input> for UnaryExprOperationContext<'input>{}

pub struct UnaryExprOperationContextExt<'input>{
	base:UnaryExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{UnaryExprOperationContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for UnaryExprOperationContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for UnaryExprOperationContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_unaryExprOperation(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_unaryExprOperation(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for UnaryExprOperationContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_unaryExprOperation(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExprOperationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpression }
}

impl<'input> Borrow<UnaryExpressionContextExt<'input>> for UnaryExprOperationContext<'input>{
	fn borrow(&self) -> &UnaryExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<UnaryExpressionContextExt<'input>> for UnaryExprOperationContext<'input>{
	fn borrow_mut(&mut self) -> &mut UnaryExpressionContextExt<'input> { &mut self.base }
}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExprOperationContext<'input> {}

impl<'input> UnaryExprOperationContextExt<'input>{
	fn new(ctx: &dyn UnaryExpressionContextAttrs<'input>) -> Rc<UnaryExpressionContextAll<'input>>  {
		Rc::new(
			UnaryExpressionContextAll::UnaryExprOperationContext(
				BaseParserRuleContext::copy_from(ctx,UnaryExprOperationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryExprNoneContext<'input> = BaseParserRuleContext<'input,UnaryExprNoneContextExt<'input>>;

pub trait UnaryExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn postfixExpression(&self) -> Option<Rc<PostfixExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> UnaryExprNoneContextAttrs<'input> for UnaryExprNoneContext<'input>{}

pub struct UnaryExprNoneContextExt<'input>{
	base:UnaryExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{UnaryExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for UnaryExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for UnaryExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_unaryExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_unaryExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for UnaryExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_unaryExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpression }
}

impl<'input> Borrow<UnaryExpressionContextExt<'input>> for UnaryExprNoneContext<'input>{
	fn borrow(&self) -> &UnaryExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<UnaryExpressionContextExt<'input>> for UnaryExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut UnaryExpressionContextExt<'input> { &mut self.base }
}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExprNoneContext<'input> {}

impl<'input> UnaryExprNoneContextExt<'input>{
	fn new(ctx: &dyn UnaryExpressionContextAttrs<'input>) -> Rc<UnaryExpressionContextAll<'input>>  {
		Rc::new(
			UnaryExpressionContextAll::UnaryExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,UnaryExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn unaryExpression(&mut self,)
	-> Result<Rc<UnaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_unaryExpression);
        let mut _localctx: Rc<UnaryExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1095);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_EXCLAMATION |WdlV1Parser_MINUS 
				=> {
					let tmp = UnaryExprOperationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(1092);
					_la = recog.base.input.la(1);
					if { !(_la==WdlV1Parser_EXCLAMATION || _la==WdlV1Parser_MINUS) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule unaryExpression*/
					recog.base.set_state(1093);
					recog.unaryExpression()?;

					}
				}

			WdlV1Parser_FLOAT |WdlV1Parser_INTEGER |WdlV1Parser_OPEN_MULTILINE_STRING |
			WdlV1Parser_SINGLE_QUOTE |WdlV1Parser_DOUBLE_QUOTE |WdlV1Parser_KEYWORD_ARRAY_TYPE |
			WdlV1Parser_KEYWORD_BOOLEAN_TYPE |WdlV1Parser_KEYWORD_DIRECTORY_TYPE |
			WdlV1Parser_KEYWORD_FILE_TYPE |WdlV1Parser_KEYWORD_FLOAT_TYPE |WdlV1Parser_KEYWORD_INT_TYPE |
			WdlV1Parser_KEYWORD_MAP_TYPE |WdlV1Parser_KEYWORD_OBJECT_TYPE |WdlV1Parser_KEYWORD_PAIR_TYPE |
			WdlV1Parser_KEYWORD_STRING_TYPE |WdlV1Parser_KEYWORD_AFTER |WdlV1Parser_KEYWORD_ALIAS |
			WdlV1Parser_KEYWORD_AS |WdlV1Parser_KEYWORD_CALL |WdlV1Parser_KEYWORD_COMMAND |
			WdlV1Parser_KEYWORD_ELSE |WdlV1Parser_KEYWORD_ENV |WdlV1Parser_KEYWORD_FALSE |
			WdlV1Parser_KEYWORD_FROM |WdlV1Parser_KEYWORD_HINTS |WdlV1Parser_KEYWORD_IF |
			WdlV1Parser_KEYWORD_IN |WdlV1Parser_KEYWORD_IMPORT |WdlV1Parser_KEYWORD_INPUT |
			WdlV1Parser_KEYWORD_META |WdlV1Parser_KEYWORD_NONE |WdlV1Parser_KEYWORD_NULL |
			WdlV1Parser_KEYWORD_OBJECT |WdlV1Parser_KEYWORD_OUTPUT |WdlV1Parser_KEYWORD_PARAMETER_META |
			WdlV1Parser_KEYWORD_REQUIREMENTS |WdlV1Parser_KEYWORD_RUNTIME |WdlV1Parser_KEYWORD_SCATTER |
			WdlV1Parser_KEYWORD_STRUCT |WdlV1Parser_KEYWORD_ENUM |WdlV1Parser_KEYWORD_TASK |
			WdlV1Parser_KEYWORD_THEN |WdlV1Parser_KEYWORD_TRUE |WdlV1Parser_KEYWORD_VERSION |
			WdlV1Parser_KEYWORD_WORKFLOW |WdlV1Parser_IDENTIFIER |WdlV1Parser_OPEN_BRACE |
			WdlV1Parser_OPEN_BRACKET |WdlV1Parser_OPEN_PAREN 
				=> {
					let tmp = UnaryExprNoneContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule postfixExpression*/
					recog.base.set_state(1094);
					recog.postfixExpression_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- postfixExpression ----------------
#[derive(Debug)]
pub enum PostfixExpressionContextAll<'input>{
	PostfixExprFieldContext(PostfixExprFieldContext<'input>),
	PostfixExprArrayIndexContext(PostfixExprArrayIndexContext<'input>),
	PostfixExprNoneContext(PostfixExprNoneContext<'input>),
Error(PostfixExpressionContext<'input>)
}
antlr4rust::tid!{PostfixExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for PostfixExpressionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for PostfixExpressionContextAll<'input>{}

impl<'input> Deref for PostfixExpressionContextAll<'input>{
	type Target = dyn PostfixExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use PostfixExpressionContextAll::*;
		match self{
			PostfixExprFieldContext(inner) => inner,
			PostfixExprArrayIndexContext(inner) => inner,
			PostfixExprNoneContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PostfixExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PostfixExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type PostfixExpressionContext<'input> = BaseParserRuleContext<'input,PostfixExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PostfixExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for PostfixExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PostfixExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PostfixExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for PostfixExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfixExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfixExpression }
}
antlr4rust::tid!{PostfixExpressionContextExt<'a>}

impl<'input> PostfixExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PostfixExpressionContextAll<'input>> {
		Rc::new(
		PostfixExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostfixExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait PostfixExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<PostfixExpressionContextExt<'input>>{


}

impl<'input> PostfixExpressionContextAttrs<'input> for PostfixExpressionContext<'input>{}

pub type PostfixExprFieldContext<'input> = BaseParserRuleContext<'input,PostfixExprFieldContextExt<'input>>;

pub trait PostfixExprFieldContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn postfixExpression(&self) -> Option<Rc<PostfixExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token DOT
	/// Returns `None` if there is no child corresponding to token DOT
	fn DOT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_DOT, 0)
	}
	fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PostfixExprFieldContextAttrs<'input> for PostfixExprFieldContext<'input>{}

pub struct PostfixExprFieldContextExt<'input>{
	base:PostfixExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PostfixExprFieldContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for PostfixExprFieldContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PostfixExprFieldContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_postfixExprField(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_postfixExprField(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PostfixExprFieldContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_postfixExprField(self);
	}
}

impl<'input> CustomRuleContext<'input> for PostfixExprFieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfixExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfixExpression }
}

impl<'input> Borrow<PostfixExpressionContextExt<'input>> for PostfixExprFieldContext<'input>{
	fn borrow(&self) -> &PostfixExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PostfixExpressionContextExt<'input>> for PostfixExprFieldContext<'input>{
	fn borrow_mut(&mut self) -> &mut PostfixExpressionContextExt<'input> { &mut self.base }
}

impl<'input> PostfixExpressionContextAttrs<'input> for PostfixExprFieldContext<'input> {}

impl<'input> PostfixExprFieldContextExt<'input>{
	fn new(ctx: &dyn PostfixExpressionContextAttrs<'input>) -> Rc<PostfixExpressionContextAll<'input>>  {
		Rc::new(
			PostfixExpressionContextAll::PostfixExprFieldContext(
				BaseParserRuleContext::copy_from(ctx,PostfixExprFieldContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PostfixExprArrayIndexContext<'input> = BaseParserRuleContext<'input,PostfixExprArrayIndexContextExt<'input>>;

pub trait PostfixExprArrayIndexContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn postfixExpression(&self) -> Option<Rc<PostfixExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
	/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
	fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
	/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
	fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
	}
}

impl<'input> PostfixExprArrayIndexContextAttrs<'input> for PostfixExprArrayIndexContext<'input>{}

pub struct PostfixExprArrayIndexContextExt<'input>{
	base:PostfixExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PostfixExprArrayIndexContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for PostfixExprArrayIndexContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PostfixExprArrayIndexContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_postfixExprArrayIndex(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_postfixExprArrayIndex(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PostfixExprArrayIndexContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_postfixExprArrayIndex(self);
	}
}

impl<'input> CustomRuleContext<'input> for PostfixExprArrayIndexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfixExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfixExpression }
}

impl<'input> Borrow<PostfixExpressionContextExt<'input>> for PostfixExprArrayIndexContext<'input>{
	fn borrow(&self) -> &PostfixExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PostfixExpressionContextExt<'input>> for PostfixExprArrayIndexContext<'input>{
	fn borrow_mut(&mut self) -> &mut PostfixExpressionContextExt<'input> { &mut self.base }
}

impl<'input> PostfixExpressionContextAttrs<'input> for PostfixExprArrayIndexContext<'input> {}

impl<'input> PostfixExprArrayIndexContextExt<'input>{
	fn new(ctx: &dyn PostfixExpressionContextAttrs<'input>) -> Rc<PostfixExpressionContextAll<'input>>  {
		Rc::new(
			PostfixExpressionContextAll::PostfixExprArrayIndexContext(
				BaseParserRuleContext::copy_from(ctx,PostfixExprArrayIndexContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PostfixExprNoneContext<'input> = BaseParserRuleContext<'input,PostfixExprNoneContextExt<'input>>;

pub trait PostfixExprNoneContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn primaryExpression(&self) -> Option<Rc<PrimaryExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PostfixExprNoneContextAttrs<'input> for PostfixExprNoneContext<'input>{}

pub struct PostfixExprNoneContextExt<'input>{
	base:PostfixExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PostfixExprNoneContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for PostfixExprNoneContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PostfixExprNoneContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_postfixExprNone(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_postfixExprNone(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PostfixExprNoneContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_postfixExprNone(self);
	}
}

impl<'input> CustomRuleContext<'input> for PostfixExprNoneContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfixExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfixExpression }
}

impl<'input> Borrow<PostfixExpressionContextExt<'input>> for PostfixExprNoneContext<'input>{
	fn borrow(&self) -> &PostfixExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PostfixExpressionContextExt<'input>> for PostfixExprNoneContext<'input>{
	fn borrow_mut(&mut self) -> &mut PostfixExpressionContextExt<'input> { &mut self.base }
}

impl<'input> PostfixExpressionContextAttrs<'input> for PostfixExprNoneContext<'input> {}

impl<'input> PostfixExprNoneContextExt<'input>{
	fn new(ctx: &dyn PostfixExpressionContextAttrs<'input>) -> Rc<PostfixExpressionContextAll<'input>>  {
		Rc::new(
			PostfixExpressionContextAll::PostfixExprNoneContext(
				BaseParserRuleContext::copy_from(ctx,PostfixExprNoneContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  postfixExpression(&mut self,)
	-> Result<Rc<PostfixExpressionContextAll<'input>>,ANTLRError> {
		self.postfixExpression_rec(0)
	}

	fn postfixExpression_rec(&mut self, _p: i32)
	-> Result<Rc<PostfixExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = PostfixExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 190, RULE_postfixExpression, _p);
	    let mut _localctx: Rc<PostfixExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 190;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			let mut tmp = PostfixExprNoneContextExt::new(&**_localctx);
			recog.ctx = Some(tmp.clone());
			_localctx = tmp;
			_prevctx = _localctx.clone();

			/*InvokeRule primaryExpression*/
			recog.base.set_state(1098);
			recog.primaryExpression()?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1110);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(115,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(1108);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(114,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = PostfixExprArrayIndexContextExt::new(&**PostfixExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_postfixExpression)?;
							_localctx = tmp;
							recog.base.set_state(1100);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(1101);
							recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1102);
							recog.expression()?;

							recog.base.set_state(1103);
							recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = PostfixExprFieldContextExt::new(&**PostfixExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_postfixExpression)?;
							_localctx = tmp;
							recog.base.set_state(1105);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(1106);
							recog.base.match_token(WdlV1Parser_DOT,&mut recog.err_handler)?;

							/*InvokeRule strictIdentifier*/
							recog.base.set_state(1107);
							recog.strictIdentifier()?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(1112);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(115,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- primaryExpression ----------------
pub type PrimaryExpressionContextAll<'input> = PrimaryExpressionContext<'input>;


pub type PrimaryExpressionContext<'input> = BaseParserRuleContext<'input,PrimaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for PrimaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PrimaryExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_primaryExpression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_primaryExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PrimaryExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_primaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primaryExpression }
}
antlr4rust::tid!{PrimaryExpressionContextExt<'a>}

impl<'input> PrimaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrimaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<PrimaryExpressionContextExt<'input>>{

fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn noneLiteral(&self) -> Option<Rc<NoneLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn booleanLiteral(&self) -> Option<Rc<BooleanLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arrayLiteral(&self) -> Option<Rc<ArrayLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mapLiteral(&self) -> Option<Rc<MapLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn objectLiteral(&self) -> Option<Rc<ObjectLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn structLiteral(&self) -> Option<Rc<StructLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pairLiteral(&self) -> Option<Rc<PairLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn groupedExpression(&self) -> Option<Rc<GroupedExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ifExpression(&self) -> Option<Rc<IfExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn callExpression(&self) -> Option<Rc<CallExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryExpressionContextAttrs<'input> for PrimaryExpressionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn primaryExpression(&mut self,)
	-> Result<Rc<PrimaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_primaryExpression);
        let mut _localctx: Rc<PrimaryExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1126);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(116,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule variable*/
					recog.base.set_state(1113);
					recog.variable()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule noneLiteral*/
					recog.base.set_state(1114);
					recog.noneLiteral()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule booleanLiteral*/
					recog.base.set_state(1115);
					recog.booleanLiteral()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(1116);
					recog.numberLiteral()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(1117);
					recog.stringLiteral()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule arrayLiteral*/
					recog.base.set_state(1118);
					recog.arrayLiteral()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					/*InvokeRule mapLiteral*/
					recog.base.set_state(1119);
					recog.mapLiteral()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					/*InvokeRule objectLiteral*/
					recog.base.set_state(1120);
					recog.objectLiteral()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					/*InvokeRule structLiteral*/
					recog.base.set_state(1121);
					recog.structLiteral()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					/*InvokeRule pairLiteral*/
					recog.base.set_state(1122);
					recog.pairLiteral()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					/*InvokeRule groupedExpression*/
					recog.base.set_state(1123);
					recog.groupedExpression()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12)?;
					recog.base.enter_outer_alt(None, 12)?;
					{
					/*InvokeRule ifExpression*/
					recog.base.set_state(1124);
					recog.ifExpression()?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13)?;
					recog.base.enter_outer_alt(None, 13)?;
					{
					/*InvokeRule callExpression*/
					recog.base.set_state(1125);
					recog.callExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_variable(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_variable(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr4rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1128);
			recog.strictIdentifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nullLiteral ----------------
pub type NullLiteralContextAll<'input> = NullLiteralContext<'input>;


pub type NullLiteralContext<'input> = BaseParserRuleContext<'input,NullLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NullLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for NullLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for NullLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nullLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nullLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for NullLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_nullLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NullLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullLiteral }
}
antlr4rust::tid!{NullLiteralContextExt<'a>}

impl<'input> NullLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NullLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NullLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NullLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<NullLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_NULL
/// Returns `None` if there is no child corresponding to token KEYWORD_NULL
fn KEYWORD_NULL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_NULL, 0)
}

}

impl<'input> NullLiteralContextAttrs<'input> for NullLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nullLiteral(&mut self,)
	-> Result<Rc<NullLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NullLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_nullLiteral);
        let mut _localctx: Rc<NullLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1130);
			recog.base.match_token(WdlV1Parser_KEYWORD_NULL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- noneLiteral ----------------
pub type NoneLiteralContextAll<'input> = NoneLiteralContext<'input>;


pub type NoneLiteralContext<'input> = BaseParserRuleContext<'input,NoneLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NoneLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for NoneLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for NoneLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_noneLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_noneLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for NoneLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_noneLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NoneLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_noneLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_noneLiteral }
}
antlr4rust::tid!{NoneLiteralContextExt<'a>}

impl<'input> NoneLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NoneLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NoneLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NoneLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<NoneLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_NONE
/// Returns `None` if there is no child corresponding to token KEYWORD_NONE
fn KEYWORD_NONE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_NONE, 0)
}

}

impl<'input> NoneLiteralContextAttrs<'input> for NoneLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn noneLiteral(&mut self,)
	-> Result<Rc<NoneLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NoneLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_noneLiteral);
        let mut _localctx: Rc<NoneLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1132);
			recog.base.match_token(WdlV1Parser_KEYWORD_NONE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- booleanLiteral ----------------
pub type BooleanLiteralContextAll<'input> = BooleanLiteralContext<'input>;


pub type BooleanLiteralContext<'input> = BaseParserRuleContext<'input,BooleanLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for BooleanLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for BooleanLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_booleanLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_booleanLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for BooleanLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_booleanLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanLiteral }
}
antlr4rust::tid!{BooleanLiteralContextExt<'a>}

impl<'input> BooleanLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BooleanLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<BooleanLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_TRUE
/// Returns `None` if there is no child corresponding to token KEYWORD_TRUE
fn KEYWORD_TRUE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_FALSE
/// Returns `None` if there is no child corresponding to token KEYWORD_FALSE
fn KEYWORD_FALSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_FALSE, 0)
}

}

impl<'input> BooleanLiteralContextAttrs<'input> for BooleanLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn booleanLiteral(&mut self,)
	-> Result<Rc<BooleanLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_booleanLiteral);
        let mut _localctx: Rc<BooleanLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1134);
			_la = recog.base.input.la(1);
			if { !(_la==WdlV1Parser_KEYWORD_FALSE || _la==WdlV1Parser_KEYWORD_TRUE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- numberLiteral ----------------
#[derive(Debug)]
pub enum NumberLiteralContextAll<'input>{
	NumberLiteralIntContext(NumberLiteralIntContext<'input>),
	NumberLiteralFloatContext(NumberLiteralFloatContext<'input>),
Error(NumberLiteralContext<'input>)
}
antlr4rust::tid!{NumberLiteralContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for NumberLiteralContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for NumberLiteralContextAll<'input>{}

impl<'input> Deref for NumberLiteralContextAll<'input>{
	type Target = dyn NumberLiteralContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use NumberLiteralContextAll::*;
		match self{
			NumberLiteralIntContext(inner) => inner,
			NumberLiteralFloatContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for NumberLiteralContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for NumberLiteralContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type NumberLiteralContext<'input> = BaseParserRuleContext<'input,NumberLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NumberLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for NumberLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for NumberLiteralContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for NumberLiteralContext<'input>{
}

impl<'input> CustomRuleContext<'input> for NumberLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}
antlr4rust::tid!{NumberLiteralContextExt<'a>}

impl<'input> NumberLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NumberLiteralContextAll<'input>> {
		Rc::new(
		NumberLiteralContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberLiteralContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait NumberLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<NumberLiteralContextExt<'input>>{


}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralContext<'input>{}

pub type NumberLiteralIntContext<'input> = BaseParserRuleContext<'input,NumberLiteralIntContextExt<'input>>;

pub trait NumberLiteralIntContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INTEGER
	/// Returns `None` if there is no child corresponding to token INTEGER
	fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_INTEGER, 0)
	}
}

impl<'input> NumberLiteralIntContextAttrs<'input> for NumberLiteralIntContext<'input>{}

pub struct NumberLiteralIntContextExt<'input>{
	base:NumberLiteralContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NumberLiteralIntContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for NumberLiteralIntContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for NumberLiteralIntContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_numberLiteralInt(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_numberLiteralInt(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for NumberLiteralIntContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_numberLiteralInt(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralIntContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}

impl<'input> Borrow<NumberLiteralContextExt<'input>> for NumberLiteralIntContext<'input>{
	fn borrow(&self) -> &NumberLiteralContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NumberLiteralContextExt<'input>> for NumberLiteralIntContext<'input>{
	fn borrow_mut(&mut self) -> &mut NumberLiteralContextExt<'input> { &mut self.base }
}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralIntContext<'input> {}

impl<'input> NumberLiteralIntContextExt<'input>{
	fn new(ctx: &dyn NumberLiteralContextAttrs<'input>) -> Rc<NumberLiteralContextAll<'input>>  {
		Rc::new(
			NumberLiteralContextAll::NumberLiteralIntContext(
				BaseParserRuleContext::copy_from(ctx,NumberLiteralIntContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NumberLiteralFloatContext<'input> = BaseParserRuleContext<'input,NumberLiteralFloatContextExt<'input>>;

pub trait NumberLiteralFloatContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FLOAT
	/// Returns `None` if there is no child corresponding to token FLOAT
	fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_FLOAT, 0)
	}
}

impl<'input> NumberLiteralFloatContextAttrs<'input> for NumberLiteralFloatContext<'input>{}

pub struct NumberLiteralFloatContextExt<'input>{
	base:NumberLiteralContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NumberLiteralFloatContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for NumberLiteralFloatContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for NumberLiteralFloatContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_numberLiteralFloat(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_numberLiteralFloat(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for NumberLiteralFloatContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_numberLiteralFloat(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralFloatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}

impl<'input> Borrow<NumberLiteralContextExt<'input>> for NumberLiteralFloatContext<'input>{
	fn borrow(&self) -> &NumberLiteralContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NumberLiteralContextExt<'input>> for NumberLiteralFloatContext<'input>{
	fn borrow_mut(&mut self) -> &mut NumberLiteralContextExt<'input> { &mut self.base }
}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralFloatContext<'input> {}

impl<'input> NumberLiteralFloatContextExt<'input>{
	fn new(ctx: &dyn NumberLiteralContextAttrs<'input>) -> Rc<NumberLiteralContextAll<'input>>  {
		Rc::new(
			NumberLiteralContextAll::NumberLiteralFloatContext(
				BaseParserRuleContext::copy_from(ctx,NumberLiteralFloatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn numberLiteral(&mut self,)
	-> Result<Rc<NumberLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_numberLiteral);
        let mut _localctx: Rc<NumberLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1138);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_INTEGER 
				=> {
					let tmp = NumberLiteralIntContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(1136);
					recog.base.match_token(WdlV1Parser_INTEGER,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_FLOAT 
				=> {
					let tmp = NumberLiteralFloatContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(1137);
					recog.base.match_token(WdlV1Parser_FLOAT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- numberLiteralSigned ----------------
pub type NumberLiteralSignedContextAll<'input> = NumberLiteralSignedContext<'input>;


pub type NumberLiteralSignedContext<'input> = BaseParserRuleContext<'input,NumberLiteralSignedContextExt<'input>>;

#[derive(Clone)]
pub struct NumberLiteralSignedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for NumberLiteralSignedContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for NumberLiteralSignedContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_numberLiteralSigned(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_numberLiteralSigned(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for NumberLiteralSignedContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_numberLiteralSigned(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralSignedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteralSigned }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteralSigned }
}
antlr4rust::tid!{NumberLiteralSignedContextExt<'a>}

impl<'input> NumberLiteralSignedContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NumberLiteralSignedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberLiteralSignedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NumberLiteralSignedContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<NumberLiteralSignedContextExt<'input>>{

fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MINUS, 0)
}

}

impl<'input> NumberLiteralSignedContextAttrs<'input> for NumberLiteralSignedContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn numberLiteralSigned(&mut self,)
	-> Result<Rc<NumberLiteralSignedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberLiteralSignedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_numberLiteralSigned);
        let mut _localctx: Rc<NumberLiteralSignedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1141);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WdlV1Parser_MINUS {
				{
				recog.base.set_state(1140);
				recog.base.match_token(WdlV1Parser_MINUS,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule numberLiteral*/
			recog.base.set_state(1143);
			recog.numberLiteral()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- arrayLiteral ----------------
pub type ArrayLiteralContextAll<'input> = ArrayLiteralContext<'input>;


pub type ArrayLiteralContext<'input> = BaseParserRuleContext<'input,ArrayLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ArrayLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ArrayLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_arrayLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_arrayLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ArrayLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_arrayLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayLiteral }
}
antlr4rust::tid!{ArrayLiteralContextExt<'a>}

impl<'input> ArrayLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ArrayLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ArrayLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACKET
/// Returns `None` if there is no child corresponding to token OPEN_BRACKET
fn OPEN_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSE_BRACKET
fn CLOSE_BRACKET(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACKET, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> ArrayLiteralContextAttrs<'input> for ArrayLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn arrayLiteral(&mut self,)
	-> Result<Rc<ArrayLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 206, RULE_arrayLiteral);
        let mut _localctx: Rc<ArrayLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1145);
			recog.base.match_token(WdlV1Parser_OPEN_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(1157);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294966720) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 1344274431) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & 41) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(1146);
				recog.expression()?;

				recog.base.set_state(1151);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(119,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(1147);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(1148);
						recog.expression()?;

						}
						} 
					}
					recog.base.set_state(1153);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(119,&mut recog.base)?;
				}
				recog.base.set_state(1155);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(1154);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1159);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- mapLiteral ----------------
pub type MapLiteralContextAll<'input> = MapLiteralContext<'input>;


pub type MapLiteralContext<'input> = BaseParserRuleContext<'input,MapLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct MapLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MapLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MapLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mapLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mapLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MapLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_mapLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for MapLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapLiteral }
}
antlr4rust::tid!{MapLiteralContextExt<'a>}

impl<'input> MapLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MapLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MapLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MapLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn mapLiteralItem_all(&self) ->  Vec<Rc<MapLiteralItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mapLiteralItem(&self, i: usize) -> Option<Rc<MapLiteralItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> MapLiteralContextAttrs<'input> for MapLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn mapLiteral(&mut self,)
	-> Result<Rc<MapLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 208, RULE_mapLiteral);
        let mut _localctx: Rc<MapLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1161);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(1173);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294966720) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 1344274431) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & 41) != 0) {
				{
				/*InvokeRule mapLiteralItem*/
				recog.base.set_state(1162);
				recog.mapLiteralItem()?;

				recog.base.set_state(1167);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(122,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(1163);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule mapLiteralItem*/
						recog.base.set_state(1164);
						recog.mapLiteralItem()?;

						}
						} 
					}
					recog.base.set_state(1169);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(122,&mut recog.base)?;
				}
				recog.base.set_state(1171);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(1170);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1175);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- mapLiteralItem ----------------
pub type MapLiteralItemContextAll<'input> = MapLiteralItemContext<'input>;


pub type MapLiteralItemContext<'input> = BaseParserRuleContext<'input,MapLiteralItemContextExt<'input>>;

#[derive(Clone)]
pub struct MapLiteralItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MapLiteralItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MapLiteralItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mapLiteralItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mapLiteralItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MapLiteralItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_mapLiteralItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for MapLiteralItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapLiteralItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapLiteralItem }
}
antlr4rust::tid!{MapLiteralItemContextExt<'a>}

impl<'input> MapLiteralItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MapLiteralItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapLiteralItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MapLiteralItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MapLiteralItemContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}

}

impl<'input> MapLiteralItemContextAttrs<'input> for MapLiteralItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn mapLiteralItem(&mut self,)
	-> Result<Rc<MapLiteralItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapLiteralItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 210, RULE_mapLiteralItem);
        let mut _localctx: Rc<MapLiteralItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule expression*/
			recog.base.set_state(1177);
			recog.expression()?;

			recog.base.set_state(1178);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1179);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- objectLiteral ----------------
pub type ObjectLiteralContextAll<'input> = ObjectLiteralContext<'input>;


pub type ObjectLiteralContext<'input> = BaseParserRuleContext<'input,ObjectLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ObjectLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ObjectLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_objectLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_objectLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ObjectLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_objectLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectLiteral }
}
antlr4rust::tid!{ObjectLiteralContextExt<'a>}

impl<'input> ObjectLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ObjectLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ObjectLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_OBJECT
/// Returns `None` if there is no child corresponding to token KEYWORD_OBJECT
fn KEYWORD_OBJECT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OBJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn objectLiteralItem_all(&self) ->  Vec<Rc<ObjectLiteralItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn objectLiteralItem(&self, i: usize) -> Option<Rc<ObjectLiteralItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> ObjectLiteralContextAttrs<'input> for ObjectLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn objectLiteral(&mut self,)
	-> Result<Rc<ObjectLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 212, RULE_objectLiteral);
        let mut _localctx: Rc<ObjectLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1181);
			recog.base.match_token(WdlV1Parser_KEYWORD_OBJECT,&mut recog.err_handler)?;

			recog.base.set_state(1182);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(1194);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule objectLiteralItem*/
				recog.base.set_state(1183);
				recog.objectLiteralItem()?;

				recog.base.set_state(1188);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(125,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(1184);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule objectLiteralItem*/
						recog.base.set_state(1185);
						recog.objectLiteralItem()?;

						}
						} 
					}
					recog.base.set_state(1190);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(125,&mut recog.base)?;
				}
				recog.base.set_state(1192);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(1191);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1196);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- objectLiteralItem ----------------
pub type ObjectLiteralItemContextAll<'input> = ObjectLiteralItemContext<'input>;


pub type ObjectLiteralItemContext<'input> = BaseParserRuleContext<'input,ObjectLiteralItemContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectLiteralItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for ObjectLiteralItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for ObjectLiteralItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_objectLiteralItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_objectLiteralItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for ObjectLiteralItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_objectLiteralItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectLiteralItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_objectLiteralItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_objectLiteralItem }
}
antlr4rust::tid!{ObjectLiteralItemContextExt<'a>}

impl<'input> ObjectLiteralItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ObjectLiteralItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectLiteralItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectLiteralItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<ObjectLiteralItemContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ObjectLiteralItemContextAttrs<'input> for ObjectLiteralItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn objectLiteralItem(&mut self,)
	-> Result<Rc<ObjectLiteralItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectLiteralItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 214, RULE_objectLiteralItem);
        let mut _localctx: Rc<ObjectLiteralItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1198);
			recog.strictIdentifier()?;

			recog.base.set_state(1199);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1200);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structLiteral ----------------
pub type StructLiteralContextAll<'input> = StructLiteralContext<'input>;


pub type StructLiteralContext<'input> = BaseParserRuleContext<'input,StructLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct StructLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StructLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_structLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structLiteral }
}
antlr4rust::tid!{StructLiteralContextExt<'a>}

impl<'input> StructLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StructLiteralContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_BRACE
/// Returns `None` if there is no child corresponding to token OPEN_BRACE
fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
fn structLiteralItem_all(&self) ->  Vec<Rc<StructLiteralItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn structLiteralItem(&self, i: usize) -> Option<Rc<StructLiteralItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> StructLiteralContextAttrs<'input> for StructLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structLiteral(&mut self,)
	-> Result<Rc<StructLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 216, RULE_structLiteral);
        let mut _localctx: Rc<StructLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1202);
			recog.strictIdentifier()?;

			recog.base.set_state(1203);
			recog.base.match_token(WdlV1Parser_OPEN_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(1215);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0) {
				{
				/*InvokeRule structLiteralItem*/
				recog.base.set_state(1204);
				recog.structLiteralItem()?;

				recog.base.set_state(1209);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(128,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(1205);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule structLiteralItem*/
						recog.base.set_state(1206);
						recog.structLiteralItem()?;

						}
						} 
					}
					recog.base.set_state(1211);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(128,&mut recog.base)?;
				}
				recog.base.set_state(1213);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(1212);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1217);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structLiteralItem ----------------
pub type StructLiteralItemContextAll<'input> = StructLiteralItemContext<'input>;


pub type StructLiteralItemContext<'input> = BaseParserRuleContext<'input,StructLiteralItemContextExt<'input>>;

#[derive(Clone)]
pub struct StructLiteralItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StructLiteralItemContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StructLiteralItemContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structLiteralItem(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structLiteralItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StructLiteralItemContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_structLiteralItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructLiteralItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structLiteralItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structLiteralItem }
}
antlr4rust::tid!{StructLiteralItemContextExt<'a>}

impl<'input> StructLiteralItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructLiteralItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructLiteralItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructLiteralItemContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StructLiteralItemContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StructLiteralItemContextAttrs<'input> for StructLiteralItemContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structLiteralItem(&mut self,)
	-> Result<Rc<StructLiteralItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructLiteralItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 218, RULE_structLiteralItem);
        let mut _localctx: Rc<StructLiteralItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1219);
			recog.strictIdentifier()?;

			recog.base.set_state(1220);
			recog.base.match_token(WdlV1Parser_COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1221);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- pairLiteral ----------------
pub type PairLiteralContextAll<'input> = PairLiteralContext<'input>;


pub type PairLiteralContext<'input> = BaseParserRuleContext<'input,PairLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct PairLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for PairLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for PairLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_pairLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_pairLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for PairLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_pairLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for PairLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pairLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pairLiteral }
}
antlr4rust::tid!{PairLiteralContextExt<'a>}

impl<'input> PairLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PairLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PairLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PairLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<PairLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_PAREN
/// Returns `None` if there is no child corresponding to token OPEN_PAREN
fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_PAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
/// Returns `None` if there is no child corresponding to token CLOSE_PAREN
fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_PAREN, 0)
}

}

impl<'input> PairLiteralContextAttrs<'input> for PairLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn pairLiteral(&mut self,)
	-> Result<Rc<PairLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PairLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 220, RULE_pairLiteral);
        let mut _localctx: Rc<PairLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1223);
			recog.base.match_token(WdlV1Parser_OPEN_PAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1224);
			recog.expression()?;

			recog.base.set_state(1225);
			recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1226);
			recog.expression()?;

			recog.base.set_state(1227);
			recog.base.match_token(WdlV1Parser_CLOSE_PAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- groupedExpression ----------------
pub type GroupedExpressionContextAll<'input> = GroupedExpressionContext<'input>;


pub type GroupedExpressionContext<'input> = BaseParserRuleContext<'input,GroupedExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct GroupedExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for GroupedExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for GroupedExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_groupedExpression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_groupedExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for GroupedExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_groupedExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for GroupedExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_groupedExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_groupedExpression }
}
antlr4rust::tid!{GroupedExpressionContextExt<'a>}

impl<'input> GroupedExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GroupedExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GroupedExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GroupedExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<GroupedExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_PAREN
/// Returns `None` if there is no child corresponding to token OPEN_PAREN
fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_PAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
/// Returns `None` if there is no child corresponding to token CLOSE_PAREN
fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_PAREN, 0)
}

}

impl<'input> GroupedExpressionContextAttrs<'input> for GroupedExpressionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn groupedExpression(&mut self,)
	-> Result<Rc<GroupedExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GroupedExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 222, RULE_groupedExpression);
        let mut _localctx: Rc<GroupedExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1229);
			recog.base.match_token(WdlV1Parser_OPEN_PAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1230);
			recog.expression()?;

			recog.base.set_state(1231);
			recog.base.match_token(WdlV1Parser_CLOSE_PAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- ifExpression ----------------
pub type IfExpressionContextAll<'input> = IfExpressionContext<'input>;


pub type IfExpressionContext<'input> = BaseParserRuleContext<'input,IfExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct IfExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for IfExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for IfExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_ifExpression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_ifExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for IfExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_ifExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ifExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ifExpression }
}
antlr4rust::tid!{IfExpressionContextExt<'a>}

impl<'input> IfExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IfExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IfExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IfExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<IfExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KEYWORD_IF
/// Returns `None` if there is no child corresponding to token KEYWORD_IF
fn KEYWORD_IF(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_IF, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_THEN
/// Returns `None` if there is no child corresponding to token KEYWORD_THEN
fn KEYWORD_THEN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_THEN, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ELSE
/// Returns `None` if there is no child corresponding to token KEYWORD_ELSE
fn KEYWORD_ELSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ELSE, 0)
}

}

impl<'input> IfExpressionContextAttrs<'input> for IfExpressionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn ifExpression(&mut self,)
	-> Result<Rc<IfExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IfExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 224, RULE_ifExpression);
        let mut _localctx: Rc<IfExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1233);
			recog.base.match_token(WdlV1Parser_KEYWORD_IF,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1234);
			recog.expression()?;

			recog.base.set_state(1235);
			recog.base.match_token(WdlV1Parser_KEYWORD_THEN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1236);
			recog.expression()?;

			recog.base.set_state(1237);
			recog.base.match_token(WdlV1Parser_KEYWORD_ELSE,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1238);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- callExpression ----------------
pub type CallExpressionContextAll<'input> = CallExpressionContext<'input>;


pub type CallExpressionContext<'input> = BaseParserRuleContext<'input,CallExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct CallExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for CallExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for CallExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_callExpression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_callExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for CallExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_callExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_callExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_callExpression }
}
antlr4rust::tid!{CallExpressionContextExt<'a>}

impl<'input> CallExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CallExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CallExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CallExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<CallExpressionContextExt<'input>>{

fn strictIdentifier(&self) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN_PAREN
/// Returns `None` if there is no child corresponding to token OPEN_PAREN
fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
/// Returns `None` if there is no child corresponding to token CLOSE_PAREN
fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_PAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_COMMA, i)
}

}

impl<'input> CallExpressionContextAttrs<'input> for CallExpressionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn callExpression(&mut self,)
	-> Result<Rc<CallExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CallExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 226, RULE_callExpression);
        let mut _localctx: Rc<CallExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1240);
			recog.strictIdentifier()?;

			recog.base.set_state(1241);
			recog.base.match_token(WdlV1Parser_OPEN_PAREN,&mut recog.err_handler)?;

			recog.base.set_state(1253);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294966720) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 1344274431) != 0) || ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & 41) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(1242);
				recog.expression()?;

				recog.base.set_state(1247);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(131,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(1243);
						recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(1244);
						recog.expression()?;

						}
						} 
					}
					recog.base.set_state(1249);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(131,&mut recog.base)?;
				}
				recog.base.set_state(1251);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==WdlV1Parser_COMMA {
					{
					recog.base.set_state(1250);
					recog.base.match_token(WdlV1Parser_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(1255);
			recog.base.match_token(WdlV1Parser_CLOSE_PAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- stringLiteral ----------------
pub type StringLiteralContextAll<'input> = StringLiteralContext<'input>;


pub type StringLiteralContext<'input> = BaseParserRuleContext<'input,StringLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct StringLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StringLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_stringLiteral(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_stringLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringLiteral }
}
antlr4rust::tid!{StringLiteralContextExt<'a>}

impl<'input> StringLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StringLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StringLiteralContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StringLiteralContextExt<'input>>{

fn quotedString(&self) -> Option<Rc<QuotedStringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn multilineString(&self) -> Option<Rc<MultilineStringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StringLiteralContextAttrs<'input> for StringLiteralContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stringLiteral(&mut self,)
	-> Result<Rc<StringLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 228, RULE_stringLiteral);
        let mut _localctx: Rc<StringLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1259);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_SINGLE_QUOTE |WdlV1Parser_DOUBLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule quotedString*/
					recog.base.set_state(1257);
					recog.quotedString()?;

					}
				}

			WdlV1Parser_OPEN_MULTILINE_STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule multilineString*/
					recog.base.set_state(1258);
					recog.multilineString()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- quotedString ----------------
pub type QuotedStringContextAll<'input> = QuotedStringContext<'input>;


pub type QuotedStringContext<'input> = BaseParserRuleContext<'input,QuotedStringContextExt<'input>>;

#[derive(Clone)]
pub struct QuotedStringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for QuotedStringContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for QuotedStringContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_quotedString(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_quotedString(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for QuotedStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_quotedString(self);
	}
}

impl<'input> CustomRuleContext<'input> for QuotedStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_quotedString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_quotedString }
}
antlr4rust::tid!{QuotedStringContextExt<'a>}

impl<'input> QuotedStringContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<QuotedStringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QuotedStringContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait QuotedStringContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<QuotedStringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTE
/// Returns `None` if there is no child corresponding to token SINGLE_QUOTE
fn SINGLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_SINGLE_QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTE_END
/// Returns `None` if there is no child corresponding to token SINGLE_QUOTE_END
fn SINGLE_QUOTE_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_SINGLE_QUOTE_END, 0)
}
fn stringElement_all(&self) ->  Vec<Rc<StringElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stringElement(&self, i: usize) -> Option<Rc<StringElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_QUOTE
/// Returns `None` if there is no child corresponding to token DOUBLE_QUOTE
fn DOUBLE_QUOTE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOUBLE_QUOTE, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_QUOTE_END
/// Returns `None` if there is no child corresponding to token DOUBLE_QUOTE_END
fn DOUBLE_QUOTE_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOUBLE_QUOTE_END, 0)
}

}

impl<'input> QuotedStringContextAttrs<'input> for QuotedStringContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn quotedString(&mut self,)
	-> Result<Rc<QuotedStringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QuotedStringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 230, RULE_quotedString);
        let mut _localctx: Rc<QuotedStringContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1277);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_SINGLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1261);
					recog.base.match_token(WdlV1Parser_SINGLE_QUOTE,&mut recog.err_handler)?;

					recog.base.set_state(1265);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & 62) != 0) {
						{
						{
						/*InvokeRule stringElement*/
						recog.base.set_state(1262);
						recog.stringElement()?;

						}
						}
						recog.base.set_state(1267);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1268);
					recog.base.match_token(WdlV1Parser_SINGLE_QUOTE_END,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_DOUBLE_QUOTE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1269);
					recog.base.match_token(WdlV1Parser_DOUBLE_QUOTE,&mut recog.err_handler)?;

					recog.base.set_state(1273);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & 62) != 0) {
						{
						{
						/*InvokeRule stringElement*/
						recog.base.set_state(1270);
						recog.stringElement()?;

						}
						}
						recog.base.set_state(1275);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1276);
					recog.base.match_token(WdlV1Parser_DOUBLE_QUOTE_END,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- stringElement ----------------
#[derive(Debug)]
pub enum StringElementContextAll<'input>{
	StringElementEscapeContext(StringElementEscapeContext<'input>),
	StringElementDollarSignContext(StringElementDollarSignContext<'input>),
	StringElementTextContext(StringElementTextContext<'input>),
	StringElementTildeContext(StringElementTildeContext<'input>),
	StringElementPlaceholderContext(StringElementPlaceholderContext<'input>),
Error(StringElementContext<'input>)
}
antlr4rust::tid!{StringElementContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for StringElementContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for StringElementContextAll<'input>{}

impl<'input> Deref for StringElementContextAll<'input>{
	type Target = dyn StringElementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StringElementContextAll::*;
		match self{
			StringElementEscapeContext(inner) => inner,
			StringElementDollarSignContext(inner) => inner,
			StringElementTextContext(inner) => inner,
			StringElementTildeContext(inner) => inner,
			StringElementPlaceholderContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringElementContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringElementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type StringElementContext<'input> = BaseParserRuleContext<'input,StringElementContextExt<'input>>;

#[derive(Clone)]
pub struct StringElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StringElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringElementContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringElementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StringElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringElement }
}
antlr4rust::tid!{StringElementContextExt<'a>}

impl<'input> StringElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StringElementContextAll<'input>> {
		Rc::new(
		StringElementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringElementContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StringElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StringElementContextExt<'input>>{


}

impl<'input> StringElementContextAttrs<'input> for StringElementContext<'input>{}

pub type StringElementEscapeContext<'input> = BaseParserRuleContext<'input,StringElementEscapeContextExt<'input>>;

pub trait StringElementEscapeContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING_ESCAPE
	/// Returns `None` if there is no child corresponding to token STRING_ESCAPE
	fn STRING_ESCAPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_STRING_ESCAPE, 0)
	}
}

impl<'input> StringElementEscapeContextAttrs<'input> for StringElementEscapeContext<'input>{}

pub struct StringElementEscapeContextExt<'input>{
	base:StringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringElementEscapeContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringElementEscapeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringElementEscapeContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringElementEscape(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringElementEscape(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringElementEscapeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringElementEscape(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringElementEscapeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringElement }
}

impl<'input> Borrow<StringElementContextExt<'input>> for StringElementEscapeContext<'input>{
	fn borrow(&self) -> &StringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringElementContextExt<'input>> for StringElementEscapeContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringElementContextExt<'input> { &mut self.base }
}

impl<'input> StringElementContextAttrs<'input> for StringElementEscapeContext<'input> {}

impl<'input> StringElementEscapeContextExt<'input>{
	fn new(ctx: &dyn StringElementContextAttrs<'input>) -> Rc<StringElementContextAll<'input>>  {
		Rc::new(
			StringElementContextAll::StringElementEscapeContext(
				BaseParserRuleContext::copy_from(ctx,StringElementEscapeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringElementDollarSignContext<'input> = BaseParserRuleContext<'input,StringElementDollarSignContextExt<'input>>;

pub trait StringElementDollarSignContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING_DOLLAR_SIGN
	/// Returns `None` if there is no child corresponding to token STRING_DOLLAR_SIGN
	fn STRING_DOLLAR_SIGN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_STRING_DOLLAR_SIGN, 0)
	}
}

impl<'input> StringElementDollarSignContextAttrs<'input> for StringElementDollarSignContext<'input>{}

pub struct StringElementDollarSignContextExt<'input>{
	base:StringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringElementDollarSignContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringElementDollarSignContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringElementDollarSignContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringElementDollarSign(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringElementDollarSign(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringElementDollarSignContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringElementDollarSign(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringElementDollarSignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringElement }
}

impl<'input> Borrow<StringElementContextExt<'input>> for StringElementDollarSignContext<'input>{
	fn borrow(&self) -> &StringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringElementContextExt<'input>> for StringElementDollarSignContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringElementContextExt<'input> { &mut self.base }
}

impl<'input> StringElementContextAttrs<'input> for StringElementDollarSignContext<'input> {}

impl<'input> StringElementDollarSignContextExt<'input>{
	fn new(ctx: &dyn StringElementContextAttrs<'input>) -> Rc<StringElementContextAll<'input>>  {
		Rc::new(
			StringElementContextAll::StringElementDollarSignContext(
				BaseParserRuleContext::copy_from(ctx,StringElementDollarSignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringElementTextContext<'input> = BaseParserRuleContext<'input,StringElementTextContextExt<'input>>;

pub trait StringElementTextContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING_TEXT
	/// Returns `None` if there is no child corresponding to token STRING_TEXT
	fn STRING_TEXT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_STRING_TEXT, 0)
	}
}

impl<'input> StringElementTextContextAttrs<'input> for StringElementTextContext<'input>{}

pub struct StringElementTextContextExt<'input>{
	base:StringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringElementTextContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringElementTextContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringElementTextContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringElementText(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringElementText(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringElementTextContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringElementText(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringElementTextContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringElement }
}

impl<'input> Borrow<StringElementContextExt<'input>> for StringElementTextContext<'input>{
	fn borrow(&self) -> &StringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringElementContextExt<'input>> for StringElementTextContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringElementContextExt<'input> { &mut self.base }
}

impl<'input> StringElementContextAttrs<'input> for StringElementTextContext<'input> {}

impl<'input> StringElementTextContextExt<'input>{
	fn new(ctx: &dyn StringElementContextAttrs<'input>) -> Rc<StringElementContextAll<'input>>  {
		Rc::new(
			StringElementContextAll::StringElementTextContext(
				BaseParserRuleContext::copy_from(ctx,StringElementTextContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringElementTildeContext<'input> = BaseParserRuleContext<'input,StringElementTildeContextExt<'input>>;

pub trait StringElementTildeContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING_TILDE
	/// Returns `None` if there is no child corresponding to token STRING_TILDE
	fn STRING_TILDE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_STRING_TILDE, 0)
	}
}

impl<'input> StringElementTildeContextAttrs<'input> for StringElementTildeContext<'input>{}

pub struct StringElementTildeContextExt<'input>{
	base:StringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringElementTildeContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringElementTildeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringElementTildeContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringElementTilde(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringElementTilde(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringElementTildeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringElementTilde(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringElementTildeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringElement }
}

impl<'input> Borrow<StringElementContextExt<'input>> for StringElementTildeContext<'input>{
	fn borrow(&self) -> &StringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringElementContextExt<'input>> for StringElementTildeContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringElementContextExt<'input> { &mut self.base }
}

impl<'input> StringElementContextAttrs<'input> for StringElementTildeContext<'input> {}

impl<'input> StringElementTildeContextExt<'input>{
	fn new(ctx: &dyn StringElementContextAttrs<'input>) -> Rc<StringElementContextAll<'input>>  {
		Rc::new(
			StringElementContextAll::StringElementTildeContext(
				BaseParserRuleContext::copy_from(ctx,StringElementTildeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringElementPlaceholderContext<'input> = BaseParserRuleContext<'input,StringElementPlaceholderContextExt<'input>>;

pub trait StringElementPlaceholderContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn stringPlaceholder(&self) -> Option<Rc<StringPlaceholderContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StringElementPlaceholderContextAttrs<'input> for StringElementPlaceholderContext<'input>{}

pub struct StringElementPlaceholderContextExt<'input>{
	base:StringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringElementPlaceholderContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringElementPlaceholderContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringElementPlaceholderContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringElementPlaceholder(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringElementPlaceholder(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringElementPlaceholderContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringElementPlaceholder(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringElementPlaceholderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringElement }
}

impl<'input> Borrow<StringElementContextExt<'input>> for StringElementPlaceholderContext<'input>{
	fn borrow(&self) -> &StringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringElementContextExt<'input>> for StringElementPlaceholderContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringElementContextExt<'input> { &mut self.base }
}

impl<'input> StringElementContextAttrs<'input> for StringElementPlaceholderContext<'input> {}

impl<'input> StringElementPlaceholderContextExt<'input>{
	fn new(ctx: &dyn StringElementContextAttrs<'input>) -> Rc<StringElementContextAll<'input>>  {
		Rc::new(
			StringElementContextAll::StringElementPlaceholderContext(
				BaseParserRuleContext::copy_from(ctx,StringElementPlaceholderContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stringElement(&mut self,)
	-> Result<Rc<StringElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 232, RULE_stringElement);
        let mut _localctx: Rc<StringElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1284);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_STRING_TEXT 
				=> {
					let tmp = StringElementTextContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(1279);
					recog.base.match_token(WdlV1Parser_STRING_TEXT,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_STRING_ESCAPE 
				=> {
					let tmp = StringElementEscapeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(1280);
					recog.base.match_token(WdlV1Parser_STRING_ESCAPE,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_STRING_DOLLAR_SIGN 
				=> {
					let tmp = StringElementDollarSignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(1281);
					recog.base.match_token(WdlV1Parser_STRING_DOLLAR_SIGN,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_STRING_TILDE 
				=> {
					let tmp = StringElementTildeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					recog.base.set_state(1282);
					recog.base.match_token(WdlV1Parser_STRING_TILDE,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_STRING_PLACEHOLDER_START 
				=> {
					let tmp = StringElementPlaceholderContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					/*InvokeRule stringPlaceholder*/
					recog.base.set_state(1283);
					recog.stringPlaceholder()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- stringPlaceholder ----------------
pub type StringPlaceholderContextAll<'input> = StringPlaceholderContext<'input>;


pub type StringPlaceholderContext<'input> = BaseParserRuleContext<'input,StringPlaceholderContextExt<'input>>;

#[derive(Clone)]
pub struct StringPlaceholderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StringPlaceholderContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringPlaceholderContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_stringPlaceholder(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_stringPlaceholder(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringPlaceholderContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringPlaceholder(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringPlaceholderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringPlaceholder }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringPlaceholder }
}
antlr4rust::tid!{StringPlaceholderContextExt<'a>}

impl<'input> StringPlaceholderContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StringPlaceholderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringPlaceholderContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StringPlaceholderContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StringPlaceholderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING_PLACEHOLDER_START
/// Returns `None` if there is no child corresponding to token STRING_PLACEHOLDER_START
fn STRING_PLACEHOLDER_START(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_STRING_PLACEHOLDER_START, 0)
}
fn stringPlaceholderExpression(&self) -> Option<Rc<StringPlaceholderExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}

}

impl<'input> StringPlaceholderContextAttrs<'input> for StringPlaceholderContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stringPlaceholder(&mut self,)
	-> Result<Rc<StringPlaceholderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringPlaceholderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 234, RULE_stringPlaceholder);
        let mut _localctx: Rc<StringPlaceholderContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1286);
			recog.base.match_token(WdlV1Parser_STRING_PLACEHOLDER_START,&mut recog.err_handler)?;

			/*InvokeRule stringPlaceholderExpression*/
			recog.base.set_state(1287);
			recog.stringPlaceholderExpression()?;

			recog.base.set_state(1288);
			recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- multilineString ----------------
pub type MultilineStringContextAll<'input> = MultilineStringContext<'input>;


pub type MultilineStringContext<'input> = BaseParserRuleContext<'input,MultilineStringContextExt<'input>>;

#[derive(Clone)]
pub struct MultilineStringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MultilineStringContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_multilineString(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_multilineString(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineString(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineString }
}
antlr4rust::tid!{MultilineStringContextExt<'a>}

impl<'input> MultilineStringContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MultilineStringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultilineStringContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MultilineStringContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MultilineStringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPEN_MULTILINE_STRING
/// Returns `None` if there is no child corresponding to token OPEN_MULTILINE_STRING
fn OPEN_MULTILINE_STRING(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_OPEN_MULTILINE_STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_END
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_END
fn MULTILINE_STRING_END(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_END, 0)
}
fn multilineStringElement_all(&self) ->  Vec<Rc<MultilineStringElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multilineStringElement(&self, i: usize) -> Option<Rc<MultilineStringElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MultilineStringContextAttrs<'input> for MultilineStringContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn multilineString(&mut self,)
	-> Result<Rc<MultilineStringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultilineStringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 236, RULE_multilineString);
        let mut _localctx: Rc<MultilineStringContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1290);
			recog.base.match_token(WdlV1Parser_OPEN_MULTILINE_STRING,&mut recog.err_handler)?;

			recog.base.set_state(1294);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 84)) & !0x3f) == 0 && ((1usize << (_la - 84)) & 503) != 0) {
				{
				{
				/*InvokeRule multilineStringElement*/
				recog.base.set_state(1291);
				recog.multilineStringElement()?;

				}
				}
				recog.base.set_state(1296);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1297);
			recog.base.match_token(WdlV1Parser_MULTILINE_STRING_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- multilineStringElement ----------------
#[derive(Debug)]
pub enum MultilineStringElementContextAll<'input>{
	MultilineStringElementTildeContext(MultilineStringElementTildeContext<'input>),
	MultilineStringElementSingleCloseAngleContext(MultilineStringElementSingleCloseAngleContext<'input>),
	MultilineStringElementEscapeContext(MultilineStringElementEscapeContext<'input>),
	MultilineStringElementPlaceholderContext(MultilineStringElementPlaceholderContext<'input>),
	MultilineStringElementDoubleCloseAngleContext(MultilineStringElementDoubleCloseAngleContext<'input>),
	MultilineStringElementTextContext(MultilineStringElementTextContext<'input>),
	MultilineStringElementDollarSignContext(MultilineStringElementDollarSignContext<'input>),
Error(MultilineStringElementContext<'input>)
}
antlr4rust::tid!{MultilineStringElementContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for MultilineStringElementContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for MultilineStringElementContextAll<'input>{}

impl<'input> Deref for MultilineStringElementContextAll<'input>{
	type Target = dyn MultilineStringElementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use MultilineStringElementContextAll::*;
		match self{
			MultilineStringElementTildeContext(inner) => inner,
			MultilineStringElementSingleCloseAngleContext(inner) => inner,
			MultilineStringElementEscapeContext(inner) => inner,
			MultilineStringElementPlaceholderContext(inner) => inner,
			MultilineStringElementDoubleCloseAngleContext(inner) => inner,
			MultilineStringElementTextContext(inner) => inner,
			MultilineStringElementDollarSignContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type MultilineStringElementContext<'input> = BaseParserRuleContext<'input,MultilineStringElementContextExt<'input>>;

#[derive(Clone)]
pub struct MultilineStringElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}
antlr4rust::tid!{MultilineStringElementContextExt<'a>}

impl<'input> MultilineStringElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MultilineStringElementContextAll<'input>> {
		Rc::new(
		MultilineStringElementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultilineStringElementContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait MultilineStringElementContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MultilineStringElementContextExt<'input>>{


}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementContext<'input>{}

pub type MultilineStringElementTildeContext<'input> = BaseParserRuleContext<'input,MultilineStringElementTildeContextExt<'input>>;

pub trait MultilineStringElementTildeContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_TILDE
	/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_TILDE
	fn MULTILINE_STRING_TILDE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MULTILINE_STRING_TILDE, 0)
	}
}

impl<'input> MultilineStringElementTildeContextAttrs<'input> for MultilineStringElementTildeContext<'input>{}

pub struct MultilineStringElementTildeContextExt<'input>{
	base:MultilineStringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineStringElementTildeContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementTildeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementTildeContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multilineStringElementTilde(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multilineStringElementTilde(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementTildeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringElementTilde(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementTildeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}

impl<'input> Borrow<MultilineStringElementContextExt<'input>> for MultilineStringElementTildeContext<'input>{
	fn borrow(&self) -> &MultilineStringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultilineStringElementContextExt<'input>> for MultilineStringElementTildeContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultilineStringElementContextExt<'input> { &mut self.base }
}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementTildeContext<'input> {}

impl<'input> MultilineStringElementTildeContextExt<'input>{
	fn new(ctx: &dyn MultilineStringElementContextAttrs<'input>) -> Rc<MultilineStringElementContextAll<'input>>  {
		Rc::new(
			MultilineStringElementContextAll::MultilineStringElementTildeContext(
				BaseParserRuleContext::copy_from(ctx,MultilineStringElementTildeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultilineStringElementSingleCloseAngleContext<'input> = BaseParserRuleContext<'input,MultilineStringElementSingleCloseAngleContextExt<'input>>;

pub trait MultilineStringElementSingleCloseAngleContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_SINGLE_CLOSE_ANGLE
	/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_SINGLE_CLOSE_ANGLE
	fn MULTILINE_STRING_SINGLE_CLOSE_ANGLE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MULTILINE_STRING_SINGLE_CLOSE_ANGLE, 0)
	}
}

impl<'input> MultilineStringElementSingleCloseAngleContextAttrs<'input> for MultilineStringElementSingleCloseAngleContext<'input>{}

pub struct MultilineStringElementSingleCloseAngleContextExt<'input>{
	base:MultilineStringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineStringElementSingleCloseAngleContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementSingleCloseAngleContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementSingleCloseAngleContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multilineStringElementSingleCloseAngle(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multilineStringElementSingleCloseAngle(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementSingleCloseAngleContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringElementSingleCloseAngle(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementSingleCloseAngleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}

impl<'input> Borrow<MultilineStringElementContextExt<'input>> for MultilineStringElementSingleCloseAngleContext<'input>{
	fn borrow(&self) -> &MultilineStringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultilineStringElementContextExt<'input>> for MultilineStringElementSingleCloseAngleContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultilineStringElementContextExt<'input> { &mut self.base }
}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementSingleCloseAngleContext<'input> {}

impl<'input> MultilineStringElementSingleCloseAngleContextExt<'input>{
	fn new(ctx: &dyn MultilineStringElementContextAttrs<'input>) -> Rc<MultilineStringElementContextAll<'input>>  {
		Rc::new(
			MultilineStringElementContextAll::MultilineStringElementSingleCloseAngleContext(
				BaseParserRuleContext::copy_from(ctx,MultilineStringElementSingleCloseAngleContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultilineStringElementEscapeContext<'input> = BaseParserRuleContext<'input,MultilineStringElementEscapeContextExt<'input>>;

pub trait MultilineStringElementEscapeContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_ESCAPE
	/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_ESCAPE
	fn MULTILINE_STRING_ESCAPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MULTILINE_STRING_ESCAPE, 0)
	}
}

impl<'input> MultilineStringElementEscapeContextAttrs<'input> for MultilineStringElementEscapeContext<'input>{}

pub struct MultilineStringElementEscapeContextExt<'input>{
	base:MultilineStringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineStringElementEscapeContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementEscapeContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementEscapeContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multilineStringElementEscape(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multilineStringElementEscape(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementEscapeContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringElementEscape(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementEscapeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}

impl<'input> Borrow<MultilineStringElementContextExt<'input>> for MultilineStringElementEscapeContext<'input>{
	fn borrow(&self) -> &MultilineStringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultilineStringElementContextExt<'input>> for MultilineStringElementEscapeContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultilineStringElementContextExt<'input> { &mut self.base }
}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementEscapeContext<'input> {}

impl<'input> MultilineStringElementEscapeContextExt<'input>{
	fn new(ctx: &dyn MultilineStringElementContextAttrs<'input>) -> Rc<MultilineStringElementContextAll<'input>>  {
		Rc::new(
			MultilineStringElementContextAll::MultilineStringElementEscapeContext(
				BaseParserRuleContext::copy_from(ctx,MultilineStringElementEscapeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultilineStringElementPlaceholderContext<'input> = BaseParserRuleContext<'input,MultilineStringElementPlaceholderContextExt<'input>>;

pub trait MultilineStringElementPlaceholderContextAttrs<'input>: WdlV1ParserContext<'input>{
	fn multilineStringPlaceholder(&self) -> Option<Rc<MultilineStringPlaceholderContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MultilineStringElementPlaceholderContextAttrs<'input> for MultilineStringElementPlaceholderContext<'input>{}

pub struct MultilineStringElementPlaceholderContextExt<'input>{
	base:MultilineStringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineStringElementPlaceholderContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementPlaceholderContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementPlaceholderContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multilineStringElementPlaceholder(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multilineStringElementPlaceholder(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementPlaceholderContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringElementPlaceholder(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementPlaceholderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}

impl<'input> Borrow<MultilineStringElementContextExt<'input>> for MultilineStringElementPlaceholderContext<'input>{
	fn borrow(&self) -> &MultilineStringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultilineStringElementContextExt<'input>> for MultilineStringElementPlaceholderContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultilineStringElementContextExt<'input> { &mut self.base }
}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementPlaceholderContext<'input> {}

impl<'input> MultilineStringElementPlaceholderContextExt<'input>{
	fn new(ctx: &dyn MultilineStringElementContextAttrs<'input>) -> Rc<MultilineStringElementContextAll<'input>>  {
		Rc::new(
			MultilineStringElementContextAll::MultilineStringElementPlaceholderContext(
				BaseParserRuleContext::copy_from(ctx,MultilineStringElementPlaceholderContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultilineStringElementDoubleCloseAngleContext<'input> = BaseParserRuleContext<'input,MultilineStringElementDoubleCloseAngleContextExt<'input>>;

pub trait MultilineStringElementDoubleCloseAngleContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_DOUBLE_CLOSE_ANGLE
	/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_DOUBLE_CLOSE_ANGLE
	fn MULTILINE_STRING_DOUBLE_CLOSE_ANGLE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MULTILINE_STRING_DOUBLE_CLOSE_ANGLE, 0)
	}
}

impl<'input> MultilineStringElementDoubleCloseAngleContextAttrs<'input> for MultilineStringElementDoubleCloseAngleContext<'input>{}

pub struct MultilineStringElementDoubleCloseAngleContextExt<'input>{
	base:MultilineStringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineStringElementDoubleCloseAngleContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementDoubleCloseAngleContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementDoubleCloseAngleContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multilineStringElementDoubleCloseAngle(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multilineStringElementDoubleCloseAngle(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementDoubleCloseAngleContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringElementDoubleCloseAngle(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementDoubleCloseAngleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}

impl<'input> Borrow<MultilineStringElementContextExt<'input>> for MultilineStringElementDoubleCloseAngleContext<'input>{
	fn borrow(&self) -> &MultilineStringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultilineStringElementContextExt<'input>> for MultilineStringElementDoubleCloseAngleContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultilineStringElementContextExt<'input> { &mut self.base }
}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementDoubleCloseAngleContext<'input> {}

impl<'input> MultilineStringElementDoubleCloseAngleContextExt<'input>{
	fn new(ctx: &dyn MultilineStringElementContextAttrs<'input>) -> Rc<MultilineStringElementContextAll<'input>>  {
		Rc::new(
			MultilineStringElementContextAll::MultilineStringElementDoubleCloseAngleContext(
				BaseParserRuleContext::copy_from(ctx,MultilineStringElementDoubleCloseAngleContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultilineStringElementTextContext<'input> = BaseParserRuleContext<'input,MultilineStringElementTextContextExt<'input>>;

pub trait MultilineStringElementTextContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_TEXT
	/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_TEXT
	fn MULTILINE_STRING_TEXT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MULTILINE_STRING_TEXT, 0)
	}
}

impl<'input> MultilineStringElementTextContextAttrs<'input> for MultilineStringElementTextContext<'input>{}

pub struct MultilineStringElementTextContextExt<'input>{
	base:MultilineStringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineStringElementTextContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementTextContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementTextContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multilineStringElementText(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multilineStringElementText(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementTextContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringElementText(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementTextContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}

impl<'input> Borrow<MultilineStringElementContextExt<'input>> for MultilineStringElementTextContext<'input>{
	fn borrow(&self) -> &MultilineStringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultilineStringElementContextExt<'input>> for MultilineStringElementTextContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultilineStringElementContextExt<'input> { &mut self.base }
}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementTextContext<'input> {}

impl<'input> MultilineStringElementTextContextExt<'input>{
	fn new(ctx: &dyn MultilineStringElementContextAttrs<'input>) -> Rc<MultilineStringElementContextAll<'input>>  {
		Rc::new(
			MultilineStringElementContextAll::MultilineStringElementTextContext(
				BaseParserRuleContext::copy_from(ctx,MultilineStringElementTextContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultilineStringElementDollarSignContext<'input> = BaseParserRuleContext<'input,MultilineStringElementDollarSignContextExt<'input>>;

pub trait MultilineStringElementDollarSignContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_DOLLAR_SIGN
	/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_DOLLAR_SIGN
	fn MULTILINE_STRING_DOLLAR_SIGN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_MULTILINE_STRING_DOLLAR_SIGN, 0)
	}
}

impl<'input> MultilineStringElementDollarSignContextAttrs<'input> for MultilineStringElementDollarSignContext<'input>{}

pub struct MultilineStringElementDollarSignContextExt<'input>{
	base:MultilineStringElementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineStringElementDollarSignContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for MultilineStringElementDollarSignContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringElementDollarSignContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multilineStringElementDollarSign(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_multilineStringElementDollarSign(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringElementDollarSignContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringElementDollarSign(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringElementDollarSignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringElement }
}

impl<'input> Borrow<MultilineStringElementContextExt<'input>> for MultilineStringElementDollarSignContext<'input>{
	fn borrow(&self) -> &MultilineStringElementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MultilineStringElementContextExt<'input>> for MultilineStringElementDollarSignContext<'input>{
	fn borrow_mut(&mut self) -> &mut MultilineStringElementContextExt<'input> { &mut self.base }
}

impl<'input> MultilineStringElementContextAttrs<'input> for MultilineStringElementDollarSignContext<'input> {}

impl<'input> MultilineStringElementDollarSignContextExt<'input>{
	fn new(ctx: &dyn MultilineStringElementContextAttrs<'input>) -> Rc<MultilineStringElementContextAll<'input>>  {
		Rc::new(
			MultilineStringElementContextAll::MultilineStringElementDollarSignContext(
				BaseParserRuleContext::copy_from(ctx,MultilineStringElementDollarSignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn multilineStringElement(&mut self,)
	-> Result<Rc<MultilineStringElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultilineStringElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 238, RULE_multilineStringElement);
        let mut _localctx: Rc<MultilineStringElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1306);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_MULTILINE_STRING_TEXT 
				=> {
					let tmp = MultilineStringElementTextContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(1299);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_TEXT,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_MULTILINE_STRING_ESCAPE 
				=> {
					let tmp = MultilineStringElementEscapeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(1300);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_ESCAPE,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_MULTILINE_STRING_DOUBLE_CLOSE_ANGLE 
				=> {
					let tmp = MultilineStringElementDoubleCloseAngleContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(1301);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_DOUBLE_CLOSE_ANGLE,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_MULTILINE_STRING_SINGLE_CLOSE_ANGLE 
				=> {
					let tmp = MultilineStringElementSingleCloseAngleContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					recog.base.set_state(1302);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_SINGLE_CLOSE_ANGLE,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_MULTILINE_STRING_DOLLAR_SIGN 
				=> {
					let tmp = MultilineStringElementDollarSignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					recog.base.set_state(1303);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_DOLLAR_SIGN,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_MULTILINE_STRING_TILDE 
				=> {
					let tmp = MultilineStringElementTildeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					recog.base.set_state(1304);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_TILDE,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_MULTILINE_STRING_DOLLAR_PLACEHOLDER_START |WdlV1Parser_MULTILINE_STRING_TILDE_PLACEHOLDER_START 
				=> {
					let tmp = MultilineStringElementPlaceholderContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					/*InvokeRule multilineStringPlaceholder*/
					recog.base.set_state(1305);
					recog.multilineStringPlaceholder()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- multilineStringPlaceholder ----------------
pub type MultilineStringPlaceholderContextAll<'input> = MultilineStringPlaceholderContext<'input>;


pub type MultilineStringPlaceholderContext<'input> = BaseParserRuleContext<'input,MultilineStringPlaceholderContextExt<'input>>;

#[derive(Clone)]
pub struct MultilineStringPlaceholderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for MultilineStringPlaceholderContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for MultilineStringPlaceholderContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_multilineStringPlaceholder(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_multilineStringPlaceholder(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for MultilineStringPlaceholderContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_multilineStringPlaceholder(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineStringPlaceholderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multilineStringPlaceholder }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multilineStringPlaceholder }
}
antlr4rust::tid!{MultilineStringPlaceholderContextExt<'a>}

impl<'input> MultilineStringPlaceholderContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MultilineStringPlaceholderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultilineStringPlaceholderContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MultilineStringPlaceholderContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<MultilineStringPlaceholderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_TILDE_PLACEHOLDER_START
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_TILDE_PLACEHOLDER_START
fn MULTILINE_STRING_TILDE_PLACEHOLDER_START(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_TILDE_PLACEHOLDER_START, 0)
}
fn stringPlaceholderExpression(&self) -> Option<Rc<StringPlaceholderExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
/// Returns `None` if there is no child corresponding to token CLOSE_BRACE
fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_CLOSE_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token MULTILINE_STRING_DOLLAR_PLACEHOLDER_START
/// Returns `None` if there is no child corresponding to token MULTILINE_STRING_DOLLAR_PLACEHOLDER_START
fn MULTILINE_STRING_DOLLAR_PLACEHOLDER_START(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_MULTILINE_STRING_DOLLAR_PLACEHOLDER_START, 0)
}

}

impl<'input> MultilineStringPlaceholderContextAttrs<'input> for MultilineStringPlaceholderContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn multilineStringPlaceholder(&mut self,)
	-> Result<Rc<MultilineStringPlaceholderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultilineStringPlaceholderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 240, RULE_multilineStringPlaceholder);
        let mut _localctx: Rc<MultilineStringPlaceholderContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1316);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_MULTILINE_STRING_TILDE_PLACEHOLDER_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1308);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_TILDE_PLACEHOLDER_START,&mut recog.err_handler)?;

					/*InvokeRule stringPlaceholderExpression*/
					recog.base.set_state(1309);
					recog.stringPlaceholderExpression()?;

					recog.base.set_state(1310);
					recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

					}
				}

			WdlV1Parser_MULTILINE_STRING_DOLLAR_PLACEHOLDER_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1312);
					recog.base.match_token(WdlV1Parser_MULTILINE_STRING_DOLLAR_PLACEHOLDER_START,&mut recog.err_handler)?;

					/*InvokeRule stringPlaceholderExpression*/
					recog.base.set_state(1313);
					recog.stringPlaceholderExpression()?;

					recog.base.set_state(1314);
					recog.base.match_token(WdlV1Parser_CLOSE_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- stringPlaceholderExpression ----------------
pub type StringPlaceholderExpressionContextAll<'input> = StringPlaceholderExpressionContext<'input>;


pub type StringPlaceholderExpressionContext<'input> = BaseParserRuleContext<'input,StringPlaceholderExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct StringPlaceholderExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StringPlaceholderExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringPlaceholderExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_stringPlaceholderExpression(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_stringPlaceholderExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringPlaceholderExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringPlaceholderExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringPlaceholderExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringPlaceholderExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringPlaceholderExpression }
}
antlr4rust::tid!{StringPlaceholderExpressionContextExt<'a>}

impl<'input> StringPlaceholderExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StringPlaceholderExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringPlaceholderExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StringPlaceholderExpressionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StringPlaceholderExpressionContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringPlaceholderOption_all(&self) ->  Vec<Rc<StringPlaceholderOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stringPlaceholderOption(&self, i: usize) -> Option<Rc<StringPlaceholderOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StringPlaceholderExpressionContextAttrs<'input> for StringPlaceholderExpressionContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stringPlaceholderExpression(&mut self,)
	-> Result<Rc<StringPlaceholderExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringPlaceholderExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 242, RULE_stringPlaceholderExpression);
        let mut _localctx: Rc<StringPlaceholderExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1321);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(142,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule stringPlaceholderOption*/
					recog.base.set_state(1318);
					recog.stringPlaceholderOption()?;

					}
					} 
				}
				recog.base.set_state(1323);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(142,&mut recog.base)?;
			}
			/*InvokeRule expression*/
			recog.base.set_state(1324);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- stringPlaceholderOption ----------------
#[derive(Debug)]
pub enum StringPlaceholderOptionContextAll<'input>{
	StringPlaceholderOptionTrueFalseContext(StringPlaceholderOptionTrueFalseContext<'input>),
	StringPlaceholderOptionFalseTrueContext(StringPlaceholderOptionFalseTrueContext<'input>),
	StringPlaceholderOptionSepDefaultContext(StringPlaceholderOptionSepDefaultContext<'input>),
Error(StringPlaceholderOptionContext<'input>)
}
antlr4rust::tid!{StringPlaceholderOptionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for StringPlaceholderOptionContextAll<'input>{}

impl<'input> WdlV1ParserParserContext<'input> for StringPlaceholderOptionContextAll<'input>{}

impl<'input> Deref for StringPlaceholderOptionContextAll<'input>{
	type Target = dyn StringPlaceholderOptionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StringPlaceholderOptionContextAll::*;
		match self{
			StringPlaceholderOptionTrueFalseContext(inner) => inner,
			StringPlaceholderOptionFalseTrueContext(inner) => inner,
			StringPlaceholderOptionSepDefaultContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringPlaceholderOptionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringPlaceholderOptionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type StringPlaceholderOptionContext<'input> = BaseParserRuleContext<'input,StringPlaceholderOptionContextExt<'input>>;

#[derive(Clone)]
pub struct StringPlaceholderOptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StringPlaceholderOptionContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringPlaceholderOptionContext<'input>{
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringPlaceholderOptionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StringPlaceholderOptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringPlaceholderOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringPlaceholderOption }
}
antlr4rust::tid!{StringPlaceholderOptionContextExt<'a>}

impl<'input> StringPlaceholderOptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StringPlaceholderOptionContextAll<'input>> {
		Rc::new(
		StringPlaceholderOptionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringPlaceholderOptionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StringPlaceholderOptionContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StringPlaceholderOptionContextExt<'input>>{


}

impl<'input> StringPlaceholderOptionContextAttrs<'input> for StringPlaceholderOptionContext<'input>{}

pub type StringPlaceholderOptionTrueFalseContext<'input> = BaseParserRuleContext<'input,StringPlaceholderOptionTrueFalseContextExt<'input>>;

pub trait StringPlaceholderOptionTrueFalseContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_TRUE
	/// Returns `None` if there is no child corresponding to token KEYWORD_TRUE
	fn KEYWORD_TRUE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_TRUE, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token ASSIGNMENT in current rule
	fn ASSIGNMENT_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token ASSIGNMENT, starting from 0.
	/// Returns `None` if number of children corresponding to token ASSIGNMENT is less or equal than `i`.
	fn ASSIGNMENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_ASSIGNMENT, i)
	}
	fn stringLiteral_all(&self) ->  Vec<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stringLiteral(&self, i: usize) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token KEYWORD_FALSE
	/// Returns `None` if there is no child corresponding to token KEYWORD_FALSE
	fn KEYWORD_FALSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_FALSE, 0)
	}
}

impl<'input> StringPlaceholderOptionTrueFalseContextAttrs<'input> for StringPlaceholderOptionTrueFalseContext<'input>{}

pub struct StringPlaceholderOptionTrueFalseContextExt<'input>{
	base:StringPlaceholderOptionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringPlaceholderOptionTrueFalseContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringPlaceholderOptionTrueFalseContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringPlaceholderOptionTrueFalseContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringPlaceholderOptionTrueFalse(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringPlaceholderOptionTrueFalse(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringPlaceholderOptionTrueFalseContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringPlaceholderOptionTrueFalse(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringPlaceholderOptionTrueFalseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringPlaceholderOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringPlaceholderOption }
}

impl<'input> Borrow<StringPlaceholderOptionContextExt<'input>> for StringPlaceholderOptionTrueFalseContext<'input>{
	fn borrow(&self) -> &StringPlaceholderOptionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringPlaceholderOptionContextExt<'input>> for StringPlaceholderOptionTrueFalseContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringPlaceholderOptionContextExt<'input> { &mut self.base }
}

impl<'input> StringPlaceholderOptionContextAttrs<'input> for StringPlaceholderOptionTrueFalseContext<'input> {}

impl<'input> StringPlaceholderOptionTrueFalseContextExt<'input>{
	fn new(ctx: &dyn StringPlaceholderOptionContextAttrs<'input>) -> Rc<StringPlaceholderOptionContextAll<'input>>  {
		Rc::new(
			StringPlaceholderOptionContextAll::StringPlaceholderOptionTrueFalseContext(
				BaseParserRuleContext::copy_from(ctx,StringPlaceholderOptionTrueFalseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringPlaceholderOptionFalseTrueContext<'input> = BaseParserRuleContext<'input,StringPlaceholderOptionFalseTrueContextExt<'input>>;

pub trait StringPlaceholderOptionFalseTrueContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_FALSE
	/// Returns `None` if there is no child corresponding to token KEYWORD_FALSE
	fn KEYWORD_FALSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_FALSE, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token ASSIGNMENT in current rule
	fn ASSIGNMENT_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token ASSIGNMENT, starting from 0.
	/// Returns `None` if number of children corresponding to token ASSIGNMENT is less or equal than `i`.
	fn ASSIGNMENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_ASSIGNMENT, i)
	}
	fn stringLiteral_all(&self) ->  Vec<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stringLiteral(&self, i: usize) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token KEYWORD_TRUE
	/// Returns `None` if there is no child corresponding to token KEYWORD_TRUE
	fn KEYWORD_TRUE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_KEYWORD_TRUE, 0)
	}
}

impl<'input> StringPlaceholderOptionFalseTrueContextAttrs<'input> for StringPlaceholderOptionFalseTrueContext<'input>{}

pub struct StringPlaceholderOptionFalseTrueContextExt<'input>{
	base:StringPlaceholderOptionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringPlaceholderOptionFalseTrueContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringPlaceholderOptionFalseTrueContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringPlaceholderOptionFalseTrueContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringPlaceholderOptionFalseTrue(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringPlaceholderOptionFalseTrue(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringPlaceholderOptionFalseTrueContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringPlaceholderOptionFalseTrue(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringPlaceholderOptionFalseTrueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringPlaceholderOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringPlaceholderOption }
}

impl<'input> Borrow<StringPlaceholderOptionContextExt<'input>> for StringPlaceholderOptionFalseTrueContext<'input>{
	fn borrow(&self) -> &StringPlaceholderOptionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringPlaceholderOptionContextExt<'input>> for StringPlaceholderOptionFalseTrueContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringPlaceholderOptionContextExt<'input> { &mut self.base }
}

impl<'input> StringPlaceholderOptionContextAttrs<'input> for StringPlaceholderOptionFalseTrueContext<'input> {}

impl<'input> StringPlaceholderOptionFalseTrueContextExt<'input>{
	fn new(ctx: &dyn StringPlaceholderOptionContextAttrs<'input>) -> Rc<StringPlaceholderOptionContextAll<'input>>  {
		Rc::new(
			StringPlaceholderOptionContextAll::StringPlaceholderOptionFalseTrueContext(
				BaseParserRuleContext::copy_from(ctx,StringPlaceholderOptionFalseTrueContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringPlaceholderOptionSepDefaultContext<'input> = BaseParserRuleContext<'input,StringPlaceholderOptionSepDefaultContextExt<'input>>;

pub trait StringPlaceholderOptionSepDefaultContextAttrs<'input>: WdlV1ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_IDENTIFIER, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ASSIGNMENT
	/// Returns `None` if there is no child corresponding to token ASSIGNMENT
	fn ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
		self.get_token(WdlV1Parser_ASSIGNMENT, 0)
	}
	fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StringPlaceholderOptionSepDefaultContextAttrs<'input> for StringPlaceholderOptionSepDefaultContext<'input>{}

pub struct StringPlaceholderOptionSepDefaultContextExt<'input>{
	base:StringPlaceholderOptionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringPlaceholderOptionSepDefaultContextExt<'a>}

impl<'input> WdlV1ParserContext<'input> for StringPlaceholderOptionSepDefaultContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StringPlaceholderOptionSepDefaultContext<'input>{
	fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_stringPlaceholderOptionSepDefault(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_stringPlaceholderOptionSepDefault(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StringPlaceholderOptionSepDefaultContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_stringPlaceholderOptionSepDefault(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringPlaceholderOptionSepDefaultContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringPlaceholderOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringPlaceholderOption }
}

impl<'input> Borrow<StringPlaceholderOptionContextExt<'input>> for StringPlaceholderOptionSepDefaultContext<'input>{
	fn borrow(&self) -> &StringPlaceholderOptionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StringPlaceholderOptionContextExt<'input>> for StringPlaceholderOptionSepDefaultContext<'input>{
	fn borrow_mut(&mut self) -> &mut StringPlaceholderOptionContextExt<'input> { &mut self.base }
}

impl<'input> StringPlaceholderOptionContextAttrs<'input> for StringPlaceholderOptionSepDefaultContext<'input> {}

impl<'input> StringPlaceholderOptionSepDefaultContextExt<'input>{
	fn new(ctx: &dyn StringPlaceholderOptionContextAttrs<'input>) -> Rc<StringPlaceholderOptionContextAll<'input>>  {
		Rc::new(
			StringPlaceholderOptionContextAll::StringPlaceholderOptionSepDefaultContext(
				BaseParserRuleContext::copy_from(ctx,StringPlaceholderOptionSepDefaultContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stringPlaceholderOption(&mut self,)
	-> Result<Rc<StringPlaceholderOptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringPlaceholderOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 244, RULE_stringPlaceholderOption);
        let mut _localctx: Rc<StringPlaceholderOptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1343);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			WdlV1Parser_IDENTIFIER 
				=> {
					let tmp = StringPlaceholderOptionSepDefaultContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(1326);
					recog.base.match_token(WdlV1Parser_IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(1327);
					recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(1328);
					recog.stringLiteral()?;

					}
				}

			WdlV1Parser_KEYWORD_TRUE 
				=> {
					let tmp = StringPlaceholderOptionTrueFalseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(1329);
					recog.base.match_token(WdlV1Parser_KEYWORD_TRUE,&mut recog.err_handler)?;

					recog.base.set_state(1330);
					recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(1331);
					recog.stringLiteral()?;

					recog.base.set_state(1332);
					recog.base.match_token(WdlV1Parser_KEYWORD_FALSE,&mut recog.err_handler)?;

					recog.base.set_state(1333);
					recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(1334);
					recog.stringLiteral()?;

					}
				}

			WdlV1Parser_KEYWORD_FALSE 
				=> {
					let tmp = StringPlaceholderOptionFalseTrueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(1336);
					recog.base.match_token(WdlV1Parser_KEYWORD_FALSE,&mut recog.err_handler)?;

					recog.base.set_state(1337);
					recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(1338);
					recog.stringLiteral()?;

					recog.base.set_state(1339);
					recog.base.match_token(WdlV1Parser_KEYWORD_TRUE,&mut recog.err_handler)?;

					recog.base.set_state(1340);
					recog.base.match_token(WdlV1Parser_ASSIGNMENT,&mut recog.err_handler)?;

					/*InvokeRule stringLiteral*/
					recog.base.set_state(1341);
					recog.stringLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- strictIdentifier ----------------
pub type StrictIdentifierContextAll<'input> = StrictIdentifierContext<'input>;


pub type StrictIdentifierContext<'input> = BaseParserRuleContext<'input,StrictIdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct StrictIdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for StrictIdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for StrictIdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_strictIdentifier(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_strictIdentifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for StrictIdentifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_strictIdentifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for StrictIdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_strictIdentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_strictIdentifier }
}
antlr4rust::tid!{StrictIdentifierContextExt<'a>}

impl<'input> StrictIdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StrictIdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StrictIdentifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StrictIdentifierContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<StrictIdentifierContextExt<'input>>{

fn anyIdentBase(&self) -> Option<Rc<AnyIdentBaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StrictIdentifierContextAttrs<'input> for StrictIdentifierContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn strictIdentifier(&mut self,)
	-> Result<Rc<StrictIdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StrictIdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 246, RULE_strictIdentifier);
        let mut _localctx: Rc<StrictIdentifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule anyIdentBase*/
			recog.base.set_state(1345);
			recog.anyIdentBase()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- dottedIdentifier ----------------
pub type DottedIdentifierContextAll<'input> = DottedIdentifierContext<'input>;


pub type DottedIdentifierContext<'input> = BaseParserRuleContext<'input,DottedIdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct DottedIdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for DottedIdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for DottedIdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dottedIdentifier(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dottedIdentifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for DottedIdentifierContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_dottedIdentifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for DottedIdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dottedIdentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dottedIdentifier }
}
antlr4rust::tid!{DottedIdentifierContextExt<'a>}

impl<'input> DottedIdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DottedIdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DottedIdentifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DottedIdentifierContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<DottedIdentifierContextExt<'input>>{

fn strictIdentifier_all(&self) ->  Vec<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn strictIdentifier(&self, i: usize) -> Option<Rc<StrictIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,WdlV1ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_DOT, i)
}

}

impl<'input> DottedIdentifierContextAttrs<'input> for DottedIdentifierContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dottedIdentifier(&mut self,)
	-> Result<Rc<DottedIdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DottedIdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 248, RULE_dottedIdentifier);
        let mut _localctx: Rc<DottedIdentifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule strictIdentifier*/
			recog.base.set_state(1347);
			recog.strictIdentifier()?;

			recog.base.set_state(1352);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==WdlV1Parser_DOT {
				{
				{
				recog.base.set_state(1348);
				recog.base.match_token(WdlV1Parser_DOT,&mut recog.err_handler)?;

				/*InvokeRule strictIdentifier*/
				recog.base.set_state(1349);
				recog.strictIdentifier()?;

				}
				}
				recog.base.set_state(1354);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- anyIdentBase ----------------
pub type AnyIdentBaseContextAll<'input> = AnyIdentBaseContext<'input>;


pub type AnyIdentBaseContext<'input> = BaseParserRuleContext<'input,AnyIdentBaseContextExt<'input>>;

#[derive(Clone)]
pub struct AnyIdentBaseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> WdlV1ParserContext<'input> for AnyIdentBaseContext<'input>{}

impl<'input,'a> Listenable<dyn WdlV1ParserListener<'input> + 'a> for AnyIdentBaseContext<'input>{
		fn enter(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_anyIdentBase(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn WdlV1ParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_anyIdentBase(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn WdlV1ParserVisitor<'input> + 'a> for AnyIdentBaseContext<'input>{
	fn accept(&self,visitor: &mut (dyn WdlV1ParserVisitor<'input> + 'a)) {
		visitor.visit_anyIdentBase(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnyIdentBaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = WdlV1ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_anyIdentBase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_anyIdentBase }
}
antlr4rust::tid!{AnyIdentBaseContextExt<'a>}

impl<'input> AnyIdentBaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn WdlV1ParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AnyIdentBaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnyIdentBaseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AnyIdentBaseContextAttrs<'input>: WdlV1ParserContext<'input> + BorrowMut<AnyIdentBaseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ARRAY_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_ARRAY_TYPE
fn KEYWORD_ARRAY_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ARRAY_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_BOOLEAN_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_BOOLEAN_TYPE
fn KEYWORD_BOOLEAN_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_BOOLEAN_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_DIRECTORY_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_DIRECTORY_TYPE
fn KEYWORD_DIRECTORY_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_DIRECTORY_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_FILE_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_FILE_TYPE
fn KEYWORD_FILE_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_FILE_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_FLOAT_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_FLOAT_TYPE
fn KEYWORD_FLOAT_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_FLOAT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_INT_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_INT_TYPE
fn KEYWORD_INT_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_INT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_MAP_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_MAP_TYPE
fn KEYWORD_MAP_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_MAP_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_OBJECT_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_OBJECT_TYPE
fn KEYWORD_OBJECT_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OBJECT_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_PAIR_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_PAIR_TYPE
fn KEYWORD_PAIR_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_PAIR_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_STRING_TYPE
/// Returns `None` if there is no child corresponding to token KEYWORD_STRING_TYPE
fn KEYWORD_STRING_TYPE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_STRING_TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_AFTER
/// Returns `None` if there is no child corresponding to token KEYWORD_AFTER
fn KEYWORD_AFTER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_AFTER, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ALIAS
/// Returns `None` if there is no child corresponding to token KEYWORD_ALIAS
fn KEYWORD_ALIAS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ALIAS, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_AS
/// Returns `None` if there is no child corresponding to token KEYWORD_AS
fn KEYWORD_AS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_AS, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_CALL
/// Returns `None` if there is no child corresponding to token KEYWORD_CALL
fn KEYWORD_CALL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_CALL, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_COMMAND
/// Returns `None` if there is no child corresponding to token KEYWORD_COMMAND
fn KEYWORD_COMMAND(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_COMMAND, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ELSE
/// Returns `None` if there is no child corresponding to token KEYWORD_ELSE
fn KEYWORD_ELSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ENV
/// Returns `None` if there is no child corresponding to token KEYWORD_ENV
fn KEYWORD_ENV(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ENV, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_FALSE
/// Returns `None` if there is no child corresponding to token KEYWORD_FALSE
fn KEYWORD_FALSE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_FALSE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_FROM
/// Returns `None` if there is no child corresponding to token KEYWORD_FROM
fn KEYWORD_FROM(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_FROM, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_HINTS
/// Returns `None` if there is no child corresponding to token KEYWORD_HINTS
fn KEYWORD_HINTS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_HINTS, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_IF
/// Returns `None` if there is no child corresponding to token KEYWORD_IF
fn KEYWORD_IF(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_IF, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_IN
/// Returns `None` if there is no child corresponding to token KEYWORD_IN
fn KEYWORD_IN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_IN, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_IMPORT
/// Returns `None` if there is no child corresponding to token KEYWORD_IMPORT
fn KEYWORD_IMPORT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_IMPORT, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_INPUT
/// Returns `None` if there is no child corresponding to token KEYWORD_INPUT
fn KEYWORD_INPUT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_INPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_META
/// Returns `None` if there is no child corresponding to token KEYWORD_META
fn KEYWORD_META(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_META, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_NONE
/// Returns `None` if there is no child corresponding to token KEYWORD_NONE
fn KEYWORD_NONE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_NONE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_NULL
/// Returns `None` if there is no child corresponding to token KEYWORD_NULL
fn KEYWORD_NULL(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_NULL, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_OBJECT
/// Returns `None` if there is no child corresponding to token KEYWORD_OBJECT
fn KEYWORD_OBJECT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OBJECT, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_OUTPUT
/// Returns `None` if there is no child corresponding to token KEYWORD_OUTPUT
fn KEYWORD_OUTPUT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_OUTPUT, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_PARAMETER_META
/// Returns `None` if there is no child corresponding to token KEYWORD_PARAMETER_META
fn KEYWORD_PARAMETER_META(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_PARAMETER_META, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_REQUIREMENTS
/// Returns `None` if there is no child corresponding to token KEYWORD_REQUIREMENTS
fn KEYWORD_REQUIREMENTS(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_REQUIREMENTS, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_RUNTIME
/// Returns `None` if there is no child corresponding to token KEYWORD_RUNTIME
fn KEYWORD_RUNTIME(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_RUNTIME, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_SCATTER
/// Returns `None` if there is no child corresponding to token KEYWORD_SCATTER
fn KEYWORD_SCATTER(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_SCATTER, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_STRUCT
/// Returns `None` if there is no child corresponding to token KEYWORD_STRUCT
fn KEYWORD_STRUCT(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_STRUCT, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_ENUM
/// Returns `None` if there is no child corresponding to token KEYWORD_ENUM
fn KEYWORD_ENUM(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_ENUM, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_TASK
/// Returns `None` if there is no child corresponding to token KEYWORD_TASK
fn KEYWORD_TASK(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_TASK, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_THEN
/// Returns `None` if there is no child corresponding to token KEYWORD_THEN
fn KEYWORD_THEN(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_THEN, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_TRUE
/// Returns `None` if there is no child corresponding to token KEYWORD_TRUE
fn KEYWORD_TRUE(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_VERSION
/// Returns `None` if there is no child corresponding to token KEYWORD_VERSION
fn KEYWORD_VERSION(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_VERSION, 0)
}
/// Retrieves first TerminalNode corresponding to token KEYWORD_WORKFLOW
/// Returns `None` if there is no child corresponding to token KEYWORD_WORKFLOW
fn KEYWORD_WORKFLOW(&self) -> Option<Rc<TerminalNode<'input,WdlV1ParserContextType>>> where Self:Sized{
	self.get_token(WdlV1Parser_KEYWORD_WORKFLOW, 0)
}

}

impl<'input> AnyIdentBaseContextAttrs<'input> for AnyIdentBaseContext<'input>{}

impl<'input, I> WdlV1Parser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn anyIdentBase(&mut self,)
	-> Result<Rc<AnyIdentBaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnyIdentBaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 250, RULE_anyIdentBase);
        let mut _localctx: Rc<AnyIdentBaseContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1355);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 4294963200) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2097151) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 109, 1358, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
		7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 
		7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 
		7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 40, 
		7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 
		7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 
		7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 
		7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 58, 2, 59, 7, 59, 2, 60, 
		7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 64, 2, 65, 
		7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 69, 2, 70, 
		7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 73, 2, 74, 7, 74, 2, 75, 
		7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2, 78, 7, 78, 2, 79, 7, 79, 2, 80, 
		7, 80, 2, 81, 7, 81, 2, 82, 7, 82, 2, 83, 7, 83, 2, 84, 7, 84, 2, 85, 
		7, 85, 2, 86, 7, 86, 2, 87, 7, 87, 2, 88, 7, 88, 2, 89, 7, 89, 2, 90, 
		7, 90, 2, 91, 7, 91, 2, 92, 7, 92, 2, 93, 7, 93, 2, 94, 7, 94, 2, 95, 
		7, 95, 2, 96, 7, 96, 2, 97, 7, 97, 2, 98, 7, 98, 2, 99, 7, 99, 2, 100, 
		7, 100, 2, 101, 7, 101, 2, 102, 7, 102, 2, 103, 7, 103, 2, 104, 7, 104, 
		2, 105, 7, 105, 2, 106, 7, 106, 2, 107, 7, 107, 2, 108, 7, 108, 2, 109, 
		7, 109, 2, 110, 7, 110, 2, 111, 7, 111, 2, 112, 7, 112, 2, 113, 7, 113, 
		2, 114, 7, 114, 2, 115, 7, 115, 2, 116, 7, 116, 2, 117, 7, 117, 2, 118, 
		7, 118, 2, 119, 7, 119, 2, 120, 7, 120, 2, 121, 7, 121, 2, 122, 7, 122, 
		2, 123, 7, 123, 2, 124, 7, 124, 2, 125, 7, 125, 1, 0, 1, 0, 5, 0, 255, 
		8, 0, 10, 0, 12, 0, 258, 9, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
		2, 1, 2, 1, 2, 1, 2, 3, 2, 270, 8, 2, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 276, 
		8, 3, 1, 3, 5, 3, 279, 8, 3, 10, 3, 12, 3, 282, 9, 3, 1, 3, 1, 3, 1, 3, 
		1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 293, 8, 3, 1, 4, 1, 4, 1, 4, 
		1, 4, 5, 4, 299, 8, 4, 10, 4, 12, 4, 302, 9, 4, 1, 4, 3, 4, 305, 8, 4, 
		3, 4, 307, 8, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 3, 5, 314, 8, 5, 1, 6, 
		1, 6, 5, 6, 318, 8, 6, 10, 6, 12, 6, 321, 9, 6, 1, 6, 1, 6, 1, 6, 5, 6, 
		326, 8, 6, 10, 6, 12, 6, 329, 9, 6, 1, 6, 3, 6, 332, 8, 6, 1, 7, 1, 7, 
		1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 5, 9, 345, 8, 9, 
		10, 9, 12, 9, 348, 9, 9, 1, 9, 1, 9, 1, 10, 1, 10, 1, 10, 3, 10, 355, 
		8, 10, 1, 11, 1, 11, 1, 11, 1, 12, 1, 12, 1, 12, 3, 12, 363, 8, 12, 1, 
		12, 1, 12, 1, 12, 1, 12, 5, 12, 369, 8, 12, 10, 12, 12, 12, 372, 9, 12, 
		1, 12, 3, 12, 375, 8, 12, 3, 12, 377, 8, 12, 1, 12, 1, 12, 1, 13, 1, 13, 
		1, 13, 1, 13, 1, 14, 1, 14, 1, 14, 3, 14, 388, 8, 14, 1, 15, 1, 15, 1, 
		15, 1, 15, 1, 15, 1, 15, 1, 15, 1, 15, 3, 15, 398, 8, 15, 1, 16, 1, 16, 
		3, 16, 402, 8, 16, 1, 17, 1, 17, 5, 17, 406, 8, 17, 10, 17, 12, 17, 409, 
		9, 17, 1, 17, 1, 17, 1, 17, 5, 17, 414, 8, 17, 10, 17, 12, 17, 417, 9, 
		17, 1, 17, 3, 17, 420, 8, 17, 1, 18, 1, 18, 1, 19, 1, 19, 5, 19, 426, 
		8, 19, 10, 19, 12, 19, 429, 9, 19, 1, 19, 1, 19, 1, 20, 1, 20, 1, 21, 
		1, 21, 1, 21, 1, 21, 5, 21, 439, 8, 21, 10, 21, 12, 21, 442, 9, 21, 1, 
		21, 3, 21, 445, 8, 21, 3, 21, 447, 8, 21, 1, 21, 1, 21, 1, 22, 1, 22, 
		1, 22, 1, 22, 5, 22, 455, 8, 22, 10, 22, 12, 22, 458, 9, 22, 1, 22, 3, 
		22, 461, 8, 22, 3, 22, 463, 8, 22, 1, 22, 1, 22, 1, 23, 1, 23, 1, 23, 
		1, 23, 1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 5, 24, 476, 8, 24, 10, 24, 12, 
		24, 479, 9, 24, 1, 24, 3, 24, 482, 8, 24, 3, 24, 484, 8, 24, 1, 24, 1, 
		24, 1, 25, 1, 25, 1, 25, 1, 25, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 5, 
		26, 497, 8, 26, 10, 26, 12, 26, 500, 9, 26, 1, 26, 3, 26, 503, 8, 26, 
		3, 26, 505, 8, 26, 1, 26, 1, 26, 1, 27, 1, 27, 1, 27, 1, 27, 1, 28, 1, 
		28, 1, 28, 1, 28, 1, 28, 1, 28, 1, 29, 1, 29, 1, 29, 1, 29, 5, 29, 523, 
		8, 29, 10, 29, 12, 29, 526, 9, 29, 1, 29, 1, 29, 1, 30, 1, 30, 1, 30, 
		1, 30, 5, 30, 534, 8, 30, 10, 30, 12, 30, 537, 9, 30, 1, 30, 1, 30, 1, 
		31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 3, 31, 547, 8, 31, 1, 32, 1, 32, 
		1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 3, 32, 556, 8, 32, 1, 33, 1, 33, 1, 
		33, 1, 33, 1, 33, 3, 33, 563, 8, 33, 1, 33, 3, 33, 566, 8, 33, 1, 34, 
		1, 34, 1, 34, 1, 34, 1, 34, 1, 34, 1, 34, 3, 34, 575, 8, 34, 1, 35, 1, 
		35, 3, 35, 579, 8, 35, 1, 36, 1, 36, 3, 36, 583, 8, 36, 1, 37, 1, 37, 
		3, 37, 587, 8, 37, 1, 38, 3, 38, 590, 8, 38, 1, 38, 1, 38, 1, 38, 1, 39, 
		3, 39, 596, 8, 39, 1, 39, 1, 39, 1, 39, 1, 39, 1, 39, 1, 40, 1, 40, 3, 
		40, 605, 8, 40, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 
		1, 41, 3, 41, 616, 8, 41, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 
		42, 1, 42, 1, 42, 3, 42, 627, 8, 42, 1, 43, 1, 43, 1, 43, 5, 43, 632, 
		8, 43, 10, 43, 12, 43, 635, 9, 43, 1, 43, 1, 43, 1, 44, 1, 44, 1, 44, 
		5, 44, 642, 8, 44, 10, 44, 12, 44, 645, 9, 44, 1, 44, 1, 44, 1, 45, 1, 
		45, 1, 45, 5, 45, 652, 8, 45, 10, 45, 12, 45, 655, 9, 45, 1, 45, 1, 45, 
		1, 46, 1, 46, 1, 46, 1, 46, 1, 47, 1, 47, 1, 47, 5, 47, 666, 8, 47, 10, 
		47, 12, 47, 669, 9, 47, 1, 47, 1, 47, 1, 48, 1, 48, 1, 48, 1, 48, 1, 49, 
		1, 49, 1, 49, 5, 49, 680, 8, 49, 10, 49, 12, 49, 683, 9, 49, 1, 49, 1, 
		49, 1, 50, 1, 50, 1, 50, 1, 50, 1, 51, 1, 51, 1, 51, 1, 51, 1, 51, 3, 
		51, 696, 8, 51, 1, 52, 1, 52, 1, 52, 1, 52, 1, 52, 5, 52, 703, 8, 52, 
		10, 52, 12, 52, 706, 9, 52, 1, 52, 3, 52, 709, 8, 52, 3, 52, 711, 8, 52, 
		1, 52, 1, 52, 1, 53, 1, 53, 1, 53, 1, 53, 1, 54, 1, 54, 1, 54, 1, 54, 
		1, 54, 5, 54, 724, 8, 54, 10, 54, 12, 54, 727, 9, 54, 1, 54, 3, 54, 730, 
		8, 54, 3, 54, 732, 8, 54, 1, 54, 1, 54, 1, 55, 1, 55, 1, 55, 1, 55, 1, 
		56, 1, 56, 1, 56, 1, 56, 1, 56, 5, 56, 745, 8, 56, 10, 56, 12, 56, 748, 
		9, 56, 1, 56, 3, 56, 751, 8, 56, 3, 56, 753, 8, 56, 1, 56, 1, 56, 1, 57, 
		1, 57, 1, 57, 1, 57, 1, 58, 1, 58, 1, 58, 1, 58, 5, 58, 765, 8, 58, 10, 
		58, 12, 58, 768, 9, 58, 1, 58, 3, 58, 771, 8, 58, 3, 58, 773, 8, 58, 1, 
		58, 1, 58, 1, 59, 1, 59, 1, 59, 5, 59, 780, 8, 59, 10, 59, 12, 59, 783, 
		9, 59, 1, 59, 1, 59, 1, 60, 1, 60, 1, 60, 1, 60, 1, 61, 1, 61, 1, 61, 
		1, 61, 1, 61, 3, 61, 796, 8, 61, 1, 62, 1, 62, 1, 62, 1, 62, 5, 62, 802, 
		8, 62, 10, 62, 12, 62, 805, 9, 62, 1, 62, 3, 62, 808, 8, 62, 3, 62, 810, 
		8, 62, 1, 62, 1, 62, 1, 63, 1, 63, 1, 63, 1, 63, 1, 64, 1, 64, 1, 64, 
		1, 64, 5, 64, 822, 8, 64, 10, 64, 12, 64, 825, 9, 64, 1, 64, 3, 64, 828, 
		8, 64, 3, 64, 830, 8, 64, 1, 64, 1, 64, 1, 65, 1, 65, 1, 65, 5, 65, 837, 
		8, 65, 10, 65, 12, 65, 840, 9, 65, 1, 65, 1, 65, 1, 66, 1, 66, 1, 66, 
		5, 66, 847, 8, 66, 10, 66, 12, 66, 850, 9, 66, 1, 66, 1, 66, 1, 67, 1, 
		67, 1, 67, 1, 67, 5, 67, 858, 8, 67, 10, 67, 12, 67, 861, 9, 67, 1, 67, 
		3, 67, 864, 8, 67, 3, 67, 866, 8, 67, 1, 67, 1, 67, 1, 68, 1, 68, 1, 68, 
		1, 68, 1, 69, 1, 69, 1, 69, 1, 69, 5, 69, 878, 8, 69, 10, 69, 12, 69, 
		881, 9, 69, 1, 69, 3, 69, 884, 8, 69, 3, 69, 886, 8, 69, 1, 69, 1, 69, 
		1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 3, 70, 896, 8, 70, 1, 71, 1, 
		71, 1, 71, 3, 71, 901, 8, 71, 1, 72, 1, 72, 5, 72, 905, 8, 72, 10, 72, 
		12, 72, 908, 9, 72, 1, 72, 1, 72, 1, 73, 1, 73, 5, 73, 914, 8, 73, 10, 
		73, 12, 73, 917, 9, 73, 1, 73, 1, 73, 1, 74, 1, 74, 1, 74, 1, 74, 3, 74, 
		925, 8, 74, 1, 75, 1, 75, 1, 75, 1, 75, 1, 75, 1, 75, 5, 75, 933, 8, 75, 
		10, 75, 12, 75, 936, 9, 75, 1, 75, 1, 75, 5, 75, 940, 8, 75, 10, 75, 12, 
		75, 943, 9, 75, 1, 75, 3, 75, 946, 8, 75, 1, 76, 1, 76, 1, 76, 1, 76, 
		1, 76, 1, 76, 1, 76, 5, 76, 955, 8, 76, 10, 76, 12, 76, 958, 9, 76, 1, 
		76, 1, 76, 1, 77, 1, 77, 1, 77, 5, 77, 965, 8, 77, 10, 77, 12, 77, 968, 
		9, 77, 1, 77, 1, 77, 1, 78, 1, 78, 1, 78, 1, 78, 1, 78, 1, 78, 1, 78, 
		1, 78, 1, 79, 1, 79, 5, 79, 982, 8, 79, 10, 79, 12, 79, 985, 9, 79, 1, 
		79, 1, 79, 1, 80, 1, 80, 1, 80, 3, 80, 992, 8, 80, 1, 80, 5, 80, 995, 
		8, 80, 10, 80, 12, 80, 998, 9, 80, 1, 80, 3, 80, 1001, 8, 80, 1, 81, 1, 
		81, 1, 81, 5, 81, 1006, 8, 81, 10, 81, 12, 81, 1009, 9, 81, 1, 82, 1, 
		82, 1, 82, 1, 83, 1, 83, 1, 83, 1, 84, 1, 84, 1, 84, 3, 84, 1020, 8, 84, 
		1, 84, 1, 84, 1, 84, 5, 84, 1025, 8, 84, 10, 84, 12, 84, 1028, 9, 84, 
		1, 84, 3, 84, 1031, 8, 84, 3, 84, 1033, 8, 84, 1, 84, 1, 84, 1, 85, 1, 
		85, 1, 85, 3, 85, 1040, 8, 85, 1, 86, 1, 86, 1, 87, 1, 87, 1, 87, 1, 87, 
		1, 87, 3, 87, 1049, 8, 87, 1, 88, 1, 88, 1, 88, 1, 88, 1, 88, 3, 88, 1056, 
		8, 88, 1, 89, 1, 89, 1, 89, 1, 89, 1, 89, 3, 89, 1063, 8, 89, 1, 90, 1, 
		90, 1, 90, 1, 90, 1, 90, 3, 90, 1070, 8, 90, 1, 91, 1, 91, 1, 91, 1, 91, 
		1, 91, 3, 91, 1077, 8, 91, 1, 92, 1, 92, 1, 92, 1, 92, 1, 92, 3, 92, 1084, 
		8, 92, 1, 93, 1, 93, 1, 93, 1, 93, 1, 93, 3, 93, 1091, 8, 93, 1, 94, 1, 
		94, 1, 94, 3, 94, 1096, 8, 94, 1, 95, 1, 95, 1, 95, 1, 95, 1, 95, 1, 95, 
		1, 95, 1, 95, 1, 95, 1, 95, 1, 95, 5, 95, 1109, 8, 95, 10, 95, 12, 95, 
		1112, 9, 95, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 
		96, 1, 96, 1, 96, 1, 96, 1, 96, 3, 96, 1127, 8, 96, 1, 97, 1, 97, 1, 98, 
		1, 98, 1, 99, 1, 99, 1, 100, 1, 100, 1, 101, 1, 101, 3, 101, 1139, 8, 
		101, 1, 102, 3, 102, 1142, 8, 102, 1, 102, 1, 102, 1, 103, 1, 103, 1, 
		103, 1, 103, 5, 103, 1150, 8, 103, 10, 103, 12, 103, 1153, 9, 103, 1, 
		103, 3, 103, 1156, 8, 103, 3, 103, 1158, 8, 103, 1, 103, 1, 103, 1, 104, 
		1, 104, 1, 104, 1, 104, 5, 104, 1166, 8, 104, 10, 104, 12, 104, 1169, 
		9, 104, 1, 104, 3, 104, 1172, 8, 104, 3, 104, 1174, 8, 104, 1, 104, 1, 
		104, 1, 105, 1, 105, 1, 105, 1, 105, 1, 106, 1, 106, 1, 106, 1, 106, 1, 
		106, 5, 106, 1187, 8, 106, 10, 106, 12, 106, 1190, 9, 106, 1, 106, 3, 
		106, 1193, 8, 106, 3, 106, 1195, 8, 106, 1, 106, 1, 106, 1, 107, 1, 107, 
		1, 107, 1, 107, 1, 108, 1, 108, 1, 108, 1, 108, 1, 108, 5, 108, 1208, 
		8, 108, 10, 108, 12, 108, 1211, 9, 108, 1, 108, 3, 108, 1214, 8, 108, 
		3, 108, 1216, 8, 108, 1, 108, 1, 108, 1, 109, 1, 109, 1, 109, 1, 109, 
		1, 110, 1, 110, 1, 110, 1, 110, 1, 110, 1, 110, 1, 111, 1, 111, 1, 111, 
		1, 111, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 112, 1, 113, 
		1, 113, 1, 113, 1, 113, 1, 113, 5, 113, 1246, 8, 113, 10, 113, 12, 113, 
		1249, 9, 113, 1, 113, 3, 113, 1252, 8, 113, 3, 113, 1254, 8, 113, 1, 113, 
		1, 113, 1, 114, 1, 114, 3, 114, 1260, 8, 114, 1, 115, 1, 115, 5, 115, 
		1264, 8, 115, 10, 115, 12, 115, 1267, 9, 115, 1, 115, 1, 115, 1, 115, 
		5, 115, 1272, 8, 115, 10, 115, 12, 115, 1275, 9, 115, 1, 115, 3, 115, 
		1278, 8, 115, 1, 116, 1, 116, 1, 116, 1, 116, 1, 116, 3, 116, 1285, 8, 
		116, 1, 117, 1, 117, 1, 117, 1, 117, 1, 118, 1, 118, 5, 118, 1293, 8, 
		118, 10, 118, 12, 118, 1296, 9, 118, 1, 118, 1, 118, 1, 119, 1, 119, 1, 
		119, 1, 119, 1, 119, 1, 119, 1, 119, 3, 119, 1307, 8, 119, 1, 120, 1, 
		120, 1, 120, 1, 120, 1, 120, 1, 120, 1, 120, 1, 120, 3, 120, 1317, 8, 
		120, 1, 121, 5, 121, 1320, 8, 121, 10, 121, 12, 121, 1323, 9, 121, 1, 
		121, 1, 121, 1, 122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 
		122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 122, 1, 
		122, 3, 122, 1344, 8, 122, 1, 123, 1, 123, 1, 124, 1, 124, 1, 124, 5, 
		124, 1351, 8, 124, 10, 124, 12, 124, 1354, 9, 124, 1, 125, 1, 125, 1, 
		125, 0, 1, 190, 126, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 
		28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 
		64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98, 
		100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126, 
		128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 
		156, 158, 160, 162, 164, 166, 168, 170, 172, 174, 176, 178, 180, 182, 
		184, 186, 188, 190, 192, 194, 196, 198, 200, 202, 204, 206, 208, 210, 
		212, 214, 216, 218, 220, 222, 224, 226, 228, 230, 232, 234, 236, 238, 
		240, 242, 244, 246, 248, 250, 0, 11, 1, 0, 1, 2, 1, 0, 1, 4, 2, 0, 86, 
		86, 88, 92, 2, 0, 13, 17, 21, 21, 1, 0, 56, 57, 2, 0, 58, 59, 76, 77, 
		1, 0, 71, 72, 1, 0, 73, 75, 2, 0, 70, 70, 72, 72, 2, 0, 29, 29, 49, 49, 
		1, 0, 12, 52, 1437, 0, 252, 1, 0, 0, 0, 2, 261, 1, 0, 0, 0, 4, 269, 1, 
		0, 0, 0, 6, 292, 1, 0, 0, 0, 8, 294, 1, 0, 0, 0, 10, 310, 1, 0, 0, 0, 
		12, 331, 1, 0, 0, 0, 14, 333, 1, 0, 0, 0, 16, 335, 1, 0, 0, 0, 18, 340, 
		1, 0, 0, 0, 20, 354, 1, 0, 0, 0, 22, 356, 1, 0, 0, 0, 24, 359, 1, 0, 0, 
		0, 26, 380, 1, 0, 0, 0, 28, 384, 1, 0, 0, 0, 30, 397, 1, 0, 0, 0, 32, 
		401, 1, 0, 0, 0, 34, 419, 1, 0, 0, 0, 36, 421, 1, 0, 0, 0, 38, 423, 1, 
		0, 0, 0, 40, 432, 1, 0, 0, 0, 42, 434, 1, 0, 0, 0, 44, 450, 1, 0, 0, 0, 
		46, 466, 1, 0, 0, 0, 48, 470, 1, 0, 0, 0, 50, 487, 1, 0, 0, 0, 52, 491, 
		1, 0, 0, 0, 54, 508, 1, 0, 0, 0, 56, 512, 1, 0, 0, 0, 58, 518, 1, 0, 0, 
		0, 60, 529, 1, 0, 0, 0, 62, 546, 1, 0, 0, 0, 64, 548, 1, 0, 0, 0, 66, 
		557, 1, 0, 0, 0, 68, 567, 1, 0, 0, 0, 70, 576, 1, 0, 0, 0, 72, 580, 1, 
		0, 0, 0, 74, 584, 1, 0, 0, 0, 76, 589, 1, 0, 0, 0, 78, 595, 1, 0, 0, 0, 
		80, 604, 1, 0, 0, 0, 82, 615, 1, 0, 0, 0, 84, 626, 1, 0, 0, 0, 86, 628, 
		1, 0, 0, 0, 88, 638, 1, 0, 0, 0, 90, 648, 1, 0, 0, 0, 92, 658, 1, 0, 0, 
		0, 94, 662, 1, 0, 0, 0, 96, 672, 1, 0, 0, 0, 98, 676, 1, 0, 0, 0, 100, 
		686, 1, 0, 0, 0, 102, 695, 1, 0, 0, 0, 104, 697, 1, 0, 0, 0, 106, 714, 
		1, 0, 0, 0, 108, 718, 1, 0, 0, 0, 110, 735, 1, 0, 0, 0, 112, 739, 1, 0, 
		0, 0, 114, 756, 1, 0, 0, 0, 116, 760, 1, 0, 0, 0, 118, 776, 1, 0, 0, 0, 
		120, 786, 1, 0, 0, 0, 122, 795, 1, 0, 0, 0, 124, 797, 1, 0, 0, 0, 126, 
		813, 1, 0, 0, 0, 128, 817, 1, 0, 0, 0, 130, 833, 1, 0, 0, 0, 132, 843, 
		1, 0, 0, 0, 134, 853, 1, 0, 0, 0, 136, 869, 1, 0, 0, 0, 138, 873, 1, 0, 
		0, 0, 140, 895, 1, 0, 0, 0, 142, 897, 1, 0, 0, 0, 144, 902, 1, 0, 0, 0, 
		146, 911, 1, 0, 0, 0, 148, 924, 1, 0, 0, 0, 150, 926, 1, 0, 0, 0, 152, 
		947, 1, 0, 0, 0, 154, 961, 1, 0, 0, 0, 156, 971, 1, 0, 0, 0, 158, 979, 
		1, 0, 0, 0, 160, 988, 1, 0, 0, 0, 162, 1002, 1, 0, 0, 0, 164, 1010, 1, 
		0, 0, 0, 166, 1013, 1, 0, 0, 0, 168, 1016, 1, 0, 0, 0, 170, 1036, 1, 0, 
		0, 0, 172, 1041, 1, 0, 0, 0, 174, 1048, 1, 0, 0, 0, 176, 1055, 1, 0, 0, 
		0, 178, 1062, 1, 0, 0, 0, 180, 1069, 1, 0, 0, 0, 182, 1076, 1, 0, 0, 0, 
		184, 1083, 1, 0, 0, 0, 186, 1090, 1, 0, 0, 0, 188, 1095, 1, 0, 0, 0, 190, 
		1097, 1, 0, 0, 0, 192, 1126, 1, 0, 0, 0, 194, 1128, 1, 0, 0, 0, 196, 1130, 
		1, 0, 0, 0, 198, 1132, 1, 0, 0, 0, 200, 1134, 1, 0, 0, 0, 202, 1138, 1, 
		0, 0, 0, 204, 1141, 1, 0, 0, 0, 206, 1145, 1, 0, 0, 0, 208, 1161, 1, 0, 
		0, 0, 210, 1177, 1, 0, 0, 0, 212, 1181, 1, 0, 0, 0, 214, 1198, 1, 0, 0, 
		0, 216, 1202, 1, 0, 0, 0, 218, 1219, 1, 0, 0, 0, 220, 1223, 1, 0, 0, 0, 
		222, 1229, 1, 0, 0, 0, 224, 1233, 1, 0, 0, 0, 226, 1240, 1, 0, 0, 0, 228, 
		1259, 1, 0, 0, 0, 230, 1277, 1, 0, 0, 0, 232, 1284, 1, 0, 0, 0, 234, 1286, 
		1, 0, 0, 0, 236, 1290, 1, 0, 0, 0, 238, 1306, 1, 0, 0, 0, 240, 1316, 1, 
		0, 0, 0, 242, 1321, 1, 0, 0, 0, 244, 1343, 1, 0, 0, 0, 246, 1345, 1, 0, 
		0, 0, 248, 1347, 1, 0, 0, 0, 250, 1355, 1, 0, 0, 0, 252, 256, 3, 2, 1, 
		0, 253, 255, 3, 4, 2, 0, 254, 253, 1, 0, 0, 0, 255, 258, 1, 0, 0, 0, 256, 
		254, 1, 0, 0, 0, 256, 257, 1, 0, 0, 0, 257, 259, 1, 0, 0, 0, 258, 256, 
		1, 0, 0, 0, 259, 260, 5, 0, 0, 1, 260, 1, 1, 0, 0, 0, 261, 262, 5, 50, 
		0, 0, 262, 263, 5, 6, 0, 0, 263, 3, 1, 0, 0, 0, 264, 270, 3, 6, 3, 0, 
		265, 270, 3, 18, 9, 0, 266, 270, 3, 24, 12, 0, 267, 270, 3, 58, 29, 0, 
		268, 270, 3, 60, 30, 0, 269, 264, 1, 0, 0, 0, 269, 265, 1, 0, 0, 0, 269, 
		266, 1, 0, 0, 0, 269, 267, 1, 0, 0, 0, 269, 268, 1, 0, 0, 0, 270, 5, 1, 
		0, 0, 0, 271, 272, 5, 34, 0, 0, 272, 275, 3, 12, 6, 0, 273, 274, 5, 24, 
		0, 0, 274, 276, 3, 246, 123, 0, 275, 273, 1, 0, 0, 0, 275, 276, 1, 0, 
		0, 0, 276, 280, 1, 0, 0, 0, 277, 279, 3, 16, 8, 0, 278, 277, 1, 0, 0, 
		0, 279, 282, 1, 0, 0, 0, 280, 278, 1, 0, 0, 0, 280, 281, 1, 0, 0, 0, 281, 
		293, 1, 0, 0, 0, 282, 280, 1, 0, 0, 0, 283, 284, 5, 34, 0, 0, 284, 285, 
		5, 73, 0, 0, 285, 286, 5, 30, 0, 0, 286, 293, 3, 12, 6, 0, 287, 288, 5, 
		34, 0, 0, 288, 289, 3, 8, 4, 0, 289, 290, 5, 30, 0, 0, 290, 291, 3, 12, 
		6, 0, 291, 293, 1, 0, 0, 0, 292, 271, 1, 0, 0, 0, 292, 283, 1, 0, 0, 0, 
		292, 287, 1, 0, 0, 0, 293, 7, 1, 0, 0, 0, 294, 306, 5, 60, 0, 0, 295, 
		300, 3, 10, 5, 0, 296, 297, 5, 66, 0, 0, 297, 299, 3, 10, 5, 0, 298, 296, 
		1, 0, 0, 0, 299, 302, 1, 0, 0, 0, 300, 298, 1, 0, 0, 0, 300, 301, 1, 0, 
		0, 0, 301, 304, 1, 0, 0, 0, 302, 300, 1, 0, 0, 0, 303, 305, 5, 66, 0, 
		0, 304, 303, 1, 0, 0, 0, 304, 305, 1, 0, 0, 0, 305, 307, 1, 0, 0, 0, 306, 
		295, 1, 0, 0, 0, 306, 307, 1, 0, 0, 0, 307, 308, 1, 0, 0, 0, 308, 309, 
		5, 61, 0, 0, 309, 9, 1, 0, 0, 0, 310, 313, 3, 246, 123, 0, 311, 312, 5, 
		24, 0, 0, 312, 314, 3, 246, 123, 0, 313, 311, 1, 0, 0, 0, 313, 314, 1, 
		0, 0, 0, 314, 11, 1, 0, 0, 0, 315, 319, 5, 10, 0, 0, 316, 318, 3, 14, 
		7, 0, 317, 316, 1, 0, 0, 0, 318, 321, 1, 0, 0, 0, 319, 317, 1, 0, 0, 0, 
		319, 320, 1, 0, 0, 0, 320, 322, 1, 0, 0, 0, 321, 319, 1, 0, 0, 0, 322, 
		332, 5, 82, 0, 0, 323, 327, 5, 11, 0, 0, 324, 326, 3, 14, 7, 0, 325, 324, 
		1, 0, 0, 0, 326, 329, 1, 0, 0, 0, 327, 325, 1, 0, 0, 0, 327, 328, 1, 0, 
		0, 0, 328, 330, 1, 0, 0, 0, 329, 327, 1, 0, 0, 0, 330, 332, 5, 83, 0, 
		0, 331, 315, 1, 0, 0, 0, 331, 323, 1, 0, 0, 0, 332, 13, 1, 0, 0, 0, 333, 
		334, 7, 0, 0, 0, 334, 15, 1, 0, 0, 0, 335, 336, 5, 23, 0, 0, 336, 337, 
		3, 246, 123, 0, 337, 338, 5, 24, 0, 0, 338, 339, 3, 246, 123, 0, 339, 
		17, 1, 0, 0, 0, 340, 341, 5, 45, 0, 0, 341, 342, 3, 246, 123, 0, 342, 
		346, 5, 60, 0, 0, 343, 345, 3, 20, 10, 0, 344, 343, 1, 0, 0, 0, 345, 348, 
		1, 0, 0, 0, 346, 344, 1, 0, 0, 0, 346, 347, 1, 0, 0, 0, 347, 349, 1, 0, 
		0, 0, 348, 346, 1, 0, 0, 0, 349, 350, 5, 61, 0, 0, 350, 19, 1, 0, 0, 0, 
		351, 355, 3, 130, 65, 0, 352, 355, 3, 132, 66, 0, 353, 355, 3, 22, 11, 
		0, 354, 351, 1, 0, 0, 0, 354, 352, 1, 0, 0, 0, 354, 353, 1, 0, 0, 0, 355, 
		21, 1, 0, 0, 0, 356, 357, 3, 62, 31, 0, 357, 358, 3, 246, 123, 0, 358, 
		23, 1, 0, 0, 0, 359, 360, 5, 46, 0, 0, 360, 362, 3, 246, 123, 0, 361, 
		363, 3, 26, 13, 0, 362, 361, 1, 0, 0, 0, 362, 363, 1, 0, 0, 0, 363, 364, 
		1, 0, 0, 0, 364, 376, 5, 60, 0, 0, 365, 370, 3, 28, 14, 0, 366, 367, 5, 
		66, 0, 0, 367, 369, 3, 28, 14, 0, 368, 366, 1, 0, 0, 0, 369, 372, 1, 0, 
		0, 0, 370, 368, 1, 0, 0, 0, 370, 371, 1, 0, 0, 0, 371, 374, 1, 0, 0, 0, 
		372, 370, 1, 0, 0, 0, 373, 375, 5, 66, 0, 0, 374, 373, 1, 0, 0, 0, 374, 
		375, 1, 0, 0, 0, 375, 377, 1, 0, 0, 0, 376, 365, 1, 0, 0, 0, 376, 377, 
		1, 0, 0, 0, 377, 378, 1, 0, 0, 0, 378, 379, 5, 61, 0, 0, 379, 25, 1, 0, 
		0, 0, 380, 381, 5, 62, 0, 0, 381, 382, 3, 62, 31, 0, 382, 383, 5, 63, 
		0, 0, 383, 27, 1, 0, 0, 0, 384, 387, 3, 246, 123, 0, 385, 386, 5, 64, 
		0, 0, 386, 388, 3, 30, 15, 0, 387, 385, 1, 0, 0, 0, 387, 388, 1, 0, 0, 
		0, 388, 29, 1, 0, 0, 0, 389, 398, 3, 204, 102, 0, 390, 398, 3, 200, 100, 
		0, 391, 398, 3, 32, 16, 0, 392, 398, 3, 42, 21, 0, 393, 398, 3, 44, 22, 
		0, 394, 398, 3, 48, 24, 0, 395, 398, 3, 52, 26, 0, 396, 398, 3, 56, 28, 
		0, 397, 389, 1, 0, 0, 0, 397, 390, 1, 0, 0, 0, 397, 391, 1, 0, 0, 0, 397, 
		392, 1, 0, 0, 0, 397, 393, 1, 0, 0, 0, 397, 394, 1, 0, 0, 0, 397, 395, 
		1, 0, 0, 0, 397, 396, 1, 0, 0, 0, 398, 31, 1, 0, 0, 0, 399, 402, 3, 34, 
		17, 0, 400, 402, 3, 38, 19, 0, 401, 399, 1, 0, 0, 0, 401, 400, 1, 0, 0, 
		0, 402, 33, 1, 0, 0, 0, 403, 407, 5, 10, 0, 0, 404, 406, 3, 36, 18, 0, 
		405, 404, 1, 0, 0, 0, 406, 409, 1, 0, 0, 0, 407, 405, 1, 0, 0, 0, 407, 
		408, 1, 0, 0, 0, 408, 410, 1, 0, 0, 0, 409, 407, 1, 0, 0, 0, 410, 420, 
		5, 82, 0, 0, 411, 415, 5, 11, 0, 0, 412, 414, 3, 36, 18, 0, 413, 412, 
		1, 0, 0, 0, 414, 417, 1, 0, 0, 0, 415, 413, 1, 0, 0, 0, 415, 416, 1, 0, 
		0, 0, 416, 418, 1, 0, 0, 0, 417, 415, 1, 0, 0, 0, 418, 420, 5, 83, 0, 
		0, 419, 403, 1, 0, 0, 0, 419, 411, 1, 0, 0, 0, 420, 35, 1, 0, 0, 0, 421, 
		422, 7, 1, 0, 0, 422, 37, 1, 0, 0, 0, 423, 427, 5, 8, 0, 0, 424, 426, 
		3, 40, 20, 0, 425, 424, 1, 0, 0, 0, 426, 429, 1, 0, 0, 0, 427, 425, 1, 
		0, 0, 0, 427, 428, 1, 0, 0, 0, 428, 430, 1, 0, 0, 0, 429, 427, 1, 0, 0, 
		0, 430, 431, 5, 87, 0, 0, 431, 39, 1, 0, 0, 0, 432, 433, 7, 2, 0, 0, 433, 
		41, 1, 0, 0, 0, 434, 446, 5, 62, 0, 0, 435, 440, 3, 30, 15, 0, 436, 437, 
		5, 66, 0, 0, 437, 439, 3, 30, 15, 0, 438, 436, 1, 0, 0, 0, 439, 442, 1, 
		0, 0, 0, 440, 438, 1, 0, 0, 0, 440, 441, 1, 0, 0, 0, 441, 444, 1, 0, 0, 
		0, 442, 440, 1, 0, 0, 0, 443, 445, 5, 66, 0, 0, 444, 443, 1, 0, 0, 0, 
		444, 445, 1, 0, 0, 0, 445, 447, 1, 0, 0, 0, 446, 435, 1, 0, 0, 0, 446, 
		447, 1, 0, 0, 0, 447, 448, 1, 0, 0, 0, 448, 449, 5, 63, 0, 0, 449, 43, 
		1, 0, 0, 0, 450, 462, 5, 60, 0, 0, 451, 456, 3, 46, 23, 0, 452, 453, 5, 
		66, 0, 0, 453, 455, 3, 46, 23, 0, 454, 452, 1, 0, 0, 0, 455, 458, 1, 0, 
		0, 0, 456, 454, 1, 0, 0, 0, 456, 457, 1, 0, 0, 0, 457, 460, 1, 0, 0, 0, 
		458, 456, 1, 0, 0, 0, 459, 461, 5, 66, 0, 0, 460, 459, 1, 0, 0, 0, 460, 
		461, 1, 0, 0, 0, 461, 463, 1, 0, 0, 0, 462, 451, 1, 0, 0, 0, 462, 463, 
		1, 0, 0, 0, 463, 464, 1, 0, 0, 0, 464, 465, 5, 61, 0, 0, 465, 45, 1, 0, 
		0, 0, 466, 467, 3, 30, 15, 0, 467, 468, 5, 65, 0, 0, 468, 469, 3, 30, 
		15, 0, 469, 47, 1, 0, 0, 0, 470, 471, 5, 39, 0, 0, 471, 483, 5, 60, 0, 
		0, 472, 477, 3, 50, 25, 0, 473, 474, 5, 66, 0, 0, 474, 476, 3, 50, 25, 
		0, 475, 473, 1, 0, 0, 0, 476, 479, 1, 0, 0, 0, 477, 475, 1, 0, 0, 0, 477, 
		478, 1, 0, 0, 0, 478, 481, 1, 0, 0, 0, 479, 477, 1, 0, 0, 0, 480, 482, 
		5, 66, 0, 0, 481, 480, 1, 0, 0, 0, 481, 482, 1, 0, 0, 0, 482, 484, 1, 
		0, 0, 0, 483, 472, 1, 0, 0, 0, 483, 484, 1, 0, 0, 0, 484, 485, 1, 0, 0, 
		0, 485, 486, 5, 61, 0, 0, 486, 49, 1, 0, 0, 0, 487, 488, 3, 246, 123, 
		0, 488, 489, 5, 65, 0, 0, 489, 490, 3, 30, 15, 0, 490, 51, 1, 0, 0, 0, 
		491, 492, 3, 246, 123, 0, 492, 504, 5, 60, 0, 0, 493, 498, 3, 54, 27, 
		0, 494, 495, 5, 66, 0, 0, 495, 497, 3, 54, 27, 0, 496, 494, 1, 0, 0, 0, 
		497, 500, 1, 0, 0, 0, 498, 496, 1, 0, 0, 0, 498, 499, 1, 0, 0, 0, 499, 
		502, 1, 0, 0, 0, 500, 498, 1, 0, 0, 0, 501, 503, 5, 66, 0, 0, 502, 501, 
		1, 0, 0, 0, 502, 503, 1, 0, 0, 0, 503, 505, 1, 0, 0, 0, 504, 493, 1, 0, 
		0, 0, 504, 505, 1, 0, 0, 0, 505, 506, 1, 0, 0, 0, 506, 507, 5, 61, 0, 
		0, 507, 53, 1, 0, 0, 0, 508, 509, 3, 246, 123, 0, 509, 510, 5, 65, 0, 
		0, 510, 511, 3, 30, 15, 0, 511, 55, 1, 0, 0, 0, 512, 513, 5, 67, 0, 0, 
		513, 514, 3, 30, 15, 0, 514, 515, 5, 66, 0, 0, 515, 516, 3, 30, 15, 0, 
		516, 517, 5, 68, 0, 0, 517, 57, 1, 0, 0, 0, 518, 519, 5, 47, 0, 0, 519, 
		520, 3, 246, 123, 0, 520, 524, 5, 60, 0, 0, 521, 523, 3, 82, 41, 0, 522, 
		521, 1, 0, 0, 0, 523, 526, 1, 0, 0, 0, 524, 522, 1, 0, 0, 0, 524, 525, 
		1, 0, 0, 0, 525, 527, 1, 0, 0, 0, 526, 524, 1, 0, 0, 0, 527, 528, 5, 61, 
		0, 0, 528, 59, 1, 0, 0, 0, 529, 530, 5, 51, 0, 0, 530, 531, 3, 246, 123, 
		0, 531, 535, 5, 60, 0, 0, 532, 534, 3, 84, 42, 0, 533, 532, 1, 0, 0, 0, 
		534, 537, 1, 0, 0, 0, 535, 533, 1, 0, 0, 0, 535, 536, 1, 0, 0, 0, 536, 
		538, 1, 0, 0, 0, 537, 535, 1, 0, 0, 0, 538, 539, 5, 61, 0, 0, 539, 61, 
		1, 0, 0, 0, 540, 547, 3, 64, 32, 0, 541, 547, 3, 66, 33, 0, 542, 547, 
		3, 68, 34, 0, 543, 547, 3, 70, 35, 0, 544, 547, 3, 72, 36, 0, 545, 547, 
		3, 74, 37, 0, 546, 540, 1, 0, 0, 0, 546, 541, 1, 0, 0, 0, 546, 542, 1, 
		0, 0, 0, 546, 543, 1, 0, 0, 0, 546, 544, 1, 0, 0, 0, 546, 545, 1, 0, 0, 
		0, 547, 63, 1, 0, 0, 0, 548, 549, 5, 18, 0, 0, 549, 550, 5, 62, 0, 0, 
		550, 551, 3, 72, 36, 0, 551, 552, 5, 66, 0, 0, 552, 553, 3, 62, 31, 0, 
		553, 555, 5, 63, 0, 0, 554, 556, 5, 69, 0, 0, 555, 554, 1, 0, 0, 0, 555, 
		556, 1, 0, 0, 0, 556, 65, 1, 0, 0, 0, 557, 558, 5, 12, 0, 0, 558, 559, 
		5, 62, 0, 0, 559, 560, 3, 62, 31, 0, 560, 562, 5, 63, 0, 0, 561, 563, 
		5, 71, 0, 0, 562, 561, 1, 0, 0, 0, 562, 563, 1, 0, 0, 0, 563, 565, 1, 
		0, 0, 0, 564, 566, 5, 69, 0, 0, 565, 564, 1, 0, 0, 0, 565, 566, 1, 0, 
		0, 0, 566, 67, 1, 0, 0, 0, 567, 568, 5, 20, 0, 0, 568, 569, 5, 62, 0, 
		0, 569, 570, 3, 62, 31, 0, 570, 571, 5, 66, 0, 0, 571, 572, 3, 62, 31, 
		0, 572, 574, 5, 63, 0, 0, 573, 575, 5, 69, 0, 0, 574, 573, 1, 0, 0, 0, 
		574, 575, 1, 0, 0, 0, 575, 69, 1, 0, 0, 0, 576, 578, 5, 19, 0, 0, 577, 
		579, 5, 69, 0, 0, 578, 577, 1, 0, 0, 0, 578, 579, 1, 0, 0, 0, 579, 71, 
		1, 0, 0, 0, 580, 582, 7, 3, 0, 0, 581, 583, 5, 69, 0, 0, 582, 581, 1, 
		0, 0, 0, 582, 583, 1, 0, 0, 0, 583, 73, 1, 0, 0, 0, 584, 586, 3, 246, 
		123, 0, 585, 587, 5, 69, 0, 0, 586, 585, 1, 0, 0, 0, 586, 587, 1, 0, 0, 
		0, 587, 75, 1, 0, 0, 0, 588, 590, 5, 28, 0, 0, 589, 588, 1, 0, 0, 0, 589, 
		590, 1, 0, 0, 0, 590, 591, 1, 0, 0, 0, 591, 592, 3, 62, 31, 0, 592, 593, 
		3, 246, 123, 0, 593, 77, 1, 0, 0, 0, 594, 596, 5, 28, 0, 0, 595, 594, 
		1, 0, 0, 0, 595, 596, 1, 0, 0, 0, 596, 597, 1, 0, 0, 0, 597, 598, 3, 62, 
		31, 0, 598, 599, 3, 246, 123, 0, 599, 600, 5, 64, 0, 0, 600, 601, 3, 172, 
		86, 0, 601, 79, 1, 0, 0, 0, 602, 605, 3, 76, 38, 0, 603, 605, 3, 78, 39, 
		0, 604, 602, 1, 0, 0, 0, 604, 603, 1, 0, 0, 0, 605, 81, 1, 0, 0, 0, 606, 
		616, 3, 86, 43, 0, 607, 616, 3, 142, 71, 0, 608, 616, 3, 88, 44, 0, 609, 
		616, 3, 90, 45, 0, 610, 616, 3, 94, 47, 0, 611, 616, 3, 98, 49, 0, 612, 
		616, 3, 130, 65, 0, 613, 616, 3, 132, 66, 0, 614, 616, 3, 78, 39, 0, 615, 
		606, 1, 0, 0, 0, 615, 607, 1, 0, 0, 0, 615, 608, 1, 0, 0, 0, 615, 609, 
		1, 0, 0, 0, 615, 610, 1, 0, 0, 0, 615, 611, 1, 0, 0, 0, 615, 612, 1, 0, 
		0, 0, 615, 613, 1, 0, 0, 0, 615, 614, 1, 0, 0, 0, 616, 83, 1, 0, 0, 0, 
		617, 627, 3, 86, 43, 0, 618, 627, 3, 88, 44, 0, 619, 627, 3, 118, 59, 
		0, 620, 627, 3, 150, 75, 0, 621, 627, 3, 156, 78, 0, 622, 627, 3, 160, 
		80, 0, 623, 627, 3, 130, 65, 0, 624, 627, 3, 132, 66, 0, 625, 627, 3, 
		78, 39, 0, 626, 617, 1, 0, 0, 0, 626, 618, 1, 0, 0, 0, 626, 619, 1, 0, 
		0, 0, 626, 620, 1, 0, 0, 0, 626, 621, 1, 0, 0, 0, 626, 622, 1, 0, 0, 0, 
		626, 623, 1, 0, 0, 0, 626, 624, 1, 0, 0, 0, 626, 625, 1, 0, 0, 0, 627, 
		85, 1, 0, 0, 0, 628, 629, 5, 35, 0, 0, 629, 633, 5, 60, 0, 0, 630, 632, 
		3, 80, 40, 0, 631, 630, 1, 0, 0, 0, 632, 635, 1, 0, 0, 0, 633, 631, 1, 
		0, 0, 0, 633, 634, 1, 0, 0, 0, 634, 636, 1, 0, 0, 0, 635, 633, 1, 0, 0, 
		0, 636, 637, 5, 61, 0, 0, 637, 87, 1, 0, 0, 0, 638, 639, 5, 40, 0, 0, 
		639, 643, 5, 60, 0, 0, 640, 642, 3, 78, 39, 0, 641, 640, 1, 0, 0, 0, 642, 
		645, 1, 0, 0, 0, 643, 641, 1, 0, 0, 0, 643, 644, 1, 0, 0, 0, 644, 646, 
		1, 0, 0, 0, 645, 643, 1, 0, 0, 0, 646, 647, 5, 61, 0, 0, 647, 89, 1, 0, 
		0, 0, 648, 649, 5, 43, 0, 0, 649, 653, 5, 60, 0, 0, 650, 652, 3, 92, 46, 
		0, 651, 650, 1, 0, 0, 0, 652, 655, 1, 0, 0, 0, 653, 651, 1, 0, 0, 0, 653, 
		654, 1, 0, 0, 0, 654, 656, 1, 0, 0, 0, 655, 653, 1, 0, 0, 0, 656, 657, 
		5, 61, 0, 0, 657, 91, 1, 0, 0, 0, 658, 659, 3, 246, 123, 0, 659, 660, 
		5, 65, 0, 0, 660, 661, 3, 172, 86, 0, 661, 93, 1, 0, 0, 0, 662, 663, 5, 
		42, 0, 0, 663, 667, 5, 60, 0, 0, 664, 666, 3, 96, 48, 0, 665, 664, 1, 
		0, 0, 0, 666, 669, 1, 0, 0, 0, 667, 665, 1, 0, 0, 0, 667, 668, 1, 0, 0, 
		0, 668, 670, 1, 0, 0, 0, 669, 667, 1, 0, 0, 0, 670, 671, 5, 61, 0, 0, 
		671, 95, 1, 0, 0, 0, 672, 673, 3, 246, 123, 0, 673, 674, 5, 65, 0, 0, 
		674, 675, 3, 172, 86, 0, 675, 97, 1, 0, 0, 0, 676, 677, 5, 31, 0, 0, 677, 
		681, 5, 60, 0, 0, 678, 680, 3, 100, 50, 0, 679, 678, 1, 0, 0, 0, 680, 
		683, 1, 0, 0, 0, 681, 679, 1, 0, 0, 0, 681, 682, 1, 0, 0, 0, 682, 684, 
		1, 0, 0, 0, 683, 681, 1, 0, 0, 0, 684, 685, 5, 61, 0, 0, 685, 99, 1, 0, 
		0, 0, 686, 687, 3, 246, 123, 0, 687, 688, 5, 65, 0, 0, 688, 689, 3, 102, 
		51, 0, 689, 101, 1, 0, 0, 0, 690, 696, 3, 172, 86, 0, 691, 696, 3, 104, 
		52, 0, 692, 696, 3, 108, 54, 0, 693, 696, 3, 112, 56, 0, 694, 696, 3, 
		116, 58, 0, 695, 690, 1, 0, 0, 0, 695, 691, 1, 0, 0, 0, 695, 692, 1, 0, 
		0, 0, 695, 693, 1, 0, 0, 0, 695, 694, 1, 0, 0, 0, 696, 103, 1, 0, 0, 0, 
		697, 698, 5, 31, 0, 0, 698, 710, 5, 60, 0, 0, 699, 704, 3, 106, 53, 0, 
		700, 701, 5, 66, 0, 0, 701, 703, 3, 106, 53, 0, 702, 700, 1, 0, 0, 0, 
		703, 706, 1, 0, 0, 0, 704, 702, 1, 0, 0, 0, 704, 705, 1, 0, 0, 0, 705, 
		708, 1, 0, 0, 0, 706, 704, 1, 0, 0, 0, 707, 709, 5, 66, 0, 0, 708, 707, 
		1, 0, 0, 0, 708, 709, 1, 0, 0, 0, 709, 711, 1, 0, 0, 0, 710, 699, 1, 0, 
		0, 0, 710, 711, 1, 0, 0, 0, 711, 712, 1, 0, 0, 0, 712, 713, 5, 61, 0, 
		0, 713, 105, 1, 0, 0, 0, 714, 715, 3, 248, 124, 0, 715, 716, 5, 65, 0, 
		0, 716, 717, 3, 102, 51, 0, 717, 107, 1, 0, 0, 0, 718, 719, 5, 35, 0, 
		0, 719, 731, 5, 60, 0, 0, 720, 725, 3, 110, 55, 0, 721, 722, 5, 66, 0, 
		0, 722, 724, 3, 110, 55, 0, 723, 721, 1, 0, 0, 0, 724, 727, 1, 0, 0, 0, 
		725, 723, 1, 0, 0, 0, 725, 726, 1, 0, 0, 0, 726, 729, 1, 0, 0, 0, 727, 
		725, 1, 0, 0, 0, 728, 730, 5, 66, 0, 0, 729, 728, 1, 0, 0, 0, 729, 730, 
		1, 0, 0, 0, 730, 732, 1, 0, 0, 0, 731, 720, 1, 0, 0, 0, 731, 732, 1, 0, 
		0, 0, 732, 733, 1, 0, 0, 0, 733, 734, 5, 61, 0, 0, 734, 109, 1, 0, 0, 
		0, 735, 736, 3, 248, 124, 0, 736, 737, 5, 65, 0, 0, 737, 738, 3, 104, 
		52, 0, 738, 111, 1, 0, 0, 0, 739, 740, 5, 40, 0, 0, 740, 752, 5, 60, 0, 
		0, 741, 746, 3, 114, 57, 0, 742, 743, 5, 66, 0, 0, 743, 745, 3, 114, 57, 
		0, 744, 742, 1, 0, 0, 0, 745, 748, 1, 0, 0, 0, 746, 744, 1, 0, 0, 0, 746, 
		747, 1, 0, 0, 0, 747, 750, 1, 0, 0, 0, 748, 746, 1, 0, 0, 0, 749, 751, 
		5, 66, 0, 0, 750, 749, 1, 0, 0, 0, 750, 751, 1, 0, 0, 0, 751, 753, 1, 
		0, 0, 0, 752, 741, 1, 0, 0, 0, 752, 753, 1, 0, 0, 0, 753, 754, 1, 0, 0, 
		0, 754, 755, 5, 61, 0, 0, 755, 113, 1, 0, 0, 0, 756, 757, 3, 248, 124, 
		0, 757, 758, 5, 65, 0, 0, 758, 759, 3, 104, 52, 0, 759, 115, 1, 0, 0, 
		0, 760, 772, 5, 62, 0, 0, 761, 766, 3, 102, 51, 0, 762, 763, 5, 66, 0, 
		0, 763, 765, 3, 102, 51, 0, 764, 762, 1, 0, 0, 0, 765, 768, 1, 0, 0, 0, 
		766, 764, 1, 0, 0, 0, 766, 767, 1, 0, 0, 0, 767, 770, 1, 0, 0, 0, 768, 
		766, 1, 0, 0, 0, 769, 771, 5, 66, 0, 0, 770, 769, 1, 0, 0, 0, 770, 771, 
		1, 0, 0, 0, 771, 773, 1, 0, 0, 0, 772, 761, 1, 0, 0, 0, 772, 773, 1, 0, 
		0, 0, 773, 774, 1, 0, 0, 0, 774, 775, 5, 63, 0, 0, 775, 117, 1, 0, 0, 
		0, 776, 777, 5, 31, 0, 0, 777, 781, 5, 60, 0, 0, 778, 780, 3, 120, 60, 
		0, 779, 778, 1, 0, 0, 0, 780, 783, 1, 0, 0, 0, 781, 779, 1, 0, 0, 0, 781, 
		782, 1, 0, 0, 0, 782, 784, 1, 0, 0, 0, 783, 781, 1, 0, 0, 0, 784, 785, 
		5, 61, 0, 0, 785, 119, 1, 0, 0, 0, 786, 787, 3, 246, 123, 0, 787, 788, 
		5, 65, 0, 0, 788, 789, 3, 122, 61, 0, 789, 121, 1, 0, 0, 0, 790, 796, 
		3, 204, 102, 0, 791, 796, 3, 228, 114, 0, 792, 796, 3, 200, 100, 0, 793, 
		796, 3, 124, 62, 0, 794, 796, 3, 128, 64, 0, 795, 790, 1, 0, 0, 0, 795, 
		791, 1, 0, 0, 0, 795, 792, 1, 0, 0, 0, 795, 793, 1, 0, 0, 0, 795, 794, 
		1, 0, 0, 0, 796, 123, 1, 0, 0, 0, 797, 809, 5, 60, 0, 0, 798, 803, 3, 
		126, 63, 0, 799, 800, 5, 66, 0, 0, 800, 802, 3, 126, 63, 0, 801, 799, 
		1, 0, 0, 0, 802, 805, 1, 0, 0, 0, 803, 801, 1, 0, 0, 0, 803, 804, 1, 0, 
		0, 0, 804, 807, 1, 0, 0, 0, 805, 803, 1, 0, 0, 0, 806, 808, 5, 66, 0, 
		0, 807, 806, 1, 0, 0, 0, 807, 808, 1, 0, 0, 0, 808, 810, 1, 0, 0, 0, 809, 
		798, 1, 0, 0, 0, 809, 810, 1, 0, 0, 0, 810, 811, 1, 0, 0, 0, 811, 812, 
		5, 61, 0, 0, 812, 125, 1, 0, 0, 0, 813, 814, 3, 248, 124, 0, 814, 815, 
		5, 65, 0, 0, 815, 816, 3, 122, 61, 0, 816, 127, 1, 0, 0, 0, 817, 829, 
		5, 62, 0, 0, 818, 823, 3, 122, 61, 0, 819, 820, 5, 66, 0, 0, 820, 822, 
		3, 122, 61, 0, 821, 819, 1, 0, 0, 0, 822, 825, 1, 0, 0, 0, 823, 821, 1, 
		0, 0, 0, 823, 824, 1, 0, 0, 0, 824, 827, 1, 0, 0, 0, 825, 823, 1, 0, 0, 
		0, 826, 828, 5, 66, 0, 0, 827, 826, 1, 0, 0, 0, 827, 828, 1, 0, 0, 0, 
		828, 830, 1, 0, 0, 0, 829, 818, 1, 0, 0, 0, 829, 830, 1, 0, 0, 0, 830, 
		831, 1, 0, 0, 0, 831, 832, 5, 63, 0, 0, 832, 129, 1, 0, 0, 0, 833, 834, 
		5, 36, 0, 0, 834, 838, 5, 60, 0, 0, 835, 837, 3, 136, 68, 0, 836, 835, 
		1, 0, 0, 0, 837, 840, 1, 0, 0, 0, 838, 836, 1, 0, 0, 0, 838, 839, 1, 0, 
		0, 0, 839, 841, 1, 0, 0, 0, 840, 838, 1, 0, 0, 0, 841, 842, 5, 61, 0, 
		0, 842, 131, 1, 0, 0, 0, 843, 844, 5, 41, 0, 0, 844, 848, 5, 60, 0, 0, 
		845, 847, 3, 136, 68, 0, 846, 845, 1, 0, 0, 0, 847, 850, 1, 0, 0, 0, 848, 
		846, 1, 0, 0, 0, 848, 849, 1, 0, 0, 0, 849, 851, 1, 0, 0, 0, 850, 848, 
		1, 0, 0, 0, 851, 852, 5, 61, 0, 0, 852, 133, 1, 0, 0, 0, 853, 865, 5, 
		60, 0, 0, 854, 859, 3, 136, 68, 0, 855, 856, 5, 66, 0, 0, 856, 858, 3, 
		136, 68, 0, 857, 855, 1, 0, 0, 0, 858, 861, 1, 0, 0, 0, 859, 857, 1, 0, 
		0, 0, 859, 860, 1, 0, 0, 0, 860, 863, 1, 0, 0, 0, 861, 859, 1, 0, 0, 0, 
		862, 864, 5, 66, 0, 0, 863, 862, 1, 0, 0, 0, 863, 864, 1, 0, 0, 0, 864, 
		866, 1, 0, 0, 0, 865, 854, 1, 0, 0, 0, 865, 866, 1, 0, 0, 0, 866, 867, 
		1, 0, 0, 0, 867, 868, 5, 61, 0, 0, 868, 135, 1, 0, 0, 0, 869, 870, 3, 
		248, 124, 0, 870, 871, 5, 65, 0, 0, 871, 872, 3, 140, 70, 0, 872, 137, 
		1, 0, 0, 0, 873, 885, 5, 62, 0, 0, 874, 879, 3, 140, 70, 0, 875, 876, 
		5, 66, 0, 0, 876, 878, 3, 140, 70, 0, 877, 875, 1, 0, 0, 0, 878, 881, 
		1, 0, 0, 0, 879, 877, 1, 0, 0, 0, 879, 880, 1, 0, 0, 0, 880, 883, 1, 0, 
		0, 0, 881, 879, 1, 0, 0, 0, 882, 884, 5, 66, 0, 0, 883, 882, 1, 0, 0, 
		0, 883, 884, 1, 0, 0, 0, 884, 886, 1, 0, 0, 0, 885, 874, 1, 0, 0, 0, 885, 
		886, 1, 0, 0, 0, 886, 887, 1, 0, 0, 0, 887, 888, 5, 63, 0, 0, 888, 139, 
		1, 0, 0, 0, 889, 896, 3, 204, 102, 0, 890, 896, 3, 228, 114, 0, 891, 896, 
		3, 200, 100, 0, 892, 896, 3, 196, 98, 0, 893, 896, 3, 134, 67, 0, 894, 
		896, 3, 138, 69, 0, 895, 889, 1, 0, 0, 0, 895, 890, 1, 0, 0, 0, 895, 891, 
		1, 0, 0, 0, 895, 892, 1, 0, 0, 0, 895, 893, 1, 0, 0, 0, 895, 894, 1, 0, 
		0, 0, 896, 141, 1, 0, 0, 0, 897, 900, 5, 26, 0, 0, 898, 901, 3, 144, 72, 
		0, 899, 901, 3, 146, 73, 0, 900, 898, 1, 0, 0, 0, 900, 899, 1, 0, 0, 0, 
		901, 143, 1, 0, 0, 0, 902, 906, 5, 8, 0, 0, 903, 905, 3, 238, 119, 0, 
		904, 903, 1, 0, 0, 0, 905, 908, 1, 0, 0, 0, 906, 904, 1, 0, 0, 0, 906, 
		907, 1, 0, 0, 0, 907, 909, 1, 0, 0, 0, 908, 906, 1, 0, 0, 0, 909, 910, 
		5, 87, 0, 0, 910, 145, 1, 0, 0, 0, 911, 915, 5, 60, 0, 0, 912, 914, 3, 
		232, 116, 0, 913, 912, 1, 0, 0, 0, 914, 917, 1, 0, 0, 0, 915, 913, 1, 
		0, 0, 0, 915, 916, 1, 0, 0, 0, 916, 918, 1, 0, 0, 0, 917, 915, 1, 0, 0, 
		0, 918, 919, 5, 61, 0, 0, 919, 147, 1, 0, 0, 0, 920, 925, 3, 150, 75, 
		0, 921, 925, 3, 156, 78, 0, 922, 925, 3, 160, 80, 0, 923, 925, 3, 78, 
		39, 0, 924, 920, 1, 0, 0, 0, 924, 921, 1, 0, 0, 0, 924, 922, 1, 0, 0, 
		0, 924, 923, 1, 0, 0, 0, 925, 149, 1, 0, 0, 0, 926, 927, 5, 32, 0, 0, 
		927, 928, 5, 67, 0, 0, 928, 929, 3, 172, 86, 0, 929, 930, 5, 68, 0, 0, 
		930, 934, 5, 60, 0, 0, 931, 933, 3, 148, 74, 0, 932, 931, 1, 0, 0, 0, 
		933, 936, 1, 0, 0, 0, 934, 932, 1, 0, 0, 0, 934, 935, 1, 0, 0, 0, 935, 
		937, 1, 0, 0, 0, 936, 934, 1, 0, 0, 0, 937, 941, 5, 61, 0, 0, 938, 940, 
		3, 152, 76, 0, 939, 938, 1, 0, 0, 0, 940, 943, 1, 0, 0, 0, 941, 939, 1, 
		0, 0, 0, 941, 942, 1, 0, 0, 0, 942, 945, 1, 0, 0, 0, 943, 941, 1, 0, 0, 
		0, 944, 946, 3, 154, 77, 0, 945, 944, 1, 0, 0, 0, 945, 946, 1, 0, 0, 0, 
		946, 151, 1, 0, 0, 0, 947, 948, 5, 27, 0, 0, 948, 949, 5, 32, 0, 0, 949, 
		950, 5, 67, 0, 0, 950, 951, 3, 172, 86, 0, 951, 952, 5, 68, 0, 0, 952, 
		956, 5, 60, 0, 0, 953, 955, 3, 148, 74, 0, 954, 953, 1, 0, 0, 0, 955, 
		958, 1, 0, 0, 0, 956, 954, 1, 0, 0, 0, 956, 957, 1, 0, 0, 0, 957, 959, 
		1, 0, 0, 0, 958, 956, 1, 0, 0, 0, 959, 960, 5, 61, 0, 0, 960, 153, 1, 
		0, 0, 0, 961, 962, 5, 27, 0, 0, 962, 966, 5, 60, 0, 0, 963, 965, 3, 148, 
		74, 0, 964, 963, 1, 0, 0, 0, 965, 968, 1, 0, 0, 0, 966, 964, 1, 0, 0, 
		0, 966, 967, 1, 0, 0, 0, 967, 969, 1, 0, 0, 0, 968, 966, 1, 0, 0, 0, 969, 
		970, 5, 61, 0, 0, 970, 155, 1, 0, 0, 0, 971, 972, 5, 44, 0, 0, 972, 973, 
		5, 67, 0, 0, 973, 974, 3, 246, 123, 0, 974, 975, 5, 33, 0, 0, 975, 976, 
		3, 172, 86, 0, 976, 977, 5, 68, 0, 0, 977, 978, 3, 158, 79, 0, 978, 157, 
		1, 0, 0, 0, 979, 983, 5, 60, 0, 0, 980, 982, 3, 148, 74, 0, 981, 980, 
		1, 0, 0, 0, 982, 985, 1, 0, 0, 0, 983, 981, 1, 0, 0, 0, 983, 984, 1, 0, 
		0, 0, 984, 986, 1, 0, 0, 0, 985, 983, 1, 0, 0, 0, 986, 987, 5, 61, 0, 
		0, 987, 159, 1, 0, 0, 0, 988, 989, 5, 25, 0, 0, 989, 991, 3, 162, 81, 
		0, 990, 992, 3, 164, 82, 0, 991, 990, 1, 0, 0, 0, 991, 992, 1, 0, 0, 0, 
		992, 996, 1, 0, 0, 0, 993, 995, 3, 166, 83, 0, 994, 993, 1, 0, 0, 0, 995, 
		998, 1, 0, 0, 0, 996, 994, 1, 0, 0, 0, 996, 997, 1, 0, 0, 0, 997, 1000, 
		1, 0, 0, 0, 998, 996, 1, 0, 0, 0, 999, 1001, 3, 168, 84, 0, 1000, 999, 
		1, 0, 0, 0, 1000, 1001, 1, 0, 0, 0, 1001, 161, 1, 0, 0, 0, 1002, 1007, 
		3, 246, 123, 0, 1003, 1004, 5, 78, 0, 0, 1004, 1006, 3, 246, 123, 0, 1005, 
		1003, 1, 0, 0, 0, 1006, 1009, 1, 0, 0, 0, 1007, 1005, 1, 0, 0, 0, 1007, 
		1008, 1, 0, 0, 0, 1008, 163, 1, 0, 0, 0, 1009, 1007, 1, 0, 0, 0, 1010, 
		1011, 5, 24, 0, 0, 1011, 1012, 3, 246, 123, 0, 1012, 165, 1, 0, 0, 0, 
		1013, 1014, 5, 22, 0, 0, 1014, 1015, 3, 246, 123, 0, 1015, 167, 1, 0, 
		0, 0, 1016, 1019, 5, 60, 0, 0, 1017, 1018, 5, 35, 0, 0, 1018, 1020, 5, 
		65, 0, 0, 1019, 1017, 1, 0, 0, 0, 1019, 1020, 1, 0, 0, 0, 1020, 1032, 
		1, 0, 0, 0, 1021, 1026, 3, 170, 85, 0, 1022, 1023, 5, 66, 0, 0, 1023, 
		1025, 3, 170, 85, 0, 1024, 1022, 1, 0, 0, 0, 1025, 1028, 1, 0, 0, 0, 1026, 
		1024, 1, 0, 0, 0, 1026, 1027, 1, 0, 0, 0, 1027, 1030, 1, 0, 0, 0, 1028, 
		1026, 1, 0, 0, 0, 1029, 1031, 5, 66, 0, 0, 1030, 1029, 1, 0, 0, 0, 1030, 
		1031, 1, 0, 0, 0, 1031, 1033, 1, 0, 0, 0, 1032, 1021, 1, 0, 0, 0, 1032, 
		1033, 1, 0, 0, 0, 1033, 1034, 1, 0, 0, 0, 1034, 1035, 5, 61, 0, 0, 1035, 
		169, 1, 0, 0, 0, 1036, 1039, 3, 246, 123, 0, 1037, 1038, 5, 64, 0, 0, 
		1038, 1040, 3, 172, 86, 0, 1039, 1037, 1, 0, 0, 0, 1039, 1040, 1, 0, 0, 
		0, 1040, 171, 1, 0, 0, 0, 1041, 1042, 3, 174, 87, 0, 1042, 173, 1, 0, 
		0, 0, 1043, 1044, 3, 176, 88, 0, 1044, 1045, 5, 54, 0, 0, 1045, 1046, 
		3, 174, 87, 0, 1046, 1049, 1, 0, 0, 0, 1047, 1049, 3, 176, 88, 0, 1048, 
		1043, 1, 0, 0, 0, 1048, 1047, 1, 0, 0, 0, 1049, 175, 1, 0, 0, 0, 1050, 
		1051, 3, 178, 89, 0, 1051, 1052, 5, 55, 0, 0, 1052, 1053, 3, 176, 88, 
		0, 1053, 1056, 1, 0, 0, 0, 1054, 1056, 3, 178, 89, 0, 1055, 1050, 1, 0, 
		0, 0, 1055, 1054, 1, 0, 0, 0, 1056, 177, 1, 0, 0, 0, 1057, 1058, 3, 180, 
		90, 0, 1058, 1059, 7, 4, 0, 0, 1059, 1060, 3, 178, 89, 0, 1060, 1063, 
		1, 0, 0, 0, 1061, 1063, 3, 180, 90, 0, 1062, 1057, 1, 0, 0, 0, 1062, 1061, 
		1, 0, 0, 0, 1063, 179, 1, 0, 0, 0, 1064, 1065, 3, 182, 91, 0, 1065, 1066, 
		7, 5, 0, 0, 1066, 1067, 3, 180, 90, 0, 1067, 1070, 1, 0, 0, 0, 1068, 1070, 
		3, 182, 91, 0, 1069, 1064, 1, 0, 0, 0, 1069, 1068, 1, 0, 0, 0, 1070, 181, 
		1, 0, 0, 0, 1071, 1072, 3, 184, 92, 0, 1072, 1073, 7, 6, 0, 0, 1073, 1074, 
		3, 182, 91, 0, 1074, 1077, 1, 0, 0, 0, 1075, 1077, 3, 184, 92, 0, 1076, 
		1071, 1, 0, 0, 0, 1076, 1075, 1, 0, 0, 0, 1077, 183, 1, 0, 0, 0, 1078, 
		1079, 3, 186, 93, 0, 1079, 1080, 7, 7, 0, 0, 1080, 1081, 3, 184, 92, 0, 
		1081, 1084, 1, 0, 0, 0, 1082, 1084, 3, 186, 93, 0, 1083, 1078, 1, 0, 0, 
		0, 1083, 1082, 1, 0, 0, 0, 1084, 185, 1, 0, 0, 0, 1085, 1086, 3, 188, 
		94, 0, 1086, 1087, 5, 53, 0, 0, 1087, 1088, 3, 186, 93, 0, 1088, 1091, 
		1, 0, 0, 0, 1089, 1091, 3, 188, 94, 0, 1090, 1085, 1, 0, 0, 0, 1090, 1089, 
		1, 0, 0, 0, 1091, 187, 1, 0, 0, 0, 1092, 1093, 7, 8, 0, 0, 1093, 1096, 
		3, 188, 94, 0, 1094, 1096, 3, 190, 95, 0, 1095, 1092, 1, 0, 0, 0, 1095, 
		1094, 1, 0, 0, 0, 1096, 189, 1, 0, 0, 0, 1097, 1098, 6, 95, -1, 0, 1098, 
		1099, 3, 192, 96, 0, 1099, 1110, 1, 0, 0, 0, 1100, 1101, 10, 3, 0, 0, 
		1101, 1102, 5, 62, 0, 0, 1102, 1103, 3, 172, 86, 0, 1103, 1104, 5, 63, 
		0, 0, 1104, 1109, 1, 0, 0, 0, 1105, 1106, 10, 2, 0, 0, 1106, 1107, 5, 
		78, 0, 0, 1107, 1109, 3, 246, 123, 0, 1108, 1100, 1, 0, 0, 0, 1108, 1105, 
		1, 0, 0, 0, 1109, 1112, 1, 0, 0, 0, 1110, 1108, 1, 0, 0, 0, 1110, 1111, 
		1, 0, 0, 0, 1111, 191, 1, 0, 0, 0, 1112, 1110, 1, 0, 0, 0, 1113, 1127, 
		3, 194, 97, 0, 1114, 1127, 3, 198, 99, 0, 1115, 1127, 3, 200, 100, 0, 
		1116, 1127, 3, 202, 101, 0, 1117, 1127, 3, 228, 114, 0, 1118, 1127, 3, 
		206, 103, 0, 1119, 1127, 3, 208, 104, 0, 1120, 1127, 3, 212, 106, 0, 1121, 
		1127, 3, 216, 108, 0, 1122, 1127, 3, 220, 110, 0, 1123, 1127, 3, 222, 
		111, 0, 1124, 1127, 3, 224, 112, 0, 1125, 1127, 3, 226, 113, 0, 1126, 
		1113, 1, 0, 0, 0, 1126, 1114, 1, 0, 0, 0, 1126, 1115, 1, 0, 0, 0, 1126, 
		1116, 1, 0, 0, 0, 1126, 1117, 1, 0, 0, 0, 1126, 1118, 1, 0, 0, 0, 1126, 
		1119, 1, 0, 0, 0, 1126, 1120, 1, 0, 0, 0, 1126, 1121, 1, 0, 0, 0, 1126, 
		1122, 1, 0, 0, 0, 1126, 1123, 1, 0, 0, 0, 1126, 1124, 1, 0, 0, 0, 1126, 
		1125, 1, 0, 0, 0, 1127, 193, 1, 0, 0, 0, 1128, 1129, 3, 246, 123, 0, 1129, 
		195, 1, 0, 0, 0, 1130, 1131, 5, 38, 0, 0, 1131, 197, 1, 0, 0, 0, 1132, 
		1133, 5, 37, 0, 0, 1133, 199, 1, 0, 0, 0, 1134, 1135, 7, 9, 0, 0, 1135, 
		201, 1, 0, 0, 0, 1136, 1139, 5, 7, 0, 0, 1137, 1139, 5, 6, 0, 0, 1138, 
		1136, 1, 0, 0, 0, 1138, 1137, 1, 0, 0, 0, 1139, 203, 1, 0, 0, 0, 1140, 
		1142, 5, 72, 0, 0, 1141, 1140, 1, 0, 0, 0, 1141, 1142, 1, 0, 0, 0, 1142, 
		1143, 1, 0, 0, 0, 1143, 1144, 3, 202, 101, 0, 1144, 205, 1, 0, 0, 0, 1145, 
		1157, 5, 62, 0, 0, 1146, 1151, 3, 172, 86, 0, 1147, 1148, 5, 66, 0, 0, 
		1148, 1150, 3, 172, 86, 0, 1149, 1147, 1, 0, 0, 0, 1150, 1153, 1, 0, 0, 
		0, 1151, 1149, 1, 0, 0, 0, 1151, 1152, 1, 0, 0, 0, 1152, 1155, 1, 0, 0, 
		0, 1153, 1151, 1, 0, 0, 0, 1154, 1156, 5, 66, 0, 0, 1155, 1154, 1, 0, 
		0, 0, 1155, 1156, 1, 0, 0, 0, 1156, 1158, 1, 0, 0, 0, 1157, 1146, 1, 0, 
		0, 0, 1157, 1158, 1, 0, 0, 0, 1158, 1159, 1, 0, 0, 0, 1159, 1160, 5, 63, 
		0, 0, 1160, 207, 1, 0, 0, 0, 1161, 1173, 5, 60, 0, 0, 1162, 1167, 3, 210, 
		105, 0, 1163, 1164, 5, 66, 0, 0, 1164, 1166, 3, 210, 105, 0, 1165, 1163, 
		1, 0, 0, 0, 1166, 1169, 1, 0, 0, 0, 1167, 1165, 1, 0, 0, 0, 1167, 1168, 
		1, 0, 0, 0, 1168, 1171, 1, 0, 0, 0, 1169, 1167, 1, 0, 0, 0, 1170, 1172, 
		5, 66, 0, 0, 1171, 1170, 1, 0, 0, 0, 1171, 1172, 1, 0, 0, 0, 1172, 1174, 
		1, 0, 0, 0, 1173, 1162, 1, 0, 0, 0, 1173, 1174, 1, 0, 0, 0, 1174, 1175, 
		1, 0, 0, 0, 1175, 1176, 5, 61, 0, 0, 1176, 209, 1, 0, 0, 0, 1177, 1178, 
		3, 172, 86, 0, 1178, 1179, 5, 65, 0, 0, 1179, 1180, 3, 172, 86, 0, 1180, 
		211, 1, 0, 0, 0, 1181, 1182, 5, 39, 0, 0, 1182, 1194, 5, 60, 0, 0, 1183, 
		1188, 3, 214, 107, 0, 1184, 1185, 5, 66, 0, 0, 1185, 1187, 3, 214, 107, 
		0, 1186, 1184, 1, 0, 0, 0, 1187, 1190, 1, 0, 0, 0, 1188, 1186, 1, 0, 0, 
		0, 1188, 1189, 1, 0, 0, 0, 1189, 1192, 1, 0, 0, 0, 1190, 1188, 1, 0, 0, 
		0, 1191, 1193, 5, 66, 0, 0, 1192, 1191, 1, 0, 0, 0, 1192, 1193, 1, 0, 
		0, 0, 1193, 1195, 1, 0, 0, 0, 1194, 1183, 1, 0, 0, 0, 1194, 1195, 1, 0, 
		0, 0, 1195, 1196, 1, 0, 0, 0, 1196, 1197, 5, 61, 0, 0, 1197, 213, 1, 0, 
		0, 0, 1198, 1199, 3, 246, 123, 0, 1199, 1200, 5, 65, 0, 0, 1200, 1201, 
		3, 172, 86, 0, 1201, 215, 1, 0, 0, 0, 1202, 1203, 3, 246, 123, 0, 1203, 
		1215, 5, 60, 0, 0, 1204, 1209, 3, 218, 109, 0, 1205, 1206, 5, 66, 0, 0, 
		1206, 1208, 3, 218, 109, 0, 1207, 1205, 1, 0, 0, 0, 1208, 1211, 1, 0, 
		0, 0, 1209, 1207, 1, 0, 0, 0, 1209, 1210, 1, 0, 0, 0, 1210, 1213, 1, 0, 
		0, 0, 1211, 1209, 1, 0, 0, 0, 1212, 1214, 5, 66, 0, 0, 1213, 1212, 1, 
		0, 0, 0, 1213, 1214, 1, 0, 0, 0, 1214, 1216, 1, 0, 0, 0, 1215, 1204, 1, 
		0, 0, 0, 1215, 1216, 1, 0, 0, 0, 1216, 1217, 1, 0, 0, 0, 1217, 1218, 5, 
		61, 0, 0, 1218, 217, 1, 0, 0, 0, 1219, 1220, 3, 246, 123, 0, 1220, 1221, 
		5, 65, 0, 0, 1221, 1222, 3, 172, 86, 0, 1222, 219, 1, 0, 0, 0, 1223, 1224, 
		5, 67, 0, 0, 1224, 1225, 3, 172, 86, 0, 1225, 1226, 5, 66, 0, 0, 1226, 
		1227, 3, 172, 86, 0, 1227, 1228, 5, 68, 0, 0, 1228, 221, 1, 0, 0, 0, 1229, 
		1230, 5, 67, 0, 0, 1230, 1231, 3, 172, 86, 0, 1231, 1232, 5, 68, 0, 0, 
		1232, 223, 1, 0, 0, 0, 1233, 1234, 5, 32, 0, 0, 1234, 1235, 3, 172, 86, 
		0, 1235, 1236, 5, 48, 0, 0, 1236, 1237, 3, 172, 86, 0, 1237, 1238, 5, 
		27, 0, 0, 1238, 1239, 3, 172, 86, 0, 1239, 225, 1, 0, 0, 0, 1240, 1241, 
		3, 246, 123, 0, 1241, 1253, 5, 67, 0, 0, 1242, 1247, 3, 172, 86, 0, 1243, 
		1244, 5, 66, 0, 0, 1244, 1246, 3, 172, 86, 0, 1245, 1243, 1, 0, 0, 0, 
		1246, 1249, 1, 0, 0, 0, 1247, 1245, 1, 0, 0, 0, 1247, 1248, 1, 0, 0, 0, 
		1248, 1251, 1, 0, 0, 0, 1249, 1247, 1, 0, 0, 0, 1250, 1252, 5, 66, 0, 
		0, 1251, 1250, 1, 0, 0, 0, 1251, 1252, 1, 0, 0, 0, 1252, 1254, 1, 0, 0, 
		0, 1253, 1242, 1, 0, 0, 0, 1253, 1254, 1, 0, 0, 0, 1254, 1255, 1, 0, 0, 
		0, 1255, 1256, 5, 68, 0, 0, 1256, 227, 1, 0, 0, 0, 1257, 1260, 3, 230, 
		115, 0, 1258, 1260, 3, 236, 118, 0, 1259, 1257, 1, 0, 0, 0, 1259, 1258, 
		1, 0, 0, 0, 1260, 229, 1, 0, 0, 0, 1261, 1265, 5, 10, 0, 0, 1262, 1264, 
		3, 232, 116, 0, 1263, 1262, 1, 0, 0, 0, 1264, 1267, 1, 0, 0, 0, 1265, 
		1263, 1, 0, 0, 0, 1265, 1266, 1, 0, 0, 0, 1266, 1268, 1, 0, 0, 0, 1267, 
		1265, 1, 0, 0, 0, 1268, 1278, 5, 82, 0, 0, 1269, 1273, 5, 11, 0, 0, 1270, 
		1272, 3, 232, 116, 0, 1271, 1270, 1, 0, 0, 0, 1272, 1275, 1, 0, 0, 0, 
		1273, 1271, 1, 0, 0, 0, 1273, 1274, 1, 0, 0, 0, 1274, 1276, 1, 0, 0, 0, 
		1275, 1273, 1, 0, 0, 0, 1276, 1278, 5, 83, 0, 0, 1277, 1261, 1, 0, 0, 
		0, 1277, 1269, 1, 0, 0, 0, 1278, 231, 1, 0, 0, 0, 1279, 1285, 5, 1, 0, 
		0, 1280, 1285, 5, 2, 0, 0, 1281, 1285, 5, 3, 0, 0, 1282, 1285, 5, 4, 0, 
		0, 1283, 1285, 3, 234, 117, 0, 1284, 1279, 1, 0, 0, 0, 1284, 1280, 1, 
		0, 0, 0, 1284, 1281, 1, 0, 0, 0, 1284, 1282, 1, 0, 0, 0, 1284, 1283, 1, 
		0, 0, 0, 1285, 233, 1, 0, 0, 0, 1286, 1287, 5, 5, 0, 0, 1287, 1288, 3, 
		242, 121, 0, 1288, 1289, 5, 61, 0, 0, 1289, 235, 1, 0, 0, 0, 1290, 1294, 
		5, 8, 0, 0, 1291, 1293, 3, 238, 119, 0, 1292, 1291, 1, 0, 0, 0, 1293, 
		1296, 1, 0, 0, 0, 1294, 1292, 1, 0, 0, 0, 1294, 1295, 1, 0, 0, 0, 1295, 
		1297, 1, 0, 0, 0, 1296, 1294, 1, 0, 0, 0, 1297, 1298, 5, 87, 0, 0, 1298, 
		237, 1, 0, 0, 0, 1299, 1307, 5, 90, 0, 0, 1300, 1307, 5, 86, 0, 0, 1301, 
		1307, 5, 88, 0, 0, 1302, 1307, 5, 89, 0, 0, 1303, 1307, 5, 91, 0, 0, 1304, 
		1307, 5, 92, 0, 0, 1305, 1307, 3, 240, 120, 0, 1306, 1299, 1, 0, 0, 0, 
		1306, 1300, 1, 0, 0, 0, 1306, 1301, 1, 0, 0, 0, 1306, 1302, 1, 0, 0, 0, 
		1306, 1303, 1, 0, 0, 0, 1306, 1304, 1, 0, 0, 0, 1306, 1305, 1, 0, 0, 0, 
		1307, 239, 1, 0, 0, 0, 1308, 1309, 5, 85, 0, 0, 1309, 1310, 3, 242, 121, 
		0, 1310, 1311, 5, 61, 0, 0, 1311, 1317, 1, 0, 0, 0, 1312, 1313, 5, 84, 
		0, 0, 1313, 1314, 3, 242, 121, 0, 1314, 1315, 5, 61, 0, 0, 1315, 1317, 
		1, 0, 0, 0, 1316, 1308, 1, 0, 0, 0, 1316, 1312, 1, 0, 0, 0, 1317, 241, 
		1, 0, 0, 0, 1318, 1320, 3, 244, 122, 0, 1319, 1318, 1, 0, 0, 0, 1320, 
		1323, 1, 0, 0, 0, 1321, 1319, 1, 0, 0, 0, 1321, 1322, 1, 0, 0, 0, 1322, 
		1324, 1, 0, 0, 0, 1323, 1321, 1, 0, 0, 0, 1324, 1325, 3, 172, 86, 0, 1325, 
		243, 1, 0, 0, 0, 1326, 1327, 5, 52, 0, 0, 1327, 1328, 5, 64, 0, 0, 1328, 
		1344, 3, 228, 114, 0, 1329, 1330, 5, 49, 0, 0, 1330, 1331, 5, 64, 0, 0, 
		1331, 1332, 3, 228, 114, 0, 1332, 1333, 5, 29, 0, 0, 1333, 1334, 5, 64, 
		0, 0, 1334, 1335, 3, 228, 114, 0, 1335, 1344, 1, 0, 0, 0, 1336, 1337, 
		5, 29, 0, 0, 1337, 1338, 5, 64, 0, 0, 1338, 1339, 3, 228, 114, 0, 1339, 
		1340, 5, 49, 0, 0, 1340, 1341, 5, 64, 0, 0, 1341, 1342, 3, 228, 114, 0, 
		1342, 1344, 1, 0, 0, 0, 1343, 1326, 1, 0, 0, 0, 1343, 1329, 1, 0, 0, 0, 
		1343, 1336, 1, 0, 0, 0, 1344, 245, 1, 0, 0, 0, 1345, 1346, 3, 250, 125, 
		0, 1346, 247, 1, 0, 0, 0, 1347, 1352, 3, 246, 123, 0, 1348, 1349, 5, 78, 
		0, 0, 1349, 1351, 3, 246, 123, 0, 1350, 1348, 1, 0, 0, 0, 1351, 1354, 
		1, 0, 0, 0, 1352, 1350, 1, 0, 0, 0, 1352, 1353, 1, 0, 0, 0, 1353, 249, 
		1, 0, 0, 0, 1354, 1352, 1, 0, 0, 0, 1355, 1356, 7, 10, 0, 0, 1356, 251, 
		1, 0, 0, 0, 145, 256, 269, 275, 280, 292, 300, 304, 306, 313, 319, 327, 
		331, 346, 354, 362, 370, 374, 376, 387, 397, 401, 407, 415, 419, 427, 
		440, 444, 446, 456, 460, 462, 477, 481, 483, 498, 502, 504, 524, 535, 
		546, 555, 562, 565, 574, 578, 582, 586, 589, 595, 604, 615, 626, 633, 
		643, 653, 667, 681, 695, 704, 708, 710, 725, 729, 731, 746, 750, 752, 
		766, 770, 772, 781, 795, 803, 807, 809, 823, 827, 829, 838, 848, 859, 
		863, 865, 879, 883, 885, 895, 900, 906, 915, 924, 934, 941, 945, 956, 
		966, 983, 991, 996, 1000, 1007, 1019, 1026, 1030, 1032, 1039, 1048, 1055, 
		1062, 1069, 1076, 1083, 1090, 1095, 1108, 1110, 1126, 1138, 1141, 1151, 
		1155, 1157, 1167, 1171, 1173, 1188, 1192, 1194, 1209, 1213, 1215, 1247, 
		1251, 1253, 1259, 1265, 1273, 1277, 1284, 1294, 1306, 1316, 1321, 1343, 
		1352
	];
}
