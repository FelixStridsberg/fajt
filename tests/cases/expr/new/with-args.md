### Input
```js parse:expr
new a(b, !null)
```

### Output: ast
```json
{
  "New": {
    "span": "0:15",
    "callee": {
      "IdentRef": {
        "span": "4:5",
        "name": "a"
      }
    },
    "arguments_span": "5:15",
    "arguments": [
      {
        "Expr": {
          "IdentRef": {
            "span": "6:7",
            "name": "b"
          }
        }
      },
      {
        "Expr": {
          "Unary": {
            "span": "9:14",
            "operator": "Not",
            "argument": {
              "Literal": {
                "span": "10:14",
                "literal": "Null"
              }
            }
          }
        }
      }
    ]
  }
}
```
