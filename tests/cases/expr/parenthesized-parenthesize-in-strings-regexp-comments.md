### Source
This edge cases where the cover production for parenthesized expression tries to
read until next matching parenthesize.

```js check-format:no
(/(/ /*(*/, "(", '(', `(`);
(/)/ /*)*/, ")", ')', `)`);
```

### Output: minified
```js
(/(/,"(",'(',`(`);(/)/,")",')',`)`);
```

### Output: ast
```json
{
  "Script": {
    "span": "0:55",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:27",
          "expr": {
            "Parenthesized": {
              "span": "0:26",
              "expression": {
                "Sequence": {
                  "span": "1:25",
                  "expr": [
                    {
                      "Literal": {
                        "span": "1:4",
                        "literal": {
                          "Regexp": "/(/"
                        }
                      }
                    },
                    {
                      "Literal": {
                        "span": "12:15",
                        "literal": {
                          "String": {
                            "value": "(",
                            "delimiter": "\""
                          }
                        }
                      }
                    },
                    {
                      "Literal": {
                        "span": "17:20",
                        "literal": {
                          "String": {
                            "value": "(",
                            "delimiter": "'"
                          }
                        }
                      }
                    },
                    {
                      "Literal": {
                        "span": "22:25",
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
                  ]
                }
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "28:55",
          "expr": {
            "Parenthesized": {
              "span": "28:54",
              "expression": {
                "Sequence": {
                  "span": "29:53",
                  "expr": [
                    {
                      "Literal": {
                        "span": "29:32",
                        "literal": {
                          "Regexp": "/)/"
                        }
                      }
                    },
                    {
                      "Literal": {
                        "span": "40:43",
                        "literal": {
                          "String": {
                            "value": ")",
                            "delimiter": "\""
                          }
                        }
                      }
                    },
                    {
                      "Literal": {
                        "span": "45:48",
                        "literal": {
                          "String": {
                            "value": ")",
                            "delimiter": "'"
                          }
                        }
                      }
                    },
                    {
                      "Literal": {
                        "span": "50:53",
                        "literal": {
                          "Template": {
                            "parts": [
                              {
                                "String": ")"
                              }
                            ]
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
    ]
  }
}
```
