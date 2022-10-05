### Source
```js parse:expr
{ method({ a, ...rest }) {} }
```

### Output: minified
```js
{method({a,...rest}){}}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:29",
    "literal": {
      "Object": {
        "props": [
          {
            "Method": {
              "span": "2:27",
              "name": {
                "Ident": {
                  "span": "2:8",
                  "name": "method"
                }
              },
              "kind": "Method",
              "parameters": {
                "span": "8:24",
                "bindings": [
                  {
                    "span": "9:23",
                    "pattern": {
                      "Object": {
                        "span": "9:23",
                        "props": [
                          {
                            "Single": {
                              "span": "11:12",
                              "ident": {
                                "span": "11:12",
                                "name": "a"
                              },
                              "initializer": null
                            }
                          }
                        ],
                        "rest": {
                          "span": "17:21",
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
                "span": "25:27",
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
