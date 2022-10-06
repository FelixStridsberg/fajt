### Source
```js
a(/=/);
```

### Output: minified
```js
a(/=/);
```

### Output: ast
```json
{
  "Script": {
    "span": "0:7",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:7",
          "expr": {
            "Call": {
              "span": "0:6",
              "callee": {
                "Expr": {
                  "IdentRef": {
                    "span": "0:1",
                    "name": "a"
                  }
                }
              },
              "arguments_span": "1:6",
              "arguments": [
                {
                  "Expr": {
                    "Literal": {
                      "span": "2:5",
                      "literal": {
                        "Regexp": "/=/"
                      }
                    }
                  }
                }
              ]
            }
          }
        }
      }
    ]
  }
}
```
