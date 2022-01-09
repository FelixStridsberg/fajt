### Source
```js
a
b
```

### Output: ast
```json
{
  "Script": {
    "span": "0:3",
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
          "span": "2:3",
          "expr": {
            "IdentRef": {
              "span": "2:3",
              "name": "b"
            }
          }
        }
      }
    ]
  }
}
```
