### Source
```js parse:stmt
function fn() {
  for await (a of b) ;
}
```

### Output: error
```txt
Syntax error: Unexpected token `await`
 --> test.js:2:7
  |
2 |   for await (a of b) ;
  |       ^^^^^ Unexpected token, found `await`, expected `(`
```
