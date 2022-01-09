### Source
```js parse:stmt
var * = c;
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Punct": "Star"
    },
    "first_on_line": false,
    "span": "4:5"
  }
}
```
