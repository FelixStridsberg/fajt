### Source
```js parse:expr
new a(...b, c, ...[])
```

### Output: ast
```json
{
  "New": {
    "span": "0:21",
    "callee": {
      "IdentRef": {
        "span": "4:5",
        "name": "a"
      }
    },
    "arguments_span": "5:21",
    "arguments": [
      {
        "Spread": {
          "IdentRef": {
            "span": "9:10",
            "name": "b"
          }
        }
      },
      {
        "Expr": {
          "IdentRef": {
            "span": "12:13",
            "name": "c"
          }
        }
      },
      {
        "Spread": {
          "Literal": {
            "span": "18:20",
            "literal": {
              "Array": {
                "elements": []
              }
            }
          }
        }
      }
    ]
  }
}
```
