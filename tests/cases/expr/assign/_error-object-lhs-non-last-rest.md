### Source
```js parse:expr
{...rest, a} = b;
```

### Output: error
```txt
Syntax error: Rest element must be last element
 --> test.js:1:5
  |
1 | {...rest, a} = b;
  |     ^^^^ 
```
