### Source
```js parse:expr
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
