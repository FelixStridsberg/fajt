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
      "AssignmentPattern": {
        "Array": {
          "span": "0:9",
          "elements": [
            {
              "span": "2:7",
              "target": {
                "Expr": {
                  "IdentRef": {
                    "span": "2:3",
                    "name": "a"
                  }
                }
              },
              "initializer": {
                "Literal": {
                  "span": "6:7",
                  "literal": {
                    "Number": {
                      "raw": "1"
                    }
                  }
                }
              }
            }
          ],
          "rest": null
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
