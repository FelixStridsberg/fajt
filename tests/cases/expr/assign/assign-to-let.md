### Source
```js parse:expr
let = b
```

### Output: minified
```js
let=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:7",
    "operator": "Assign",
    "left": {
      "IdentRef": {
        "span": "0:3",
        "name": "let"
      }
    },
    "right": {
      "IdentRef": {
        "span": "6:7",
        "name": "b"
      }
    }
  }
}
```
