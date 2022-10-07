### Source
```js parse:expr
("a" in b)
```

### Output: ast
```json
{
  "Parenthesized": {
    "span": "0:10",
    "expression": {
      "Binary": {
        "span": "1:9",
        "operator": "In",
        "left": {
          "Literal": {
            "span": "1:4",
            "literal": {
              "String": {
                "value": "a",
                "delimiter": "\""
              }
            }
          }
        },
        "right": {
          "IdentRef": {
            "span": "8:9",
            "name": "b"
          }
        }
      }
    }
  }
}
```
