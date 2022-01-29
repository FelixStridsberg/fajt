### Source
```js
a`template`;
```

### Output: minified
```js
a`template`
```

### Output: ast
```json
{
  "Script": {
    "span": "0:12",
    "directives": [],
    "body": [
      {
        "Expr": {
          "span": "0:12",
          "expr": {
            "TaggedTemplate": {
              "span": "0:11",
              "callee": {
                "IdentRef": {
                  "span": "0:1",
                  "name": "a"
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
