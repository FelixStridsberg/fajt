### Source
```js source:module
export var a = 1,
           b = 2,
           c;
```

### Output: minified
```js
export var a=1,b=2,c
```

### Output: ast
```json
{
  "Module": {
    "span": "0:49",
    "directives": [],
    "body": [
      {
        "ExportDecl": {
          "Decl": {
            "span": "0:49",
            "decl": {
              "Variable": {
                "span": "7:49",
                "kind": "Var",
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
                            "raw": "1"
                          }
                        }
                      }
                    }
                  },
                  {
                    "span": "29:34",
                    "pattern": {
                      "Ident": {
                        "span": "29:30",
                        "name": "b"
                      }
                    },
                    "initializer": {
                      "Literal": {
                        "span": "33:34",
                        "literal": {
                          "Number": {
                            "raw": "2"
                          }
                        }
                      }
                    }
                  },
                  {
                    "span": "47:48",
                    "pattern": {
                      "Ident": {
                        "span": "47:48",
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
