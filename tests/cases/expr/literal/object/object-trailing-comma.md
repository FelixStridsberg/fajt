### Source
```js parse:expr check-format:no
{ a, }
```

### Output: minified
```js
{a}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:6",
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
