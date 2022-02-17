# AST
This is an AST ([abstract syntax tree]) representation of ECMAScript source code.
It can be converted back to source code via the [codegen] crate.

The AST structure is inspired by [babel-parser ast] and [syn ast].


[abstract syntax tree]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[babel-parser ast]: https://github.com/babel/babel/blob/fa98a0a5b355a34b1c737d782f0e8969c55b72c6/packages/babel-parser/ast/spec.md
[syn ast]: https://github.com/dtolnay/syn/blob/8d24d277565648273337fa212229ccd9a4f6a48a/src/expr.rs
[codegen]: ../codegen/README.md
