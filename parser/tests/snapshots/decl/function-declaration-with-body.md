```js
function fn() { var a = 1 }
```

```json
{
  "FunctionDecl": {
    "span": "0:27",
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
      "span": "14:27",
      "directives": [],
      "statements": [
        {
          "Variable": {
            "span": "16:25",
            "kind": "Var",
            "declarations": [
              {
                "span": "20:25",
                "pattern": {
                  "Ident": {
                    "span": "20:21",
                    "name": "a"
                  }
                },
                "initializer": {
                  "Literal": {
                    "span": "24:25",
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
