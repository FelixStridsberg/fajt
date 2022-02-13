### Source
```js
class Test extends Test1 {
    get a() {
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
