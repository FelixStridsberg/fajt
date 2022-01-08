### Input
```js parse:expr
--a
```

### Output: ast
```json
{
  "Update": {
    "span": "0:3",
    "operator": "Decrease",
    "prefix": true,
    "argument": {
      "IdentRef": {
        "span": "2:3",
        "name": "a"
      }
    }
  }
}
```
