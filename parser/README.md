# Parser
This is a [recursive descent parser] of ECMAScript.

The parser consumes *tokens* from the [lexer] and turns them into an [AST]. During the parsing it performs
[static semantic] analysis to produce [early errors].


### Relevant documentation
[16 ECMAScript Language: Scripts and Modules] is the entry point, follow the productions down from there.

In the source code you'll see comments like
```
/// Parses the `IdentifierReference` production.
```
search for `IdentifierReference` in the ECMA-262 specification, and you'll find the relevant specification for that
piece of code.

Note that some productions that are on the same level are combined in the same function in the code, because they can't
be parsed unambiguously in isolation.


[lexer]: ../lexer/README.md
[AST]: ../ast/README.md
[static semantic]: https://262.ecma-international.org/12.0/#sec-static-semantic-rules
[early errors]: https://262.ecma-international.org/12.0/#early-error
[recursive descent parser]: https://en.wikipedia.org/wiki/Recursive_descent_parser
[16 ECMAScript Language: Scripts and Modules]: https://262.ecma-international.org/12.0/#sec-ecmascript-language-scripts-and-modules
