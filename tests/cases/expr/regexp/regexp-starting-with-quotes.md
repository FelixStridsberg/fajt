### Source
```js
/"reg"/;
/'reg'/;
```

### Output: minified
```js
/"reg"/;/'reg'/
```

### Output: ast
```json
{
  "Script": {
    "span": "0:17",
    "body": [
      {
        "Expr": {
          "span": "0:8",
          "expr": {
            "Literal": {
              "span": "0:7",
              "literal": {
                "Regexp": "/\"reg\"/"
              }
            }
          }
        }
      },
      {
        "Expr": {
          "span": "9:17",
          "expr": {
            "Literal": {
              "span": "9:16",
              "literal": {
                "Regexp": "/'reg'/"
              }
            }
          }
        }
      }
    ]
  }
}
```
