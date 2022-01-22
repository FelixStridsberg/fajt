### Source
```js parse:stmt
var [ a b ] = c;
```

### Output: error
```txt
Syntax error: Unexpected token `b`
 --> test.js:1:9
  |
1 | var [ a b ] = c;
  |         ^ Unexpected token, found `b`, expected `,`
```
