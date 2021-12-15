```js
if ( a ) b; else c
```

```json
{
  "If": {
    "span": "0:18",
    "condition": {
      "IdentRef": {
        "span": "5:6",
        "name": "a"
      }
    },
    "consequent": {
      "Expr": {
        "span": "9:11",
        "expr": {
          "IdentRef": {
            "span": "9:10",
            "name": "b"
          }
        }
      }
    },
    "alternate": {
      "Expr": {
        "span": "17:18",
        "expr": {
          "IdentRef": {
            "span": "17:18",
            "name": "c"
          }
        }
      }
    }
  }
}
```
