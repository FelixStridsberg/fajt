```js
async function fn() {
    var a = 1;
}
```

```json
{
  "FunctionDecl": {
    "span": "0:38",
    "asynchronous": true,
    "generator": false,
    "identifier": {
      "span": "15:17",
      "name": "fn"
    },
    "parameters": {
      "span": "17:19",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "20:38",
      "directives": [],
      "statements": [
        {
          "Variable": {
            "span": "26:36",
            "kind": "Var",
            "declarations": [
              {
                "span": "30:35",
                "pattern": {
                  "Ident": {
                    "span": "30:31",
                    "name": "a"
                  }
                },
                "initializer": {
                  "Literal": {
                    "span": "34:35",
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
