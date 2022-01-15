### Source
```js parse:expr
a, b, c
```

### Output: ast
```json
{
  "Sequence": {
    "span": "0:7",
    "expr": [
      {
        "IdentRef": {
          "span": "0:1",
          "name": "a"
        }
      },
      {
        "IdentRef": {
          "span": "3:4",
          "name": "b"
        }
      },
      {
        "IdentRef": {
          "span": "6:7",
          "name": "c"
        }
      }
    ]
  }
}
```
