```js
function* fn() {
  yield
  a
}
```

```json
{
  "FunctionDecl": {
    "span": "0:30",
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
      "span": "15:30",
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
        },
        {
          "Expr": {
            "span": "27:28",
            "expr": {
              "IdentRef": {
                "span": "27:28",
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
