### Source
```js parse:stmt
async function fn() {
    for await (a in b) ;
}
```

### Output: error
```txt
Syntax error: Unexpected token `in`
 --> test.js:2:18
  |
2 |     for await (a in b) ;
  |                  ^^ Unexpected token
```
