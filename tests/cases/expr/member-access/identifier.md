### Source
```js parse:expr
a.b
```

### Output: ast
```json
{
  "Member": {
    "span": "0:3",
    "object": {
      "Expr": {
        "IdentRef": {
          "span": "0:1",
          "name": "a"
        }
      }
    },
    "property": {
      "Ident": {
        "span": "2:3",
        "name": "b"
      }
    }
  }
}
```
