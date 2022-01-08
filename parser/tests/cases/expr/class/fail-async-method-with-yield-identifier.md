### Input
```js
class {
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
    "span": "32:37"
  }
}
```
