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
            "AssignmentPattern": {
              "Array": {
                "span": "5:13",
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
                  },
                  null,
                  {
                    "span": "10:11",
                    "target": {
                      "IdentRef": {
                        "span": "10:11",
                        "name": "b"
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
