# Lexer

The lexer reads the raw input (*[code points]*) from a stream and turns them into *[tokens]*. The lexer repeatedly reads
and consumes the longest possible sequence of code points to create a token, until the stream is empty.

### Example
```js
var a = 1 + 1;
```
will lex into:
```js
Keyword("var")
Identifier("a")
Punctuator("=")
Literal(Integer(1))
Punctuator("+")
Literal(Integer(1))
```

These tokens can then be [parsed] into an [AST].

### Relevant references
- [11 ECMAScript Language: Source Code]
- [12 ECMAScript Language: Lexical Grammar]


[code points]: https://262.ecma-international.org/12.0/#sec-ecmascript-language-source-code
[tokens]: https://262.ecma-international.org/12.0/#sec-ecmascript-language-source-code
[parsed]: ../parser
[ast]: ../ast
[11 ECMAScript Language: Source Code]: https://262.ecma-international.org/12.0/#sec-ecmascript-language-source-code
[12 ECMAScript Language: Lexical Grammar]: https://262.ecma-international.org/12.0/#sec-ecmascript-language-lexical-grammar
