[![Test](https://github.com/FelixStridsberg/fajt/actions/workflows/test.yaml/badge.svg)](https://github.com/FelixStridsberg/fajt/actions/workflows/test.yaml)

# Experimental ECMAScript parser

This is a parser that parses ecmascript based on the ECMA-262 (edition 12)
specification.

The goal of this project is to provide:
- an as simple as possible recursive descent [parser] (with [lexer])
- [AST] representation with traversal algorithms
- basic [code generation]


[lexer]: ./lexer
[parser]: ./parser
[AST]: ./ast
[code generation]: ./codegen
