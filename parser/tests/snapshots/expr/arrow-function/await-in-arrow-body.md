```js
() => {
  await;
}
```

```json
{
  "ArrowFunction": {
    "span": "0:18",
    "asynchronous": false,
    "binding_parameter": false,
    "parameters": {
      "span": "0:2",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Block": [
        {
          "Expr": {
            "span": "10:16",
            "expr": {
              "IdentRef": {
                "span": "10:15",
                "name": "await"
              }
            }
          }
        }
      ]
    }
  }
}
```