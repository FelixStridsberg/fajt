### Source
```js parse:expr
{ ...a, ...b }
```

### Output: minified
```js
{...a,...b}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:14",
    "literal": {
      "Object": {
        "props": [
          {
            "Spread": {
              "IdentRef": {
                "span": "5:6",
                "name": "a"
              }
            }
          },
          {
            "Spread": {
              "IdentRef": {
                "span": "11:12",
                "name": "b"
              }
            }
          }
        ]
      }
    }
  }
}
```
