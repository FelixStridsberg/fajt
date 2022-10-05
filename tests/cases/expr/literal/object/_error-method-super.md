### Source
```js parse:expr
{
    method() {
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
