### Source
```js
/reg/;
```

### Output: minified
```js
/reg/
```

### Output: ast
```json
{
  "Script": {
    "span": "0:6",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:6",
          "expr": {
            "Literal": {
              "span": "0:5",
              "literal": {
                "Regexp": "/reg/"
              }
            }
          }
        }
      }
    ]
  }
}
```
