### Input
```js parse:expr
class {
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
    "span": "26:31"
  }
}
```
