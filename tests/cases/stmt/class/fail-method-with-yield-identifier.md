### Source
```js parse:stmt
class cls {
  method1() {
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
    "span": "30:35"
  }
}
```
