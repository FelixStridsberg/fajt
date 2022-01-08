### Input
```js
a--
```

### Output: ast
```json
{
  "Update": {
    "span": "0:3",
    "operator": "Decrease",
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
