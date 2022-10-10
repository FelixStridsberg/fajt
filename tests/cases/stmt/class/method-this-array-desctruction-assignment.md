### Source
```js parse:stmt
class cls {
    method() {
        [ this.a ] = [ 1 ];
    }
}
```

### Output: minified
```js
class cls{method(){[this.a]=[1]}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:62",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:60",
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
            "span": "25:60",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "35:54",
                  "expr": {
                    "Assignment": {
                      "span": "35:53",
                      "operator": "Assign",
                      "left": {
                        "Literal": {
                          "span": "35:45",
                          "literal": {
                            "Array": {
                              "elements": [
                                {
                                  "Expr": {
                                    "Member": {
                                      "span": "37:43",
                                      "object": {
                                        "Expr": {
                                          "This": {
                                            "span": "37:41"
                                          }
                                        }
                                      },
                                      "property": {
                                        "Ident": {
                                          "span": "42:43",
                                          "name": "a"
                                        }
                                      }
                                    }
                                  }
                                }
                              ]
                            }
                          }
                        }
                      },
                      "right": {
                        "Literal": {
                          "span": "48:53",
                          "literal": {
                            "Array": {
                              "elements": [
                                {
                                  "Expr": {
                                    "Literal": {
                                      "span": "50:51",
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
