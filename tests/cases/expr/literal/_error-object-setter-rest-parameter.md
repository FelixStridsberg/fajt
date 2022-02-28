### Source
```js parse:expr
{
    set a(...b) {}
}
```

### Output: error
```txt
Syntax error: Setter function parameter must not be a rest parameter
 --> test.js:2:10
  |
2 |     set a(...b) {}
  |          ^^^^^^ 
```
