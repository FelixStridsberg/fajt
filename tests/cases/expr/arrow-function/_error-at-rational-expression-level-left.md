### Source
See the [arrow function level problem](../../../../parser/docs/arrow-function-level-problem.md).

```js
(() => {} instanceof a)
```

### Output: error
```txt
Syntax error: Unexpected token `instanceof`
 --> test.js:1:11
  |
1 | (() => {} instanceof a)
  |           ^^^^^^^^^^ Unexpected token, found `instanceof`, expected `)`
```
