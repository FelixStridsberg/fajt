### Source
```js parse:stmt
class cls {
    method({ a: b }) {}
}
```

### Output: minified
```js
class cls{method({a:b}){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:37",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:35",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:32",
            "bindings": [
              {
                "span": "23:31",
                "pattern": {
                  "Object": {
                    "span": "23:31",
                    "props": [
                      {
                        "Named": {
                          "span": "25:29",
                          "property": {
                            "Ident": {
                              "span": "25:26",
                              "name": "a"
                            }
                          },
                          "binding": {
                            "span": "28:29",
                            "pattern": {
                              "Ident": {
                                "span": "28:29",
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
            "span": "33:35",
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
