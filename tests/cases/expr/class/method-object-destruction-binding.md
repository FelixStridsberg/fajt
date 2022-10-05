### Source
```js parse:expr
class {
    method({ a: b }) {}
}
```

### Output: minified
```js
class{method({a:b}){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:33",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:31",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:28",
            "bindings": [
              {
                "span": "19:27",
                "pattern": {
                  "Object": {
                    "span": "19:27",
                    "props": [
                      {
                        "Named": {
                          "span": "21:25",
                          "property": {
                            "Ident": {
                              "span": "21:22",
                              "name": "a"
                            }
                          },
                          "binding": {
                            "span": "24:25",
                            "pattern": {
                              "Ident": {
                                "span": "24:25",
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
            "span": "29:31",
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
