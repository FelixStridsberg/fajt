```js
async function* fn() {
  yield* a
}
```

```json
{
  "FunctionDecl": {
    "span": "0:35",
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
      "span": "21:35",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "25:33",
            "expr": {
              "Yield": {
                "span": "25:33",
                "argument": {
                  "IdentRef": {
                    "span": "32:33",
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
