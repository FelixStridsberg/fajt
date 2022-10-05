### Source
```js parse:stmt
class cls {
    method({ a: { b: c } }) {}
}
```

### Output: minified
```js
class cls{method({a:{b:c}}){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:44",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:42",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:39",
            "bindings": [
              {
                "span": "23:38",
                "pattern": {
                  "Object": {
                    "span": "23:38",
                    "props": [
                      {
                        "Named": {
                          "span": "25:36",
                          "property": {
                            "Ident": {
                              "span": "25:26",
                              "name": "a"
                            }
                          },
                          "binding": {
                            "span": "28:36",
                            "pattern": {
                              "Object": {
                                "span": "28:36",
                                "props": [
                                  {
                                    "Named": {
                                      "span": "30:34",
                                      "property": {
                                        "Ident": {
                                          "span": "30:31",
                                          "name": "b"
                                        }
                                      },
                                      "binding": {
                                        "span": "33:34",
                                        "pattern": {
                                          "Ident": {
                                            "span": "33:34",
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
            "span": "40:42",
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
```
