### Input
```js
function* () {
    yield
    *a
}
```

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
