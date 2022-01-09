### Source
```js parse:expr
+a
```

### Output: minified
```js
+a
```

### Output: ast
```json
{
  "Unary": {
    "span": "0:2",
    "operator": "Plus",
    "argument": {
      "IdentRef": {
        "span": "1:2",
        "name": "a"
      }
    }
  }
}
```
