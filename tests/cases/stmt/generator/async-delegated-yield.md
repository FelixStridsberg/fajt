### Input
```js
async function* fn() {
    yield* a;
}
```

```js min
async function*fn(){yield*a}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:38",
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
      "span": "21:38",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "27:36",
            "expr": {
              "Yield": {
                "span": "27:35",
                "argument": {
                  "IdentRef": {
                    "span": "34:35",
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
