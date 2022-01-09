### Source
```js check-format:no
throw
a
```

### Output: minified
```js
throw;a
```

### Output: ast
```json
{
  "Script": {
    "span": "0:7",
    "body": [
      {
        "Throw": {
          "span": "0:5",
          "argument": null
        }
      },
      {
        "Expr": {
          "span": "6:7",
          "expr": {
            "IdentRef": {
              "span": "6:7",
              "name": "a"
            }
          }
        }
      }
    ]
  }
}
```
