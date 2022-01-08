### Input
```js
function* fn() {
    yield* yield* yield;
}
```

### Output: minified
```js min
function*fn(){yield*yield*yield}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:43",
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
      "span": "15:43",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "21:41",
            "expr": {
              "Yield": {
                "span": "21:40",
                "argument": {
                  "Yield": {
                    "span": "28:40",
                    "argument": {
                      "Yield": {
                        "span": "35:40",
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
