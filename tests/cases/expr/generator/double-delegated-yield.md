### Input
```js
function* fn() {
  yield* yield* yield
}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:40",
    "asynchronous": false,
    "generator": true,
    "identifier": {
      "span": "10:12",
      "name": "fn"
    },
    "parameters": {
      "span": "12:14",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "15:40",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "19:38",
            "expr": {
              "Yield": {
                "span": "19:38",
                "argument": {
                  "Yield": {
                    "span": "26:38",
                    "argument": {
                      "Yield": {
                        "span": "33:38",
                        "argument": null,
                        "delegate": false
                      }
                    },
                    "delegate": true
                  }
                },
                "delegate": true
              }
            }
          }
        }
      ]
    }
  }
}
```
