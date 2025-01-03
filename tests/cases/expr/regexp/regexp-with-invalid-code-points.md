### Source
```js
/\uD800\u{110000}/
```

### Output: ast
```json
{
  "Script": {
    "span": "0:18",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:18",
          "expr": {
            "Literal": {
              "span": "0:18",
              "literal": {
                "Regexp": "/\\uD800\\u{110000}/"
              }
            }
          }
        }
      }
    ]
  }
}
```
