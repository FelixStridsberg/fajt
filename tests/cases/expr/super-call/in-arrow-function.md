### Source
TODO format error

```js check-format:no
class Test extends Test2 {
    constructor() {
        const a = () => {
            super();
        };
    }
}
```

### Output: minified
```js
class Test extends Test2{constructor(){const a=()=>{super()}}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:112",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:112",
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
                "span": "31:110",
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
                  "span": "45:110",
                  "directives": [],
                  "statements": [
                    {
                      "Variable": {
                        "span": "55:104",
                        "kind": "Const",
                        "declarations": [
                          {
                            "span": "61:103",
                            "pattern": {
                              "Ident": {
                                "span": "61:62",
                                "name": "a"
                              }
                            },
                            "initializer": {
                              "ArrowFunction": {
                                "span": "65:103",
                                "asynchronous": false,
                                "binding_parameter": false,
                                "parameters": {
                                  "span": "65:67",
                                  "bindings": [],
                                  "rest": null
                                },
                                "body": {
                                  "Body": {
                                    "span": "71:103",
                                    "directives": [],
                                    "statements": [
                                      {
                                        "Expr": {
                                          "span": "85:93",
                                          "expr": {
                                            "Call": {
                                              "span": "85:92",
                                              "callee": "Super",
                                              "arguments_span": "90:92",
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
