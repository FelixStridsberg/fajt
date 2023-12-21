### Source
```js parse:stmt
if (1) function a() {}
```

### Output: error
```txt
Syntax error: Unexpected token `function`
 --> test.js:1:8
  |
1 | if (1) function a() {}
  |        ^^^^^^^^ Unexpected token
```
