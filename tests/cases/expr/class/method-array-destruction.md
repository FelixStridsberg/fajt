### Source
```js parse:expr
class {
    method([ a ]) {}
}
```

### Output: minified
```js
class{method([a]){}}
```

### Output: ast
```json
{
  "Class": {
    "span": "0:30",
    "identifier": null,
    "super_class": null,
    "body": [
      {
        "Method": {
          "span": "12:28",
          "name": {
            "Ident": {
              "span": "12:18",
              "name": "method"
            }
          },
          "kind": "Method",
          "parameters": {
            "span": "18:25",
            "bindings": [
              {
                "span": "19:24",
                "pattern": {
                  "Array": {
                    "span": "19:24",
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
                    "rest": null
                  }
                },
                "initializer": null
              }
            ],
            "rest": null
          },
          "body": {
            "span": "26:28",
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
