### Source
```js parse:expr
{ ...[ a ] } = b
```

### Output: minified
```js
{...[a]}=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:16",
    "operator": "Assign",
    "left": {
      "Expr": {
        "AssignmentPattern": {
          "Object": {
            "span": "0:12",
            "props": [],
            "rest": {
              "AssignmentPattern": {
                "Array": {
                  "span": "5:10",
                  "elements": [
                    {
                      "span": "7:8",
                      "target": {
                        "IdentRef": {
                          "span": "7:8",
                          "name": "a"
                        }
                      },
                      "initializer": null
                    }
                  ],
                  "rest": null
                }
              }
            }
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
