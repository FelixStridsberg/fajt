### Source
```js parse:expr
([{a = b}]) => 1
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:16",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:11",
      "bindings": [
        {
          "span": "1:10",
          "pattern": {
            "Array": {
              "span": "1:10",
              "elements": [
                {
                  "span": "2:9",
                  "pattern": {
                    "Object": {
                      "span": "2:9",
                      "props": [
                        {
                          "Single": {
                            "span": "3:8",
                            "ident": {
                              "span": "3:4",
                              "name": "a"
                            },
                            "initializer": {
                              "IdentRef": {
                                "span": "7:8",
                                "name": "b"
                              }
                            }
                          }
                        }
                      ],
                      "rest": null
                    }
                  },
                  "initializer": null
                }
              ],
              "rest": null
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "Expr": {
        "Literal": {
          "span": "15:16",
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
```
