### Input
```js
if (a)
    b;
else
    c;
```

```js min
if(a)b;else c
```

```json
{
  "If": {
    "span": "0:25",
    "condition": {
      "IdentRef": {
        "span": "4:5",
        "name": "a"
      }
    },
    "consequent": {
      "Expr": {
        "span": "11:13",
        "expr": {
          "IdentRef": {
            "span": "11:12",
            "name": "b"
          }
        }
      }
    },
    "alternate": {
      "Expr": {
        "span": "23:25",
        "expr": {
          "IdentRef": {
            "span": "23:24",
            "name": "c"
          }
        }
      }
    }
  }
}
```
