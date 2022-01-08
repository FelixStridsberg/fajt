### Input
```js parse:stmt
var { ...rest, b } = c;
```

### Output: ast
```json
{
  "SyntaxError": [
    "Rest element must be last element",
    "9:13"
  ]
}
```
