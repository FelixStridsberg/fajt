### Input
```js
a[b][c]
```

### Output: ast
```json
{
  "Member": {
    "span": "0:7",
    "object": {
      "Expr": {
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
    },
    "property": {
      "Expr": {
        "IdentRef": {
          "span": "5:6",
          "name": "c"
        }
      }
    }
  }
}
```
