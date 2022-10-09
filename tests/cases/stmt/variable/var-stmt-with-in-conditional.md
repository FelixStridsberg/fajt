### Source
```js
var a = "b" in c ? true : false,
    d = "e" in f ? true : false;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:65",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:65",
          "kind": "Var",
          "declarations": [
            {
              "span": "4:31",
              "pattern": {
                "Ident": {
                  "span": "4:5",
                  "name": "a"
                }
              },
              "initializer": {
                "Conditional": {
                  "span": "8:31",
                  "condition": {
                    "Binary": {
                      "span": "8:16",
                      "operator": "In",
                      "left": {
                        "Literal": {
                          "span": "8:11",
                          "literal": {
                            "String": {
                              "value": "b",
                              "delimiter": "\""
                            }
                          }
                        }
                      },
                      "right": {
                        "IdentRef": {
                          "span": "15:16",
                          "name": "c"
                        }
                      }
                    }
                  },
                  "consequent": {
                    "Literal": {
                      "span": "19:23",
                      "literal": {
                        "Boolean": true
                      }
                    }
                  },
                  "alternate": {
                    "Literal": {
                      "span": "26:31",
                      "literal": {
                        "Boolean": false
                      }
                    }
                  }
                }
              }
            },
            {
              "span": "37:64",
              "pattern": {
                "Ident": {
                  "span": "37:38",
                  "name": "d"
                }
              },
              "initializer": {
                "Conditional": {
                  "span": "41:64",
                  "condition": {
                    "Binary": {
                      "span": "41:49",
                      "operator": "In",
                      "left": {
                        "Literal": {
                          "span": "41:44",
                          "literal": {
                            "String": {
                              "value": "e",
                              "delimiter": "\""
                            }
                          }
                        }
                      },
                      "right": {
                        "IdentRef": {
                          "span": "48:49",
                          "name": "f"
                        }
                      }
                    }
                  },
                  "consequent": {
                    "Literal": {
                      "span": "52:56",
                      "literal": {
                        "Boolean": true
                      }
                    }
                  },
                  "alternate": {
                    "Literal": {
                      "span": "59:64",
                      "literal": {
                        "Boolean": false
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
  }
}
```
