### Source
```js parse:stmt
if (a)
    b;
```

### Output: minified
```js
if(a)b;
```

### Output: ast
```json
{
  "If": {
    "span": "0:13",
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
    "alternate": null
  }
}
```
