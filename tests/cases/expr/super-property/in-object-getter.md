### Source
TODO bad formatting

```js check-format:no
const a = {
    get v() {
        super.a;
    }
}
```

### Output: minified
```js
const a={get v(){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:50",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:50",
          "kind": "Const",
          "declarations": [
            {
              "span": "6:50",
              "pattern": {
                "Ident": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "10:50",
                  "literal": {
                    "Object": {
                      "props": [
                        {
                          "Method": {
                            "span": "16:48",
                            "name": {
                              "Ident": {
                                "span": "20:21",
                                "name": "v"
                              }
                            },
                            "kind": "Get",
                            "parameters": {
                              "span": "21:23",
                              "bindings": [],
                              "rest": null
                            },
                            "body": {
                              "span": "24:48",
                              "directives": [],
                              "statements": [
                                {
                                  "Expr": {
                                    "span": "34:42",
                                    "expr": {
                                      "Member": {
                                        "span": "34:41",
                                        "object": {
                                          "Super": {
                                            "span": "34:39"
                                          }
                                        },
                                        "property": {
                                          "Ident": {
                                            "span": "40:41",
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
