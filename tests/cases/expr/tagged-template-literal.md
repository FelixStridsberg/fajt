### Source
```js
call()`template`;
```

### Output: minified
```js
call()`template`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:17",
    "body": [
      {
        "Expr": {
          "span": "0:17",
          "expr": {
            "TaggedTemplate": {
              "span": "0:16",
              "callee": {
                "Call": {
                  "span": "0:6",
                  "callee": {
                    "Expr": {
                      "IdentRef": {
                        "span": "0:4",
                        "name": "call"
                      }
                    }
                  },
                  "arguments_span": "4:6",
                  "arguments": []
                }
              },
              "template": {
                "parts": [
                  {
                    "String": "template"
                  }
                ]
              }
            }
          }
        }
      }
    ]
  }
}
```
