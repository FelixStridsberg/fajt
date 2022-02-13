### Source
TODO bad formatting

```js check-format:no
const a = {
    async method() {
        super.a;
    }
}
```

### Output: minified
```js
const a={async method(){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:57",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:57",
          "kind": "Const",
          "declarations": [
            {
              "span": "6:57",
              "pattern": {
                "Ident": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "10:57",
                  "literal": {
                    "Object": {
                      "props": [
                        {
                          "Method": {
                            "span": "16:55",
                            "name": {
                              "Ident": {
                                "span": "22:28",
                                "name": "method"
                              }
                            },
                            "kind": "Method",
                            "parameters": {
                              "span": "28:30",
                              "bindings": [],
                              "rest": null
                            },
                            "body": {
                              "span": "31:55",
                              "directives": [],
                              "statements": [
                                {
                                  "Expr": {
                                    "span": "41:49",
                                    "expr": {
                                      "Member": {
                                        "span": "41:48",
                                        "object": {
                                          "Super": {
                                            "span": "41:46"
                                          }
                                        },
                                        "property": {
                                          "Ident": {
                                            "span": "47:48",
                                            "name": "a"
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              ]
                            },
                            "generator": false,
                            "asynchronous": true
                          }
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
    ]
  }
}
```
