### Source
```js
for ({a(){}} of b);
```

### Output: error
```txt
Syntax error: Invalid destructuring assignment target
 --> test.js:1:7
  |
1 | for ({a(){}} of b);
  |       ^^^^^ 
```
