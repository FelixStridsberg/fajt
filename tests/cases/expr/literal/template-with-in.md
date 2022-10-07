### Source
```js
`a${"b" in c}d`;
```

### Output: minified
```js
`a${"b"in c}d`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:16",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:16",
          "expr": {
            "Literal": {
              "span": "0:15",
              "literal": {
                "Template": {
                  "parts": [
                    {
                      "String": "a"
                    },
                    {
                      "Expr": {
                        "Binary": {
                          "span": "4:12",
                          "operator": "In",
                          "left": {
                            "Literal": {
                              "span": "4:7",
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
                              "span": "11:12",
                              "name": "c"
                            }
                          }
                        }
                      }
                    },
                    {
                      "String": "d"
                    }
                  ]
                }
              }
            }
          }
        }
      }
    ]
  }
}
```
