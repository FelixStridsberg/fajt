### Source
```js
call()`1``2`;
```

### Output: minified
```js
call()`1``2`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:13",
    "body": [
      {
        "Expr": {
          "span": "0:13",
          "expr": {
            "TaggedTemplate": {
              "span": "0:12",
              "callee": {
                "TaggedTemplate": {
                  "span": "0:9",
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
                        "String": "1"
                      }
                    ]
                  }
                }
              },
              "template": {
                "parts": [
                  {
                    "String": "2"
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
