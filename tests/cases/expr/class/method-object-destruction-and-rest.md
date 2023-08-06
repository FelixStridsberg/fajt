### Source
```js parse:expr
class {
    method({ a, ...rest }) {}
}
```

### Output: minified
```js
class{method({a,...rest}){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:39",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:37",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:34",
            "bindings": [
              {
                "span": "19:33",
                "pattern": {
                  "Object": {
                    "span": "19:33",
                    "props": [
                      {
                        "Single": {
                          "span": "21:22",
                          "ident": {
                            "span": "21:22",
                            "name": "a"
                          },
                          "initializer": null
                        }
                      }
                    ],
                    "rest": {
                      "Ident": {
                        "span": "27:31",
                        "name": "rest"
                      }
                    }
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "35:37",
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
