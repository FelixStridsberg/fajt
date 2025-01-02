### Source
```js parse:expr
{ a = "b" in c } = d
```

### Output: minified
```js
{a="b"in c}=d
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:20",
    "operator": "Assign",
    "left": {
      "AssignmentPattern": {
        "Object": {
          "span": "0:16",
          "props": [
            {
              "Single": {
                "span": "2:14",
                "ident": {
                  "span": "2:3",
                  "name": "a"
                },
                "initializer": {
                  "Binary": {
                    "span": "6:14",
                    "operator": "In",
                    "left": {
                      "Literal": {
                        "span": "6:9",
                        "literal": {
                          "String": {
                            "value": "b",
                            "delimiter": "\""
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
              }
            }
          ],
          "rest": null
        }
      }
    },
    "right": {
      "IdentRef": {
        "span": "19:20",
        "name": "d"
      }
    }
  }
}
```
