### Source
```js parse:stmt
class cls {
    method() {
        ({ a: this.a } = { a: 1 });
    }
}
```

### Output: minified
```js
class cls{method(){({a:this.a}={a:1})}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:70",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:68",
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
            "span": "25:68",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "35:62",
                  "expr": {
                    "Parenthesized": {
                      "span": "35:61",
                      "expression": {
                        "Assignment": {
                          "span": "36:60",
                          "operator": "Assign",
                          "left": {
                            "AssignmentPattern": {
                              "Object": {
                                "span": "36:49",
                                "props": [
                                  {
                                    "Named": {
                                      "span": "38:47",
                                      "name": {
                                        "Ident": {
                                          "span": "38:39",
                                          "name": "a"
                                        }
                                      },
                                      "value": {
                                        "Expr": {
                                          "Member": {
                                            "span": "41:47",
                                            "object": {
                                              "Expr": {
                                                "This": {
                                                  "span": "41:45"
                                                }
                                              }
                                            },
                                            "property": {
                                              "Ident": {
                                                "span": "46:47",
                                                "name": "a"
                                              }
                                            }
                                          }
                                        }
                                      },
                                      "initializer": null
                                    }
                                  }
                                ],
                                "rest": null
                              }
                            }
                          },
                          "right": {
                            "Literal": {
                              "span": "52:60",
                              "literal": {
                                "Object": {
                                  "props": [
                                    {
                                      "Named": {
                                        "span": "54:58",
                                        "name": {
                                          "Ident": {
                                            "span": "54:55",
                                            "name": "a"
                                          }
                                        },
                                        "value": {
                                          "Literal": {
                                            "span": "57:58",
                                            "literal": {
                                              "Number": {
                                                "Integer": [
                                                  1,
                                                  "Decimal"
                                                ]
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
