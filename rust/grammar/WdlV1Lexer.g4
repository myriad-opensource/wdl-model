lexer grammar WdlV1Lexer;

channels { COMMENTS }

// Shared token aliases emitted by string modes.
tokens {
    STRING_TEXT,
    STRING_ESCAPE,
    STRING_DOLLAR_SIGN,
    STRING_TILDE,
    STRING_PLACEHOLDER_START
}

// -----------------------------------------------------------------------------
// Base Mode: numeric fragments and literals
// -----------------------------------------------------------------------------

fragment FLOAT_FRAG
    : [0-9]+ [eE] [+-]? [0-9]+
    | [0-9]+ '.' [0-9]* ([eE] [+-]? [0-9]+)?
    | [0-9]* '.' [0-9]+ ([eE] [+-]? [0-9]+)?
    ;

fragment INTEGER_FRAG
    : '0'
    | [1-9] [0-9]*
    | '0' [0-7]+
    | '0' [xX] [0-9a-fA-F]+
    ;

FLOAT
    : FLOAT_FRAG
    ;

INTEGER
    : INTEGER_FRAG
    ;

// Enter string/multilineString modes from base mode.
OPEN_MULTILINE_STRING
    : '<<<' -> pushMode(MULTILINE_STRING)
    ;

CLOSE_MULTILINE_STRING
    : '>>>'
    ;

SINGLE_QUOTE
    : '\'' -> pushMode(STRING_SINGLE_QUOTE)
    ;

DOUBLE_QUOTE
    : '"' -> pushMode(STRING_DOUBLE_QUOTE)
    ;

// -----------------------------------------------------------------------------
// Base Mode: keywords
// -----------------------------------------------------------------------------

KEYWORD_ARRAY_TYPE: 'Array';
KEYWORD_BOOLEAN_TYPE: 'Boolean';
KEYWORD_DIRECTORY_TYPE: 'Directory';
KEYWORD_FILE_TYPE: 'File';
KEYWORD_FLOAT_TYPE: 'Float';
KEYWORD_INT_TYPE: 'Int';
KEYWORD_MAP_TYPE: 'Map';
KEYWORD_OBJECT_TYPE: 'Object';
KEYWORD_PAIR_TYPE: 'Pair';
KEYWORD_STRING_TYPE: 'String';

KEYWORD_AFTER: 'after';
KEYWORD_ALIAS: 'alias';
KEYWORD_AS: 'as';
KEYWORD_CALL: 'call';
KEYWORD_COMMAND: 'command' -> pushMode(COMMAND_START);
KEYWORD_ELSE: 'else';
KEYWORD_ENV: 'env';
KEYWORD_FALSE: 'false';
KEYWORD_FROM: 'from';
KEYWORD_HINTS: 'hints';
KEYWORD_IF: 'if';
KEYWORD_IN: 'in';
KEYWORD_IMPORT: 'import';
KEYWORD_INPUT: 'input';
KEYWORD_META: 'meta';
KEYWORD_NONE: 'None';
KEYWORD_NULL: 'null';
KEYWORD_OBJECT: 'object';
KEYWORD_OUTPUT: 'output';
KEYWORD_PARAMETER_META: 'parameter_meta';
KEYWORD_REQUIREMENTS: 'requirements';
KEYWORD_RUNTIME: 'runtime';
KEYWORD_SCATTER: 'scatter';
KEYWORD_STRUCT: 'struct';
KEYWORD_ENUM: 'enum';
KEYWORD_TASK: 'task';
KEYWORD_THEN: 'then';
KEYWORD_TRUE: 'true';
KEYWORD_VERSION: 'version';
KEYWORD_WORKFLOW: 'workflow';

IDENTIFIER
    : [a-zA-Z] [a-zA-Z0-9_]*
    ;

// -----------------------------------------------------------------------------
// Base Mode: operators, delimiters, trivia
// -----------------------------------------------------------------------------

EXPONENTIATION: '**';
LOGICAL_OR: '||';
LOGICAL_AND: '&&';
EQUAL: '==';
NOT_EQUAL: '!=';
LESS_EQUAL: '<=';
GREATER_EQUAL: '>=';
OPEN_BRACE: '{';
CLOSE_BRACE: '}';
OPEN_BRACKET: '[';
CLOSE_BRACKET: ']';
ASSIGNMENT: '=';
COLON: ':';
COMMA: ',';
OPEN_PAREN: '(';
CLOSE_PAREN: ')';
QUESTION_MARK: '?';
EXCLAMATION: '!';
PLUS: '+';
MINUS: '-';
ASTERISK: '*';
SLASH: '/';
PERCENT: '%';
LESS: '<';
GREATER: '>';
DOT: '.';

