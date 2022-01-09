### Source
```js parse:expr
typeof a
```

### Output: minified
```js
typeof a
```

### Output: ast
```json
{
  "Unary": {
    "span": "0:8",
    "operator": "Typeof",
    "argument": {
      "IdentRef": {
        "span": "7:8",
        "name": "a"
      }
    }
  }
}
```
