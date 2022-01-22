### Source
```js parse:stmt
var { ...rest, b } = c;
```

### Output: error
```txt
Syntax error: Rest element must be last element
 --> test.js:1:7
  |
1 | var { ...rest, b } = c;
  |       ^^^^^^^ 
```
