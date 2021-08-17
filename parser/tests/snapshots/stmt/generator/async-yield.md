```js
async function* fn() {
  yield a
}
```

```json
{
  "FunctionDecl": {
    "span": "0:34",
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
      "span": "21:34",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "25:32",
            "expr": {
              "Yield": {
                "span": "25:32",
                "argument": {
                  "IdentRef": {
                    "span": "31:32",
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
