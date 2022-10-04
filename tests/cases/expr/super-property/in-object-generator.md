### Source
TODO bad formatting

```js check-format:no
const a = {
    *method() {
        super.a;
    }
}
```

### Output: minified
```js
const a={*method(){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:52",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:52",
          "kind": "Const",
          "declarations": [
            {
              "span": "6:52",
              "pattern": {
                "Ident": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "10:52",
                  "literal": {
                    "Object": {
                      "props": [
                        {
                          "Method": {
                            "span": "16:50",
                            "name": {
                              "Ident": {
                                "span": "17:23",
                                "name": "method"
                              }
                            },
                            "kind": "Method",
                            "parameters": {
                              "span": "23:25",
                              "bindings": [],
                              "rest": null
                            },
                            "body": {
                              "span": "26:50",
                              "directives": [],
                              "statements": [
                                {
                                  "Expr": {
                                    "span": "36:44",
                                    "expr": {
                                      "Member": {
                                        "span": "36:43",
                                        "object": {
                                          "Super": {
                                            "span": "36:41"
                                          }
                                        },
                                        "property": {
                                          "Ident": {
                                            "span": "42:43",
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
                            "asynchronous": false,
                            "is_static": false
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
