### Source
```js parse:expr
[ ...[ a, b ] ] = c
```

### Output: minified
```js
[...[a,b]]=c
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:19",
    "operator": "Assign",
    "left": {
      "AssignmentPattern": {
        "Array": {
          "span": "0:15",
          "elements": [],
          "rest": {
            "Literal": {
              "span": "5:13",
              "literal": {
                "Array": {
                  "elements": [
                    {
                      "Expr": {
                        "IdentRef": {
                          "span": "7:8",
                          "name": "a"
                        }
                      }
                    },
                    {
                      "Expr": {
                        "IdentRef": {
                          "span": "10:11",
                          "name": "b"
                        }
                      }
                    }
                  ]
                }
              }
            }
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "18:19",
        "name": "c"
      }
    }
  }
}
```
