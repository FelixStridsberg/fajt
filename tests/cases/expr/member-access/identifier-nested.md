### Input
```js parse:expr
a.b.c
```

### Output: ast
```json
{
  "Member": {
    "span": "0:5",
    "object": {
      "Expr": {
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
    },
    "property": {
      "Ident": {
        "span": "4:5",
        "name": "c"
      }
    }
  }
}
```
