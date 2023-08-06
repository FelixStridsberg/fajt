### Source
```js parse:expr
{ ...{ a } } = b
```

### Output: minified
```js
{...{a}}=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:16",
    "operator": "Assign",
    "left": {
      "Literal": {
        "span": "0:12",
        "literal": {
          "Object": {
            "props": [
              {
                "Spread": {
                  "Literal": {
                    "span": "5:10",
                    "literal": {
                      "Object": {
                        "props": [
                          {
                            "IdentRef": {
                              "span": "7:8",
                              "name": "a"
                            }
                          }
                        ]
                      }
                    }
                  }
                }
              }
            ]
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "15:16",
        "name": "b"
      }
    }
  }
}
```
