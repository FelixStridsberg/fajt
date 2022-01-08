### Input
```js
async function* fn() {
    yield a;
}
```

### Output: minified
```js min
async function*fn(){yield a}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:37",
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
      "span": "21:37",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "27:35",
            "expr": {
              "Yield": {
                "span": "27:34",
                "argument": {
                  "IdentRef": {
                    "span": "33:34",
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
