### Source
```js parse:expr
import(("a" in b))
```

### Output: ast
```json
{
  "Call": {
    "span": "0:18",
    "callee": "Import",
    "arguments_span": "6:18",
    "arguments": [
      {
        "Expr": {
          "Parenthesized": {
            "span": "7:17",
            "expression": {
              "Binary": {
                "span": "8:16",
                "operator": "In",
                "left": {
                  "Literal": {
                    "span": "8:11",
                    "literal": {
                      "String": {
                        "value": "a",
                        "delimiter": "\""
                      }
                    }
                  }
                },
                "right": {
                  "IdentRef": {
                    "span": "15:16",
                    "name": "b"
                  }
                }
              }
            }
          }
        }
      }
    ]
  }
}
```
