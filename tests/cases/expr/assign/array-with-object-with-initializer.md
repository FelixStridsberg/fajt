### Source
```js parse:expr
[ { a = 1 } ] = b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:17",
    "operator": "Assign",
    "left": {
      "AssignmentPattern": {
        "Array": {
          "span": "0:13",
          "elements": [
            {
              "span": "2:11",
              "target": {
                "AssignmentPattern": {
                  "Object": {
                    "span": "2:11",
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
                                  "Integer": [
                                    1,
                                    "Decimal"
                                  ]
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
        "span": "16:17",
        "name": "b"
      }
    }
  }
}
```
