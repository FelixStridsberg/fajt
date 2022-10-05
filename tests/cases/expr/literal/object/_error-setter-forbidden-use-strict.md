### Source
```js parse:expr
{
    set a(b = 1) {
        "use strict";
    }
}
```

### Output: error
```txt
Syntax error: Only name parameters allowed in method with "use strict"
 --> test.js:2:10
  |
2 |     set a(b = 1) {
  |          ^^^^^^^ 
```
