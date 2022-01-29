### Source
```js
`start ${"sub"} end`;
```

### Output: minified
```js
`start ${"sub"} end`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:21",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:21",
          "expr": {
            "Literal": {
              "span": "0:20",
              "literal": {
                "Template": {
                  "parts": [
                    {
                      "String": "start "
                    },
                    {
                      "Expr": {
                        "Literal": {
                          "span": "9:14",
                          "literal": {
                            "String": {
                              "value": "sub",
                              "delimiter": "\""
                            }
                          }
                        }
                      }
                    },
                    {
                      "String": " end"
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
