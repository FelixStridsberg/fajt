### Source
```js parse:expr
{ {a}: 'a' }
```

### Output: error
```txt
Syntax error: Unexpected token `{`
 --> test.js:1:3
  |
1 | { {a}: 'a' }
  |   ^ Unexpected token, found `{`, expected identifier
```
