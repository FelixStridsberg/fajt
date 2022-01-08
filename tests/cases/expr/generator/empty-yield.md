### Input
```js
function* fn() {
  yield
}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:26",
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
      "span": "15:26",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "19:24",
            "expr": {
              "Yield": {
                "span": "19:24",
                "argument": null,
                "delegate": false
              }
            }
          }
        }
      ]
    }
  }
}
```
