### Source
```js check-format:no
a
a;
```

### Output: ast
```json
{
  "Script": {
    "span": "0:4",
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
          "span": "2:4",
          "expr": {
            "IdentRef": {
              "span": "2:3",
              "name": "a"
            }
          }
        }
      }
    ]
  }
}
```
