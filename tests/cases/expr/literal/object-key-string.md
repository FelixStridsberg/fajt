### Source
```js parse:expr
{ 'a': b }
```

### Output: minified
```js
{'a':b}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:10",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "2:8",
              "name": {
                "String": {
                  "value": "a",
                  "delimiter": "'"
                }
              },
              "value": {
                "IdentRef": {
                  "span": "7:8",
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
