### Source
```js parse:stmt
async function fn() { var await = 1 }
```

### Output: error
```txt
Syntax error: Forbidden identifier `await`
 --> test.js:1:26
  |
1 | async function fn() { var await = 1 }
  |                           ^^^^^ `await` is not allowed as an identifier in this context
```
