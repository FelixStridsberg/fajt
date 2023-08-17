### Source
```js
for(var x="HELLO" in [1,2,3]) 0
```

### Output: error
```txt
Syntax error: Initializers are not allowed in this context
 --> test.js:1:5
  |
1 | for(var x="HELLO" in [1,2,3]) 0
  |     ^^^^^^^^^^^^^ 
```
