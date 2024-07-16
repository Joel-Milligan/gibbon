# Gibbon

## Program Flow

1. Text is passed to the lexer.
2. Lexer outputs discrete tokens based on the text.
3. Parser uses PRATT parsing on these tokens to construct the abstract syntax tree (AST).
4. The AST represents the program as a tree.