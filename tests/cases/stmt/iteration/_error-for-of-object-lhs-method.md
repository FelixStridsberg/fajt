### Source
```js
for ({a(){}} of b);
```

### Output: error
```txt
Syntax error: Unexpected token `(`
 --> test.js:1:8
  |
1 | for ({a(){}} of b);
  |        ^ Unexpected token, found `(`, expected `,`
```
