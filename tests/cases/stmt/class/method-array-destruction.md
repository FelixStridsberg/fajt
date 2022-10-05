### Source
```js parse:stmt
class cls {
    method([ a ]) {}
}
```

### Output: minified
```js
class cls{method([a]){}}
```

### Output: ast
```json
{
  "ClassDecl": {
    "span": "0:34",
    "identifier": {
      "span": "6:9",
      "name": "cls"
    },
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "16:32",
          "name": {
            "Ident": {
              "span": "16:22",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "22:29",
            "bindings": [
              {
                "span": "23:28",
                "pattern": {
                  "Array": {
                    "span": "23:28",
                    "elements": [
                      {
                        "span": "25:26",
                        "pattern": {
                          "Ident": {
                            "span": "25:26",
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
