### Source
```js
class Test extends Test1 {
    method1() {
        super();
    }
}
```

### Output: error
```txt
Syntax error: super() now allowed here
 --> test.js:3:9
  |
3 |         super();
  |         ^^^^^^^ 
```
