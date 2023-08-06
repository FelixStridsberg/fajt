### Source
```js parse:stmt
class cls {
    method({ a, ...rest }) {}
}
```

### Output: minified
```js
class cls{method({a,...rest}){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:43",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:41",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:38",
            "bindings": [
              {
                "span": "23:37",
                "pattern": {
                  "Object": {
                    "span": "23:37",
                    "props": [
                      {
                        "Single": {
                          "span": "25:26",
                          "ident": {
                            "span": "25:26",
                            "name": "a"
                          },
                          "initializer": null
                        }
                      }
                    ],
                    "rest": {
                      "Ident": {
                        "span": "31:35",
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
            "span": "39:41",
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
