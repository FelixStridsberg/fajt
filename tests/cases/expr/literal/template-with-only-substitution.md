### Source
```js
`${a + a}`;
```

### Output: minified
```js
`${a+a}`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:11",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:11",
          "expr": {
            "Literal": {
              "span": "0:10",
              "literal": {
                "Template": {
                  "parts": [
                    {
                      "Expr": {
                        "Binary": {
                          "span": "3:8",
                          "operator": "Plus",
                          "left": {
                            "IdentRef": {
                              "span": "3:4",
                              "name": "a"
                            }
                          },
                          "right": {
                            "IdentRef": {
                              "span": "7:8",
                              "name": "a"
                            }
                          }
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
    ]
  }
}
```
