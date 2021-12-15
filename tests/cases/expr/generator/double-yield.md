```js
function* fn() {
  yield yield
}
```

```json
{
  "Function": {
    "span": "0:32",
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
      "span": "15:32",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "19:30",
            "expr": {
              "Yield": {
                "span": "19:30",
                "argument": {
                  "Yield": {
                    "span": "25:30",
                    "argument": null,
                    "delegate": false
                  }
                },
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
