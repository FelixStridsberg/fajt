### Source
```js
const fn = () => {
  "use strict";
  var static = 1;
}
```

### Output: error
```txt
Syntax error: Forbidden identifier `static`
 --> test.js:3:7
  |
3 |   var static = 1;
  |       ^^^^^^ `static` is not allowed as an identifier in this context
```
