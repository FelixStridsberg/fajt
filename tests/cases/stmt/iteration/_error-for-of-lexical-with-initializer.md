### Source
```js
for(const x = 1 of [1,2,3]) 0
```

### Output: error
```txt
Syntax error: Unexpected token `=`
 --> test.js:1:13
  |
1 | for(const x = 1 of [1,2,3]) 0
  |             ^ Unexpected token
```
