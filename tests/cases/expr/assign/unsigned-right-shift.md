### Source
```js parse:expr
a >>>= b
```

### Output: minified
```js
a>>>=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:8",
    "operator": "UnsignedRightShift",
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
        "span": "7:8",
        "name": "b"
      }
    }
  }
}
```
