```js
function* fn() {
  yield a
}
```

```json
{
  "Function": {
    "span": "0:28",
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
      "span": "15:28",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "19:26",
            "expr": {
              "Yield": {
                "span": "19:26",
                "argument": {
                  "IdentRef": {
                    "span": "25:26",
                    "name": "a"
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
