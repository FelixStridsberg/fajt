### Input
```js parse:expr
a[b]
```

### Output: ast
```json
{
  "Member": {
    "span": "0:4",
    "object": {
      "Expr": {
        "IdentRef": {
          "span": "0:1",
          "name": "a"
        }
      }
    },
    "property": {
      "Expr": {
        "IdentRef": {
          "span": "2:3",
          "name": "b"
        }
      }
    }
  }
}
```
