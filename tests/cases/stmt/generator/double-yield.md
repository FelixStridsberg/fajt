### Input
```js
function* fn() {
    yield yield;
}
```

```js min
function*fn(){yield yield}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:35",
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
      "span": "15:35",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "21:33",
            "expr": {
              "Yield": {
                "span": "21:32",
                "argument": {
                  "Yield": {
                    "span": "27:32",
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
