### Input
```js
function* fn() {
  yield* a
}
```

```json
{
  "Function": {
    "span": "0:29",
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
      "span": "15:29",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "19:27",
            "expr": {
              "Yield": {
                "span": "19:27",
                "argument": {
                  "IdentRef": {
                    "span": "26:27",
                    "name": "a"
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
