### Source
```js parse:expr
{ a: { b: { c } } } = d
```

### Output: minified
```js
{a:{b:{c}}}=d
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
              "Named": {
                "span": "2:17",
                "name": {
                  "Ident": {
                    "span": "2:3",
                    "name": "a"
                  }
                },
                "value": {
                  "AssignmentPattern": {
                    "Object": {
                      "span": "5:17",
                      "props": [
                        {
                          "Named": {
                            "span": "7:15",
                            "name": {
                              "Ident": {
                                "span": "7:8",
                                "name": "b"
                              }
                            },
                            "value": {
                              "AssignmentPattern": {
                                "Object": {
                                  "span": "10:15",
                                  "props": [
                                    {
                                      "Single": {
                                        "span": "12:13",
                                        "ident": {
                                          "span": "12:13",
                                          "name": "c"
                                        },
                                        "initializer": null
                                      }
                                    }
                                  ],
                                  "rest": null
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
        "name": "d"
      }
    }
  }
}
```
