### Source
```js
/reg[/]/;
```

### Output: minified
```js
/reg[/]/
```

### Output: ast
```json
{
  "Script": {
    "span": "0:9",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:9",
          "expr": {
            "Literal": {
              "span": "0:8",
              "literal": {
                "Regexp": "/reg[/]/"
              }
            }
          }
        }
      }
    ]
  }
}
```
