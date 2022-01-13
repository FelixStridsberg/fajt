### Source
```js parse:expr
new.non_existent
```

### Output: ast
```json
{
  "UnexpectedIdent": {
    "span": "4:16",
    "name": "non_existent"
  }
}
```
