### Source
```js parse:expr
[a b]
```

### Output: ast
```json
{
  "UnexpectedToken": {
    "value": {
      "Identifier": "b"
    },
    "first_on_line": false,
    "span": "3:4"
  }
}
```

### Output: error
```txt
  |
2 | [a b]
  |    ^ Unexpected token, found #offending#, expected [Punct(Comma), Punct(BraceClose)]
```
