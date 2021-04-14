# Fajt (Fast Advanced Javascript Tinkerer)


Concept:
```
let mut parser = Parser::new("file.js", ...);
let mut ast = parser.parse();

let mut codegen = Codegen::new(...);
codegen.write("out.js", ast, ...);
```


# Dependency tree

```
fajt ->
 |  parser -> lexer
 |  codegen -> parser -> lexer
```