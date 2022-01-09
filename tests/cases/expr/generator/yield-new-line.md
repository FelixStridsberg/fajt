### Source
```js parse:expr check-format:no
function* () {
    yield
    a
}
```

### Output: minified
```js
function*(){yield;a}
```

### Output: ast
```json
{
  "Function": {
    "span": "0:32",
    "asynchronous": false,
    "generator": true,
    "identifier": null,
    "parameters": {
      "span": "10:12",
      "bindings": [],
      "rest": null
    },
    "body": {
      "span": "13:32",
      "directives": [],
      "statements": [
        {
          "Expr": {
            "span": "19:24",
            "expr": {
              "Yield": {
                "span": "19:24",
                "argument": null,
                "delegate": false
              }
            }
          }
        },
        {
          "Expr": {
            "span": "29:30",
            "expr": {
              "IdentRef": {
                "span": "29:30",
                "name": "a"
              }
            }
          }
        }
      ]
    }
  }
}
```
