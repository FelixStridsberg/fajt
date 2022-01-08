### Input
```js
new a.b()
```

```json
{
  "New": {
    "span": "0:9",
    "callee": {
      "Member": {
        "span": "4:7",
        "object": {
          "Expr": {
            "IdentRef": {
              "span": "4:5",
              "name": "a"
            }
          }
        },
        "property": {
          "Ident": {
            "span": "6:7",
            "name": "b"
          }
        }
      }
    },
    "arguments_span": "7:9",
    "arguments": []
  }
}
```
