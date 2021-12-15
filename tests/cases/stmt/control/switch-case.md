```js
switch (a) { case b: c; d }
```

```json
{
  "Switch": {
    "span": "0:27",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "13:25",
        "test": {
          "IdentRef": {
            "span": "18:19",
            "name": "b"
          }
        },
        "consequent": [
          {
            "Expr": {
              "span": "21:23",
              "expr": {
                "IdentRef": {
                  "span": "21:22",
                  "name": "c"
                }
              }
            }
          },
          {
            "Expr": {
              "span": "24:25",
              "expr": {
                "IdentRef": {
                  "span": "24:25",
                  "name": "d"
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
