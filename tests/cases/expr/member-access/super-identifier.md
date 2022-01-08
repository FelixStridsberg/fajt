### Input
```js parse:expr
super.b
```

### Output: ast
```json
{
  "Member": {
    "span": "0:7",
    "object": {
      "Super": {
        "span": "0:5"
      }
    },
    "property": {
      "Ident": {
        "span": "6:7",
        "name": "b"
      }
    }
  }
}
```
