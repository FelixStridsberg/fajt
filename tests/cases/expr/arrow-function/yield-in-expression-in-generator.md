### Source
```js
function* a() {
    () => yield;
}
```

### Output: minified
```js
function*a(){()=>yield}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:34",
    "directives": [],
    "body": [
      {
        "FunctionDecl": {
          "span": "0:34",
          "asynchronous": false,
          "generator": true,
          "identifier": {
            "span": "10:11",
            "name": "a"
          },
          "parameters": {
            "span": "11:13",
            "bindings": [],
            "rest": null
          },
          "body": {
            "span": "14:34",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "20:32",
                  "expr": {
                    "ArrowFunction": {
                      "span": "20:31",
                      "asynchronous": false,
                      "binding_parameter": false,
                      "parameters": {
                        "span": "20:22",
                        "bindings": [],
                        "rest": null
                      },
                      "body": {
                        "Expr": {
                          "IdentRef": {
                            "span": "26:31",
                            "name": "yield"
                          }
                        }
                      }
                    }
                  }
                }
              }
            ]
          }
        }
      }
    ]
  }
}
```