COMMENT
    : '#' ~[\r\n]* -> channel(COMMENTS)
    ;

WHITESPACE
    : [ \t\r\n]+ -> channel(HIDDEN)
    ;

UNEXPECTED_CHAR
    : .
    ;

// -----------------------------------------------------------------------------
// INTERPOLATION mode: expression lexing inside ${...} and ~{...}
// -----------------------------------------------------------------------------

mode INTERPOLATION;

I_WHITESPACE
    : [ \t\r\n]+ -> channel(HIDDEN), type(WHITESPACE)
    ;

I_COMMENT
    : '#' ~[\r\n]* -> channel(COMMENTS), type(COMMENT)
    ;

I_FLOAT: FLOAT_FRAG -> type(FLOAT);
I_INTEGER: INTEGER_FRAG -> type(INTEGER);

I_OPEN_MULTILINE_STRING
    : '<<<' -> pushMode(MULTILINE_STRING), type(OPEN_MULTILINE_STRING)
    ;

I_CLOSE_MULTILINE_STRING
    : '>>>' -> type(CLOSE_MULTILINE_STRING)
    ;

I_SINGLE_QUOTE
    : '\'' -> pushMode(STRING_SINGLE_QUOTE), type(SINGLE_QUOTE)
    ;

I_DOUBLE_QUOTE
    : '"' -> pushMode(STRING_DOUBLE_QUOTE), type(DOUBLE_QUOTE)
    ;

I_KEYWORD_ARRAY_TYPE: 'Array' -> type(KEYWORD_ARRAY_TYPE);
I_KEYWORD_BOOLEAN_TYPE: 'Boolean' -> type(KEYWORD_BOOLEAN_TYPE);
I_KEYWORD_DIRECTORY_TYPE: 'Directory' -> type(KEYWORD_DIRECTORY_TYPE);
I_KEYWORD_FILE_TYPE: 'File' -> type(KEYWORD_FILE_TYPE);
I_KEYWORD_FLOAT_TYPE: 'Float' -> type(KEYWORD_FLOAT_TYPE);
I_KEYWORD_INT_TYPE: 'Int' -> type(KEYWORD_INT_TYPE);
I_KEYWORD_MAP_TYPE: 'Map' -> type(KEYWORD_MAP_TYPE);
I_KEYWORD_OBJECT_TYPE: 'Object' -> type(KEYWORD_OBJECT_TYPE);
I_KEYWORD_PAIR_TYPE: 'Pair' -> type(KEYWORD_PAIR_TYPE);
I_KEYWORD_STRING_TYPE: 'String' -> type(KEYWORD_STRING_TYPE);

I_KEYWORD_AFTER: 'after' -> type(KEYWORD_AFTER);
I_KEYWORD_ALIAS: 'alias' -> type(KEYWORD_ALIAS);
I_KEYWORD_AS: 'as' -> type(KEYWORD_AS);
I_KEYWORD_CALL: 'call' -> type(KEYWORD_CALL);
I_KEYWORD_COMMAND: 'command' -> type(KEYWORD_COMMAND);
I_KEYWORD_ELSE: 'else' -> type(KEYWORD_ELSE);
I_KEYWORD_ENV: 'env' -> type(KEYWORD_ENV);
I_KEYWORD_FALSE: 'false' -> type(KEYWORD_FALSE);
I_KEYWORD_FROM: 'from' -> type(KEYWORD_FROM);
I_KEYWORD_HINTS: 'hints' -> type(KEYWORD_HINTS);
I_KEYWORD_IF: 'if' -> type(KEYWORD_IF);
I_KEYWORD_IN: 'in' -> type(KEYWORD_IN);
I_KEYWORD_IMPORT: 'import' -> type(KEYWORD_IMPORT);
I_KEYWORD_INPUT: 'input' -> type(KEYWORD_INPUT);
I_KEYWORD_META: 'meta' -> type(KEYWORD_META);
I_KEYWORD_NONE: 'None' -> type(KEYWORD_NONE);
I_KEYWORD_NULL: 'null' -> type(KEYWORD_NULL);
I_KEYWORD_OBJECT: 'object' -> type(KEYWORD_OBJECT);
I_KEYWORD_OUTPUT: 'output' -> type(KEYWORD_OUTPUT);
I_KEYWORD_PARAMETER_META: 'parameter_meta' -> type(KEYWORD_PARAMETER_META);
I_KEYWORD_REQUIREMENTS: 'requirements' -> type(KEYWORD_REQUIREMENTS);
I_KEYWORD_RUNTIME: 'runtime' -> type(KEYWORD_RUNTIME);
I_KEYWORD_SCATTER: 'scatter' -> type(KEYWORD_SCATTER);
I_KEYWORD_STRUCT: 'struct' -> type(KEYWORD_STRUCT);
I_KEYWORD_ENUM: 'enum' -> type(KEYWORD_ENUM);
I_KEYWORD_TASK: 'task' -> type(KEYWORD_TASK);
I_KEYWORD_THEN: 'then' -> type(KEYWORD_THEN);
I_KEYWORD_TRUE: 'true' -> type(KEYWORD_TRUE);
I_KEYWORD_VERSION: 'version' -> type(KEYWORD_VERSION);
I_KEYWORD_WORKFLOW: 'workflow' -> type(KEYWORD_WORKFLOW);

