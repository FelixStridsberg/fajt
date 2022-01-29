### Source
```js check-format:no
return
a
```

### Output: minified
```js
return;a
```

### Output: ast
```json
{
  "Script": {
    "span": "0:8",
    "directives": [],
    "body": [
      {
        "Return": {
          "span": "0:6",
          "argument": null
        }
      },
      {
        "Expr": {
          "span": "7:8",
          "expr": {
            "IdentRef": {
              "span": "7:8",
              "name": "a"
            }
          }
        }
      }
    ]
  }
}
```
