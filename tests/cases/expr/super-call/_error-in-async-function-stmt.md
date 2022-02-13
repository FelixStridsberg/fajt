### Source
```js
class Test extends Test1 {
    constructor() {
        async function test() {
            super();
        }
    }
}
```

### Output: error
```txt
Syntax error: super() now allowed here
 --> test.js:4:13
  |
4 |             super();
  |             ^^^^^^^ 
```
