### Input
```js parse:expr
0xff
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:4",
    "literal": {
      "Number": {
        "Integer": [
          255,
          "Hex"
        ]
      }
    }
  }
}
```
