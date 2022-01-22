### Source
```js parse:stmt
var { * } = c;
```

### Output: error
```txt
Syntax error: Unexpected token `*`
 --> test.js:1:7
  |
1 | var { * } = c;
  |       ^ Unexpected token, found `*`, expected identifier
```
