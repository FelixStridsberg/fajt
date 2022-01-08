### Input
```js
class cls {
  method1() {
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
    "span": "30:35"
  }
}
```
