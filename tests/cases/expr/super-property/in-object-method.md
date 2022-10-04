### Source
TODO bad formatting

```js check-format:no
const a = {
    method() {
        super.a;
    }
}
```

### Output: minified
```js
const a={method(){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:51",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:51",
          "kind": "Const",
          "declarations": [
            {
              "span": "6:51",
              "pattern": {
                "Ident": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "10:51",
                  "literal": {
                    "Object": {
                      "props": [
                        {
                          "Method": {
                            "span": "16:49",
                            "name": {
                              "Ident": {
                                "span": "16:22",
                                "name": "method"
                              }
                            },
                            "kind": "Method",
                            "parameters": {
                              "span": "22:24",
                              "bindings": [],
                              "rest": null
                            },
                            "body": {
                              "span": "25:49",
                              "directives": [],
                              "statements": [
                                {
                                  "Expr": {
                                    "span": "35:43",
                                    "expr": {
                                      "Member": {
                                        "span": "35:42",
                                        "object": {
                                          "Super": {
                                            "span": "35:40"
                                          }
                                        },
                                        "property": {
                                          "Ident": {
                                            "span": "41:42",
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
