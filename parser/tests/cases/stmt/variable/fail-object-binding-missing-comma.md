### Input
```js parse:stmt
var { a b } = c;
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Identifier": "b"
    },
    "first_on_line": false,
    "span": "8:9"
  }
}
```
