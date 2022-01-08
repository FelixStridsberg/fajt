### Input
```js parse:expr
delete a
```

### Output: ast
```json
{
  "Unary": {
    "span": "0:8",
    "operator": "Delete",
    "argument": {
      "IdentRef": {
        "span": "7:8",
        "name": "a"
      }
    }
  }
}
```
