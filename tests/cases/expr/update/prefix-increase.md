### Input
```js
++a
```

### Output: ast
```json
{
  "Update": {
    "span": "0:3",
    "operator": "Increase",
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
