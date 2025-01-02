### Source
This is an edge case where the cover production for parenthesized expression parses
code points that is not a token.

```js parse:expr
((a = /#/), (b = /@/), (c = /☂/))
```

### Output: ast
```json
{
  "Parenthesized": {
    "span": "0:35",
    "expression": {
      "Sequence": {
        "span": "1:34",
        "expr": [
          {
            "Parenthesized": {
              "span": "1:10",
              "expression": {
                "Assignment": {
                  "span": "2:9",
                  "operator": "Assign",
                  "left": {
                    "Expr": {
                      "IdentRef": {
                        "span": "2:3",
                        "name": "a"
                      }
                    }
                  },
                  "right": {
                    "Literal": {
                      "span": "6:9",
                      "literal": {
                        "Regexp": "/#/"
                      }
                    }
                  }
                }
              }
            }
          },
          {
            "Parenthesized": {
              "span": "12:21",
              "expression": {
                "Assignment": {
                  "span": "13:20",
                  "operator": "Assign",
                  "left": {
                    "Expr": {
                      "IdentRef": {
                        "span": "13:14",
                        "name": "b"
                      }
                    }
                  },
                  "right": {
                    "Literal": {
                      "span": "17:20",
                      "literal": {
                        "Regexp": "/@/"
                      }
                    }
                  }
                }
              }
            }
          },
          {
            "Parenthesized": {
              "span": "23:34",
              "expression": {
                "Assignment": {
                  "span": "24:33",
                  "operator": "Assign",
                  "left": {
                    "Expr": {
                      "IdentRef": {
                        "span": "24:25",
                        "name": "c"
                      }
                    }
                  },
                  "right": {
                    "Literal": {
                      "span": "28:33",
                      "literal": {
                        "Regexp": "/☂/"
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
