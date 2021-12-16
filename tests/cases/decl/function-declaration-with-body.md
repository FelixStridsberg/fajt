```js
function fn() {
    var a = 1;
}
```

```json
{
  "FunctionDecl": {
    "span": "0:32",
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
      "span": "14:32",
      "directives": [],
      "statements": [
        {
          "Variable": {
            "span": "20:30",
            "kind": "Var",
            "declarations": [
              {
                "span": "24:29",
                "pattern": {
                  "Ident": {
                    "span": "24:25",
                    "name": "a"
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
