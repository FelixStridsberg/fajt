### Source
```js parse:expr
{ get: 1, set: 2 }
```

### Output: minified
```js
{get:1,set:2}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:18",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "2:8",
              "name": {
                "Ident": {
                  "span": "2:5",
                  "name": "get"
                }
              },
              "value": {
                "Literal": {
                  "span": "7:8",
                  "literal": {
                    "Number": {
                      "raw": "1"
                    }
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "10:16",
              "name": {
                "Ident": {
                  "span": "10:13",
                  "name": "set"
                }
              },
              "value": {
                "Literal": {
                  "span": "15:16",
                  "literal": {
                    "Number": {
                      "raw": "2"
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
