### Source
```js parse:expr
a["b" in c]
```

### Output: ast
```json
{
  "Member": {
    "span": "0:11",
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
        "Binary": {
          "span": "2:10",
          "operator": "In",
          "left": {
            "Literal": {
              "span": "2:5",
              "literal": {
                "String": {
                  "value": "b",
                  "delimiter": "\""
                }
              }
            }
          },
          "right": {
            "IdentRef": {
              "span": "9:10",
              "name": "c"
            }
          }
        }
      }
    }
  }
}
```
