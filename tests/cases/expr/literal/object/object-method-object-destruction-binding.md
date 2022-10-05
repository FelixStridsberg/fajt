### Source
```js parse:expr
{ method({ a: b }) {} }
```

### Output: minified
```js
{method({a:b}){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:23",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:21",
              "name": {
                "Ident": {
                  "span": "2:8",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "8:18",
                "bindings": [
                  {
                    "span": "9:17",
                    "pattern": {
                      "Object": {
                        "span": "9:17",
                        "props": [
                          {
                            "Named": {
                              "span": "11:15",
                              "property": {
                                "Ident": {
                                  "span": "11:12",
                                  "name": "a"
                                }
                              },
                              "binding": {
                                "span": "14:15",
                                "pattern": {
                                  "Ident": {
                                    "span": "14:15",
                                    "name": "b"
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
                "span": "19:21",
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
