### Source
```js parse:stmt
do a;
while (true)
```

### Output: ast
```json
{
  "DoWhile": {
    "span": "0:18",
    "body": {
      "Expr": {
        "span": "3:5",
        "expr": {
          "IdentRef": {
            "span": "3:4",
            "name": "a"
          }
        }
      }
    },
    "test": {
      "Literal": {
        "span": "13:17",
        "literal": {
          "Boolean": true
        }
      }
    }
  }
}
```
