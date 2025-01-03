### Source
```js parse:expr
`foo${11;abc`
```

### Output: error
```txt
Syntax error: Expected '}' in template expression
 --> test.js:1:9
  |
1 | `foo${11;abc`
  |         ^ 
```
