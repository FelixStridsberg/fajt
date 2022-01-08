### Input
```js
function fn() {
  "use strict";
  var let = 1;
}
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Keyword": "Let"
    },
    "first_on_line": false,
    "span": "38:41"
  }
}
```
