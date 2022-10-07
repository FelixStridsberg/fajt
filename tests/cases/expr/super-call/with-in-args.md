### Source
```js parse:expr
class Test extends Test2 {
    constructor() {
        super("a" in b, ...("c" in d ? [] : []));
    }
}
```

### Output: minified
```js
class Test extends Test2{constructor(){super("a"in b,...("c"in d?[]:[]))}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:104",
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
          "span": "31:102",
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
            "span": "45:102",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "55:96",
                  "expr": {
                    "Call": {
                      "span": "55:95",
                      "callee": "Super",
                      "arguments_span": "60:95",
                      "arguments": [
                        {
                          "Expr": {
                            "Binary": {
                              "span": "61:69",
                              "operator": "In",
                              "left": {
                                "Literal": {
                                  "span": "61:64",
                                  "literal": {
                                    "String": {
                                      "value": "a",
                                      "delimiter": "\""
                                    }
                                  }
                                }
                              },
                              "right": {
                                "IdentRef": {
                                  "span": "68:69",
                                  "name": "b"
                                }
                              }
                            }
                          }
                        },
                        {
                          "Spread": {
                            "Parenthesized": {
                              "span": "74:94",
                              "expression": {
                                "Conditional": {
                                  "span": "75:93",
                                  "condition": {
                                    "Binary": {
                                      "span": "75:83",
                                      "operator": "In",
                                      "left": {
                                        "Literal": {
                                          "span": "75:78",
                                          "literal": {
                                            "String": {
                                              "value": "c",
                                              "delimiter": "\""
                                            }
                                          }
                                        }
                                      },
                                      "right": {
                                        "IdentRef": {
                                          "span": "82:83",
                                          "name": "d"
                                        }
                                      }
                                    }
                                  },
                                  "consequent": {
                                    "Literal": {
                                      "span": "86:88",
                                      "literal": {
                                        "Array": {
                                          "elements": []
                                        }
                                      }
                                    }
                                  },
                                  "alternate": {
                                    "Literal": {
                                      "span": "91:93",
                                      "literal": {
                                        "Array": {
                                          "elements": []
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
