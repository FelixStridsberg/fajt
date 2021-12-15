```js
a[b].c[d]
```

```json
{
  "Member": {
    "span": "0:9",
    "object": {
      "Expr": {
        "Member": {
          "span": "0:6",
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
            "Ident": {
              "span": "5:6",
              "name": "c"
            }
          }
        }
      }
    },
    "property": {
      "Expr": {
        "IdentRef": {
          "span": "7:8",
          "name": "d"
        }
      }
    }
  }
}
```
