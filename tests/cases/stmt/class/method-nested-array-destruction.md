### Source
```js parse:stmt
class cls {
    method([ [ a ] ]) {}
}
```

### Output: minified
```js
class cls{method([[a]]){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:38",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:36",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:33",
            "bindings": [
              {
                "span": "23:32",
                "pattern": {
                  "Array": {
                    "span": "23:32",
                    "elements": [
                      {
                        "span": "25:30",
                        "pattern": {
                          "Array": {
                            "span": "25:30",
                            "elements": [
                              {
                                "span": "27:28",
                                "pattern": {
                                  "Ident": {
                                    "span": "27:28",
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
            "span": "34:36",
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
