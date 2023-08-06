### Source
```js
class Test extends Test2 {
    constructor() {
        const a = () => super();
    }
}
```

### Output: minified
```js
class Test extends Test2{constructor(){const a=()=>super()}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:87",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:87",
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
                "span": "31:85",
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
                  "span": "45:85",
                  "directives": [],
                  "statements": [
                    {
                      "Variable": {
                        "span": "55:79",
                        "kind": "Const",
                        "declarations": [
                          {
                            "span": "61:78",
                            "pattern": {
                              "Ident": {
                                "span": "61:62",
                                "name": "a"
                              }
                            },
                            "initializer": {
                              "ArrowFunction": {
                                "span": "65:78",
                                "asynchronous": false,
                                "binding_parameter": false,
                                "parameters": {
                                  "span": "65:67",
                                  "bindings": [],
                                  "rest": null
                                },
                                "body": {
                                  "Expr": {
                                    "Call": {
                                      "span": "71:78",
                                      "callee": "Super",
                                      "arguments_span": "76:78",
                                      "arguments": []
                                    }
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
                "asynchronous": false,
                "is_static": false
              }
            }
          ]
        }
      }
    ]
  }
}
```
