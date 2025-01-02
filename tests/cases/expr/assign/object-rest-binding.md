### Source
```js parse:expr
{ ...a } = b
```

### Output: minified
```js
{...a}=b
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:12",
    "operator": "Assign",
    "left": {
      "AssignmentPattern": {
        "Object": {
          "span": "0:8",
          "props": [],
          "rest": {
            "Expr": {
              "IdentRef": {
                "span": "5:6",
                "name": "a"
              }
            }
          }
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "11:12",
        "name": "b"
      }
    }
  }
}
```
