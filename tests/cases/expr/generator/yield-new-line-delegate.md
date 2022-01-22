### Source
```js parse:expr
function* () {
    yield
    *a
}
```

### Output: error
```txt
Syntax error: Unexpected token `*`
 --> test.js:3:5
  |
3 |     *a
  |     ^ Unexpected token
```
