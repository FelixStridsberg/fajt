This may seem weird at first glance that `new new a()?.b` fails but `new a()?.b` is valid. That's
because only nested `new` are `NewExpression`, non nested `new` are `MemberExpression`.

```js
new new a()?.b
```

```json
{
  "SyntaxError": [
    "Invalid optional chain from new expression",
    "11:13"
  ]
}
```
