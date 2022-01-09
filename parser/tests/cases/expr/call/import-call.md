### Source
```js parse:expr
import(a)
```

### Output: ast
```json
{
  "Call": {
    "span": "0:9",
    "callee": "Import",
    "arguments_span": "6:9",
    "arguments": [
      {
        "Expr": {
          "IdentRef": {
            "span": "7:8",
            "name": "a"
          }
        }
      }
    ]
  }
}
```
