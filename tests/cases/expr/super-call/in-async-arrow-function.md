### Source
TODO format error

```js check-format:no
class Test extends Test2 {
    constructor() {
        const a = async () => {
            super();
        };
    }
}
```

### Output: minified
```js
class Test extends Test2{constructor(){const a=async()=>{super()}}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:118",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:118",
          "identifier": {
            "span": "6:10",
            "name": "Test"
          },
          "super_class": {
            "IdentRef": {
              "span": "19:24",
              "name": "Test2"
            }
          },
          "body": [
            {
              "Method": {
                "span": "31:116",
                "name": {
                  "Ident": {
                    "span": "31:42",
                    "name": "constructor"
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "42:44",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "45:116",
                  "directives": [],
                  "statements": [
                    {
                      "Variable": {
                        "span": "55:110",
                        "kind": "Const",
                        "declarations": [
                          {
                            "span": "61:109",
                            "pattern": {
                              "Ident": {
                                "span": "61:62",
                                "name": "a"
                              }
                            },
                            "initializer": {
                              "ArrowFunction": {
                                "span": "65:109",
                                "asynchronous": true,
                                "binding_parameter": false,
                                "parameters": {
                                  "span": "71:73",
                                  "bindings": [],
                                  "rest": null
                                },
                                "body": {
                                  "Body": {
                                    "span": "77:109",
                                    "directives": [],
                                    "statements": [
                                      {
                                        "Expr": {
                                          "span": "91:99",
                                          "expr": {
                                            "Call": {
                                              "span": "91:98",
                                              "callee": "Super",
                                              "arguments_span": "96:98",
                                              "arguments": []
                                            }
                                          }
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
                },
                "generator": false,
                "asynchronous": false
              }
            }
          ]
        }
      }
    ]
  }
}
```
