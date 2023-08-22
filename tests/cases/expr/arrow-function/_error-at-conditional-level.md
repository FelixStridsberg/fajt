### Source
See the [arrow function level problem](../../../../parser/docs/arrow-function-level-problem.md).

```js
(() => {} ? 1 : 2)
```

### Output: error
```txt
Syntax error: Unexpected token `?`
 --> test.js:1:11
  |
1 | (() => {} ? 1 : 2)
  |           ^ Unexpected token, found `?`, expected `)`
```
