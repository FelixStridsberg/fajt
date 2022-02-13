### Source
```js
class Test extends Test2 {
    method() {
        function fn(a = super.a) {}
    }
}
```

### Output: error
```txt
Syntax error: `super` property access only valid inside methods
 --> test.js:3:25
  |
3 |         function fn(a = super.a) {}
  |                         ^^^^^ 
```
