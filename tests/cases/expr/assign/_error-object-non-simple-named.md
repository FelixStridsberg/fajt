### Source
```js parse:expr
{a: b + 1} = c;
```

### Output: error
```txt
Syntax error: Unexpected token `+`
 --> test.js:1:7
  |
1 | {a: b + 1} = c;
  |       ^ Unexpected token, found `+`, expected `,`
```
