### Input
```js parse:expr
class {
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
    "span": "32:37"
  }
}
```
