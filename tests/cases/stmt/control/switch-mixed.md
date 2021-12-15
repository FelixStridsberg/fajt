```js
switch (a) { case b: case c: d; default: }
```

```json
{
  "Switch": {
    "span": "0:42",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "13:20",
        "test": {
          "IdentRef": {
            "span": "18:19",
            "name": "b"
          }
        },
        "consequent": []
      },
      {
        "span": "21:31",
        "test": {
          "IdentRef": {
            "span": "26:27",
            "name": "c"
          }
        },
        "consequent": [
          {
            "Expr": {
              "span": "29:31",
              "expr": {
                "IdentRef": {
                  "span": "29:30",
                  "name": "d"
                }
              }
            }
          }
        ]
      },
      {
        "span": "32:40",
        "test": null,
        "consequent": []
      }
    ]
  }
}
```
