### Source
```js check-format:no
a.b
`template`;
```

### Output: minified
```js
a.b`template`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:15",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:15",
          "expr": {
            "TaggedTemplate": {
              "span": "0:14",
              "callee": {
                "Member": {
                  "span": "0:3",
                  "object": {
                    "Expr": {
                      "IdentRef": {
                        "span": "0:1",
                        "name": "a"
                      }
                    }
                  },
                  "property": {
                    "Ident": {
                      "span": "2:3",
                      "name": "b"
                    }
                  }
                }
              },
              "template": {
                "parts": [
                  {
                    "String": "template"
                  }
                ]
              }
            }
          }
        }
      }
    ]
  }
}
```
