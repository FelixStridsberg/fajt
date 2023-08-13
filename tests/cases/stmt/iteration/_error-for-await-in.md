### Source
```js parse:stmt
async function fn() {
    for await (a in b) ;
}
```

### Output: error
```txt
Syntax error: 'for await' loops must be used with 'of'
 --> test.js:2:5
  |
2 |     for await (a in b) ;
  |     ^^^^^^^^^^^^ 
```
