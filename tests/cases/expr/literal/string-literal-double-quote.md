### Source
```js parse:expr
"this is string"
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:16",
    "literal": {
      "String": {
        "value": "this is string",
        "delimiter": "\""
      }
    }
  }
}
```
