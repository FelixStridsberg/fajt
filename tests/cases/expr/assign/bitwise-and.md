### Input
```js parse:expr
a &= b;
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:6",
    "operator": "BitwiseAnd",
    "left": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    },
    "right": {
      "IdentRef": {
        "span": "5:6",
        "name": "b"
      }
    }
  }
}
```
