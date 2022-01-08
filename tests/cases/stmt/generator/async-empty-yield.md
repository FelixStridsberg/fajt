### Input
```js parse:stmt
async function* fn() {
    yield;
}
```

### Output: minified
```js
async function*fn(){yield}
```

### Output: ast
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
            "span": "27:33",
            "expr": {
              "Yield": {
                "span": "27:32",
                "argument": null,
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
