%{
#include "Object.hpp"
#include "ObjLexer.hpp"

int yyerror(yyscan_t scanner, SExpression **expression, const char *msg);

%}

%output  "ObjParser.cpp"
%defines "ObjParser.hpp"

%define api.pure
%lex-param   { yyscan_t scanner }
%parse-param { SExpression **expression }
%parse-param { yyscan_t scanner }

%union {
    int value;
    SExpression* expression;
}

%token TOKEN_SLASH
%token TOKEN_OFF
%token TOKEN_MATERIAL
%token TOKEN_FACE
%token TOKEN_SHADING
%token TOKEN_VERTEX
%token TOKEN_NORMAL
%token TOKEN_TEXTURECOORDINATE

%token <value> TOKEN_STRING
%token <value> TOKEN_FLOAT
%token <value> TOKEN_INTEGER

%type <expression> expr

%%

input
    : expr { *expression = $1; }
    ;

data
    : TOKEN_VERTEX expr { $$ = ; }
    | expr TOKEN_MULTIPLY expr { $$ = ; }
    | TOKEN_LPAREN expr TOKEN_RPAREN { $$ = $2; }
    | TOKEN_NUMBER { $$ = ; }
    ;

%%
