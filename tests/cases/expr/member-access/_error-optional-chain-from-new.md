### Source
This may seem weird at first glance that `new new a()?.b` fails but `new a()?.b` is valid. That's
because only nested `new` are `NewExpression`, non nested `new` are `MemberExpression`.

```js parse:expr
new new a()?.b
```

### Output: error
```txt
Syntax error: Unexpected token `?.`
 --> test.js:1:12
  |
1 | new new a()?.b
  |            ^^ Unexpected token
```
