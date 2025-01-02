### Source
```js parse:expr
[ a = 1 ] = b
```

### Output: minified
```js
[a=1]=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:13",
    "operator": "Assign",
    "left": {
      "Expr": {
        "AssignmentPattern": {
          "Array": {
            "span": "0:9",
            "elements": [
              {
                "span": "2:7",
                "target": {
                  "IdentRef": {
                    "span": "2:3",
                    "name": "a"
                  }
                },
                "initializer": {
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
            ],
            "rest": null
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "12:13",
        "name": "b"
      }
    }
  }
}
```
