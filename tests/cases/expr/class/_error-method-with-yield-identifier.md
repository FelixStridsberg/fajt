### Source
```js parse:expr
class {
  method1() {
    yield
  }
}
```

### Output: error
```txt
Syntax error: Forbidden identifier `yield`
 --> test.js:3:5
  |
3 |     yield
  |     ^^^^^ `yield` is not allowed as an identifier in this context
```
