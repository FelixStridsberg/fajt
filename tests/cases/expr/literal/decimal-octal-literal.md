### Input
```js parse:expr
0o77
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:4",
    "literal": {
      "Number": {
        "Integer": [
          63,
          "Octal"
        ]
      }
    }
  }
}
```
