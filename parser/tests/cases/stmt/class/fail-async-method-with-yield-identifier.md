### Source
```js parse:stmt
class cls {
  async method1() {
    yield
  }
}
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Keyword": "Yield"
    },
    "first_on_line": true,
    "span": "36:41"
  }
}
```
