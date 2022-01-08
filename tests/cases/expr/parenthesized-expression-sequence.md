### Input
```js
(a, b)
```

### Output: ast
```json
{
  "Parenthesized": {
    "span": "0:6",
    "expression": {
      "Sequence": {
        "span": "1:5",
        "expr": [
          {
            "IdentRef": {
              "span": "1:2",
              "name": "a"
            }
          },
          {
            "IdentRef": {
              "span": "4:5",
              "name": "b"
            }
          }
        ]
      }
    }
  }
}
```
