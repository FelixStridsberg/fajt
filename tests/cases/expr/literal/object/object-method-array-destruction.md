### Source
```js parse:expr
{ method([ a ]) {} }
```

### Output: minified
```js
{method([a]){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:20",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:18",
              "name": {
                "Ident": {
                  "span": "2:8",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "8:15",
                "bindings": [
                  {
                    "span": "9:14",
                    "pattern": {
                      "Array": {
                        "span": "9:14",
                        "elements": [
                          {
                            "span": "11:12",
                            "pattern": {
                              "Ident": {
                                "span": "11:12",
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
                "span": "16:18",
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
