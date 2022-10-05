### Source
```js parse:expr
{
    set a(b, c) {}
}
```

### Output: error
```txt
Syntax error: Setter must have exactly one parameter
 --> test.js:2:10
  |
2 |     set a(b, c) {}
  |          ^^^^^^ 
```
