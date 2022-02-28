### Source
```js parse:expr
{
    set a() {}
}
```

### Output: error
```txt
Syntax error: Setter must have exactly one parameter
 --> test.js:2:10
  |
2 |     set a() {}
  |          ^^ 
```
