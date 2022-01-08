### Input
```js
switch (a) {
    case b:
    case c:
        d;
    default:
}
```

### Output: minified
```js min
switch(a){case b:case c:d;default:}
```

### Output: ast
```json
{
  "Switch": {
    "span": "0:62",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "17:24",
        "test": {
          "IdentRef": {
            "span": "22:23",
            "name": "b"
          }
        },
        "consequent": []
      },
      {
        "span": "29:47",
        "test": {
          "IdentRef": {
            "span": "34:35",
            "name": "c"
          }
        },
        "consequent": [
          {
            "Expr": {
              "span": "45:47",
              "expr": {
                "IdentRef": {
                  "span": "45:46",
                  "name": "d"
                }
              }
            }
          }
        ]
      },
      {
        "span": "52:60",
        "test": null,
        "consequent": []
      }
    ]
  }
}
```
