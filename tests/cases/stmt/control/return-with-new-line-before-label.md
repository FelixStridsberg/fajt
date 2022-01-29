### Source
```js check-format:no
break
a
```

### Output: minified
```js
break;a
```

### Output: ast
```json
{
  "Script": {
    "span": "0:7",
    "directives": [],
    "body": [
      {
        "Break": {
          "span": "0:5",
          "label": null
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
