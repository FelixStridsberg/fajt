### Source
```js
function fn() {
  "use strict";
  eval = true;
}
```

### Output: error
```txt
Syntax error: Unexpected `eval` or `arguments` in strict mode
 --> test.js:3:3
  |
3 |   eval = true;
  |   ^^^^ 
```
