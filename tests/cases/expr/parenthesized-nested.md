### Source
```js parse:expr
((1 + 1) + 1)
```

### Output: minified
```js
((1+1)+1)
```

### Output: ast
```json
{
  "Parenthesized": {
    "span": "0:13",
    "expression": {
      "Binary": {
        "span": "1:12",
        "operator": "Plus",
        "left": {
          "Parenthesized": {
            "span": "1:8",
            "expression": {
              "Binary": {
                "span": "2:7",
                "operator": "Plus",
                "left": {
                  "Literal": {
                    "span": "2:3",
                    "literal": {
                      "Number": {
                        "Integer": [
                          1,
                          "Decimal"
                        ]
                      }
                    }
                  }
                },
                "right": {
                  "Literal": {
                    "span": "6:7",
                    "literal": {
                      "Number": {
                        "Integer": [
                          1,
                          "Decimal"
                        ]
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "right": {
          "Literal": {
            "span": "11:12",
            "literal": {
              "Number": {
                "Integer": [
                  1,
                  "Decimal"
                ]
              }
            }
          }
        }
      }
    }
  }
}
```
