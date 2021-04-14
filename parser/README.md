# Parser

Parses the tokens from the lexer and turns them into an abstract syntax tree (AST).

The AST should as far as possible be immutable, changes during transformations should be kept separately and applied
either by creating a whole new AST (which may not be suitable because of the performance hit), or mutate the tree but
as its own step rather than during traversal.