### Input
```js parse:stmt
switch (a) {
    case b:
        c;
        d;
}
```

### Output: minified
```js
switch(a){case b:c;d}
```

### Output: ast
```json
{
  "Switch": {
    "span": "0:48",
    "discriminant": {
      "IdentRef": {
        "span": "8:9",
        "name": "a"
      }
    },
    "cases": [
      {
        "span": "17:46",
        "test": {
          "IdentRef": {
            "span": "22:23",
            "name": "b"
          }
        },
        "consequent": [
          {
            "Expr": {
              "span": "33:35",
              "expr": {
                "IdentRef": {
                  "span": "33:34",
                  "name": "c"
                }
              }
            }
          },
          {
            "Expr": {
              "span": "44:46",
              "expr": {
                "IdentRef": {
                  "span": "44:45",
                  "name": "d"
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
