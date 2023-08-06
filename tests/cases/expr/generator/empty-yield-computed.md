### Source
```js parse:expr
function* fn() {
    ({ [yield]: a } = {});
}
```

### Output: minified
```js
function*fn(){({[yield]:a}={})}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:45",
    "asynchronous": false,
    "generator": true,
    "identifier": {
      "span": "10:12",
      "name": "fn"
    },
    "parameters": {
      "span": "12:14",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "15:45",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "21:43",
            "expr": {
              "Parenthesized": {
                "span": "21:42",
                "expression": {
                  "Assignment": {
                    "span": "22:41",
                    "operator": "Assign",
                    "left": {
                      "Literal": {
                        "span": "22:36",
                        "literal": {
                          "Object": {
                            "props": [
                              {
                                "Named": {
                                  "span": "24:34",
                                  "name": {
                                    "Computed": {
                                      "Yield": {
                                        "span": "25:30",
                                        "argument": null,
                                        "delegate": false
                                      }
                                    }
                                  },
                                  "value": {
                                    "IdentRef": {
                                      "span": "33:34",
                                      "name": "a"
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
                        "span": "39:41",
                        "literal": {
                          "Object": {
                            "props": []
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
    }
  }
}
```
