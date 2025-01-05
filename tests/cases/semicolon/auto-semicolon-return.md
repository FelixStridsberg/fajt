### Source
```js check-format:no
(function () {if (1) {return} return})
```

### Output: ast
```json
{
  "Script": {
    "span": "0:38",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:38",
          "expr": {
            "Parenthesized": {
              "span": "0:38",
              "expression": {
                "Function": {
                  "span": "1:37",
                  "asynchronous": false,
                  "generator": false,
                  "identifier": null,
                  "parameters": {
                    "span": "10:12",
                    "bindings": [],
                    "rest": null
                  },
                  "body": {
                    "span": "13:37",
                    "directives": [],
                    "statements": [
                      {
                        "If": {
                          "span": "14:29",
                          "condition": {
                            "Literal": {
                              "span": "18:19",
                              "literal": {
                                "Number": {
                                  "raw": "1"
                                }
                              }
                            }
                          },
                          "consequent": {
                            "Block": {
                              "span": "21:29",
                              "statements": [
                                {
                                  "Return": {
                                    "span": "22:28",
                                    "argument": null
                                  }
                                }
                              ]
                            }
                          },
                          "alternate": null
                        }
                      },
                      {
                        "Return": {
                          "span": "30:36",
                          "argument": null
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
    ]
  }
}
```
