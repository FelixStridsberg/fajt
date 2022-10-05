### Source
```js parse:expr
(`${a}`)
```

### Output: ast
```json
{
  "Parenthesized": {
    "span": "0:8",
    "expression": {
      "Literal": {
        "span": "1:7",
        "literal": {
          "Template": {
            "parts": [
              {
                "Expr": {
                  "IdentRef": {
                    "span": "4:5",
                    "name": "a"
                  }
                }
              }
            ]
          }
        }
      }
    }
  }
}
```