I_IDENTIFIER
    : [a-zA-Z] [a-zA-Z0-9_]* -> type(IDENTIFIER)
    ;

I_EXPONENTIATION: '**' -> type(EXPONENTIATION);
I_LOGICAL_OR: '||' -> type(LOGICAL_OR);
I_LOGICAL_AND: '&&' -> type(LOGICAL_AND);
I_EQUAL: '==' -> type(EQUAL);
I_NOT_EQUAL: '!=' -> type(NOT_EQUAL);
I_LESS_EQUAL: '<=' -> type(LESS_EQUAL);
I_GREATER_EQUAL: '>=' -> type(GREATER_EQUAL);
I_OPEN_BRACE: '{' -> pushMode(INTERPOLATION), type(OPEN_BRACE);
// Pop one interpolation nesting level.
I_CLOSE_BRACE: '}' -> popMode, type(CLOSE_BRACE);
I_OPEN_BRACKET: '[' -> type(OPEN_BRACKET);
I_CLOSE_BRACKET: ']' -> type(CLOSE_BRACKET);
I_ASSIGNMENT: '=' -> type(ASSIGNMENT);
I_COLON: ':' -> type(COLON);
I_COMMA: ',' -> type(COMMA);
I_OPEN_PAREN: '(' -> type(OPEN_PAREN);
I_CLOSE_PAREN: ')' -> type(CLOSE_PAREN);
I_QUESTION_MARK: '?' -> type(QUESTION_MARK);
I_EXCLAMATION: '!' -> type(EXCLAMATION);
I_PLUS: '+' -> type(PLUS);
I_MINUS: '-' -> type(MINUS);
I_ASTERISK: '*' -> type(ASTERISK);
I_SLASH: '/' -> type(SLASH);
I_PERCENT: '%' -> type(PERCENT);
I_LESS: '<' -> type(LESS);
I_GREATER: '>' -> type(GREATER);
I_DOT: '.' -> type(DOT);
I_UNEXPECTED_CHAR: . -> type(UNEXPECTED_CHAR);

// -----------------------------------------------------------------------------
// STRING_SINGLE_QUOTE mode: single-quoted strings with interpolation starts
// -----------------------------------------------------------------------------

mode STRING_SINGLE_QUOTE;

SINGLE_QUOTE_PLACEHOLDER_START
    : ('~{' | '${') -> pushMode(INTERPOLATION), type(STRING_PLACEHOLDER_START)
    ;

SINGLE_QUOTE_ESCAPE
    : '\\' ('\n' | '\r' | .) -> type(STRING_ESCAPE)
    ;

