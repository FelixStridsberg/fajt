### Source
```js parse:expr
{ method({ a: { b: c } }) {} }
```

### Output: minified
```js
{method({a:{b:c}}){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:30",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:28",
              "name": {
                "Ident": {
                  "span": "2:8",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "8:25",
                "bindings": [
                  {
                    "span": "9:24",
                    "pattern": {
                      "Object": {
                        "span": "9:24",
                        "props": [
                          {
                            "Named": {
                              "span": "11:22",
                              "property": {
                                "Ident": {
                                  "span": "11:12",
                                  "name": "a"
                                }
                              },
                              "binding": {
                                "span": "14:22",
                                "pattern": {
                                  "Object": {
                                    "span": "14:22",
                                    "props": [
                                      {
                                        "Named": {
                                          "span": "16:20",
                                          "property": {
                                            "Ident": {
                                              "span": "16:17",
                                              "name": "b"
                                            }
                                          },
                                          "binding": {
                                            "span": "19:20",
                                            "pattern": {
                                              "Ident": {
                                                "span": "19:20",
                                                "name": "c"
                                              }
                                            },
                                            "initializer": null
                                          }
                                        }
                                      }
                                    ],
                                    "rest": null
                                  }
                                },
                                "initializer": null
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
              },
              "body": {
                "span": "26:28",
                "directives": [],
                "statements": []
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
