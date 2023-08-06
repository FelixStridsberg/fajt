### Source
```js parse:expr
function* (param) {
    function a(yield) {
        yield;
    }
}
```

### Output: minified
```js
function*(param){function a(yield){yield}}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:66",
    "asynchronous": false,
    "generator": true,
    "identifier": null,
    "parameters": {
      "span": "10:17",
      "bindings": [
        {
          "span": "11:16",
          "pattern": {
            "Ident": {
              "span": "11:16",
              "name": "param"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "span": "18:66",
      "directives": [],
      "statements": [
        {
          "FunctionDecl": {
            "span": "24:64",
            "asynchronous": false,
            "generator": false,
            "identifier": {
              "span": "33:34",
              "name": "a"
            },
            "parameters": {
              "span": "34:41",
              "bindings": [
                {
                  "span": "35:40",
                  "pattern": {
                    "Ident": {
                      "span": "35:40",
                      "name": "yield"
                    }
                  },
                  "initializer": null
                }
              ],
              "rest": null
            },
            "body": {
              "span": "42:64",
              "directives": [],
              "statements": [
                {
                  "Expr": {
                    "span": "52:58",
                    "expr": {
                      "IdentRef": {
                        "span": "52:57",
                        "name": "yield"
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
}
```
