```js
if ( a ) b
```

```json
{
  "If": {
    "span": "0:10",
    "condition": {
      "IdentRef": {
        "span": "5:6",
        "name": "a"
      }
    },
    "consequent": {
      "Expr": {
        "span": "9:10",
        "expr": {
          "IdentRef": {
            "span": "9:10",
            "name": "b"
          }
        }
      }
    },
    "alternate": null
  }
}
```
