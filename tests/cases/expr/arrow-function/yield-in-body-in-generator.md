### Source
```js
function* a() {
    () => {
        yield;
    };
}
```

### Output: minified
```js
function*a(){()=>{yield}}
```

### Output: ast
```json
{
  "Script": {
    "span": "0:51",
    "directives": [],
    "body": [
      {
        "FunctionDecl": {
          "span": "0:51",
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
            "span": "14:51",
            "directives": [],
            "statements": [
              {
                "Expr": {
                  "span": "20:49",
                  "expr": {
                    "ArrowFunction": {
                      "span": "20:48",
                      "asynchronous": false,
                      "binding_parameter": false,
                      "parameters": {
                        "span": "20:22",
                        "bindings": [],
                        "rest": null
                      },
                      "body": {
                        "Body": {
                          "span": "26:48",
                          "directives": [],
                          "statements": [
                            {
                              "Expr": {
                                "span": "36:42",
                                "expr": {
                                  "IdentRef": {
                                    "span": "36:41",
                                    "name": "yield"
                                  }
                                }
                              }
                            }
                          ]
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
