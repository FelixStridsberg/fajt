### Source
```js parse:expr
[ { a = 1, b = 2 } ] = c
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:24",
    "operator": "Assign",
    "left": {
      "AssignmentPattern": {
        "Array": {
          "span": "0:20",
          "elements": [
            {
              "span": "2:18",
              "target": {
                "AssignmentPattern": {
                  "Object": {
                    "span": "2:18",
                    "props": [
                      {
                        "Single": {
                          "span": "4:9",
                          "ident": {
                            "span": "4:5",
                            "name": "a"
                          },
                          "initializer": {
                            "Literal": {
                              "span": "8:9",
                              "literal": {
                                "Number": {
                                  "raw": "1"
                                }
                              }
                            }
                          }
                        }
                      },
                      {
                        "Single": {
                          "span": "11:16",
                          "ident": {
                            "span": "11:12",
                            "name": "b"
                          },
                          "initializer": {
                            "Literal": {
                              "span": "15:16",
                              "literal": {
                                "Number": {
                                  "raw": "2"
                                }
                              }
                            }
                          }
                        }
                      }
                    ],
                    "rest": null
                  }
                }
              },
              "initializer": null
            }
          ],
          "rest": null
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "23:24",
        "name": "c"
      }
    }
  }
}
```
