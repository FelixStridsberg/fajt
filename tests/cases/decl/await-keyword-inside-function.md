```js
function fn() { var await = 1 }
```

```json
{
  "FunctionDecl": {
    "span": "0:31",
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
      "span": "14:31",
      "directives": [],
      "statements": [
        {
          "Variable": {
            "span": "16:29",
            "kind": "Var",
            "declarations": [
              {
                "span": "20:29",
                "pattern": {
                  "Ident": {
                    "span": "20:25",
                    "name": "await"
                  }
                },
                "initializer": {
                  "Literal": {
                    "span": "28:29",
                    "literal": {
                      "Number": {
                        "Integer": [
                          1,
                          "Decimal"
                        ]
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
