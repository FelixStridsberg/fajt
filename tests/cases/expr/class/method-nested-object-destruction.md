### Source
```js parse:expr
class {
    method({ a: { b: c } }) {}
}
```

### Output: minified
```js
class{method({a:{b:c}}){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:40",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:38",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:35",
            "bindings": [
              {
                "span": "19:34",
                "pattern": {
                  "Object": {
                    "span": "19:34",
                    "props": [
                      {
                        "Named": {
                          "span": "21:32",
                          "property": {
                            "Ident": {
                              "span": "21:22",
                              "name": "a"
                            }
                          },
                          "binding": {
                            "span": "24:32",
                            "pattern": {
                              "Object": {
                                "span": "24:32",
                                "props": [
                                  {
                                    "Named": {
                                      "span": "26:30",
                                      "property": {
                                        "Ident": {
                                          "span": "26:27",
                                          "name": "b"
                                        }
                                      },
                                      "binding": {
                                        "span": "29:30",
                                        "pattern": {
                                          "Ident": {
                                            "span": "29:30",
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
            "span": "36:38",
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
