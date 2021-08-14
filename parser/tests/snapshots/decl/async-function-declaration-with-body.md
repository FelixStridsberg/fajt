```js
async function fn() { var a = 1 }
```

```json
{
  "FunctionDecl": {
    "span": "0:33",
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
    "body": [
      {
        "Variable": {
          "span": "22:31",
          "kind": "Var",
          "declarations": [
            {
              "span": "26:31",
              "pattern": {
                "Ident": {
                  "span": "26:27",
                  "name": "a"
                }
              },
              "initializer": {
                "Literal": {
                  "span": "30:31",
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
```
