### Source
```js parse:expr
{ 1234: b }
```

### Output: minified
```js
{1234:b}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:11",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "2:9",
              "name": {
                "Number": {
                  "Integer": [
                    1234,
                    "Decimal"
                  ]
                }
              },
              "value": {
                "IdentRef": {
                  "span": "8:9",
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
