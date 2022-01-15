### Source
```js check-format:no
a // Comment
b // Comment
```

### Output: ast
```json
{
  "Script": {
    "span": "0:14",
    "body": [
      {
        "Expr": {
          "span": "0:1",
          "expr": {
            "IdentRef": {
              "span": "0:1",
              "name": "a"
            }
          }
        }
      },
      {
        "Expr": {
          "span": "13:14",
          "expr": {
            "IdentRef": {
              "span": "13:14",
              "name": "b"
            }
          }
        }
      }
    ]
  }
}
```
