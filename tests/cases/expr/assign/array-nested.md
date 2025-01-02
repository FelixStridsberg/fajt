### Source
```js parse:expr
[ [ [ a ] ] ] = b
```

### Output: minified
```js
[[[a]]]=b
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
                  "Array": {
                    "span": "2:11",
                    "elements": [
                      {
                        "span": "4:9",
                        "target": {
                          "AssignmentPattern": {
                            "Array": {
                              "span": "4:9",
                              "elements": [
                                {
                                  "span": "6:7",
                                  "target": {
                                    "Expr": {
                                      "IdentRef": {
                                        "span": "6:7",
                                        "name": "a"
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
                        "initializer": null
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
