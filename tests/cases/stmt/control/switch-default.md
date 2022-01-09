### Source
```js parse:stmt
switch (a) {
    default:
        b;
}
```

### Output: minified
```js
switch(a){default:b}
```

### Output: ast
```json
{
  "Switch": {
    "span": "0:38",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "17:36",
        "test": null,
        "consequent": [
          {
            "Expr": {
              "span": "34:36",
              "expr": {
                "IdentRef": {
                  "span": "34:35",
                  "name": "b"
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
