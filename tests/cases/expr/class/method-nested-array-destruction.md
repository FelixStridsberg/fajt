### Source
```js parse:expr
class {
    method([ [ a ] ]) {}
}
```

### Output: minified
```js
class{method([[a]]){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:34",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:32",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:29",
            "bindings": [
              {
                "span": "19:28",
                "pattern": {
                  "Array": {
                    "span": "19:28",
                    "elements": [
                      {
                        "span": "21:26",
                        "pattern": {
                          "Array": {
                            "span": "21:26",
                            "elements": [
                              {
                                "span": "23:24",
                                "pattern": {
                                  "Ident": {
                                    "span": "23:24",
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
            "span": "30:32",
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
