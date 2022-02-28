### Source
```js parse:expr
{
    get a(...b) {}
}
```

### Output: error
```txt
Syntax error: Getter must not have any formal parameters
 --> test.js:2:10
  |
2 |     get a(...b) {}
  |          ^^^^^^ 
```
