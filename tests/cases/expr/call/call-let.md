### Source
```js
let();
```

### Output: minified
TODO the semicolon should not be there

```js
let();
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
        }
      }
    ]
  }
}
```