SINGLE_QUOTE_TEXT
    : ~[\\$~']+ -> type(STRING_TEXT)
    ;

SINGLE_QUOTE_DOLLAR_SIGN
    : '$' -> type(STRING_DOLLAR_SIGN)
    ;

SINGLE_QUOTE_TILDE
    : '~' -> type(STRING_TILDE)
    ;

SINGLE_QUOTE_END
    : '\'' -> popMode
    ;

// -----------------------------------------------------------------------------
// STRING_DOUBLE_QUOTE mode: double-quoted strings with interpolation starts
// -----------------------------------------------------------------------------

mode STRING_DOUBLE_QUOTE;

DOUBLE_QUOTE_PLACEHOLDER_START
    : ('~{' | '${') -> pushMode(INTERPOLATION), type(STRING_PLACEHOLDER_START)
    ;

DOUBLE_QUOTE_ESCAPE
    : '\\' ('\n' | '\r' | .) -> type(STRING_ESCAPE)
    ;

DOUBLE_QUOTE_TEXT
    : ~[\\$~"]+ -> type(STRING_TEXT)
    ;

DOUBLE_QUOTE_DOLLAR_SIGN
    : '$' -> type(STRING_DOLLAR_SIGN)
    ;

DOUBLE_QUOTE_TILDE
    : '~' -> type(STRING_TILDE)
    ;

DOUBLE_QUOTE_END
    : '"' -> popMode
    ;

// -----------------------------------------------------------------------------
// MULTILINE_STRING mode: multilineString body and interpolation starts
// -----------------------------------------------------------------------------

mode MULTILINE_STRING;

MULTILINE_STRING_DOLLAR_PLACEHOLDER_START
    : '${' -> pushMode(INTERPOLATION)
    ;

MULTILINE_STRING_TILDE_PLACEHOLDER_START
    : '~{' -> pushMode(INTERPOLATION)
    ;

MULTILINE_STRING_ESCAPE
    : '\\' ('\n' | '\r' | .)
    ;

MULTILINE_STRING_END
    : '>>>' -> popMode
    ;

MULTILINE_STRING_DOUBLE_CLOSE_ANGLE
    : '>>'
    ;

MULTILINE_STRING_SINGLE_CLOSE_ANGLE
    : '>'
    ;

MULTILINE_STRING_TEXT
    : ~[\\~$>]+
    ;

MULTILINE_STRING_DOLLAR_SIGN
    : '$'
    ;

MULTILINE_STRING_TILDE
    : '~'
    ;

// -----------------------------------------------------------------------------
// COMMAND_START mode: consume trivia between `command` and its opener.
// -----------------------------------------------------------------------------

mode COMMAND_START;

COMMAND_START_WHITESPACE
    : [ \t\r\n]+ -> channel(HIDDEN), type(WHITESPACE)
    ;

COMMAND_START_COMMENT
    : '#' ~[\r\n]* -> channel(COMMENTS), type(COMMENT)
    ;

COMMAND_START_OPEN_MULTILINE_STRING
    : '<<<' -> type(OPEN_MULTILINE_STRING), popMode, pushMode(MULTILINE_STRING)
    ;

COMMAND_START_OPEN_BRACE
    : '{' -> type(OPEN_BRACE), popMode, pushMode(BRACE_COMMAND)
    ;

COMMAND_START_RECOVERY
    : . -> type(UNEXPECTED_CHAR), popMode
    ;

// -----------------------------------------------------------------------------
// BRACE_COMMAND mode: command { ... } payload tokenization
// -----------------------------------------------------------------------------

mode BRACE_COMMAND;

BRACE_COMMAND_PLACEHOLDER_START
    : ('~{' | '${') -> type(STRING_PLACEHOLDER_START), pushMode(INTERPOLATION)
    ;

BRACE_COMMAND_ESCAPE
    : '\\' ('\n' | '\r' | .) -> type(STRING_ESCAPE)
    ;

BRACE_COMMAND_TEXT
    : ~[\\$~}]+ -> type(STRING_TEXT)
    ;

BRACE_COMMAND_DOLLAR_SIGN
    : '$' -> type(STRING_DOLLAR_SIGN)
    ;

BRACE_COMMAND_TILDE
    : '~' -> type(STRING_TILDE)
    ;

BRACE_COMMAND_END
    : '}' -> type(CLOSE_BRACE), popMode
    ;

// -----------------------------------------------------------------------------
// ESCAPE mode: escape-sequence analysis tokens
// -----------------------------------------------------------------------------

mode ESCAPE;

ESC_VALID
    : '\\\\'
    | '\\n'
    | '\\r'
    | '\\t'
    | '\\\''
    | '\\"'
    | '\\~'
    | '\\$'
    ;

ESC_CONTINUATION
    : '\\' '\r'? '\n'
    ;

ESC_VALID_OCTAL
    : '\\' [0-7] [0-7] [0-7]
    ;

ESC_INVALID_OCTAL
    : '\\' [0-9]
    ;

ESC_VALID_HEX
    : '\\x' [0-9a-fA-F] [0-9a-fA-F]
    ;

ESC_INVALID_HEX
    : '\\x'
    ;

ESC_VALID_UNICODE
    : '\\u' [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F]
    | '\\U' [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F] [0-9a-fA-F]
    ;

ESC_INVALID_SHORT_UNICODE
    : '\\u'
    ;

ESC_INVALID_UNICODE
    : '\\U'
    ;

ESC_NEWLINE
    : '\n'
    ;

ESC_TAB
    : '\t'
    ;

ESC_UNKNOWN
    : '\\' .
    ;

ESC_TEXT
    : ~[\\\n\t]+
    ;

// -----------------------------------------------------------------------------
// VERSION_DECLARATION mode: strict version value after `version`
// -----------------------------------------------------------------------------

mode VERSION_DECLARATION;

VERSION_DECLARATION_WHITESPACE
    : [ \t]+ -> channel(HIDDEN)
    ;

VERSION_NUMBER
    : '1' '.' [0-9]+ -> popMode
    ;

VERSION_DECLARATION_RECOVERY
    : . -> type(UNEXPECTED_CHAR), popMode
    ;
