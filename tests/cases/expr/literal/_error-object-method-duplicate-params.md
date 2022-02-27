### Source
```js parse:expr
{
    method(a, a) { }
}
```

### Output: error
```txt
Syntax error: Found duplicate parameter 'a', duplicates not allowed here
 --> test.js:2:11
  |
2 |     method(a, a) { }
  |           ^^^^^^ 
```
