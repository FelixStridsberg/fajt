### Source
```js parse:expr
{a(){}} = b;
```

### Output: error
```txt
Syntax error: Invalid destructuring assignment target
 --> test.js:1:2
  |
1 | {a(){}} = b;
  |  ^^^^^ 
```
