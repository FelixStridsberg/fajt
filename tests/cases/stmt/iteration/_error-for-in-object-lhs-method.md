### Source
```js
for ({a(){}} in b);
```

### Output: error
```txt
Syntax error: Unexpected token `(`
 --> test.js:1:8
  |
1 | for ({a(){}} in b);
  |        ^ Unexpected token, found `(`, expected `,`
```
