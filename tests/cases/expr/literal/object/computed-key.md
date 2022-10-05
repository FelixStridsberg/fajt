### Source
```js parse:expr
{ [a]: 'b' }
```

### Output: minified
```js
{[a]:'b'}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:12",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "2:10",
              "name": {
                "Computed": {
                  "IdentRef": {
                    "span": "3:4",
                    "name": "a"
                  }
                }
              },
              "value": {
                "Literal": {
                  "span": "7:10",
                  "literal": {
                    "String": {
                      "value": "b",
                      "delimiter": "'"
                    }
                  }
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
