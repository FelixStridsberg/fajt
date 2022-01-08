### Input
```js
[ a,, b ]
```

```json
{
  "Literal": {
    "span": "0:9",
    "literal": {
      "Array": {
        "elements": [
          {
            "Expr": {
              "IdentRef": {
                "span": "2:3",
                "name": "a"
              }
            }
          },
          "None",
          {
            "Expr": {
              "IdentRef": {
                "span": "6:7",
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
