### Source
```js source:module
export let a = 1, b = 2, c
```

### Output: ast
```json
{
  "Module": {
    "span": "0:26",
    "body": [
      {
        "ExportDeclaration": {
          "Decl": {
            "span": "0:26",
            "decl": {
              "Variable": {
                "span": "7:26",
                "kind": "Let",
                "declarations": [
                  {
                    "span": "11:16",
                    "pattern": {
                      "Ident": {
                        "span": "11:12",
                        "name": "a"
                      }
                    },
                    "initializer": {
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
                    }
                  },
                  {
                    "span": "18:23",
                    "pattern": {
                      "Ident": {
                        "span": "18:19",
                        "name": "b"
                      }
                    },
                    "initializer": {
                      "Literal": {
                        "span": "22:23",
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
                    "span": "25:26",
                    "pattern": {
                      "Ident": {
                        "span": "25:26",
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
