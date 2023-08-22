### Source
See the [arrow function level problem](../../../../parser/docs/arrow-function-level-problem.md).

```js
(typeof () => {})
```

### Output: error
```txt
Syntax error: Arrow function not allowed here
 --> test.js:1:9
  |
1 | (typeof () => {})
  |         ^^^^^^^^ 
```
