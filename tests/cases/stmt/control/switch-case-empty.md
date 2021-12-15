```js
switch (a) { case b: }
```

```json
{
  "Switch": {
    "span": "0:22",
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
      }
    ]
  }
}
```
