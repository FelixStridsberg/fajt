### Source
```js
class Test extends Test1 {
    constructor() {
        const a = function*() {
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
