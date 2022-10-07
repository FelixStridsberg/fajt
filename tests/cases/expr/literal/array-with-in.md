### Source
```js parse:expr
[ "a" in b, ...("c" in d ? [] : []) ]
```

### Output: minified
```js
["a"in b,...("c"in d?[]:[])]
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:37",
    "literal": {
      "Array": {
        "elements": [
          {
            "Expr": {
              "Binary": {
                "span": "2:10",
                "operator": "In",
                "left": {
                  "Literal": {
                    "span": "2:5",
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
                    "span": "9:10",
                    "name": "b"
                  }
                }
              }
            }
          },
          {
            "Spread": {
              "Parenthesized": {
                "span": "15:35",
                "expression": {
                  "Conditional": {
                    "span": "16:34",
                    "condition": {
                      "Binary": {
                        "span": "16:24",
                        "operator": "In",
                        "left": {
                          "Literal": {
                            "span": "16:19",
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
                            "span": "23:24",
                            "name": "d"
                          }
                        }
                      }
                    },
                    "consequent": {
                      "Literal": {
                        "span": "27:29",
                        "literal": {
                          "Array": {
                            "elements": []
                          }
                        }
                      }
                    },
                    "alternate": {
                      "Literal": {
                        "span": "32:34",
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
```
