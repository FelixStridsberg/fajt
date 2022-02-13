### Source
TODO bad formatting

```js check-format:no
const a = {
    async *method() {
        super.a;
    }
}
```

### Output: minified
```js
const a={async*method(){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:58",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:58",
          "kind": "Const",
          "declarations": [
            {
              "span": "6:58",
              "pattern": {
                "Ident": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "10:58",
                  "literal": {
                    "Object": {
                      "props": [
                        {
                          "Method": {
                            "span": "16:56",
                            "name": {
                              "Ident": {
                                "span": "23:29",
                                "name": "method"
                              }
                            },
                            "kind": "Method",
                            "parameters": {
                              "span": "29:31",
                              "bindings": [],
                              "rest": null
                            },
                            "body": {
                              "span": "32:56",
                              "directives": [],
                              "statements": [
                                {
                                  "Expr": {
                                    "span": "42:50",
                                    "expr": {
                                      "Member": {
                                        "span": "42:49",
                                        "object": {
                                          "Super": {
                                            "span": "42:47"
                                          }
                                        },
                                        "property": {
                                          "Ident": {
                                            "span": "48:49",
                                            "name": "a"
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              ]
                            },
                            "generator": true,
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
