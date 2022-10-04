### Source
```js parse:expr
{
    static get a(b) {}
}
```

### Output: error
```txt
Syntax error: Unexpected token `get`
 --> test.js:2:12
  |
2 |     static get a(b) {}
  |            ^^^ Unexpected token, found `get`, expected `(`
```
