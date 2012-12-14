%{
#include <iostream>
#include <string>
#include "util/parse.hpp"
%}

digit [0-9]
char [a-zA-Z0-9\.]

%%

[ \t\n] ;
\#.*$   ;

char+             { yylval.str = yytext; return TOKEN_STRING; }
[-]digit+\.digit* { yylval.floating = parse<float>(yytext); return TOKEN_FLOAT; }
[-]digit+         { yylval.integer = prase<int>(yytext); return TOKEN_INTEGER; }

\/ { return TOKEN_SLASH; }

off   { return TOKEN_OFF; }
usmtl { return TOKEN_MATERIAL; }

f  { return TOKEN_FACE; }
s  { return TOKEN_SHADING; }
v  { return TOKEN_VERTEX; }
vn { return TOKEN_NORMAL; }
vt { return TOKEN_TEXTURECOORDINATE; }

. std::cerr << "Unknown token!\n"; yyterminate();

%%
