### Source
This is an edge case where cover reads passed a regexp that contains tokens
that would generate lexer error.

```js parse:expr
((a = /1__2__/))
```

### Output: ast
```json
{
  "Parenthesized": {
    "span": "0:16",
    "expression": {
      "Parenthesized": {
        "span": "1:15",
        "expression": {
          "Assignment": {
            "span": "2:14",
            "operator": "Assign",
            "left": {
              "IdentRef": {
                "span": "2:3",
                "name": "a"
              }
            },
            "right": {
              "Literal": {
                "span": "6:14",
                "literal": {
                  "Regexp": "/1__2__/"
                }
              }
            }
          }
        }
      }
    }
  }
}
```
