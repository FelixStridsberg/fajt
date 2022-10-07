### Source
```js parse:expr
a?.["b" in c]
```

### Output: minified
```js
a?.["b"in c]
```

### Output: ast
```json
{
  "OptionalMember": {
    "span": "0:13",
    "object": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    },
    "property": {
      "Expr": {
        "Binary": {
          "span": "4:12",
          "operator": "In",
          "left": {
            "Literal": {
              "span": "4:7",
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
              "span": "11:12",
              "name": "c"
            }
          }
        }
      }
    },
    "optional": true
  }
}
```
