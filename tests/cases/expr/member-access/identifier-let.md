### Source
```js parse:expr
let.let
```

### Output: ast
```json
{
  "Member": {
    "span": "0:7",
    "object": {
      "Expr": {
        "IdentRef": {
          "span": "0:3",
          "name": "let"
        }
      }
    },
    "property": {
      "Ident": {
        "span": "4:7",
        "name": "let"
      }
    }
  }
}
```
