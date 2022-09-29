### Source
```js parse:expr
({ a = b } = c)
```

### Output: minified
```js
({a=b}=c)
```

### Output: ast
```json
{
  "Parenthesized": {
    "span": "0:15",
    "expression": {
      "Assignment": {
        "span": "1:14",
        "operator": "Assign",
        "left": {
          "Object": {
            "span": "1:10",
            "props": [
              {
                "Single": {
                  "span": "3:8",
                  "ident": {
                    "span": "3:4",
                    "name": "a"
                  },
                  "initializer": {
                    "IdentRef": {
                      "span": "7:8",
                      "name": "b"
                    }
                  }
                }
              }
            ],
            "rest": null
          }
        },
        "right": {
          "IdentRef": {
            "span": "13:14",
            "name": "c"
          }
        }
      }
    }
  }
}
```
