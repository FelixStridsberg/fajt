### Source
```js parse:expr
class { get getter(a) {} }
```

### Output: error
```txt
Syntax error: Getter must not have any formal parameters
 --> test.js:1:19
  |
1 | class { get getter(a) {} }
  |                   ^^^ 
```
