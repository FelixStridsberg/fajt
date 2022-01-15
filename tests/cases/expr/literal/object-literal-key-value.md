### Source
```js parse:expr
{ a: b }
```

### Output: minified
```js
{a:b}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:8",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "2:6",
              "name": {
                "Ident": {
                  "span": "2:3",
                  "name": "a"
                }
              },
              "value": {
                "IdentRef": {
                  "span": "5:6",
                  "name": "b"
                }
              }
            }
          }
        ]
      }
    }
  }
}
```
