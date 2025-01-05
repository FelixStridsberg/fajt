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
    "directives": [],
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
                            "raw": "1"
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
                            "raw": "2"
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
