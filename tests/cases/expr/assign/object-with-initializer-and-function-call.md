### Source
```js parse:expr
{ a = 1, b: c().d } = e
```

### Output: minified
```js
{a=1,b:c().d}=e
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:23",
    "operator": "Assign",
    "left": {
      "AssignmentPattern": {
        "Object": {
          "span": "0:19",
          "props": [
            {
              "Single": {
                "span": "2:7",
                "ident": {
                  "span": "2:3",
                  "name": "a"
                },
                "initializer": {
                  "Literal": {
                    "span": "6:7",
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
            },
            {
              "Named": {
                "span": "9:17",
                "name": {
                  "Ident": {
                    "span": "9:10",
                    "name": "b"
                  }
                },
                "value": {
                  "Expr": {
                    "Member": {
                      "span": "12:17",
                      "object": {
                        "Expr": {
                          "Call": {
                            "span": "12:15",
                            "callee": {
                              "Expr": {
                                "IdentRef": {
                                  "span": "12:13",
                                  "name": "c"
                                }
                              }
                            },
                            "arguments_span": "13:15",
                            "arguments": []
                          }
                        }
                      },
                      "property": {
                        "Ident": {
                          "span": "16:17",
                          "name": "d"
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
      "IdentRef": {
        "span": "22:23",
        "name": "e"
      }
    }
  }
}
```
