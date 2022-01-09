### Source
```js parse:expr
[ ...a ]
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:8",
    "literal": {
      "Array": {
        "elements": [
          {
            "Spread": {
              "IdentRef": {
                "span": "5:6",
                "name": "a"
              }
            }
          }
        ]
      }
    }
  }
}
```
