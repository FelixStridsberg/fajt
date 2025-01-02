### Source
```js parse:expr
{ a: b().b } = c
```

### Output: minified
```js
{a:b().b}=c
```

### Output: ast
```json
{
  "Assignment": {
    "span": "0:16",
    "operator": "Assign",
    "left": {
      "AssignmentPattern": {
        "Object": {
          "span": "0:12",
          "props": [
            {
              "Named": {
                "span": "2:10",
                "name": {
                  "Ident": {
                    "span": "2:3",
                    "name": "a"
                  }
                },
                "value": {
                  "Member": {
                    "span": "5:10",
                    "object": {
                      "Expr": {
                        "Call": {
                          "span": "5:8",
                          "callee": {
                            "Expr": {
                              "IdentRef": {
                                "span": "5:6",
                                "name": "b"
                              }
                            }
                          },
                          "arguments_span": "6:8",
                          "arguments": []
                        }
                      }
                    },
                    "property": {
                      "Ident": {
                        "span": "9:10",
                        "name": "b"
                      }
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
        "span": "15:16",
        "name": "c"
      }
    }
  }
}
```
