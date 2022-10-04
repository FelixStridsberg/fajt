### Source
```js parse:expr
{
    static a(b) {}
}
```

### Output: error
```txt
Syntax error: Unexpected token `a`
 --> test.js:2:12
  |
2 |     static a(b) {}
  |            ^ Unexpected token, found `a`, expected `(`
```
