### Source
This is a potentially tricky case when reading CoverCallExpressionAndAsyncArrow head production.

```js check-format:no
var a = async("(", '(', `(`, /(/ /*(*/);
```

### Output: minified
```js
var a=async("(",'(',`(`,/(/);
```

### Output: ast
```json
{
  "Script": {
    "span": "0:40",
    "directives": [],
    "body": [
      {
        "Variable": {
          "span": "0:40",
          "kind": "Var",
          "declarations": [
            {
              "span": "4:39",
              "pattern": {
                "Ident": {
                  "span": "4:5",
                  "name": "a"
                }
              },
              "initializer": {
                "Call": {
                  "span": "8:39",
                  "callee": {
                    "Expr": {
                      "IdentRef": {
                        "span": "8:13",
                        "name": "async"
                      }
                    }
                  },
                  "arguments_span": "13:39",
                  "arguments": [
                    {
                      "Expr": {
                        "Literal": {
                          "span": "14:17",
                          "literal": {
                            "String": {
                              "value": "(",
                              "delimiter": "\""
                            }
                          }
                        }
                      }
                    },
                    {
                      "Expr": {
                        "Literal": {
                          "span": "19:22",
                          "literal": {
                            "String": {
                              "value": "(",
                              "delimiter": "'"
                            }
                          }
                        }
                      }
                    },
                    {
                      "Expr": {
                        "Literal": {
                          "span": "24:27",
                          "literal": {
                            "Template": {
                              "parts": [
                                {
                                  "String": "("
                                }
                              ]
                            }
                          }
                        }
                      }
                    },
                    {
                      "Expr": {
                        "Literal": {
                          "span": "29:32",
                          "literal": {
                            "Regexp": "/(/"
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
    ]
  }
}
```
