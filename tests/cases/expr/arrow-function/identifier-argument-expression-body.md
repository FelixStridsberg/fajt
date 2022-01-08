### Input
```js parse:expr
a => b
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:6",
    "asynchronous": false,
    "binding_parameter": true,
    "parameters": {
      "span": "0:1",
      "bindings": [
        {
          "span": "0:1",
          "pattern": {
            "Ident": {
              "span": "0:1",
              "name": "a"
            }
          },
          "initializer": null
        }
      ],
      "rest": null
    },
    "body": {
      "Expr": {
        "IdentRef": {
          "span": "5:6",
          "name": "b"
        }
      }
    }
  }
}
```
