### Source
```js
function* a() {
    ({ name(yield) {} });
    ({ set name(yield) {} });
}
```

### Output: minified
```js
function*a(){({name(yield){}})({set name(yield){}})}

```

### Output: ast
```json
{
  "Script": {
    "span": "0:73",
    "directives": [],
    "body": [
      {
        "FunctionDecl": {
          "span": "0:73",
          "asynchronous": false,
          "generator": true,
          "identifier": {
            "span": "10:11",
            "name": "a"
          },
          "parameters": {
            "span": "11:13",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "14:73",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "20:41",
                  "expr": {
                    "Parenthesized": {
                      "span": "20:40",
                      "expression": {
                        "Literal": {
                          "span": "21:39",
                          "literal": {
                            "Object": {
                              "props": [
                                {
                                  "Method": {
                                    "span": "23:37",
                                    "name": {
                                      "Ident": {
                                        "span": "23:27",
                                        "name": "name"
                                      }
                                    },
                                    "kind": "Method",
                                    "parameters": {
                                      "span": "27:34",
                                      "bindings": [
                                        {
                                          "span": "28:33",
                                          "pattern": {
                                            "Ident": {
                                              "span": "28:33",
                                              "name": "yield"
                                            }
                                          },
                                          "initializer": null
                                        }
                                      ],
                                      "rest": null
                                    },
                                    "body": {
                                      "span": "35:37",
                                      "directives": [],
                                      "statements": []
                                    },
                                    "generator": true,
                                    "asynchronous": false,
                                    "is_static": false
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
              },
              {
                "Expr": {
                  "span": "46:71",
                  "expr": {
                    "Parenthesized": {
                      "span": "46:70",
                      "expression": {
                        "Literal": {
                          "span": "47:69",
                          "literal": {
                            "Object": {
                              "props": [
                                {
                                  "Method": {
                                    "span": "49:67",
                                    "name": {
                                      "Ident": {
                                        "span": "53:57",
                                        "name": "name"
                                      }
                                    },
                                    "kind": "Set",
                                    "parameters": {
                                      "span": "57:64",
                                      "bindings": [
                                        {
                                          "span": "58:63",
                                          "pattern": {
                                            "Ident": {
                                              "span": "58:63",
                                              "name": "yield"
                                            }
                                          },
                                          "initializer": null
                                        }
                                      ],
                                      "rest": null
                                    },
                                    "body": {
                                      "span": "65:67",
                                      "directives": [],
                                      "statements": []
                                    },
                                    "generator": true,
                                    "asynchronous": false,
                                    "is_static": false
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
          }
        }
      }
    ]
  }
}
```
