Class definitions are always strict mode.
```js
var cls = class {
  method1() {
    var implements = 1;
  }
}
```

```json
{
  "UnexpectedToken": {
    "value": {
      "Keyword": "Implements"
    },
    "first_on_line": false,
    "span": "40:50"
  }
}
```
