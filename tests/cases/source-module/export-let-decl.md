### Source
```js source:module
export const a = 1,
             b = 2,
             c;
```

### Output: minified
```js
export const a=1,b=2,c
```

### Output: ast
```json
{
  "Module": {
    "span": "0:55",
    "body": [
      {
        "ExportDecl": {
          "Decl": {
            "span": "0:55",
            "decl": {
              "Variable": {
                "span": "7:55",
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
                    "span": "33:38",
                    "pattern": {
                      "Ident": {
                        "span": "33:34",
                        "name": "b"
                      }
                    },
                    "initializer": {
                      "Literal": {
                        "span": "37:38",
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
                    "span": "53:54",
                    "pattern": {
                      "Ident": {
                        "span": "53:54",
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
