%{
#include "Object.hpp"
#include "ObjLexer.hpp"

int yyerror(std::string);
int yyerror(char*);

%}

%output  "ObjParser.cpp"
%defines "ObjParser.hpp"

%define api.pure
%lex-param   { yyscan_t scanner }
%parse-param { SExpression **expression }
%parse-param { yyscan_t scanner }

%union {
  int int_value;
  float float_value;
  char* string_value;
}

%token TOKEN_SLASH
%token TOKEN_OFF
%token TOKEN_MATERIAL
%token TOKEN_FACE
%token TOKEN_VERTEX
%token TOKEN_NORMAL
%token TOKEN_TEXTURECOORDINATE

%token <string_value> TOKEN_STRING
%token <float_value> TOKEN_FLOAT
%token <int_value> TOKEN_INTEGER

%type <expression> expr

%%

input
    : input data
    ;

data
    : 
    | TOKEN_VERTEX value value
    ;

value
    : TOKEN_FLOAT { $$ = $1; }
    | TOKEN_INTEGER { $$ = $1; }
    ;

shading_value
    : TOKEN_FLOAT { $$ = $1; }
    ;

%%

int yyerror(std::string s)
{
  extern int yylineno; // defined and maintained in lex.c
  extern char *yytext; // defined and maintained in lex.c

  cerr << "ERROR: " << s << " at symbol \"" << yytext;
  cerr << "\" on line " << yylineno << "\n";
  exit(1);
}

int yyerror(char* s)
{
  return yyerror(std::string(s));
}
