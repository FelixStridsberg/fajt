### Input
```js
function* fn() {
    yield;
}
```

### Output: minified
```js min
function*fn(){yield}
```

### Output: ast
```json
{
  "FunctionDecl": {
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
            "span": "21:27",
            "expr": {
              "Yield": {
                "span": "21:26",
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
