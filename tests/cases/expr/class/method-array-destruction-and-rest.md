### Source
```js parse:expr
class {
    method([ a, ...rest ]) {}
}
```

### Output: minified
```js
class{method([a,...rest]){}}
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
                  "Array": {
                    "span": "19:33",
                    "elements": [
                      {
                        "span": "21:22",
                        "pattern": {
                          "Ident": {
                            "span": "21:22",
                            "name": "a"
                          }
                        },
                        "initializer": null
                      }
                    ],
                    "rest": {
                      "span": "27:31",
                      "name": "rest"
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
