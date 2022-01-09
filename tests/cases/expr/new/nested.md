### Source
```js parse:expr
new new a
```

### Output: minified
```js
new new a
```

### Output: ast
```json
{
  "New": {
    "span": "0:9",
    "callee": {
      "New": {
        "span": "4:9",
        "callee": {
          "IdentRef": {
            "span": "8:9",
            "name": "a"
          }
        },
        "arguments_span": null,
        "arguments": []
      }
    },
    "arguments_span": null,
    "arguments": []
  }
}
```
