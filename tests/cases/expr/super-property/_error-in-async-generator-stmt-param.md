### Source
```js
class Test extends Test2 {
    method() {
        async function* fn(a = super.a) {}
    }
}
```

### Output: error
```txt
Syntax error: `super` property access only valid inside methods
 --> test.js:3:32
  |
3 |         async function* fn(a = super.a) {}
  |                                ^^^^^ 
```
