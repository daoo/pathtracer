%{
#include <iostream>
#include <string>
#include "util/strings.hpp"
%}

digit [0-9]
char [a-zA-Z0-9\.]

%%

[ \t\n] ;
\#.*$   ;

[-]digit+\.digit* { yylval.float_value  = parse<float>(yytext); return TOKEN_FLOAT; }
[-]digit+         { yylval.int_value    = parse<int>(yytext); return TOKEN_INTEGER; }
char+             { yylval.string_value = yytext; return TOKEN_STRING; }

\/ { return TOKEN_SLASH; }

off   { return TOKEN_OFF; }
usmtl { return TOKEN_MATERIAL; }

f  { return TOKEN_FACE; }
v  { return TOKEN_VERTEX; }
vn { return TOKEN_NORMAL; }
vt { return TOKEN_TEXTURECOORDINATE; }

s  ;

. std::cerr << "Unknown token!\n"; yyterminate();

%%
