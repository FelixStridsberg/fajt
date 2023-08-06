### Source
```js
class Test extends Test2 {
    method() {
        () => super.a;
    }
}
```

### Output: minified
```js
class Test extends Test2{method(){()=>super.a}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:72",
    "directives": [],
    "body": [
      {
        "ClassDecl": {
          "span": "0:72",
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
                "span": "31:70",
                "name": {
                  "Ident": {
                    "span": "31:37",
                    "name": "method"
                  }
                },
                "kind": "Method",
                "parameters": {
                  "span": "37:39",
                  "bindings": [],
                  "rest": null
                },
                "body": {
                  "span": "40:70",
                  "directives": [],
                  "statements": [
                    {
                      "Expr": {
                        "span": "50:64",
                        "expr": {
                          "ArrowFunction": {
                            "span": "50:63",
                            "asynchronous": false,
                            "binding_parameter": false,
                            "parameters": {
                              "span": "50:52",
                              "bindings": [],
                              "rest": null
                            },
                            "body": {
                              "Expr": {
                                "Member": {
                                  "span": "56:63",
                                  "object": {
                                    "Super": {
                                      "span": "56:61"
                                    }
                                  },
                                  "property": {
                                    "Ident": {
                                      "span": "62:63",
                                      "name": "a"
                                    }
                                  }
                                }
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
    ]
  }
}
```
