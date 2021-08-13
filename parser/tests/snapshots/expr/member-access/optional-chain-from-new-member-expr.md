This may seem weird at first glance that `new new a()?.b` fails but `new a()?.b` is valid. That's
because only nested `new` are `NewExpression`, non nested `new` are `MemberExpression`.

```js
new a()?.b
```

```json
{
  "OptionalMember": {
    "span": "0:10",
    "object": {
      "New": {
        "span": "0:7",
        "callee": {
          "IdentRef": {
            "span": "4:5",
            "name": "a"
          }
        },
        "arguments_span": "5:7",
        "arguments": []
      }
    },
    "property": {
      "Ident": {
        "span": "9:10",
        "name": "b"
      }
    },
    "optional": true
  }
}
```
