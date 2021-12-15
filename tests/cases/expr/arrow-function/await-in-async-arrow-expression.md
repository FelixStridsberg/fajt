```js
async () => await b
```

```json
{
  "ArrowFunction": {
    "span": "0:19",
    "asynchronous": true,
    "binding_parameter": false,
    "parameters": {
      "span": "6:8",
      "bindings": [],
      "rest": null
    },
    "body": {
      "Expr": {
        "Await": {
          "span": "12:19",
          "argument": {
            "IdentRef": {
              "span": "18:19",
              "name": "b"
            }
          }
        }
      }
    }
  }
}
```
