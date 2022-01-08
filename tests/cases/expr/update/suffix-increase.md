### Input
```js parse:expr
a++
```

### Output: ast
```json
{
  "Update": {
    "span": "0:3",
    "operator": "Increase",
    "prefix": false,
    "argument": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    }
  }
}
```
