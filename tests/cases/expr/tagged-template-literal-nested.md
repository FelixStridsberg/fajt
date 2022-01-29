### Source
```js
a`1``2`;
```

### Output: minified
```js
a`1``2`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:8",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:8",
          "expr": {
            "TaggedTemplate": {
              "span": "0:7",
              "callee": {
                "TaggedTemplate": {
                  "span": "0:4",
                  "callee": {
                    "IdentRef": {
                      "span": "0:1",
                      "name": "a"
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
