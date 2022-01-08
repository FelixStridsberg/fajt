### Input
```js parse:expr
~a
```

### Output: ast
```json
{
  "Unary": {
    "span": "0:2",
    "operator": "BitwiseNot",
    "argument": {
      "IdentRef": {
        "span": "1:2",
        "name": "a"
      }
    }
  }
}
```
