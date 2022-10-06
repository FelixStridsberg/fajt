### Source
```js parse:expr
123__456.7
```

### Output: error
```txt
Syntax error: number cannot contain multiple adjacent underscores
 --> test.js:1:1
  |
1 | 123__456.7
  | ^^^^^^^^ 
```
