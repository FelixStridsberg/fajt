### Source
```js parse:expr
fn()
```

### Output: minified
```js
fn()
```

### Output: ast
```json
{
  "Call": {
    "span": "0:4",
    "callee": {
      "Expr": {
        "IdentRef": {
          "span": "0:2",
          "name": "fn"
        }
      }
    },
    "arguments_span": "2:4",
    "arguments": []
  }
}
```
