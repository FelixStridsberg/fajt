### Input
```js parse:stmt
function* fn() {
    yield a;
}
```

### Output: minified
```js
function*fn(){yield a}
```

### Output: ast
```json
{
  "FunctionDecl": {
    "span": "0:31",
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
      "span": "15:31",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "21:29",
            "expr": {
              "Yield": {
                "span": "21:28",
                "argument": {
                  "IdentRef": {
                    "span": "27:28",
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
