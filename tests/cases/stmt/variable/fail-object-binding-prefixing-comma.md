### Source
```js parse:stmt
var { , a, b } = c;
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Punct": "Comma"
    },
    "first_on_line": false,
    "span": "6:7"
  }
}
```