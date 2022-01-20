### Source
```js
new fn()`test`;
```

### Output: minified
```js
new fn()`test`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:15",
    "body": [
      {
        "Expr": {
          "span": "0:15",
          "expr": {
            "TaggedTemplate": {
              "span": "0:14",
              "callee": {
                "New": {
                  "span": "0:8",
                  "callee": {
                    "IdentRef": {
                      "span": "4:6",
                      "name": "fn"
                    }
                  },
                  "arguments_span": "6:8",
                  "arguments": []
                }
              },
              "template": {
                "parts": [
                  {
                    "String": "test"
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
