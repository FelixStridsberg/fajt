### Source
```js parse:expr
a = b
```

### Output: minified
```js
a=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:5",
    "operator": "Assign",
    "left": {
      "Expr": {
        "IdentRef": {
          "span": "0:1",
          "name": "a"
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "4:5",
        "name": "b"
      }
    }
  }
}
```
