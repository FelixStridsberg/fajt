### Source
```js parse:expr
[ a, , b ]
```

### Output: minified
```js
[a,,b]
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:10",
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
          "Elision",
          {
            "Expr": {
              "IdentRef": {
                "span": "7:8",
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
