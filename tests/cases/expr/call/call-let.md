### Source
```js
let()
```

### Output: minified
```js
let()
```

### Output: ast
```json
{
  "Script": {
    "span": "0:5",
    "directives": [],
    "body": [
      {
        "Call": {
          "span": "0:5",
          "callee": {
            "Expr": {
              "IdentRef": {
                "span": "0:3",
                "name": "let"
              }
            }
          },
          "arguments_span": "3:5",
          "arguments": []
        }
      }
    ]
  }
}
```
