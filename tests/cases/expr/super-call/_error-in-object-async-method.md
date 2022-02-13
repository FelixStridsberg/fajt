### Source
```js
class Test extends Test1 {
    constructor() {
        const a = {
            async method() {
                super();
            }
        };
    }
}
```

### Output: error
```txt
Syntax error: super() now allowed here
 --> test.js:5:17
  |
5 |                 super();
  |                 ^^^^^^^ 
```
