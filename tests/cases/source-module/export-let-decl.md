### Source
```js source:module
export const a = 1, b = 2, c
```

### Output: ast
```json
{
  "Module": {
    "span": "0:28",
    "body": [
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "0:28",
            "decl": {
              "Variable": {
                "span": "7:28",
                "kind": "Const",
                "declarations": [
                  {
                    "span": "13:18",
                    "pattern": {
                      "Ident": {
                        "span": "13:14",
                        "name": "a"
                      }
                    },
                    "initializer": {
                      "Literal": {
                        "span": "17:18",
                        "literal": {
                          "Number": {
                            "Integer": [
                              1,
                              "Decimal"
                            ]
                          }
                        }
                      }
                    }
                  },
                  {
                    "span": "20:25",
                    "pattern": {
                      "Ident": {
                        "span": "20:21",
                        "name": "b"
                      }
                    },
                    "initializer": {
                      "Literal": {
                        "span": "24:25",
                        "literal": {
                          "Number": {
                            "Integer": [
                              2,
                              "Decimal"
                            ]
                          }
                        }
                      }
                    }
                  },
                  {
                    "span": "27:28",
                    "pattern": {
                      "Ident": {
                        "span": "27:28",
                        "name": "c"
                      }
                    },
                    "initializer": null
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
