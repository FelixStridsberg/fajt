### Source
```js
function fn() {
  "use strict";
  var let = 1;
}
```

### Output: error
```txt
Syntax error: Forbidden identifier `let`
 --> test.js:3:6
  |
3 |   var let = 1;
  |       ^^^ `let` is not allowed as an identifier in this context
```
