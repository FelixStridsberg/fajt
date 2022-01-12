### Source
Class definitions are always strict mode.

```js
class cls {
  method1() {
    var implements = 1;
  }
}
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Keyword": "Implements"
    },
    "first_on_line": false,
    "span": "34:44"
  }
}
```