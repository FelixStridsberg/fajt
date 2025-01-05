### Source
```js parse:stmt
class cls {
    method() {
        ({ [this.a]: 1 });
    }
}
```

### Output: minified
```js
class cls{method(){({[this.a]:1})}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:61",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:59",
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
            "span": "25:59",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "35:53",
                  "expr": {
                    "Parenthesized": {
                      "span": "35:52",
                      "expression": {
                        "Literal": {
                          "span": "36:51",
                          "literal": {
                            "Object": {
                              "props": [
                                {
                                  "Named": {
                                    "span": "38:49",
                                    "name": {
                                      "Computed": {
                                        "Member": {
                                          "span": "39:45",
                                          "object": {
                                            "Expr": {
                                              "This": {
                                                "span": "39:43"
                                              }
                                            }
                                          },
                                          "property": {
                                            "Ident": {
                                              "span": "44:45",
                                              "name": "a"
                                            }
                                          }
                                        }
                                      }
                                    },
                                    "value": {
                                      "Literal": {
                                        "span": "48:49",
                                        "literal": {
                                          "Number": {
                                            "raw": "1"
                                          }
                                        }
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
```
