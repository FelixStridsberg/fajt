### Source
```js
class Test extends Test1 {
    constructor() {
        function test(a = super()) {}
    }
}
```

### Output: error
```txt
Syntax error: super() now allowed here
 --> test.js:3:27
  |
3 |         function test(a = super()) {}
  |                           ^^^^^^^ 
```
