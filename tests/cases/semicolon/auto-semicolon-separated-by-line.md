### Source
```js check-format:no
a
b
```

### Output: ast
```json
{
  "Script": {
    "span": "0:3",
    "directives": [],
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
