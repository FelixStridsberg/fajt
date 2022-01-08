### Input
```js parse:expr
function* () {
  yield
  a
}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:28",
    "asynchronous": false,
    "generator": true,
    "identifier": null,
    "parameters": {
      "span": "10:12",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "13:28",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "17:22",
            "expr": {
              "Yield": {
                "span": "17:22",
                "argument": null,
                "delegate": false
              }
            }
          }
        },
        {
          "Expr": {
            "span": "25:26",
            "expr": {
              "IdentRef": {
                "span": "25:26",
                "name": "a"
              }
            }
          }
        }
      ]
    }
  }
}
```
