### Source
```js check-format:no
a b
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Identifier": "b"
    },
    "first_on_line": false,
    "span": "2:3"
  }
}
```