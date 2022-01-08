### Input
```js
a?.b
```

### Output: ast
```json
{
  "OptionalMember": {
    "span": "0:4",
    "object": {
      "IdentRef": {
        "span": "0:1",
        "name": "a"
      }
    },
    "property": {
      "Ident": {
        "span": "3:4",
        "name": "b"
      }
    },
    "optional": true
  }
}
```
