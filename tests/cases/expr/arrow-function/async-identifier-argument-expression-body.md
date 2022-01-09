### Source
```js parse:expr
async a => b
```

### Output: ast
```json
{
  "ArrowFunction": {
    "span": "0:12",
    "asynchronous": true,
    "binding_parameter": true,
    "parameters": {
      "span": "6:7",
      "bindings": [
        {
          "span": "6:7",
          "pattern": {
            "Ident": {
              "span": "6:7",
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
          "span": "11:12",
          "name": "b"
        }
      }
    }
  }
}
```
