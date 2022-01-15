### Source
```js parse:expr
{ [a, b, c]: 'a' }
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Punct": "Comma"
    },
    "first_on_line": false,
    "span": "4:5"
  }
}
```
