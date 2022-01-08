### Input
```js
class cls {
  async method1() {
    yield
  }
}
```

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
