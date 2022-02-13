### Source
TODO bad formatting

```js check-format:no
const a = {
    set v(val) {
        super.a;
    }
}
```

### Output: minified
```js
const a={set v(val){super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:53",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:53",
          "kind": "Const",
          "declarations": [
            {
              "span": "6:53",
              "pattern": {
                "Ident": {
                  "span": "6:7",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "10:53",
                  "literal": {
                    "Object": {
                      "props": [
                        {
                          "Method": {
                            "span": "16:51",
                            "name": {
                              "Ident": {
                                "span": "20:21",
                                "name": "v"
                              }
                            },
                            "kind": "Set",
                            "parameters": {
                              "span": "21:26",
                              "bindings": [
                                {
                                  "span": "22:25",
                                  "pattern": {
                                    "Ident": {
                                      "span": "22:25",
                                      "name": "val"
                                    }
                                  },
                                  "initializer": null
                                }
                              ],
                              "rest": null
                            },
                            "body": {
                              "span": "27:51",
                              "directives": [],
                              "statements": [
                                {
                                  "Expr": {
                                    "span": "37:45",
                                    "expr": {
                                      "Member": {
                                        "span": "37:44",
                                        "object": {
                                          "Super": {
                                            "span": "37:42"
                                          }
                                        },
                                        "property": {
                                          "Ident": {
                                            "span": "43:44",
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
                            "asynchronous": false
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
