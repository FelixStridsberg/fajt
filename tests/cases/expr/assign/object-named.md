### Source
```js parse:expr
{ a: b } = c
```

### Output: minified
```js
{a:b}=c
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
          "props": [
            {
              "Named": {
                "span": "2:6",
                "name": {
                  "Ident": {
                    "span": "2:3",
                    "name": "a"
                  }
                },
                "value": {
                  "Expr": {
                    "IdentRef": {
                      "span": "5:6",
                      "name": "b"
                    }
                  }
                },
                "initializer": null
              }
            }
          ],
          "rest": null
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "11:12",
        "name": "c"
      }
    }
  }
}
```
