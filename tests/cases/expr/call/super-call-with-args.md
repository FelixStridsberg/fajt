### Source
```js parse:expr
super(a, b)
```

### Output: ast
```json
{
  "Call": {
    "span": "0:11",
    "callee": "Super",
    "arguments_span": "5:11",
    "arguments": [
      {
        "Expr": {
          "IdentRef": {
            "span": "6:7",
            "name": "a"
          }
        }
      },
      {
        "Expr": {
          "IdentRef": {
            "span": "9:10",
            "name": "b"
          }
        }
      }
    ]
  }
}
```
