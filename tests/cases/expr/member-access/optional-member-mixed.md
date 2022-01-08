### Input
```js
a.b?.c[d]
```

### Output: ast
```json
{
  "OptionalMember": {
    "span": "0:9",
    "object": {
      "OptionalMember": {
        "span": "0:6",
        "object": {
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
        },
        "property": {
          "Ident": {
            "span": "5:6",
            "name": "c"
          }
        },
        "optional": true
      }
    },
    "property": {
      "Expr": {
        "IdentRef": {
          "span": "7:8",
          "name": "d"
        }
      }
    },
    "optional": false
  }
}
```
