### Source
```js parse:expr
{ a }
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:5",
    "literal": {
      "Object": {
        "props": [
          {
            "IdentRef": {
              "span": "2:3",
              "name": "a"
            }
          }
        ]
      }
    }
  }
}
```
