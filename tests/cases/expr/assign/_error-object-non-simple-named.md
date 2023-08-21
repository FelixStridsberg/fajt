### Source
```js parse:expr
{a: b + 1} = c;
```

### Output: error
```txt
Syntax error: Invalid destructuring assignment target
 --> test.js:1:5
  |
1 | {a: b + 1} = c;
  |     ^^^^^ 
```
