### Source
```js parse:expr
{ [a]: b } = c
```

### Output: minified
```js
{[a]:b}=c
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:14",
    "operator": "Assign",
    "left": {
      "Expr": {
        "AssignmentPattern": {
          "Object": {
            "span": "0:10",
            "props": [
              {
                "Named": {
                  "span": "2:8",
                  "name": {
                    "Computed": {
                      "IdentRef": {
                        "span": "3:4",
                        "name": "a"
                      }
                    }
                  },
                  "value": {
                    "IdentRef": {
                      "span": "7:8",
                      "name": "b"
                    }
                  },
                  "initializer": null
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
        "span": "13:14",
        "name": "c"
      }
    }
  }
}
```
