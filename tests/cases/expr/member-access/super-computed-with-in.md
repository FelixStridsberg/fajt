### Source
```js parse:expr check-format:no
{
    method() {
        super["a" in b]
    }
}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:48",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "6:46",
              "name": {
                "Ident": {
                  "span": "6:12",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "12:14",
                "bindings": [],
                "rest": null
              },
              "body": {
                "span": "15:46",
                "directives": [],
                "statements": [
                  {
                    "Expr": {
                      "span": "25:40",
                      "expr": {
                        "Member": {
                          "span": "25:40",
                          "object": {
                            "Super": {
                              "span": "25:30"
                            }
                          },
                          "property": {
                            "Expr": {
                              "Binary": {
                                "span": "31:39",
                                "operator": "In",
                                "left": {
                                  "Literal": {
                                    "span": "31:34",
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
                                    "span": "38:39",
                                    "name": "b"
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
              },
              "generator": false,
              "asynchronous": false,
              "is_static": false
            }
          }
        ]
      }
    }
  }
}
```
