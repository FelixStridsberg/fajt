### Source
```js parse:expr
a?.(b)
```

### Output: minified
```js
a?.(b)
```

### Output: ast
```json
{
  "OptionalCall": {
    "span": "0:6",
    "callee": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    },
    "arguments_span": "3:6",
    "arguments": [
      {
        "Expr": {
          "IdentRef": {
            "span": "4:5",
            "name": "b"
          }
        }
      }
    ],
    "optional": true
  }
}
```
