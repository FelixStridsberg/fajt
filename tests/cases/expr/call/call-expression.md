### Source
```js parse:expr
fn(a, ...b)
```

### Output: minified
```js
fn(a,...b)
```

### Output: ast
```json
{
  "Call": {
    "span": "0:11",
    "callee": {
      "Expr": {
        "IdentRef": {
          "span": "0:2",
          "name": "fn"
        }
      }
    },
    "arguments_span": "2:11",
    "arguments": [
      {
        "Expr": {
          "IdentRef": {
            "span": "3:4",
            "name": "a"
          }
        }
      },
      {
        "Spread": {
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
