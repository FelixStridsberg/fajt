### Source
```js parse:expr
f1()["f2"]();
```

### Output: minified
```js
f1()["f2"]();
```

### Output: ast
```json
{
  "Call": {
    "span": "0:12",
    "callee": {
      "Expr": {
        "Member": {
          "span": "0:10",
          "object": {
            "Expr": {
              "Call": {
                "span": "0:4",
                "callee": {
                  "Expr": {
                    "IdentRef": {
                      "span": "0:2",
                      "name": "f1"
                    }
                  }
                },
                "arguments_span": "2:4",
                "arguments": []
              }
            }
          },
          "property": {
            "Expr": {
              "Literal": {
                "span": "5:9",
                "literal": {
                  "String": {
                    "value": "f2",
                    "delimiter": "\""
                  }
                }
              }
            }
          }
        }
      }
    },
    "arguments_span": "10:12",
    "arguments": []
  }
}
```
