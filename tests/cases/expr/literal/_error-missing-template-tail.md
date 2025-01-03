### Source
```js parse:expr
`foo${11`
```

### Output: error
```txt
Lexer error 'Unexpected end of stream'
 --> test.js
  |
1 | `foo${11`
  | ^ 
```
