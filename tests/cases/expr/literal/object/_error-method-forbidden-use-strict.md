### Source
```js parse:expr
{
    method(a = 1) {
        "use strict";
    }
}
```

### Output: error
```txt
Syntax error: Only name parameters allowed in method with "use strict"
 --> test.js:2:11
  |
2 |     method(a = 1) {
  |           ^^^^^^^ 
```
