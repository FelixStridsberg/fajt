### Input
```js parse:expr
super[b]
```

### Output: ast
```json
{
  "Member": {
    "span": "0:8",
    "object": {
      "Super": {
        "span": "0:5"
      }
    },
    "property": {
      "Expr": {
        "IdentRef": {
          "span": "6:7",
          "name": "b"
        }
      }
    }
  }
}
```
