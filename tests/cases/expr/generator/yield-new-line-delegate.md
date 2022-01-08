### Input
```js parse:expr
function* () {
    yield
    *a
}
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Punct": "Star"
    },
    "first_on_line": true,
    "span": "29:30"
  }
}
```
