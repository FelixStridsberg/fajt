### Input
```js
{ ...a, ...b }
```

```json
{
  "Literal": {
    "span": "0:14",
    "literal": {
      "Object": {
        "props": [
          {
            "Spread": {
              "IdentRef": {
                "span": "5:6",
                "name": "a"
              }
            }
          },
          {
            "Spread": {
              "IdentRef": {
                "span": "11:12",
                "name": "b"
              }
            }
          }
        ]
      }
    }
  }
}
```
