### Source
Class definitions are always strict mode.

```js
var cls = class {
  method1() {
    var implements = 1;
  }
}
```

### Output: error
```txt
Syntax error: Forbidden identifier `implements`
 --> test.js:3:9
  |
3 |     var implements = 1;
  |         ^^^^^^^^^^ `implements` is not allowed as an identifier in this context
```
