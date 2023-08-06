### Source
```js parse:expr
() => {
    yield;
}
```

### Output: minified
```js
()=>{yield}
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:20",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:2",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Body": {
        "span": "6:20",
        "directives": [],
        "statements": [
          {
            "Expr": {
              "span": "12:18",
              "expr": {
                "IdentRef": {
                  "span": "12:17",
                  "name": "yield"
                }
              }
            }
          }
        ]
      }
    }
  }
}
```
