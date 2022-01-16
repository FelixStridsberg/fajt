### Source
```js
/reg/i;
```

### Output: minified
```js
/reg/i
```

### Output: ast
```json
{
  "Script": {
    "span": "0:7",
    "body": [
      {
        "Expr": {
          "span": "0:7",
          "expr": {
            "Literal": {
              "span": "0:6",
              "literal": {
                "Regexp": "/reg/i"
              }
            }
          }
        }
      }
    ]
  }
}
```
