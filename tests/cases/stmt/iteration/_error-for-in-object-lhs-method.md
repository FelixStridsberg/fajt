### Source
```js
for ({a(){}} in b);
```

### Output: error
```txt
Syntax error: Invalid destructuring assignment target
 --> test.js:1:7
  |
1 | for ({a(){}} in b);
  |       ^^^^^ 
```
