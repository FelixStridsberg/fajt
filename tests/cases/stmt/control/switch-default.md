```js
switch (a) { default: b }
```

```json
{
  "Switch": {
    "span": "0:25",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "13:23",
        "test": null,
        "consequent": [
          {
            "Expr": {
              "span": "22:23",
              "expr": {
                "IdentRef": {
                  "span": "22:23",
                  "name": "b"
                }
              }
            }
          }
        ]
      }
    ]
  }
}
```
