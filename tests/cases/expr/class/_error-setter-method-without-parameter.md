### Source
```js parse:expr
class { set setter() {} }
```

### Output: error
```txt
Syntax error: Setter must have exactly one parameter
 --> test.js:1:19
  |
1 | class { set setter() {} }
  |                   ^^ 
```
