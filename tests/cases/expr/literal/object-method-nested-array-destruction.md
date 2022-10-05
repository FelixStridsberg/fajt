### Source
```js parse:expr
{ method([ [ a ] ]) {} }
```

### Output: minified
```js
{method([[a]]){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:24",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:22",
              "name": {
                "Ident": {
                  "span": "2:8",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "8:19",
                "bindings": [
                  {
                    "span": "9:18",
                    "pattern": {
                      "Array": {
                        "span": "9:18",
                        "elements": [
                          {
                            "span": "11:16",
                            "pattern": {
                              "Array": {
                                "span": "11:16",
                                "elements": [
                                  {
                                    "span": "13:14",
                                    "pattern": {
                                      "Ident": {
                                        "span": "13:14",
                                        "name": "a"
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
                      }
                    },
                    "initializer": null
                  }
                ],
                "rest": null
              },
              "body": {
                "span": "20:22",
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
