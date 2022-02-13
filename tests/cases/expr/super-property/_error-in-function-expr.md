### Source
```js
class Test extends Test2 {
    method() {
        const a = function () {
            super.a;
        };
    }
}
```

### Output: error
```txt
Syntax error: `super` property access only valid inside methods
 --> test.js:4:13
  |
4 |             super.a;
  |             ^^^^^ 
```
