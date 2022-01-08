### Input
```js
async function* fn() {
  yield
}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:32",
    "asynchronous": true,
    "generator": true,
    "identifier": {
      "span": "16:18",
      "name": "fn"
    },
    "parameters": {
      "span": "18:20",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "21:32",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "25:30",
            "expr": {
              "Yield": {
                "span": "25:30",
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
