# The arrow function level problem

The syntax for ecma script allows the arrow function (`() => {}`) in the
`AssignmentExpression` production:

```
AssignmentExpression :
    ArrowFunction
    AsyncArrowFunction
```

However, an arrow function can be discovered much further down in the tree via
the cover productions:

```
CallExpression :
    CoverCallExpressionAndAsyncArrowHead

PrimaryExpression :
    CoverParenthesizedExpressionAndArrowParameterList
```

But just because an arrow function can be discovered at a lower level does not
mean it is allowed at that position.

This creates some complexity since when the arrow function is discovered, we
do not have any natural context to know if it is actually allowed at that
position.

