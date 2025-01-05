### Source
```js parse:stmt
class cls {
    method() {
        this.a = 1;
    }
}
```

### Output: minified
```js
class cls{method(){this.a=1}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:54",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:52",
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
            "span": "25:52",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "35:46",
                  "expr": {
                    "Assignment": {
                      "span": "35:45",
                      "operator": "Assign",
                      "left": {
                        "Expr": {
                          "Member": {
                            "span": "35:41",
                            "object": {
                              "Expr": {
                                "This": {
                                  "span": "35:39"
                                }
                              }
                            },
                            "property": {
                              "Ident": {
                                "span": "40:41",
                                "name": "a"
                              }
                            }
                          }
                        }
                      },
                      "right": {
                        "Literal": {
                          "span": "44:45",
                          "literal": {
                            "Number": {
                              "raw": "1"
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
