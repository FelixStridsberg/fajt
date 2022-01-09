### Source
```js parse:expr
async => await
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:14",
    "asynchronous": false,
    "binding_parameter": true,
    "parameters": {
      "span": "0:5",
      "bindings": [
        {
          "span": "0:5",
          "pattern": {
            "Ident": {
              "span": "0:5",
              "name": "async"
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
          "span": "9:14",
          "name": "await"
        }
      }
    }
  }
}
```
