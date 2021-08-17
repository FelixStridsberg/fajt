```js
export default 1 + 10
```

```json
{
  "Module": {
    "span": "0:21",
    "body": [
      {
        "ExportDeclaration": {
          "DefaultExpr": {
            "span": "0:21",
            "expr": {
              "Binary": {
                "span": "15:21",
                "operator": "Plus",
                "left": {
                  "Literal": {
                    "span": "15:16",
                    "literal": {
                      "Number": {
                        "Integer": [
                          1,
                          "Decimal"
                        ]
                      }
                    }
                  }
                },
                "right": {
                  "Literal": {
                    "span": "19:21",
                    "literal": {
                      "Number": {
                        "Integer": [
                          10,
                          "Decimal"
                        ]
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
