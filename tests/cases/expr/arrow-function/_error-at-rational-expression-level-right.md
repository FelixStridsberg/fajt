### Source
See the [arrow function level problem](../../../../parser/docs/arrow-function-level-problem.md).

```js
(a instanceof () => {})
```

### Output: error
```txt
Syntax error: Arrow function not allowed here
 --> test.js:1:15
  |
1 | (a instanceof () => {})
  |               ^^^^^^^^ 
```
