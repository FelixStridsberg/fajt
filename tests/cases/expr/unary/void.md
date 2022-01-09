### Source
```js parse:expr
void a
```

### Output: ast
```json
{
  "Unary": {
    "span": "0:6",
    "operator": "Void",
    "argument": {
      "IdentRef": {
        "span": "5:6",
        "name": "a"
      }
    }
  }
}
```
