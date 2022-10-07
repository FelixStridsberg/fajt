### Source
```js check-format:no
export default ("a" in b);
```

### Output: minified
```js
export default("a"in b);
```

### Output: ast
```json
{
  "Module": {
    "span": "0:26",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "DefaultExpr": {
            "span": "0:26",
            "expr": {
              "Parenthesized": {
                "span": "15:25",
                "expression": {
                  "Binary": {
                    "span": "16:24",
                    "operator": "In",
                    "left": {
                      "Literal": {
                        "span": "16:19",
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
                        "span": "23:24",
                        "name": "b"
                      }
                    }
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
