### Source
```js
class Test extends Test2 {
    method() {
        const a = async function (b = super.a) {};
    }
}
```

### Output: error
```txt
Syntax error: `super` property access only valid inside methods
 --> test.js:3:39
  |
3 |         const a = async function (b = super.a) {};
  |                                       ^^^^^ 
```
