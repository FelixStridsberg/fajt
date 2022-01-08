### Input
```js
const fn = () => {
  "use strict";
  var static = 1;
}
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Keyword": "Static"
    },
    "first_on_line": false,
    "span": "41:47"
  }
}
```
