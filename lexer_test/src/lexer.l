%%
[A-Za-z][A-Za-z0-9_]* "NAME"
\"[0-9]+\" "NUMBER"
\'.\' "QCHAR"
\\\\.*\n "COMMENT"
[-]?[1-9][0-9]*|0 "INT"
if "IF"
else "ELSE"
\!\= "NEQUAL"
return "RETURN"
\( "LPAR"
\( "RPAR"
\[ "LBRACE"
\] "RBRACE"
\{ "LBRACK"
\} "RBRACK"
\= "ASSIGN"
\; "SEMICOLON"
\, "COMMA"
\+ "PLUS"
\- "MINUS"
\* "TIMES"
\/ "DIVIDE"
\=\= "EQUAL"
char "CHAR"
write "WRITE"
read "READ"
\> "GREATER"
\< "LESS"
\! "NOT"
length "LENGTH"
while "WHILE"