### Source
```js parse:stmt
function* fn() {
    (yield);
}
```

### Output: minified
```js
function*fn(){(yield)}
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
              "Parenthesized": {
                "span": "21:28",
                "expression": {
                  "Yield": {
                    "span": "22:27",
                    "argument": null,
                    "delegate": false
                  }
                }
              }
            }
          }
        }
      ]
    }
  }
}
```
