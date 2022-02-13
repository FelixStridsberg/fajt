### Source
TODO bad formatting

```js parse:expr check-format:no
{
    method() {
        super[b]
    }
}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:41",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "6:39",
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
                "span": "15:39",
                "directives": [],
                "statements": [
                  {
                    "Expr": {
                      "span": "25:33",
                      "expr": {
                        "Member": {
                          "span": "25:33",
                          "object": {
                            "Super": {
                              "span": "25:30"
                            }
                          },
                          "property": {
                            "Expr": {
                              "IdentRef": {
                                "span": "31:32",
                                "name": "b"
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
              "asynchronous": false
            }
          }
        ]
      }
    }
  }
}
```
