### Source
```js parse:expr
class { set setter(a, b) {} }
```

### Output: error
```txt
Syntax error: Setter must have exactly one parameter
 --> test.js:1:19
  |
1 | class { set setter(a, b) {} }
  |                   ^^^^^^ 
```
