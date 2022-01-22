### Source
```js parse:expr
{ [a, b, c]: 'a' }
```

### Output: error
```txt
Syntax error: Unexpected token `,`
 --> test.js:1:5
  |
1 | { [a, b, c]: 'a' }
  |     ^ Unexpected token, found `,`, expected `]`
```
