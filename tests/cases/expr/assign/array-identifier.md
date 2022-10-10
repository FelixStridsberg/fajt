### Source
```js parse:expr
[ a ] = b
```

### Output: minified
```js
[a]=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:9",
    "operator": "Assign",
    "left": {
      "Literal": {
        "span": "0:5",
        "literal": {
          "Array": {
            "elements": [
              {
                "Expr": {
                  "IdentRef": {
                    "span": "2:3",
                    "name": "a"
                  }
                }
              }
            ]
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
```
