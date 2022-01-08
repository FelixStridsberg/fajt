### Input
```js
function* fn() {
    yield* a;
}
```

```js min
function*fn(){yield*a}
```

### Output: ast
```json
{
  "FunctionDecl": {
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
            "span": "21:30",
            "expr": {
              "Yield": {
                "span": "21:29",
                "argument": {
                  "IdentRef": {
                    "span": "28:29",
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
