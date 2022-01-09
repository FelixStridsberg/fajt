### Source
```js parse:expr
new a
```

### Output: minified
```js
new a
```

### Output: ast
```json
{
  "New": {
    "span": "0:5",
    "callee": {
      "IdentRef": {
        "span": "4:5",
        "name": "a"
      }
    },
    "arguments_span": null,
    "arguments": []
  }
}
```
