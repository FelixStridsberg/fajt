### Source
```js parse:stmt
for ( in b) ;
```

### Output: error
```txt
Syntax error: Forbidden identifier `in`
 --> test.js:1:7
  |
1 | for ( in b) ;
  |       ^^ `in` is not allowed as an identifier in this context
```
