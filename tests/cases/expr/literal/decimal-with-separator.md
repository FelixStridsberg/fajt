### Source
```js parse:expr check-format:no
123_456.7
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:9",
    "literal": {
      "Number": {
        "raw": "123_456.7"
      }
    }
  }
}
```
