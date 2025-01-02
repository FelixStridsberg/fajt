### Source
```js parse:expr
[ a ] = b
```

### Output: minified
```js
[a]=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:9",
    "operator": "Assign",
    "left": {
      "Expr": {
        "AssignmentPattern": {
          "Array": {
            "span": "0:5",
            "elements": [
              {
                "span": "2:3",
                "target": {
                  "IdentRef": {
                    "span": "2:3",
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
    },
    "right": {
      "IdentRef": {
        "span": "8:9",
        "name": "b"
      }
    }
  }
}
```
