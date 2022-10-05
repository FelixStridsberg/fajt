### Source
```js parse:expr check-format:no
1_234
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:5",
    "literal": {
      "Number": {
        "Integer": [
          1234,
          "Decimal"
        ]
      }
    }
  }
}
```
