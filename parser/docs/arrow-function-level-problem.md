# The arrow function level problem

The syntax for ecma script allows the arrow function (`() => {}`) in the
`AssignmentExpression` production:
```
AssignmentExpression :
    ArrowFunction
    AsyncArrowFunction
```

However, an arrow function can be discovered much further down in the tree via
cover productions:
```
CallExpression :
    CoverCallExpressionAndAsyncArrowHead

PrimaryExpression :
    CoverParenthesizedExpressionAndArrowParameterList
```

But just because an arrow function can be discovered at a lower level does not
mean it is allowed there.

This creates some complexity since when the arrow function is discovered, we
do not have any natural context to know if it is actually allowed there.


# Potential solution

## Hand back control to a higher level of the tree

Concider this example:
```js
(() => {} + 1)
```

This expression is illegal because the `AddativeExpression` with the `+`
terminal symbol is defined as:
```
AddativeExpression :
    AddativeExpression + MultiplicativeExpression
```

The `AddativeExpression` may not contain an arrow function, but it may contain
the cover productions that may resolve to an arrow function.

The solution that feels most natural when the `AddativeExpression` resolves to
an arrow function is to "bubble up" the whole way to where that is valid, i.e.
`AssignmentExpression`, and continue from there. Then the `+` terminal would
result in the expected `Unexpected token '+'` error.

This "bubble up" can be achived in several ways. For example we could add the
cover productions to the AST and resolve to that, then handle that AST node in
the `AssignmentExpression` parser function. The drawback with this approach is
that the AST would be cluttered with cover productions that should never exist
in a resolved AST. And we would have to traverse the AST to find and convert
all potentially nested cover productions.

Another easier approach is to return an error when the cover production is
resolved to the (at that level) illegal arrow function, and handle that error
in the `AssignmentExpression` parser function to resolve it to a valid arrow
function and continue the parsing, now at the correct level. This is the
approach this parser has taken.


### Problem with nested productions

Concider this example with the solution above in mind:
```js
(typeof () => {})
```

This expression is illegal because the `UnaryExpression` with the `typeof`
terminal symbol is defined as:
```
UnaryExpression :
    typeof UnaryExpression
```

The `UnaryExpression` may not contain an arrow function, but it may contain a
cover production that resolves to an arrow function.

If we concider the solution proposed above, where we return an error when a
cover production resolves the arrow function inside the `UnaryExpression`,
we will find an edge case here.

The error will happen after we have successfully read the `typeof` token. If we
convert the error to a valid arrow function, the AST will look valid, but we
have lost the `typeof` token and altered the behaviour of the program.

However, this is easy to solve. Since arrow functions are only valid in an
`AssignmentPattern`, and that production does not allow any symbols before the
start of the arrow function, we know that the error can only be converted to
an arrow function if no other tokens have been read. I.e. the span of the arrow
function in the "bubbled up" error must match the position of when the
`AssignmentPattern` was last visited. Otherwise the arrow function exists in
an illegal position.

