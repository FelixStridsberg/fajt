### Source
```js parse:expr
{ a: 'b', [c]: 'd' }
```

### Output: minified
```js
{a:'b',[c]:'d'}
```

### Output: ast
```json
{
  "Literal": {
    "span": "0:20",
    "literal": {
      "Object": {
        "props": [
          {
            "Named": {
              "span": "2:8",
              "name": {
                "Ident": {
                  "span": "2:3",
                  "name": "a"
                }
              },
              "value": {
                "Literal": {
                  "span": "5:8",
                  "literal": {
                    "String": {
                      "value": "b",
                      "delimiter": "'"
                    }
                  }
                }
              }
            }
          },
          {
            "Named": {
              "span": "10:18",
              "name": {
                "Computed": {
                  "IdentRef": {
                    "span": "11:12",
                    "name": "c"
                  }
                }
              },
              "value": {
                "Literal": {
                  "span": "15:18",
                  "literal": {
                    "String": {
                      "value": "d",
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
