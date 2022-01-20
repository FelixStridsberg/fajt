### Source
```js
new fn`test`;
```

### Output: minified
```js
new fn`test`
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
            "New": {
              "span": "0:12",
              "callee": {
                "TaggedTemplate": {
                  "span": "4:12",
                  "callee": {
                    "IdentRef": {
                      "span": "4:6",
                      "name": "fn"
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
              },
              "arguments_span": null,
              "arguments": []
            }
          }
        }
      }
    ]
  }
}
```
