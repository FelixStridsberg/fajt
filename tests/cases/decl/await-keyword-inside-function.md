### Source
```js parse:stmt
function fn() {
    var await = 1;
}
```

### Output: minified
```js
function fn(){var await=1}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:36",
    "asynchronous": false,
    "generator": false,
    "identifier": {
      "span": "9:11",
      "name": "fn"
    },
    "parameters": {
      "span": "11:13",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "14:36",
      "directives": [],
      "statements": [
        {
          "Variable": {
            "span": "20:34",
            "kind": "Var",
            "declarations": [
              {
                "span": "24:33",
                "pattern": {
                  "Ident": {
                    "span": "24:29",
                    "name": "await"
                  }
                },
                "initializer": {
                  "Literal": {
                    "span": "32:33",
                    "literal": {
                      "Number": {
                        "raw": "1"
                      }
                    }
                  }
                }
              }
            ]
          }
        }
      ]
    }
  }
}
```
