### Source
```js source:module
export default 1 + 10;
```

### Output: minified
```js
export default 1+10
```

### Output: ast
```json
{
  "Module": {
    "span": "0:22",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "DefaultExpr": {
            "span": "0:22",
            "expr": {
              "Binary": {
                "span": "15:21",
                "operator": "Plus",
                "left": {
                  "Literal": {
                    "span": "15:16",
                    "literal": {
                      "Number": {
                        "raw": "1"
                      }
                    }
                  }
                },
                "right": {
                  "Literal": {
                    "span": "19:21",
                    "literal": {
                      "Number": {
                        "raw": "10"
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
