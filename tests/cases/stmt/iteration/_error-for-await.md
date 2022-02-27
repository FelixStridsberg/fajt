### Source
```js parse:stmt
async function fn() {
    for await (;;;) ;
}
```

### Output: error
```txt
Syntax error: Unexpected token `;`
 --> test.js:2:16
  |
2 |     for await (;;;) ;
  |                ^ Unexpected token
```
